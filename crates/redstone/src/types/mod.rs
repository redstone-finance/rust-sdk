mod bytes;
mod feed_id;
mod signer_address;
mod timestamp_millis;
mod value;
use alloc::vec::Vec;

pub use bytes::Bytes;
pub use feed_id::FeedId;
pub use signer_address::SignerAddress;
pub use timestamp_millis::TimestampMillis;
pub use value::Value;

/// We don't expect value to be larger than u256.
///  Adjust this once this no longer hold :)
pub(crate) const VALUE_SIZE: usize = 32;

pub trait Sanitized {
    fn sanitized(self) -> Self;
}

impl Sanitized for Vec<u8> {
    fn sanitized(mut self) -> Self {
        if self.len() <= VALUE_SIZE {
            return self;
        }

        let index = self.len().max(VALUE_SIZE) - VALUE_SIZE;
        let remainder = &self[0..index];

        if remainder.iter().any(|&byte| byte != 0) {
            panic!("Number to big: {:?} digits", self.len())
        }

        self.split_off(index)
    }
}
