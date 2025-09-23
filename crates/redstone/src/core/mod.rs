mod aggregator;
pub mod config;
mod matrix;
pub mod processor;
pub mod processor_result;
pub mod validator;

pub use aggregator::FeedValue;
#[cfg(feature = "bench")]
pub use processor::decode_payload;
pub use processor::process_payload;
pub use processor_result::ProcessorResult;

#[cfg(test)]
mod test_helpers;
