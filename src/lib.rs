//! # RedStone
//!
//! `redstone` is a collection of utilities to make deserializing&decrypting RedStone payload.
//! It includes a pure Rust implementation, along with extensions for certain networks.
//!
//! Different crypto-mechanisms are easily injectable.
//! The current implementation contains `secp256k1`- and `k256`-based variants.
#![cfg_attr(not(feature = "std"), no_std)]

// todo: uncomment #![cfg_attr(not(test), warn(unused_crate_dependencies))]

#[macro_use]
extern crate alloc;

pub mod core;
mod crypto;
pub mod network;
mod protocol;
mod types;
mod utils;

#[cfg(feature = "solana")]
pub mod solana;

#[cfg(feature = "casper")]
pub mod casper;

#[cfg(feature = "radix")]
#[cfg(target_arch = "wasm32")]
pub mod radix;

use ::core::marker::PhantomData;
#[cfg(feature = "default-crypto")]
pub use crypto::DefaultCrypto;
pub use crypto::{Crypto, CryptoError};
use network::Environment;
pub use types::{Bytes, FeedId, SignerAddress, TimestampMillis, Value};

use crate::core::config::Config;

/// Configuration for the redstone protocol.
/// Pluggable with custom environments and possible specialized crypto operations.
pub trait RedStoneConfig {
    /// Crypto operations needed for address recovery.
    type Crypto: Crypto;
    /// Environment in which we execute. Provides logging etc
    type Environment: Environment;

    /// Returns config for payload decoding and validation.
    fn config(&self) -> &Config;
}

pub struct RedStoneConfigImpl<C, Env> {
    inner: Config,
    _phantom: PhantomData<(C, Env)>,
}

#[cfg(feature = "default-crypto")]
use crate::network::StdEnv;

#[cfg(feature = "default-crypto")]
/// Standard nonspecialized implementation of the RedStoneConfig.
/// See [crate::crypto::DefaultCrypto] for more information about crypto ops used.
/// Constructuble from the [crate::core::config::Config].
pub type StdRedStoneConfig = RedStoneConfigImpl<DefaultCrypto, StdEnv>;

impl<C, Env> From<Config> for RedStoneConfigImpl<C, Env> {
    fn from(value: Config) -> Self {
        Self {
            inner: value,
            _phantom: PhantomData,
        }
    }
}

impl<C, E> RedStoneConfig for RedStoneConfigImpl<C, E>
where
    C: Crypto,
    E: Environment,
{
    type Crypto = C;
    type Environment = E;

    fn config(&self) -> &Config {
        &self.inner
    }
}

#[cfg(feature = "helpers")]
pub mod helpers;
