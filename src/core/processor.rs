use crate::{
    core::processor_result::ProcessorResult,
    network::Environment,
    protocol::{payload::Payload, PayloadDecoder},
    Bytes, RedStoneConfig,
};

use crate::core::{aggregator::aggregate_values, config::Config, validator::Validator};

use crate::core::processor_result::ValidatedPayload;

use crate::core::processor_result::ProcessorError;

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
/// * Returns a `ProcessorResult` in case of succesfull payload processing. Will panic in case of bad input.
pub fn process_payload(
    config: &impl RedStoneConfig,
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
    /// * Returns a `ProcessorResult` in case of succesfull payload processing. Will panic in case of bad input.
    fn process_payload(&self, payload_bytes: impl Into<Bytes>) -> ProcessorResult;
}

impl<T: RedStoneConfig> RedStonePayloadProcessor for T {
    fn process_payload(&self, payload_bytes: impl Into<Bytes>) -> ProcessorResult {
        let mut bytes = payload_bytes.into();
        let payload = PayloadDecoder::<T::Environment, T::Crypto>::make_payload(&mut bytes.0)?;

        T::Environment::print(|| format!("{:?}", payload));

        make_processor_result::<T::Environment>(self.config(), payload)
    }
}

fn make_processor_result<Env: Environment>(config: &Config, payload: Payload) -> ProcessorResult {
    let min_timestamp = payload
        .data_packages
        .iter()
        .enumerate()
        .map(|(index, dp)| config.validate_timestamp(index, dp.timestamp))
        .min_by(|a, b| match (a, b) {
            (Ok(a), Ok(b)) => a.cmp(b),
            (Err(_), _) => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Greater,
        })
        .ok_or(ProcessorError::NoDataPackages)??;

    let values = aggregate_values(payload.data_packages, config)?;

    Env::print(|| format!("{:?} {:?}", min_timestamp, values));

    Ok(ValidatedPayload {
        values,
        min_timestamp,
    })
}

#[cfg(feature = "helpers")]
#[cfg(test)]
mod tests {
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
        network::StdEnv,
        protocol::{data_package::DataPackage, payload::Payload},
    };

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn test_make_processor_result() {
        let data_packages = vec![
            DataPackage::test(
                ETH,
                11,
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP + 5).into(),
            ),
            DataPackage::test(
                ETH,
                13,
                TEST_SIGNER_ADDRESS_2,
                (TEST_BLOCK_TIMESTAMP + 3).into(),
            ),
            DataPackage::test(
                BTC,
                32,
                TEST_SIGNER_ADDRESS_2,
                (TEST_BLOCK_TIMESTAMP - 2).into(),
            ),
            DataPackage::test(
                BTC,
                31,
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP + 400).into(),
            ),
        ];

        let result = make_processor_result::<StdEnv>(&Config::test(), Payload { data_packages });

        assert_eq!(
            result,
            Ok(ValidatedPayload {
                min_timestamp: (TEST_BLOCK_TIMESTAMP - 2).into(),
                values: vec![12u8, 31].iter_into()
            })
        );
    }
}
