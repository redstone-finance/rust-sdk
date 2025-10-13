use alloc::{string::String, vec::Vec};
use core::num::TryFromIntError;

use thiserror::Error;

use crate::{
    network::as_str::AsHexStr, types::Value, CryptoError, FeedId, SignerAddress, TimestampMillis,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContractErrorContent {
    pub code: u8,
    pub msg: String,
}

/// Errors that can be encountered in the deserializing&decrypting the RedStone payload or just contract execution process.
///
/// These errors include issues with contract logic, data types,
/// cryptographic operations, and conditions specific to the requirements.
#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum Error {
    /// Represents errors that arise from the contract itself.
    ///
    /// This variant is used for encapsulating errors that are specific to the contract's logic
    /// or execution conditions that aren't covered by more specific error types.
    #[error("Contract error: {}", .0.msg)]
    ContractError(ContractErrorContent),

    /// Indicates an overflow error with `U256` numbers.
    ///
    /// Used when operations on `U256` numbers exceed their maximum value, potentially leading
    /// to incorrect calculations or state.
    #[error("Number overflow: {}", .0)]
    NumberOverflow(Value),

    /// Used when an expected non-empty array or vector is found to be empty.
    ///
    /// This could occur in scenarios where the contract logic requires a non-empty collection
    /// of items for the correct operation, for example, during aggregating the values.
    #[error("Array is empty")]
    ArrayIsEmpty,

    /// Represents errors related to cryptographic operations.
    ///
    /// This includes failures in signature verification, hashing, or other cryptographic
    /// processes.
    #[error("Cryptographic Error: {0}")]
    CryptographicError(CryptoError),

    /// Signifies that an unsupported size was encountered.
    ///
    /// This could be used when a data structure or input does not meet the expected size
    /// requirements for processing.
    #[error("Size not supported: {0}")]
    SizeNotSupported(usize),

    /// Indicates that the marker bytes for RedStone are incorrect.
    ///
    /// This error is specific to scenarios where marker or identifier bytes do not match
    /// expected values, potentially indicating corrupted or tampered data.
    #[error("Wrong RedStone marker: {}", .0.as_hex_str())]
    WrongRedStoneMarker(Vec<u8>),

    /// Used when there is leftover data in a payload that should have been empty.
    ///
    /// This could indicate an error in data parsing or that additional, unexpected data
    /// was included in a message or transaction.
    #[error("Non empty payload len remainder: {0}")]
    NonEmptyPayloadRemainder(usize),

    /// Indicates that the recovered signer address is not recognized.
    ///
    /// Includes SignerAddress that was recovered.
    #[error("Signer not recognized: {0:?}")]
    SignerNotRecognized(SignerAddress),

    /// Indicates that the number of signers does not meet the required threshold.
    ///
    /// This variant includes the current number of signers, the required threshold, and
    /// potentially a feed_id related to the operation that failed due to insufficient signers.
    #[error("Insufficient signer count {} for #{} ({})", .1, .0, .2.as_hex_str())]
    InsufficientSignerCount(usize, usize, FeedId),

    /// Used when a timestamp is older than allowed by the processor logic.
    ///
    /// Includes the position or identifier of the timestamp and the threshold value,
    /// indicating that the provided timestamp is too far in the past.
    #[error("Timestamp {1:?} is too old for #{0}")]
    TimestampTooOld(usize, TimestampMillis),

    /// Indicates that a timestamp is further in the future than allowed.
    ///
    /// Similar to `TimestampTooOld`, but for future timestamps exceeding the contract's
    /// acceptance window.
    #[error("Timestamp {1:?} is too future for #{0}")]
    TimestampTooFuture(usize, TimestampMillis),

    /// Indicates that a FeedId is reoccurring in data points.
    ///
    /// Includes FeedId that is reoccurring.
    #[error("Reoccurring FeedId: {0:?} in data points")]
    ReoccurringFeedId(FeedId),

    /// ConfigInsufficientSignerCount occurs
    /// when the number of signers is not at least equal the required signer threshold.
    ///
    /// Includes current config signer list length and minimum required signer count.
    #[error("Wrong configuration signer count, got {0} signers, expected at minimum {1}")]
    ConfigInsufficientSignerCount(u8, u8),

    /// ConfigExceededSignerCount occurs
    /// when the number of signers is larger than the config max-allowed signer count.
    /// Look in to core::config crate to acknowledge constant value.
    ///
    /// Includes current config signer list length and maximum allowed signer count per config.
    #[error("Wrong configuration signer count, got {0} signers, allowed maximum is {1}")]
    ConfigExceededSignerCount(usize, usize),

    /// ConfigInvalidSignerAddress occurs
    /// when there is at least one invalid signer present in configuration.
    ///
    /// Includes SignerAddress that is invalid.
    #[error("Wrong configuration, invalid signer {}", .0.as_hex_str())]
    ConfigInvalidSignerAddress(SignerAddress),

    /// Indicates that a SignerAddress is reoccurring on the config signer list.
    ///
    /// Includes SignerAddress that is reoccurring.
    #[error("Wrong configuration, signer address {} is reoccurring on the signer list", .0.as_hex_str())]
    ConfigReoccurringSigner(SignerAddress),

    /// Indicates that the list doesn't contain FeedIds.
    #[error("Empty configuration feed ids list")]
    ConfigEmptyFeedIds,

    /// Indicates that the list contains too many FeedIds.
    #[error("Wrong configuration, got {0} feedIds, allowed maximum is {1}")]
    ConfigExceededFeedIdsLength(usize, usize),

    /// Indicates that the list contains invalid FeedId.
    #[error("Wrong configuration, contains invalid feed id {}", .0.as_hex_str())]
    ConfigInvalidFeedId(FeedId),

    /// Indicates that a FeedId is reoccurring on the config feed_ids list.
    ///
    /// Includes FeedId that is reoccurring.
    #[error("Wrong configuration, feed id {} is reoccurring on the feed_ids list", .0.as_hex_str())]
    ConfigReoccurringFeedId(FeedId),

    /// Indicates that payload timestamps are not equal.
    ///
    /// Contains the first timestamp and the one that is not equal to the first one.
    #[error("Timestamp {1:?} is not equal to the first on {0:?} in the payload.")]
    TimestampDifferentThanOthers(TimestampMillis, TimestampMillis),

    /// Indicates that the provided data timestamp is not greater than a previously written package timestamp.
    ///
    /// For the price adapter to accept a new price update, the associated timestamp must be
    /// strictly greater than the timestamp of the last update. This error is raised if a new
    /// timestamp does not meet this criterion, ensuring the chronological integrity of price data.
    ///
    /// Includes the value of a current package timestamp and the timestamp of the previous package.
    #[error("Package timestamp: {0:?} must be greater than package timestamp before: {1:?}")]
    DataTimestampMustBeGreaterThanBefore(TimestampMillis, TimestampMillis),

    /// Indicates that the current update timestamp is not greater than the last update timestamp.
    ///
    /// This error is raised to ensure that the data being written has a timestamp strictly greater
    /// than the most recent timestamp already stored in the system. It guarantees that new data
    /// is not outdated or stale compared to the existing records, thereby maintaining the chronological
    /// integrity and consistency of the updates.
    ///
    /// Includes the value of the current update timestamp and the last update timestamp.
    #[error("Current update timestamp: {0:?} must be greater than latest update timestamp: {1:?}")]
    CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp(TimestampMillis, TimestampMillis),

    /// Indicates error while converting from one type of the integer to the other.
    #[error("Number conversion failed")]
    NumberConversionFail,

    /// Indicates error of usize overflow.
    #[error("Usize overflow")]
    UsizeOverflow,

    /// Indicates data on chain is stale.
    #[error("Stale data: write time {write_time:?} valid till {staleness_threshold:?} time now: {time_now:?}, ")]
    DataStaleness {
        write_time: TimestampMillis,
        staleness_threshold: TimestampMillis,
        time_now: TimestampMillis,
    },

    /// Indicates error of overflowing buffer.
    #[error("buffer overflow")]
    BufferOverflow,
}

