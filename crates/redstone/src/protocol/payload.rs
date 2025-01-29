use crate::{
    core::validator::Validator, network::error::Error, protocol::data_package::DataPackage,
    TimestampMillis,
};
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct Payload {
    pub(crate) data_packages: Vec<DataPackage>,
}

impl Payload {
    pub fn get_validated_timestamp(
        &self,
        validator: &impl Validator,
    ) -> Result<TimestampMillis, Error> {
        let Some(first_package) = self.data_packages.get(0) else {
            return Err(Error::ArrayIsEmpty);
        };

        let first_timestamp = validator.validate_timestamp(0, first_package.timestamp)?;

        if let Some(outstanding_ts) = self
            .data_packages
            .iter()
            .map(|package| package.timestamp)
            .skip(1)
            .find(|ts| *ts != first_timestamp)
        {
            return Err(Error::TimestampDifferentThanOthers(
                first_timestamp,
                outstanding_ts,
            ));
        }

        Ok(first_timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::Payload;
    use crate::{core::config::Config, network::error::Error, protocol::data_package::DataPackage};

    const TEST_BLOCK_TIMESTAMP: u64 = 2000000000000;
    const TEST_SIGNER_ADDRESS_1: &str = "1ea62d73edF8ac05dfcea1a34b9796e937a29eFF";
    const ETH: &str = "ETH";
    const BTC: &str = "BTC";

    #[test]
    fn test_validate_all_timestamps_in_payload_are_the_same() -> Result<(), Error> {
        let config = Config::test_with_signer_count_threshold_or_default(None);
        let data_packages = vec![
            DataPackage::test_multi_data_point(
                vec![(BTC, 30), (ETH, 11)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 10), (BTC, 31)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
            DataPackage::test_multi_data_point(
                vec![(BTC, 34), (ETH, 12)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 13), (BTC, 32)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
        ];
        let payload = Payload { data_packages };
        let ts = payload.get_validated_timestamp(&config)?;
        assert_eq!(ts, TEST_BLOCK_TIMESTAMP.into());

        Ok(())
    }

    #[test]
    fn test_validate_all_timestamps_in_payload_one_is_wrong() {
        let config = Config::test_with_signer_count_threshold_or_default(None);
        let data_packages = vec![
            DataPackage::test_multi_data_point(
                vec![(BTC, 30), (ETH, 11)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 10), (BTC, 31)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
            DataPackage::test_multi_data_point(
                vec![(BTC, 34), (ETH, 12)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP + 5).into(),
            ),
            DataPackage::test_multi_data_point(
                vec![(ETH, 13), (BTC, 32)],
                TEST_SIGNER_ADDRESS_1,
                (TEST_BLOCK_TIMESTAMP).into(),
            ),
        ];
        let payload = Payload { data_packages };
        let result = payload.get_validated_timestamp(&config);

        assert_eq!(
            result,
            Err(Error::TimestampDifferentThanOthers(
                TEST_BLOCK_TIMESTAMP.into(),
                (TEST_BLOCK_TIMESTAMP + 5).into()
            ))
        );
    }

    #[test]
    fn test_validate_all_timestamps_in_payload_is_empty() {
        let config = Config::test_with_signer_count_threshold_or_default(None);
        let data_packages = vec![];
        let payload = Payload { data_packages };
        let result = payload.get_validated_timestamp(&config);

        assert_eq!(result, Err(Error::ArrayIsEmpty));
    }
}
