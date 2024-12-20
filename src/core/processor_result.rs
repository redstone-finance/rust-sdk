use crate::{network::error::Error, types::Value, TimestampMillis};

pub type ProcessorResult = Result<ValidatedPayload, Error>;

/// Represents the result of processing the RedStone payload.
///
/// This structure is used to encapsulate the outcome of a RedStone payload processing operation,
/// particularly focusing on time-sensitive data and its associated values, according to the `Config`.
#[derive(Debug, Eq, PartialEq)]
pub struct ValidatedPayload {
    /// The minimum timestamp encountered during processing.
    ///
    /// This field captures the earliest time point (in milliseconds since the Unix epoch)
    /// among the processed data packages, indicating the starting boundary of the dataset's time range.
    pub min_timestamp: TimestampMillis,

    /// A collection of values processed during the operation.
    ///
    /// Each element in this vector represents a processed value corresponding
    /// to the passed data_feed item in the `Config`.
    pub values: Vec<Value>,
}

impl From<ValidatedPayload> for (TimestampMillis, Vec<Value>) {
    fn from(validated_payload: ValidatedPayload) -> Self {
        (validated_payload.min_timestamp, validated_payload.values)
    }
}
