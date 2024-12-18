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

use core::config::Config;
use network::{Environment, StdEnv};

pub use crypto::{Crypto, CryptoError, DefaultCrypto};
pub use types::{Bytes, FeedId, SignerAddress, TimestampMillis, Value};

/// Configuration for the redstone protocol.
/// Pluggable with custom environments and possible speciallized crypto operations.
pub trait RedStoneConfig {
    /// Crypto operations needed for address recovery.
    type Crypto: Crypto;
    /// Environment in which we execute. Provides logging etc
    type Environment: Environment;

    /// returns config for payload decoding and validation.
    fn config(&self) -> &Config;
}

/// Standard nonspecialized implementation of the RedStoneConfig.
/// See [crate::crypto::DefaultCrypto] for more information about crypto ops used.
/// Constructuble from the [crate::core::config::Config]
pub struct StdConfig(Config);

impl From<Config> for StdConfig {
    fn from(value: Config) -> Self {
        Self(value)
    }
}

impl RedStoneConfig for StdConfig {
    type Crypto = DefaultCrypto;
    type Environment = StdEnv;

    fn config(&self) -> &Config {
        &self.0
    }
}

#[cfg(feature = "helpers")]
pub mod helpers;
