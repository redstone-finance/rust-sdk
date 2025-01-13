#[cfg(feature = "radix")]
use scrypto::prelude::*;

use crate::types::{Sanitized, VALUE_SIZE};
use alloc::vec::Vec;

/// Type describing feed ids.
/// We expect FeedId to be byte string like b"EUR"
/// converted to bytearray and padded with zeroes to the right.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "radix", derive(ScryptoSbor))]
pub struct FeedId([u8; VALUE_SIZE]);

impl AsRef<[u8]> for FeedId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

// trim zeros from both sides
fn trim_zeros(v: Vec<u8>) -> Vec<u8> {
    if v.is_empty() {
        return v;
    }
    let l_index = match v.iter().position(|&byte| byte != 0) {
        Some(position) => position,
        _ => return vec![], // vec of all zeroes
    };

    let r_index = match v.iter().rposition(|&byte| byte != 0) {
        Some(position) => position,
        _ => return vec![], // not possible but vec of all zeroes
    };

    v[l_index..=r_index].into()
}

impl From<Vec<u8>> for FeedId {
    fn from(value: Vec<u8>) -> Self {
        let value = trim_zeros(value);
        let value = value.sanitized();

        let mut buff = [0; VALUE_SIZE];
        buff[0..value.len()].copy_from_slice(&value);

        Self(buff)
    }
}
