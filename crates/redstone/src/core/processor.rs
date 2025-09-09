use crate::{
    core::{
        aggregator::process_values,
        config::Config,
        processor_result::{ProcessorResult, ValidatedPayload},
    },
    network::Environment,
    protocol::{payload::Payload, PayloadDecoder},
    u256::U256,
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

        make_processor_result::<T::Environment, T::U256>(self.config(), payload)
    }
}

fn make_processor_result<Env: Environment, U: U256>(
    config: &Config,
    payload: Payload,
) -> ProcessorResult {
    let timestamp = payload.get_validated_timestamp(config)?;

    let values = process_values::<U>(config, payload.data_packages)?;

    Env::print(|| format!("{:?} {:?}", timestamp, values));

    Ok(ValidatedPayload { values, timestamp })
}

#[cfg(test)]
#[cfg(feature = "alloy")]
mod tests {
    use alloy_primitives::U256;
    use redstone_utils::hex::make_hex_value_from_string;
    use redstone_utils::iter_into::IterInto;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use crate::core::aggregator::FeedValue;
    use crate::{
        core::{
            config::Config,
            processor::make_processor_result,
            processor_result::ValidatedPayload,
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

        let result = make_processor_result::<StdEnv, U256>(
            &Config::test_with_signer_count_threshold_or_default(None),
            Payload { data_packages },
        );

        assert_eq!(
            result,
            Ok(ValidatedPayload {
                timestamp: (TEST_BLOCK_TIMESTAMP + 400).into(),
                values: vec![
                    FeedValue {
                        value: 12u32.into(),
                        feed: make_hex_value_from_string(ETH)
                    },
                    FeedValue {
                        value: 31u32.into(),
                        feed: make_hex_value_from_string(BTC)
                    }
                ]
                .iter_into()
            })
        );
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

        let result = make_processor_result::<StdEnv, U256>(
            &Config::test_with_signer_count_threshold_or_default(None),
            Payload { data_packages },
        );

        assert_eq!(
            result,
            Ok(ValidatedPayload {
                timestamp: (TEST_BLOCK_TIMESTAMP + 400).into(),
                values: vec![FeedValue {
                    value: 31u32.into(),
                    feed: make_hex_value_from_string(BTC)
                }]
                .iter_into()
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

        let result = make_processor_result::<StdEnv, U256>(
            &Config::test_with_signer_count_threshold_or_default(None),
            Payload { data_packages },
        );

        assert_eq!(
            result,
            Ok(ValidatedPayload {
                timestamp: (TEST_BLOCK_TIMESTAMP + 5).into(),
                values: vec![
                    FeedValue {
                        value: 11u32.into(),
                        feed: make_hex_value_from_string(ETH)
                    },
                    FeedValue {
                        value: 31u32.into(),
                        feed: make_hex_value_from_string(BTC)
                    }
                ]
                .iter_into()
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

        let result = make_processor_result::<StdEnv, U256>(
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

        let result = make_processor_result::<StdEnv, U256>(
            &Config::test_with_signer_count_threshold_or_default(None),
            Payload { data_packages },
        );

        assert_eq!(
            result,
            Err(Error::ReoccurringFeedId(BTC.as_bytes().to_vec().into()))
        );
    }
}
