//! # RedStone
//!
//! `redstone` is a collection of utilities to make deserializing&decrypting RedStone payload.
//! It includes a pure Rust implementation, along with extensions for certain networks.
//!
//! Different crypto-mechanisms are easily injectable.
//! The current implementation contains `secp256k1`- and `k256`-based variants.

use network::from_bytes_repr::Sanitized;

pub mod core;
mod crypto;
pub mod network;
mod protocol;
mod utils;

#[cfg(feature = "helpers")]
pub mod helpers;

/// Type describing feed ids.
/// We expect FeedId to be byte string like b"EUR"
/// converted to bytearray and padded with zeroes to the right.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FeedId(pub [u8; 32]);

impl From<Vec<u8>> for FeedId {
    fn from(value: Vec<u8>) -> Self {
        let value = value.sanitized();
        let mut buff = [0; 32];
        buff[0..value.len()].copy_from_slice(&value);

        Self(buff)
    }
}
