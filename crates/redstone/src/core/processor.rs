use crate::{
    core::{
        aggregator::aggregate_values,
        config::Config,
        processor_result::{ProcessorResult, ValidatedPayload},
    },
    network::Environment,
    protocol::{payload::Payload, PayloadDecoder},
    Bytes, RedStoneConfig,
};

/// The main processor of the RedStone payload.
///
///
/// # Arguments
///
/// * `config` - Something that implements `RedStoneConfig`. Provides environment and crypto operations.
/// * `payload_bytes` - Network-specific byte-list of the payload to be processed.
///
/// # Returns
///
/// * Returns a `ProcessorResult` in case of successful payload processing. Will panic in case of bad input.
pub fn process_payload(
    config: &mut impl RedStoneConfig,
    payload_bytes: impl Into<Bytes>,
) -> ProcessorResult {
    config.process_payload(payload_bytes)
}

/// Internal trait, designed to extend `RedStoneConfig` implementations with ability to process payloads.
trait RedStonePayloadProcessor {
    /// Process given payload, panics in case of badly formed payload.
    ///
    /// # Arguments
    /// * `payload_bytes` - Anything that can be transformed into `Bytes`
    ///
    /// # Returns
    ///
    /// * Returns a `ProcessorResult` in case of successful payload processing. Will panic in case of bad input.
    fn process_payload(&mut self, payload_bytes: impl Into<Bytes>) -> ProcessorResult;
}

impl<T: RedStoneConfig> RedStonePayloadProcessor for T {
    fn process_payload(&mut self, payload_bytes: impl Into<Bytes>) -> ProcessorResult {
        let mut bytes = payload_bytes.into();
        let payload = PayloadDecoder::new(self.crypto_mut()).make_payload(&mut bytes.0)?;

        // T::Environment::print(|| format!("{:?}", payload));

        make_processor_result::<T::Environment>(self.config(), payload)
    }
}

fn make_processor_result<Env: Environment>(config: &Config, payload: Payload) -> ProcessorResult {
    let timestamp = payload.get_validated_timestamp(config)?;

    let values = aggregate_values(payload.data_packages, config)?;

    // Env::print(|| format!("{:?} {:?}", timestamp, values));

    Ok(ValidatedPayload { values, timestamp })
}

#[cfg(feature = "helpers")]
#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use crate::{
        core::{
            config::Config,
            processor::make_processor_result,
            processor_result::ValidatedPayload,
            test_helpers::{
                BTC, ETH, TEST_BLOCK_TIMESTAMP, TEST_SIGNER_ADDRESS_1, TEST_SIGNER_ADDRESS_2,
            },
        },
        helpers::iter_into::IterInto,
        network::{error::Error, StdEnv},
        protocol::{data_package::DataPackage, payload::Payload},
    };

    #[test]
    fn test_make_processor_result_for_single_datapoint() {
        let data_packages = vec![
            DataPackage::test_single_data_point(
                ETH,
                11,
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP + 400).into(),
            ),
            DataPackage::test_single_data_point(
                ETH,
                13,
                TEST_SIGNER_ADDRESS_2,
                (TEST_BLOCK_TIMESTAMP + 400).into(),
            ),
            DataPackage::test_single_data_point(
                BTC,
                32,
                TEST_SIGNER_ADDRESS_2,
                (TEST_BLOCK_TIMESTAMP + 400).into(),
            ),
            DataPackage::test_single_data_point(
                BTC,
                31,
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP + 400).into(),
            ),
        ];

        let result = make_processor_result::<StdEnv>(
            &Config::test_with_signer_count_threshold_or_default(None),
            Payload { data_packages },
        );

        assert_eq!(
            result,
            Ok(ValidatedPayload {
                timestamp: (TEST_BLOCK_TIMESTAMP + 400).into(),
                values: vec![12u8, 31].iter_into()
            })
        );
    }

    #[test]
    fn test_make_processor_result_for_multi_datapoint() {
        let data_packages = vec![
            DataPackage::test_multi_data_point(
                vec![(ETH, 10), (BTC, 31)],
                TEST_SIGNER_ADDRESS_2,
                (TEST_BLOCK_TIMESTAMP + 5).into(),
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 13), (BTC, 32)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP + 5).into(),
            ),
        ];

        let result = make_processor_result::<StdEnv>(
            &Config::test_with_signer_count_threshold_or_default(None),
            Payload { data_packages },
        );

        assert_eq!(
            result,
            Ok(ValidatedPayload {
                timestamp: (TEST_BLOCK_TIMESTAMP + 5).into(),
                values: vec![11u8, 31].iter_into()
            })
        );
    }

    #[test]
    fn test_make_processor_result_for_multi_datapoint_package_repetition() {
        let data_packages = vec![
            DataPackage::test_multi_data_point(
                vec![(BTC, 30), (ETH, 11)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 10), (BTC, 31)],
                TEST_SIGNER_ADDRESS_2,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
            DataPackage::test_multi_data_point(
                vec![(BTC, 34), (ETH, 12)],
                TEST_SIGNER_ADDRESS_2, // REPETITION OF A SIGNER
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 13), (BTC, 32)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
        ];

        let result = make_processor_result::<StdEnv>(
            &Config::test_with_signer_count_threshold_or_default(None),
            Payload { data_packages },
        );

        assert_eq!(
            result,
            Err(Error::ReoccurringFeedId(BTC.as_bytes().to_vec().into()))
        );
    }

    #[test]
    fn test_make_processor_result_for_multi_datapoint_with_datapoint_repetition() {
        // given
        let data_packages = vec![
            DataPackage::test_multi_data_point(
                vec![(ETH, 10), (BTC, 31), (BTC, 33)], // REPETITION IN DATAPOINTS HERE.
                TEST_SIGNER_ADDRESS_2,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 13), (BTC, 32)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
        ];

        // when, then
        let result = make_processor_result::<StdEnv>(
            &Config::test_with_signer_count_threshold_or_default(None),
            Payload { data_packages },
        );

        assert_eq!(
            result,
            Err(Error::ReoccurringFeedId(BTC.as_bytes().to_vec().into()))
        );
    }
}
