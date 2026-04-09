use crate::{
    core::{
        aggregator::process_values,
        config::Config,
        processor_result::{ProcessorResult, ValidatedPayload},
    },
    network::Environment,
    protocol::{payload::Payload, PayloadDecoder},
    Bytes, RedStoneConfig,
};
#[cfg(feature = "bench")]
use crate::{network::error::Error, Crypto};

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

#[cfg(feature = "bench")]
pub fn decode_payload<C: Crypto>(
    crypto: &mut C,
    payload_bytes: impl Into<Bytes>,
) -> Result<Payload, Error> {
    let mut bytes = payload_bytes.into();

    PayloadDecoder::new(crypto).make_payload(&mut bytes.0)
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

        T::Environment::print(|| format!("{:?}", payload));

        make_processor_result::<T::Environment>(self.config(), payload)
    }
}

fn make_processor_result<Env: Environment>(config: &Config, payload: Payload) -> ProcessorResult {
    let timestamp = payload.get_validated_timestamp(config)?;

    let values = process_values(config, payload.data_packages)?;

    Env::print(|| format!("{:?} {:?}", timestamp, values));

    Ok(ValidatedPayload { values, timestamp })
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use redstone_utils::hex::make_hex_value_from_string;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use crate::{
        core::{
            config::Config,
            processor::make_processor_result,
            test_helpers::{
                BTC, ETH, TEST_BLOCK_TIMESTAMP, TEST_SIGNER_ADDRESS_1, TEST_SIGNER_ADDRESS_2,
            },
        },
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
        )
        .unwrap();

        assert_eq!(result.timestamp, (TEST_BLOCK_TIMESTAMP + 400).into());

        let ok: Vec<_> = result
            .values
            .iter()
            .filter_map(|r| r.result.as_ref().ok().map(|v| (r.feed, *v)))
            .collect();

        assert_eq!(ok.len(), 2);
        assert_eq!(ok[0], (make_hex_value_from_string(ETH), 12u128.into()));
        assert_eq!(ok[1], (make_hex_value_from_string(BTC), 31u128.into()));
    }

    #[test]
    fn test_make_processor_one_bad_feed() {
        let data_packages = vec![
            DataPackage::test_single_data_point(
                ETH,
                11,
                TEST_SIGNER_ADDRESS_1,
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
        )
        .unwrap();

        assert_eq!(result.timestamp, (TEST_BLOCK_TIMESTAMP + 400).into());

        let eth_feed = make_hex_value_from_string(ETH);
        let btc_feed = make_hex_value_from_string(BTC);

        let eth_result = result.values.iter().find(|r| r.feed == eth_feed).unwrap();
        assert!(eth_result.result.is_err());

        let btc_result = result.values.iter().find(|r| r.feed == btc_feed).unwrap();
        assert_eq!(*btc_result.result.as_ref().unwrap(), 31u128.into());
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
        )
        .unwrap();

        assert_eq!(result.timestamp, (TEST_BLOCK_TIMESTAMP + 5).into());

        let ok: Vec<_> = result
            .values
            .iter()
            .filter_map(|r| r.result.as_ref().ok().map(|v| (r.feed, *v)))
            .collect();

        assert_eq!(ok.len(), 2);
        assert_eq!(ok[0], (make_hex_value_from_string(ETH), 11u128.into()));
        assert_eq!(ok[1], (make_hex_value_from_string(BTC), 31u128.into()));
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
                TEST_SIGNER_ADDRESS_2,
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
        let data_packages = vec![
            DataPackage::test_multi_data_point(
                vec![(ETH, 10), (BTC, 31), (BTC, 33)],
                TEST_SIGNER_ADDRESS_2,
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
}
