use alloc::vec::Vec;

use derive_getters::Getters;

use crate::{
    contract::verification::verify_signers_config,
    network::error::Error,
    protocol::constants::{MAX_TIMESTAMP_AHEAD_MS, MAX_TIMESTAMP_DELAY_MS},
    utils::slice::check_no_duplicates,
    FeedId, SignerAddress, TimestampMillis,
};

/// Configuration for a RedStone payload processor.
///
/// Specifies the parameters necessary for the verification and aggregation of values
/// from various data points passed by the RedStone payload.
#[derive(Debug, Getters)]
pub struct Config {
    /// The minimum number of signers required validating the data.
    ///
    /// Specifies how many unique signers (from different addresses) are required
    /// for the data to be considered valid and trustworthy.
    signer_count_threshold: u8,

    /// List of identifiers for signers authorized to sign the data.
    ///
    /// Each signer is identified by a unique, network-specific byte string (`Bytes`),
    /// which represents their address.
    signers: Vec<SignerAddress>,

    /// Identifiers for the data feeds from which values are aggregated.
    ///
    /// Each data feed id is represented by the `FeedId` type.
    feed_ids: Vec<FeedId>,

    /// The current block time in timestamp format, used for verifying data timeliness.
    ///
    /// The value's been expressed in milliseconds since the Unix epoch (January 1, 1970) and allows
    /// for determining whether the data is current in the context of blockchain time.
    block_timestamp: TimestampMillis,

    /// The maximum delay of the package in regards to the current block in the blockchain.
    ///
    /// The value's been expressed in milliseconds since the Unix epoch (January 1, 1970).
    max_timestamp_delay_ms: TimestampMillis,

    /// The maximum time package was created ahead of the current block in the blockchain.
    ///
    /// The value's been expressed in milliseconds since the Unix epoch (January 1, 1970).
    max_timestamp_ahead_ms: TimestampMillis,
}

impl Config {
    /// Verifies all members of the config.
    ///
    /// This method checks whether all config members are correct.
    ///
    /// # Arguments
    ///
    /// * `signer_count_threshold` - The minimum number of signers required validating the data.
    /// * `signers` - List of identifiers for signers authorized to sign the data.
    /// * `feed_ids` - Identifiers for the data feeds from which values are aggregated.
    /// * `block_timestamp` - The current block time in timestamp format, used for verifying data timeliness.
    /// * `max_timestamp_delay_ms` - Maximum delay of the package against the current block timestamp.
    ///    If None is provided then default config value is used.
    /// * `max_timestamp_ahead_ms` - Maximum ahead of time of the package against current block timestamp.
    ///    If None is provided then default config value is used.
    ///
    /// # Returns
    ///
    /// * Success `Self` if arguments to the functions are correct
    ///   or cresponding Err with `redstone::network::Error` otherwise.
    pub fn try_new(
        signer_count_threshold: u8,
        signers: Vec<SignerAddress>,
        feed_ids: Vec<FeedId>,
        block_timestamp: TimestampMillis,
        max_timestamp_delay_ms: Option<TimestampMillis>,
        max_timestamp_ahead_ms: Option<TimestampMillis>,
    ) -> Result<Self, Error> {
        let config = Self {
            signer_count_threshold,
            signers,
            feed_ids,
            block_timestamp,
            max_timestamp_delay_ms: max_timestamp_delay_ms.unwrap_or(MAX_TIMESTAMP_DELAY_MS.into()),
            max_timestamp_ahead_ms: max_timestamp_ahead_ms.unwrap_or(MAX_TIMESTAMP_AHEAD_MS.into()),
        };

        config.verify_signer_list()?;
        config.verify_feed_id_list()?;

        Ok(config)
    }

    #[inline]
    fn verify_feed_id_list(&self) -> Result<(), Error> {
        self.verify_feed_id_list_empty()?;
        check_no_duplicates(&self.feed_ids).map_err(Error::ConfigReoccurringFeedId)
    }

    #[inline(always)]
    fn verify_feed_id_list_empty(&self) -> Result<(), Error> {
        if self.feed_ids.is_empty() {
            return Err(Error::ConfigEmptyFeedIds);
        }

        Ok(())
    }

    #[inline]
    fn verify_signer_list(&self) -> Result<(), Error> {
        verify_signers_config(&self.signers, self.signer_count_threshold)
    }
}

#[cfg(test)]
#[cfg(feature = "helpers")]
mod tests {
    use super::*;
    use crate::{
        core::test_helpers::MAX_TIMESTAMP_DELAY_MS,
        helpers::{
            hex::{hex_to_bytes, make_feed_id},
            iter_into::IterInto,
        },
    };

    #[test]
    fn test_config_correct_feed_ids() -> Result<(), Error> {
        let config = Config {
            signer_count_threshold: 2,
            signers: vec![
                "dd34329d2fc551bea8ee480c2d35d09b75cea39e",
                "582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4",
            ]
            .iter_into(),
            feed_ids: vec!["ETH", "BTC", "BTS", "SOL"].iter_into(),
            block_timestamp: 2000000000000.into(),
            max_timestamp_delay_ms: MAX_TIMESTAMP_AHEAD_MS.into(),
            max_timestamp_ahead_ms: MAX_TIMESTAMP_DELAY_MS.into(),
        };

        config.verify_feed_id_list()
    }

