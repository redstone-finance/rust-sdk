use alloc::vec::Vec;

use crate::{
    core::config::Config, network::error::Error, types::Value, utils::filter::FilterSome, FeedId,
    SignerAddress, TimestampMillis,
};
/// A trait defining validation operations for data feeds and signers.
///
/// This trait specifies methods for validating aspects of data feeds and signers within a system that
/// requires data integrity and authenticity checks. Implementations of this trait are responsible for
/// defining the logic behind each validation step, ensuring that data conforms to expected rules and
/// conditions.
pub trait Validator {
    /// Retrieves the index of a given data feed.
    ///
    /// This method takes a `feed_id` representing the unique identifier of a data feed and
    /// returns an `Option<usize>` indicating the index of the feed within a collection of feeds.
    /// If the feed does not exist, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `feed_id`: `U256` - The unique identifier of the data feed.
    ///
    /// # Returns
    ///
    /// * `Option<usize>` - The index of the feed if it exists, or `None` if it does not.
    fn feed_index(&self, feed_id: FeedId) -> Option<usize>;

    /// Retrieves the index of a given signer.
    ///
    /// This method accepts a signer identifier and returns an
    /// `Option<usize>` indicating the signer's index within a collection of signers. If the signer
    /// is not found, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `signer`: `&SignerAddress` - The signer's identifier.
    ///
    /// # Returns
    ///
    /// * `Option<usize>` - The index of the signer if found, or `None` if not found.
    fn signer_index(&self, signer: &SignerAddress) -> Option<usize>;

    /// Validates the signer count threshold for a given index within a set of values.
    ///
    /// This method is responsible for ensuring that the number of valid signers meets or exceeds
    /// a specified threshold necessary for a set of data values to be considered valid. It returns
    /// a vector of `Value` if the values pass the validation, to be processed in other steps.
    ///
    /// # Arguments
    ///
    /// * `index`: `usize` - The index of the data value being validated.
    /// * `values`: `&[Option<Value>]` - A slice of optional `Value` values associated with the data.
    ///
    /// # Returns
    ///
    /// * `Vec<U256>` - A vector of `U256` values that meet the validation criteria.
    fn validate_signer_count_threshold(
        &self,
        index: usize,
        values: &[Option<Value>],
    ) -> Result<Vec<Value>, Error>;

    /// Validates the timestamp for a given index.
    ///
    /// This method checks whether a timestamp associated with a data value at a given index
    /// meets specific conditions (e.g., being within an acceptable time range). It returns
    /// the validated timestamp if it's valid, to be processed in other steps.
    ///
    /// # Arguments
    ///
    /// * `index`: `usize` - The index of the data value whose timestamp is being validated.
    /// * `timestamp`: `BlockTimestampMillis` - The timestamp to be validated.
    ///
    /// # Returns
    ///
    /// * `BlockTimestampMillis` - The validated timestamp.
    fn validate_timestamp(
        &self,
        index: usize,
        timestamp: TimestampMillis,
    ) -> Result<TimestampMillis, Error>;
}

impl Validator for Config {
    #[cfg_attr(feature = "extra", inline)]
    fn feed_index(&self, feed_id: FeedId) -> Option<usize> {
        self.feed_ids().iter().position(|&elt| elt == feed_id)
    }

    #[cfg_attr(feature = "extra", inline)]
    fn signer_index(&self, signer: &SignerAddress) -> Option<usize> {
        self.signers().iter().position(|elt| elt == signer)
    }

    #[cfg_attr(feature = "extra", inline)]
    fn validate_signer_count_threshold(
        &self,
        index: usize,
        values: &[Option<Value>],
    ) -> Result<Vec<Value>, Error> {
        let values = values.filter_some();
        if values.len() < *self.signer_count_threshold() as usize {
            return Err(Error::InsufficientSignerCount(
                index,
                values.len(),
                self.feed_ids()[index],
            ));
        }

        Ok(values)
    }

    #[cfg_attr(feature = "extra", inline)]
    fn validate_timestamp(
        &self,
        index: usize,
        timestamp: TimestampMillis,
    ) -> Result<TimestampMillis, Error> {
        if !timestamp
            .add(*self.max_timestamp_delay_ms())
            .is_same_or_after(*self.block_timestamp())
        {
            return Err(Error::TimestampTooOld(index, timestamp));
        }
        if !timestamp.is_same_or_before(self.block_timestamp().add(*self.max_timestamp_ahead_ms()))
        {
            return Err(Error::TimestampTooFuture(index, timestamp));
        }

        Ok(timestamp)
    }
}

