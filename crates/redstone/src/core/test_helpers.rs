use alloc::vec::Vec;
use redstone_utils::{
    hex::{hex_to_bytes, make_from_hex, make_hex_value_from_string},
    iter_into::IterInto,
};

use crate::{
    core::config::Config,
    protocol::{data_package::DataPackage, data_point::DataPoint},
    FeedId, SignerAddress, TimestampMillis,
};

pub(crate) const TEST_BLOCK_TIMESTAMP: u64 = 2000000000000;
pub(crate) const MAX_TIMESTAMP_DELAY_MS: u64 = 15 * 60 * 1000; // 15 minutes in milliseconds
pub(crate) const MAX_TIMESTAMP_AHEAD_MS: u64 = 3 * 60 * 1000; // 3 minutes in milliseconds

pub(crate) const TEST_SIGNER_ADDRESS_1: &str = "1ea62d73edF8ac05dfcea1a34b9796e937a29eFF";
pub(crate) const TEST_SIGNER_ADDRESS_2: &str = "109b4a318a4f5ddcbca6349b45f881b4137deafb";
pub(crate) const TEST_SIGNER_ADDRESS_3: &str = "01c9827101a57ac9e9fb7034510e19afcf9c0207";
pub(crate) const TEST_SIGNER_ADDRESS_4: &str = "264dee744b727613cb76e0cb2f97cd6eda95b39e";

pub(crate) const ETH: &str = "ETH";
pub(crate) const BTC: &str = "BTC";
pub(crate) const AVAX: &str = "AVAX";

impl From<&str> for SignerAddress {
    fn from(value: &str) -> Self {
        make_from_hex(value)
    }
}

impl From<&str> for FeedId {
    fn from(value: &str) -> Self {
        make_hex_value_from_string(value)
    }
}

impl Config {
    /// Creates config with default signer_count_threshold equal 2 if not specified otherwise.
    ///
    /// It uses as default 2 distinct signers.
    /// It uses as default 2 distinct feed_ids.
    /// It uses default block timestamp.
    pub(crate) fn test_with_signer_count_threshold_or_default(
        signer_count_threshold: Option<u8>,
    ) -> Self {
        Self::test(
            signer_count_threshold,
            vec![TEST_SIGNER_ADDRESS_1, TEST_SIGNER_ADDRESS_2],
            vec!["ETH", "BTC"],
            None,
            None,
            None,
        )
    }

    /// Creates config with given block_timestamp and default signer_count_threshold equal 2 if not specified otherwise.
    ///
    /// It uses as default 2 distinct signers.
    /// It uses as default 2 distinct feed_ids.
    pub(crate) fn test_with_signer_count_threshold_block_timestamp(
        signer_count_threshold: Option<u8>,
        block_timestamp: TimestampMillis,
    ) -> Self {
        Self::test(
            signer_count_threshold,
            vec![TEST_SIGNER_ADDRESS_1, TEST_SIGNER_ADDRESS_2],
            vec!["ETH", "BTC"],
            Some(block_timestamp),
            None,
            None,
        )
    }

    /// Creates config with default signer_count_threshold equal 2 if not specified otherwise.
    ///
    /// It uses as default 2 distinct feed_ids.
    pub(crate) fn test_with_signer_count_threshold_block_timestamp_signers(
        signer_count_threshold: Option<u8>,
        block_timestamp: TimestampMillis,
        signers: Vec<&str>,
    ) -> Self {
        Self::test(
            signer_count_threshold,
            signers,
            vec!["ETH", "BTC"],
            Some(block_timestamp),
            None,
            None,
        )
    }

    /// Creates config with default signer_count_threshold equal 2 if not specified otherwise, and feed_ids.
    pub(crate) fn test(
        signer_count_threshold: Option<u8>,
        signers: Vec<&str>,
        feed_ids: Vec<&str>,
        block_timestamp: Option<TimestampMillis>,
        max_timestamp_delay_ms: Option<TimestampMillis>,
        max_timestamp_ahead_ms: Option<TimestampMillis>,
    ) -> Self {
        Self::try_new(
            signer_count_threshold.unwrap_or(2),
            signers.iter_into(),
            feed_ids.iter_into(),
            block_timestamp.unwrap_or(TEST_BLOCK_TIMESTAMP.into()),
            Some(max_timestamp_delay_ms.unwrap_or(MAX_TIMESTAMP_DELAY_MS.into())),
            Some(max_timestamp_ahead_ms.unwrap_or(MAX_TIMESTAMP_AHEAD_MS.into())),
        )
        .unwrap()
    }
}

impl DataPackage {
    pub(crate) fn test_single_data_point(
        feed_id: &str,
        value: u128,
        signer_address: &str,
        timestamp: Option<u64>,
    ) -> Self {
        DataPackage {
            signer_address: Some(hex_to_bytes(signer_address.into()).into()),
            timestamp: timestamp.unwrap_or(TEST_BLOCK_TIMESTAMP).into(),
            data_points: vec![DataPoint {
                feed_id: make_hex_value_from_string(feed_id),
                value: value.into(),
            }],
        }
    }

    pub(crate) fn test_multi_data_point(
        data_points: Vec<(&str, u128)>,
        signer_address: &str,
        timestamp: Option<u64>,
    ) -> Self {
        DataPackage {
            signer_address: Some(hex_to_bytes(signer_address.into()).into()),
            timestamp: timestamp.unwrap_or(TEST_BLOCK_TIMESTAMP).into(),
            data_points: data_points
                .into_iter()
                .map(|(feed_id, value)| DataPoint {
                    feed_id: make_hex_value_from_string(feed_id),
                    value: value.into(),
                })
                .collect(),
        }
    }
}
