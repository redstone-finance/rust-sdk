use crate::{
    network::as_str::{AsAsciiStr, AsHexStr},
    types::Value,
    CryptoError, FeedId, TimestampMillis,
};
use alloc::{string::String, vec::Vec};
use core::fmt::{Debug, Display, Formatter};
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
}

impl From<CryptoError> for Error {
    fn from(value: CryptoError) -> Self {
        Self::CryptographicError(value)
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
            Error::InsufficientSignerCount(data_package_index, value, _) => {
                (2000 + data_package_index * 10 + value) as u16
            }
            Error::SizeNotSupported(size) => 600 + *size as u16,
            Error::CryptographicError(error) => 700 + error.code(),
            Error::TimestampTooOld(data_package_index, _) => 1000 + *data_package_index as u16,
            Error::TimestampTooFuture(data_package_index, _) => 1050 + *data_package_index as u16,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
        }
    }
}
