use alloc::vec::Vec;

use crate::{core::aggregator::FeedValue, network::error::Error, TimestampMillis};

pub type ProcessorResult = Result<ValidatedPayload, Error>;

/// Represents the result of processing the RedStone payload.
///
/// This structure is used to encapsulate the outcome of a RedStone payload processing operation,
/// particularly focusing on time-sensitive data and its associated values, according to the `Config`.
#[derive(Debug, Eq, PartialEq)]
pub struct ValidatedPayload {
    /// The timestamp encountered during processing.
    ///
    /// This field captures the time point (in milliseconds since the Unix epoch)
    /// among the processed data packages, indicating the starting boundary of the dataset's time range.
    pub timestamp: TimestampMillis,

    /// A collection of values processed during the operation.
    ///
    /// Each element in this vector represents a processed value corresponding
    /// to the passed data_feed item in the `Config`.
    pub values: Vec<FeedValue>,
}

impl From<ValidatedPayload> for (TimestampMillis, Vec<FeedValue>) {
    fn from(validated_payload: ValidatedPayload) -> Self {
        (validated_payload.timestamp, validated_payload.values)
    }
}