    #[test]
    fn test_config_empty_feed_ids() {
        let config = Config {
            signer_count_threshold: 2,
            signers: vec![
                "dd34329d2fc551bea8ee480c2d35d09b75cea39e",
                "582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4",
            ]
            .iter_into(),
            feed_ids: vec![],
            block_timestamp: 2000000000000.into(),
            max_timestamp_delay_ms: MAX_TIMESTAMP_AHEAD_MS.into(),
            max_timestamp_ahead_ms: MAX_TIMESTAMP_DELAY_MS.into(),
        };

        let resutlt = config.verify_feed_id_list();

        assert_eq!(resutlt, Err(Error::ConfigEmptyFeedIds));
    }

    #[test]
    fn test_config_repeated_feed_ids() {
        let repeated_feed_id = "BTC";
        let config = Config {
            signer_count_threshold: 2,
            signers: vec![
                "dd34329d2fc551bea8ee480c2d35d09b75cea39e",
                "582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4",
            ]
            .iter_into(),
            feed_ids: vec!["ETH", repeated_feed_id, "SOL", repeated_feed_id, "BTS"].iter_into(),
            block_timestamp: 2000000000000.into(),
            max_timestamp_delay_ms: MAX_TIMESTAMP_AHEAD_MS.into(),
            max_timestamp_ahead_ms: MAX_TIMESTAMP_DELAY_MS.into(),
        };

        let resutlt = config.verify_feed_id_list();

        assert_eq!(
            resutlt,
            Err(Error::ConfigReoccurringFeedId(make_feed_id(
                repeated_feed_id
            )))
        );
    }

    #[test]
    fn test_config_correct_signers() -> Result<(), Error> {
        let config = Config {
            signer_count_threshold: 4,
            signers: vec![
                "dd34329d2fc551bea8ee480c2d35d09b75cea39e",
                "582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4",
                "6809c0b4ab2fc9960c8fd6e5448ac9be10aa8fe3",
                "97c037f86c10c7c4f2dc19f6b8f707137e2ab34c",
                "934ff84d7b374601d535217977515797589220e3",
            ]
            .iter_into(),
            feed_ids: vec!["ETH", "BTC", "BTS", "SOL"].iter_into(),
            block_timestamp: 2000000000000.into(),
            max_timestamp_delay_ms: MAX_TIMESTAMP_AHEAD_MS.into(),
            max_timestamp_ahead_ms: MAX_TIMESTAMP_DELAY_MS.into(),
        };

        config.verify_signer_list()
    }

    #[test]
    fn test_config_empty_signers() {
        let config = Config {
            signer_count_threshold: 0,
            signers: vec![],
            feed_ids: vec!["ETH", "BTC", "SOL", "BTS"].iter_into(),
            block_timestamp: 2000000000000.into(),
            max_timestamp_delay_ms: MAX_TIMESTAMP_AHEAD_MS.into(),
            max_timestamp_ahead_ms: MAX_TIMESTAMP_DELAY_MS.into(),
        };

        let resutlt = config.verify_signer_list();

        assert_eq!(resutlt, Err(Error::ConfigInsufficientSignerCount(0, 0)));
    }

    #[test]
    fn test_config_not_enough_signers() {
        let config = Config {
            signer_count_threshold: 6,
            signers: vec![
                "dd34329d2fc551bea8ee480c2d35d09b75cea39e",
                "582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4",
                "6809c0b4ab2fc9960c8fd6e5448ac9be10aa8fe3",
                "97c037f86c10c7c4f2dc19f6b8f707137e2ab34c",
                "934ff84d7b374601d535217977515797589220e3",
            ]
            .iter_into(),
            feed_ids: vec!["ETH", "BTC", "SOL", "BTS"].iter_into(),
            block_timestamp: 2000000000000.into(),
            max_timestamp_delay_ms: MAX_TIMESTAMP_AHEAD_MS.into(),
            max_timestamp_ahead_ms: MAX_TIMESTAMP_DELAY_MS.into(),
        };

        let resutlt = config.verify_signer_list();

        assert_eq!(resutlt, Err(Error::ConfigInsufficientSignerCount(5, 6)));
    }

    #[test]
    fn test_config_repeated_signers() {
        let repeated = "6809c0b4ab2fc9960c8fd6e5448ac9be10aa8fe3";

        let config = Config {
            signer_count_threshold: 4,
            signers: vec![
                "dd34329d2fc551bea8ee480c2d35d09b75cea39e",
                "582ad60bedebfc21cfee1e1cb025cd2c77fc2bf4",
                repeated,
                "97c037f86c10c7c4f2dc19f6b8f707137e2ab34c",
                repeated,
                "934ff84d7b374601d535217977515797589220e3",
            ]
            .iter_into(),
            feed_ids: vec!["ETH", "BTC", "SOL", "BTS"].iter_into(),
            block_timestamp: 2000000000000.into(),
            max_timestamp_delay_ms: MAX_TIMESTAMP_AHEAD_MS.into(),
            max_timestamp_ahead_ms: MAX_TIMESTAMP_DELAY_MS.into(),
        };

        let resutlt = config.verify_signer_list();

        assert_eq!(
            resutlt,
            Err(Error::ConfigReoccurringSigner(
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
            feed_ids: vec!["ETH", "BTC", "SOL", "BTS"].iter_into(),
            block_timestamp: 2000000000000.into(),
            max_timestamp_delay_ms: MAX_TIMESTAMP_AHEAD_MS.into(),
            max_timestamp_ahead_ms: MAX_TIMESTAMP_DELAY_MS.into(),
        };

        let resutlt = config.verify_signer_list();

        assert_eq!(resutlt, Err(Error::ConfigExceededSignerCount(257, 255)));
    }

    fn helper_generate_random_hex(size: usize) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0u8; size];
        for x in data.iter_mut() {
            *x = rand::random()
        }

        data
    }
}