impl From<CryptoError> for Error {
    fn from(value: CryptoError) -> Self {
        Self::CryptographicError(value)
    }
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self::NumberConversionFail
    }
}

impl Error {
    pub fn code(&self) -> u16 {
        match self {
            Error::ContractError(content) => content.code as u16,
            Error::NumberOverflow(_) => 509,
            Error::ArrayIsEmpty => 510,
            Error::WrongRedStoneMarker(_) => 511,
            Error::NonEmptyPayloadRemainder(_) => 512,
            Error::ReoccurringFeedId(_) => 513,
            Error::ConfigInsufficientSignerCount(_, _) => 514,
            Error::ConfigExceededSignerCount(_, _) => 515,
            Error::ConfigInvalidSignerAddress(_) => 521,
            Error::ConfigReoccurringSigner(_) => 516,
            Error::ConfigEmptyFeedIds => 517,
            Error::ConfigExceededFeedIdsLength(_, _) => 522,
            Error::ConfigInvalidFeedId(_) => 523,
            Error::ConfigReoccurringFeedId(_) => 518,
            Error::TimestampDifferentThanOthers(_, _) => 519,
            Error::SignerNotRecognized(_) => 520,
            Error::InsufficientSignerCount(data_package_index, value, _) => {
                (2000 + data_package_index * 10 + value) as u16
            }
            Error::SizeNotSupported(size) => 600 + *size as u16,
            Error::CryptographicError(error) => 700 + error.code(),
            Error::TimestampTooOld(data_package_index, _) => 1000 + *data_package_index as u16,
            Error::TimestampTooFuture(data_package_index, _) => 1050 + *data_package_index as u16,
            Error::DataTimestampMustBeGreaterThanBefore(_, _) => 1101,
            Error::CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp(_, _) => 1102,
            Error::NumberConversionFail => 1200,
            Error::UsizeOverflow => 1300,
            Error::DataStaleness { .. } => 1400,
            Error::BufferOverflow => 1500,
        }
    }
}
