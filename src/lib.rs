//! # RedStone
//!
//! `redstone` is a collection of utilities to make deserializing&decrypting RedStone payload.
//! It includes a pure Rust implementation, along with extensions for certain networks.
//!
//! Different crypto-mechanisms are easily injectable.
//! The current implementation contains `secp256k1`- and `k256`-based variants.

pub mod core;
mod crypto;
pub mod network;
mod protocol;
mod types;
mod utils;

pub use types::{Bytes, FeedId, SignerAddress, TimestampMillis, Value};

#[cfg(feature = "helpers")]
pub mod helpers;
