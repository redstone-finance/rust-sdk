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

pub mod contract;
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
pub mod radix;

#[cfg(feature = "soroban")]
pub mod soroban;

use ::core::marker::PhantomData;
#[cfg(feature = "default-crypto")]
pub mod default_ext;

pub use crypto::{Crypto, CryptoError};
use network::Environment;
pub use types::{Bytes, FeedId, SignerAddress, TimestampMillis, Value};

use crate::core::config::Config;

/// Configuration for the redstone protocol.
/// Pluggable with custom environments and possible specialized crypto operations.
pub trait RedStoneConfig {
    /// Environment in which we execute. Provides logging etc
    type Environment: Environment;

    /// Returns config for payload decoding and validation.
    fn config(&self) -> &Config;
    /// Crypto operations needed for address recovery.
    fn crypto_mut(&mut self) -> &mut impl Crypto;
}

pub struct RedStoneConfigImpl<C, E> {
    inner: Config,
    crypto: C,
    _phantom: PhantomData<E>,
}

impl<C, E> From<(Config, C)> for RedStoneConfigImpl<C, E> {
    fn from(value: (Config, C)) -> Self {
        Self {
            inner: value.0,
            crypto: value.1,
            _phantom: PhantomData,
        }
    }
}

impl<C, E> RedStoneConfig for RedStoneConfigImpl<C, E>
where
    C: Crypto,
    E: Environment,
{
    type Environment = E;

    fn config(&self) -> &Config {
        &self.inner
    }

    fn crypto_mut(&mut self) -> &mut impl Crypto {
        &mut self.crypto
    }
}

#[cfg(feature = "helpers")]
pub mod helpers;
