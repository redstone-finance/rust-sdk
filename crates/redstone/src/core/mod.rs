mod aggregator;
pub mod config;
pub mod processor;
pub mod processor_result;
pub mod validator;

pub use aggregator::FeedValue;
pub use processor::process_payload;
pub use processor_result::ProcessorResult;

#[cfg(test)]
mod test_helpers;