#[cfg(feature = "helpers")]
#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use itertools::Itertools;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use crate::{
        core::{
            config::Config,
            test_helpers::{
                AVAX, BTC, ETH, TEST_BLOCK_TIMESTAMP, TEST_SIGNER_ADDRESS_1, TEST_SIGNER_ADDRESS_2,
                TEST_SIGNER_ADDRESS_3, TEST_SIGNER_ADDRESS_4,
            },
            validator::Validator,
        },
        helpers::{
            hex::{hex_to_bytes, make_feed_id},
            iter_into::{IterInto, IterIntoOpt, OptIterIntoOpt},
        },
        network::error::Error,
        protocol::constants::{MAX_TIMESTAMP_AHEAD_MS, MAX_TIMESTAMP_DELAY_MS},
        Value,
    };

    #[test]
    fn test_feed_index() {
        let config = Config::test_with_signer_count_threshold_or_default(None);

        let eth_index = config.feed_index(make_feed_id(ETH));
        assert_eq!(eth_index, 0.into());

        let eth_index = config.feed_index(make_feed_id("778680")); //eth
        assert_eq!(eth_index, None);

        let btc_index = config.feed_index(make_feed_id(BTC));
        assert_eq!(btc_index, 1.into());

        let avax_index = config.feed_index(make_feed_id(AVAX));
        assert_eq!(avax_index, None);
    }

    #[test]
    fn test_signer_index() {
        let config = Config::test_with_signer_count_threshold_or_default(None);
        let index = config.signer_index(&hex_to_bytes(TEST_SIGNER_ADDRESS_1.into()).into());
        assert_eq!(index, 0.into());

        let index = config.signer_index(&hex_to_bytes(TEST_SIGNER_ADDRESS_1.to_uppercase()).into());
        assert_eq!(index, 0.into());

        let index = config.signer_index(&hex_to_bytes(TEST_SIGNER_ADDRESS_2.into()).into());
        assert_eq!(index, 1.into());

        let index =
            config.signer_index(&hex_to_bytes(TEST_SIGNER_ADDRESS_2.replace('0', "1")).into());
        assert_eq!(index, None);
    }

    #[test]
    fn test_validate_timestamp() {
        let config = Config::test_with_signer_count_threshold_or_default(None);

        assert!(config
            .validate_timestamp(0, TEST_BLOCK_TIMESTAMP.into())
            .is_ok());
        assert!(config
            .validate_timestamp(1, (TEST_BLOCK_TIMESTAMP + 60000).into())
            .is_ok());
        assert!(config
            .validate_timestamp(2, (TEST_BLOCK_TIMESTAMP + MAX_TIMESTAMP_AHEAD_MS).into())
            .is_ok());
        assert!(config
            .validate_timestamp(3, (TEST_BLOCK_TIMESTAMP - MAX_TIMESTAMP_DELAY_MS).into())
            .is_ok());
        assert!(config
            .validate_timestamp(4, (TEST_BLOCK_TIMESTAMP - 60000).into())
            .is_ok());
    }

    #[test]
    fn test_validate_timestamp_too_future() {
        let timestamp = (TEST_BLOCK_TIMESTAMP + MAX_TIMESTAMP_AHEAD_MS + 1).into();
        let res = Config::test_with_signer_count_threshold_or_default(None)
            .validate_timestamp(0, timestamp);

        assert_eq!(res, Err(Error::TimestampTooFuture(0, timestamp)));
    }

    #[test]
    fn test_validate_timestamp_too_old() {
        let timestamp = (TEST_BLOCK_TIMESTAMP - MAX_TIMESTAMP_DELAY_MS - 1).into();
        let res = Config::test_with_signer_count_threshold_or_default(None)
            .validate_timestamp(1, timestamp);
        assert_eq!(res, Err(Error::TimestampTooOld(1, timestamp)));
    }

    #[test]
    fn test_validate_timestamp_zero() {
        let res = Config::test_with_signer_count_threshold_or_default(None)
            .validate_timestamp(2, 0.into());
        assert_eq!(res, Err(Error::TimestampTooOld(2, 0.into())));
    }

    #[test]
    fn test_validate_timestamp_big() {
        let timestamp = (TEST_BLOCK_TIMESTAMP + TEST_BLOCK_TIMESTAMP).into();
        let res = Config::test_with_signer_count_threshold_or_default(None)
            .validate_timestamp(3, timestamp);
        assert_eq!(res, Err(Error::TimestampTooFuture(3, timestamp)));
    }

    #[test]
    fn test_validate_timestamp_no_block_timestamp() {
        let config = Config::test_with_signer_count_threshold_block_timestamp(None, 0.into());

        let res = config.validate_timestamp(4, TEST_BLOCK_TIMESTAMP.into());

        assert_eq!(
            res,
            Err(Error::TimestampTooFuture(4, TEST_BLOCK_TIMESTAMP.into()))
        );
    }

    #[test]
    fn test_validate_signer_count_threshold_empty_list() {
        let test_config = Config::test_with_signer_count_threshold_or_default(None);
        let res = test_config.validate_signer_count_threshold(0, vec![].as_slice());
        assert_eq!(
            res,
            Err(Error::InsufficientSignerCount(
                0,
                0,
                test_config.feed_ids()[0]
            ))
        );
    }

    #[test]
    fn test_validate_signer_count_threshold_shorter_list() {
        let test_config = Config::test_with_signer_count_threshold_or_default(None);
        let res =
            test_config.validate_signer_count_threshold(1, vec![1u8].iter_into_opt().as_slice());
        assert_eq!(
            res,
            Err(Error::InsufficientSignerCount(
                1,
                1,
                test_config.feed_ids()[1]
            ))
        );
    }

    #[test]
    fn test_validate_signer_count_threshold_list_with_nones() {
        let test_config = Config::test_with_signer_count_threshold_or_default(None);
        let res = test_config.validate_signer_count_threshold(
            1,
            vec![None, 1u8.into(), None].opt_iter_into_opt().as_slice(),
        );

        assert_eq!(
            res,
            Err(Error::InsufficientSignerCount(
                1,
                1,
                test_config.feed_ids()[1]
            ))
        );
    }

    #[test]
    fn test_validate_signer_count_threshold_with_exact_size() {
        validate_with_all_permutations(vec![1u8, 2].iter_into_opt(), vec![1u8, 2].iter_into());
    }

    #[test]
    fn test_validate_signer_count_threshold_with_exact_signer_count() {
        validate_with_all_permutations(
            vec![None, 1u8.into(), None, 2.into()].opt_iter_into_opt(),
            vec![1u8, 2].iter_into(),
        );
    }

    #[test]
    fn test_validate_signer_count_threshold_with_larger_size() {
        validate_with_all_permutations(
            vec![
                1u8.into(),
                None,
                None,
                2.into(),
                3.into(),
                None,
                4.into(),
                None,
            ]
            .opt_iter_into_opt(),
            vec![1u8, 2, 3, 4].iter_into(),
        );
    }

    fn validate_with_all_permutations(numbers: Vec<Option<Value>>, expected_value: Vec<Value>) {
        let perms: Vec<Vec<_>> = numbers.iter().permutations(numbers.len()).collect();

        let result = Config::test_with_signer_count_threshold_or_default(None)
            .validate_signer_count_threshold(0, &numbers)
            .unwrap();
        assert_eq!(result, expected_value);

        for threshold in 0..expected_value.len() + 1 {
            let config = Config::test_with_signer_count_threshold_block_timestamp_signers(
                Some(threshold as u8),
                TEST_BLOCK_TIMESTAMP.into(),
                vec![
                    TEST_SIGNER_ADDRESS_1,
                    TEST_SIGNER_ADDRESS_2,
                    TEST_SIGNER_ADDRESS_3,
                    TEST_SIGNER_ADDRESS_4,
                ],
            );

            for (index, perm) in perms.iter().enumerate() {
                let p: Vec<_> = perm.iter().map(|&&v| v).collect();

                let result = config
                    .validate_signer_count_threshold(index % config.feed_ids().len(), &p)
                    .unwrap();
                assert_eq!(result.len(), expected_value.len());
            }
        }
    }
}
