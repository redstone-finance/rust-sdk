use crate::{
    helpers::slice::has_repetition, network::error::Error, FeedId, SignerAddress, TimestampMillis,
};
use alloc::vec::Vec;

/// MAX_SIGNERS_COUNT describes maximum number of signers in Config.
const MAX_SIGNERS_COUNT: usize = u8::MAX as usize;

/// Configuration for a RedStone payload processor.
///
/// Specifies the parameters necessary for the verification and aggregation of values
/// from various data points passed by the RedStone payload.
#[derive(Debug)]
pub struct Config {
    /// The minimum number of signers required validating the data.
    ///
    /// Specifies how many unique signers (from different addresses) are required
    /// for the data to be considered valid and trustworthy.
    pub signer_count_threshold: u8,

    /// List of identifiers for signers authorized to sign the data.
    ///
    /// Each signer is identified by a unique, network-specific byte string (`Bytes`),
    /// which represents their address.
    pub signers: Vec<SignerAddress>,

    /// Identifiers for the data feeds from which values are aggregated.
    ///
    /// Each data feed id is represented by the `FeedId` type.
    pub feed_ids: Vec<FeedId>,

    /// The current block time in timestamp format, used for verifying data timeliness.
    ///
    /// The value's been expressed in milliseconds since the Unix epoch (January 1, 1970) and allows
    /// for determining whether the data is current in the context of blockchain time.
    pub block_timestamp: TimestampMillis,
}

impl Config {
    /// Verifies members of the config.
    ///
    /// This method checks whether all configs members are correct.
    ///
    /// # Returns
    ///
    /// * Success `()` if config is valid or Err with `Error` otherwise.
    pub(crate) fn verify_members(&self) -> Result<(), Error> {
        self.validate_feed_ids_list()?;
        self.validate_signers_list()
    }

    #[inline]
    fn validate_feed_ids_list(&self) -> Result<(), Error> {
        self.is_feed_ids_empty()?;
        has_repetition(&self.feed_ids)
            .map_or_else(|| Ok(()), |v| Err(Error::ConfigReocuringFeedId(v)))
    }

    #[inline(always)]
    fn is_feed_ids_empty(&self) -> Result<(), Error> {
        if self.feed_ids.is_empty() {
            return Err(Error::ConfigEmptyFeedIds);
        }

        Ok(())
    }

    #[inline]
    fn validate_signers_list(&self) -> Result<(), Error> {
        self.is_signers_count_in_threshold()?;
        self.is_signer_count_not_exceeded()?;
        has_repetition(&self.signers)
            .map_or_else(|| Ok(()), |v| Err(Error::ConfigReocuringSigner(v)))
    }

    #[inline(always)]
    fn is_signers_count_in_threshold(&self) -> Result<(), Error> {
        if self.signers.len() < self.signer_count_threshold as usize || self.signers.is_empty() {
            return Err(Error::ConfigInsufficientSignersCount(
                self.signers.len() as u8,
                self.signer_count_threshold,
            ));
        }

        Ok(())
    }

