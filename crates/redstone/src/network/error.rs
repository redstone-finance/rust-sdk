use alloc::{string::String, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter},
    num::TryFromIntError,
};

use crate::{
    network::as_str::{AsAsciiStr, AsHexStr},
    types::Value,
    CryptoError, FeedId, SignerAddress, TimestampMillis,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ContractErrorContent {
    pub code: u8,
    pub msg: String,
}

/// Errors that can be encountered in the deserializing&decrypting the RedStone payload or just contract execution process.
///
/// These errors include issues with contract logic, data types,
/// cryptographic operations, and conditions specific to the requirements.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Error {
    /// Represents errors that arise from the contract itself.
    ///
    /// This variant is used for encapsulating errors that are specific to the contract's logic
    /// or execution conditions that aren't covered by more specific error types.
    ContractError(ContractErrorContent),

    /// Indicates an overflow error with `U256` numbers.
    ///
    /// Used when operations on `U256` numbers exceed their maximum value, potentially leading
    /// to incorrect calculations or state.
    NumberOverflow(Value),

    /// Used when an expected non-empty array or vector is found to be empty.
    ///
    /// This could occur in scenarios where the contract logic requires a non-empty collection
    /// of items for the correct operation, for example, during aggregating the values.
    ArrayIsEmpty,

    /// Represents errors related to cryptographic operations.
    ///
    /// This includes failures in signature verification, hashing, or other cryptographic
    /// processes.
    CryptographicError(CryptoError),

    /// Signifies that an unsupported size was encountered.
    ///
    /// This could be used when a data structure or input does not meet the expected size
    /// requirements for processing.
    SizeNotSupported(usize),

    /// Indicates that the marker bytes for RedStone are incorrect.
    ///
    /// This error is specific to scenarios where marker or identifier bytes do not match
    /// expected values, potentially indicating corrupted or tampered data.
    WrongRedStoneMarker(Vec<u8>),

    /// Used when there is leftover data in a payload that should have been empty.
    ///
    /// This could indicate an error in data parsing or that additional, unexpected data
    /// was included in a message or transaction.
    NonEmptyPayloadRemainder(usize),

    /// Indicates that the recovered signer address is not recognized.
    ///
    /// Includes SignerAddress that was recovered.
    SignerNotRecognized(SignerAddress),

    /// Indicates that the number of signers does not meet the required threshold.
    ///
    /// This variant includes the current number of signers, the required threshold, and
    /// potentially a feed_id related to the operation that failed due to insufficient signers.
    InsufficientSignerCount(usize, usize, FeedId),

    /// Used when a timestamp is older than allowed by the processor logic.
    ///
    /// Includes the position or identifier of the timestamp and the threshold value,
    /// indicating that the provided timestamp is too far in the past.
    TimestampTooOld(usize, TimestampMillis),

    /// Indicates that a timestamp is further in the future than allowed.
    ///
    /// Similar to `TimestampTooOld`, but for future timestamps exceeding the contract's
    /// acceptance window.
    TimestampTooFuture(usize, TimestampMillis),

    /// Indicates that a FeedId is reoccurring in data points.
    ///
    /// Includes FeedId that is reoccurring.
    ReoccurringFeedId(FeedId),

    /// ConfigInsufficientSignerCount occurs
    /// when the number of signers is not at least equal the required signer threshold.
    ///
    /// Includes current config signer list length and minimum required signer count.
    ConfigInsufficientSignerCount(u8, u8),

    /// ConfigExceededSignerCount occurs
    /// when the number of signers is larger than the config max-allowed signer count.
    /// Look in to core::config crate to acknowledge constant value.
    ///
    /// Includes current config signer list length and maximum allowed signer count per config.
    ConfigExceededSignerCount(usize, usize),

    /// Indicates that a SignerAddress is reoccurring on the config signer list.
    ///
    /// Includes SignerAddress that is reoccurring.
    ConfigReoccurringSigner(SignerAddress),

    /// Indicates that the list doesn't contain FeedIds.
    ConfigEmptyFeedIds,

    /// Indicates that a FeedId is reoccurring on the config feed_ids list.
    ///
    /// Includes FeedId that is reoccurring.
    ConfigReoccurringFeedId(FeedId),

    /// Indicates that payload timestamps are not equal.
    ///
    /// Contains the first timestamp and the one that is not equal to the first one.
    TimestampDifferentThanOthers(TimestampMillis, TimestampMillis),

    /// Indicates that the provided data timestamp is not greater than a previously written package timestamp.
    ///
    /// For the price adapter to accept a new price update, the associated timestamp must be
    /// strictly greater than the timestamp of the last update. This error is raised if a new
    /// timestamp does not meet this criterion, ensuring the chronological integrity of price data.
    ///
    /// Includes the value of a current package timestamp and the timestamp of the previous package.
    DataTimestampMustBeGreaterThanBefore(TimestampMillis, TimestampMillis),

    /// Indicates that the current update timestamp is not greater than the last update timestamp.
    ///
    /// This error is raised to ensure that the data being written has a timestamp strictly greater
    /// than the most recent timestamp already stored in the system. It guarantees that new data
    /// is not outdated or stale compared to the existing records, thereby maintaining the chronological
    /// integrity and consistency of the updates.
    ///
    /// Includes the value of the current update timestamp and the last update timestamp.
    CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp(TimestampMillis, TimestampMillis),

    /// Indicates error while converting from one type of the integer to the other.
    NumberConversionFail,

    /// Indicates error of usize overflow.
    UsizeOverflow,
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
            Error::ConfigReoccurringSigner(_) => 516,
            Error::ConfigEmptyFeedIds => 517,
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
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "helpers")]
        match self {
            Error::ContractError(boxed) => write!(f, "Contract error: {}", boxed.msg),
            Error::NumberOverflow(number) => write!(f, "Number overflow: {}", number.to_u256()),
            Error::ArrayIsEmpty => write!(f, "Array is empty"),
            Error::CryptographicError(error) => write!(f, "Cryptographic Error: {:?}", error),
            Error::SizeNotSupported(size) => write!(f, "Size not supported: {}", size),
            Error::WrongRedStoneMarker(bytes) => {
                write!(f, "Wrong RedStone marker: {}", bytes.as_hex_str())
            }
            Error::NonEmptyPayloadRemainder(len) => {
                write!(f, "Non empty payload len remainder: {}", len)
            }
            Error::SignerNotRecognized(signer) => {
                write!(f, "Signer not recognized: {:?}", signer)
            }
            Error::InsufficientSignerCount(data_package_index, value, feed_id) => write!(
                f,
                "Insufficient signer count {} for #{} ({})",
                value,
                data_package_index,
                feed_id.as_ascii_str()
            ),
            Error::TimestampTooOld(data_package_index, value) => {
                write!(
                    f,
                    "Timestamp {:?} is too old for #{}",
                    value, data_package_index
                )
            }
            Error::TimestampTooFuture(data_package_index, value) => write!(
                f,
                "Timestamp {:?} is too future for #{}",
                value, data_package_index
            ),
            Error::ReoccurringFeedId(feed) => {
                write!(f, "Reoccurring FeedId: {feed:?} in data points")
            }
            Error::ConfigInsufficientSignerCount(got, expected) => {
                write!(f, "Wrong configuration signer count, got {got} signers, expected at minimum {expected}")
            }
            Error::ConfigExceededSignerCount(got, allowed) => {
                write!(f, "Wrong configuration signer count, got {got} signers, allowed maximum is {allowed}")
            }
            Error::ConfigReoccurringSigner(signer_address) => {
                write!(
                    f,
                    "Wrong configuration, signer address {} is reoccurring on the signer list",
                    signer_address.as_hex_str()
                )
            }
            Error::ConfigEmptyFeedIds => {
                write!(f, "Empty configuration feed ids list")
            }
            Error::ConfigReoccurringFeedId(feed_id) => {
                write!(
                    f,
                    "Wrong configuration, feed id {} is reoccurring on the feed_ids list",
                    feed_id.as_hex_str()
                )
            }
            Error::TimestampDifferentThanOthers(first, outstanding) => write!(
                f,
                "Timestamp {:?} is not equal to the first on {:?} in the payload.",
                outstanding, first
            ),
            Error::DataTimestampMustBeGreaterThanBefore(current, before) => {
                write!(
                            f,
                            "Package timestamp: {current:?} must be greater than package timestamp before: {before:?}"
                        )
            }
            Error::CurrentTimestampMustBeGreaterThanLatestUpdateTimestamp(current, last) => {
                write!(
                            f,
                            "Current update timestamp: {current:?} must be greater than latest update timestamp: {last:?}"
                        )
            }
            Error::NumberConversionFail => {
                write!(f, "Number conversion failed")
            }
            Error::UsizeOverflow => write!(f, "Usize overflow"),
        }

        #[cfg(not(feature = "helpers"))]
        Ok(())
    }
}
