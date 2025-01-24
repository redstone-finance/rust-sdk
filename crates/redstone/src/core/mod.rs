pub mod config;
pub mod processor;
pub mod processor_result;

mod aggregator;
pub(super) mod validator;

pub use processor::process_payload;
pub use processor_result::ProcessorResult;

#[cfg(feature = "helpers")]
#[cfg(test)]
mod test_helpers;