    #[inline(always)]
    fn is_signer_count_not_exceeded(&self) -> Result<(), Error> {
        if self.signers.len() > MAX_SIGNERS_COUNT {
            return Err(Error::ConfigExceededSignersCount(
                self.signers.len(),
                MAX_SIGNERS_COUNT,
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::hex::{hex_to_bytes, make_feed_id, make_feed_ids};

    #[test]
    fn test_config_correct_feed_ids() -> Result<(), Error> {
        let config = Config {
            signer_count_threshold: 2,
            signers: vec![
                hex_to_bytes("dd34329d2fc551bea8ee480c2d35d09b75cea39e".into()).into(),
                hex_to_bytes("582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4".into()).into(),
            ],
            feed_ids: make_feed_ids(vec!["ETH", "BTC", "BTS", "SOL"]),
            block_timestamp: 2000000000000.into(),
        };

        config.validate_feed_ids_list()
    }

    #[test]
    fn test_config_empty_feed_ids() {
        let config = Config {
            signer_count_threshold: 2,
            signers: vec![
                hex_to_bytes("dd34329d2fc551bea8ee480c2d35d09b75cea39e".into()).into(),
                hex_to_bytes("582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4".into()).into(),
            ],
            feed_ids: make_feed_ids(vec![]),
            block_timestamp: 2000000000000.into(),
        };

        let resutlt = config.validate_feed_ids_list();

        assert_eq!(resutlt, Err(Error::ConfigEmptyFeedIds));
    }

    #[test]
    fn test_config_repeated_feed_ids() {
        let repeated_feed_id = "BTC";
        let config = Config {
            signer_count_threshold: 2,
            signers: vec![
                hex_to_bytes("dd34329d2fc551bea8ee480c2d35d09b75cea39e".into()).into(),
                hex_to_bytes("582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4".into()).into(),
            ],
            feed_ids: make_feed_ids(vec![
                "ETH",
                repeated_feed_id,
                "SOL",
                repeated_feed_id,
                "BTS",
            ]),
            block_timestamp: 2000000000000.into(),
        };

        let resutlt = config.validate_feed_ids_list();

        assert_eq!(
            resutlt,
            Err(Error::ConfigReocuringFeedId(make_feed_id(repeated_feed_id)))
        );
    }

    #[test]
    fn test_config_correct_signers() -> Result<(), Error> {
        let config = Config {
            signer_count_threshold: 4,
            signers: vec![
                hex_to_bytes("dd34329d2fc551bea8ee480c2d35d09b75cea39e".into()).into(),
                hex_to_bytes("582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4".into()).into(),
                hex_to_bytes("6809c0b4ab2fc9960c8fd6e5448ac9be10aa8fe3".into()).into(),
                hex_to_bytes("97c037f86c10c7c4f2dc19f6b8f707137e2ab34c".into()).into(),
                hex_to_bytes("934ff84d7b374601d535217977515797589220e3".into()).into(),
            ],
            feed_ids: make_feed_ids(vec!["ETH", "BTC", "BTS", "SOL"]),
            block_timestamp: 2000000000000.into(),
        };

        config.validate_signers_list()
    }

    #[test]
    fn test_config_empty_signers() {
        let config = Config {
            signer_count_threshold: 0,
            signers: vec![],
            feed_ids: make_feed_ids(vec!["ETH", "BTC", "SOL", "BTS"]),
            block_timestamp: 2000000000000.into(),
        };

        let resutlt = config.validate_signers_list();

        assert_eq!(resutlt, Err(Error::ConfigInsufficientSignersCount(0, 0)));
    }

    #[test]
    fn test_config_not_enough_signers() {
        let config = Config {
            signer_count_threshold: 6,
            signers: vec![
                hex_to_bytes("dd34329d2fc551bea8ee480c2d35d09b75cea39e".into()).into(),
                hex_to_bytes("582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4".into()).into(),
                hex_to_bytes("6809c0b4ab2fc9960c8fd6e5448ac9be10aa8fe3".into()).into(),
                hex_to_bytes("97c037f86c10c7c4f2dc19f6b8f707137e2ab34c".into()).into(),
                hex_to_bytes("934ff84d7b374601d535217977515797589220e3".into()).into(),
            ],
            feed_ids: make_feed_ids(vec!["ETH", "BTC", "SOL", "BTS"]),
            block_timestamp: 2000000000000.into(),
        };

        let resutlt = config.validate_signers_list();

        assert_eq!(resutlt, Err(Error::ConfigInsufficientSignersCount(5, 6)));
    }

    #[test]
    fn test_config_repeated_signers() {
        let repeated = "6809c0b4ab2fc9960c8fd6e5448ac9be10aa8fe3";

        let config = Config {
            signer_count_threshold: 4,
            signers: vec![
                hex_to_bytes("dd34329d2fc551bea8ee480c2d35d09b75cea39e".into()).into(),
                hex_to_bytes("582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4".into()).into(),
                hex_to_bytes(repeated.into()).into(),
                hex_to_bytes("97c037f86c10c7c4f2dc19f6b8f707137e2ab34c".into()).into(),
                hex_to_bytes(repeated.into()).into(),
                hex_to_bytes("934ff84d7b374601d535217977515797589220e3".into()).into(),
            ],
            feed_ids: make_feed_ids(vec!["ETH", "BTC", "SOL", "BTS"]),
            block_timestamp: 2000000000000.into(),
        };

        let resutlt = config.validate_signers_list();

        assert_eq!(
            resutlt,
            Err(Error::ConfigReocuringSigner(
                hex_to_bytes(repeated.into()).into()
            ))
        );
    }

    #[test]
    fn test_config_to_many_signers() {
        let signer_exceeded_count: usize = 257;
        let mut signers: Vec<SignerAddress> = Vec::with_capacity(signer_exceeded_count);
        for _ in 0..signer_exceeded_count {
            signers.push(helper_generate_random_hex(20).into());
        }

        let config = Config {
            signer_count_threshold: 6,
            signers,
            feed_ids: make_feed_ids(vec!["ETH", "BTC", "SOL", "BTS"]),
            block_timestamp: 2000000000000.into(),
        };

        let resutlt = config.validate_signers_list();

        assert_eq!(
            resutlt,
            Err(Error::ConfigExceededSignersCount(257, MAX_SIGNERS_COUNT))
        );
    }

    fn helper_generate_random_hex(size: usize) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0u8; size];
        for x in data.iter_mut() {
            *x = rand::random()
        }

        data
    }
}
