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

pub use core::FeedValue;

pub use crypto::{Crypto, CryptoError};
use network::Environment;
pub use types::{Bytes, FeedId, SignerAddress, TimestampMillis, Value};
pub use utils::median::Avg;

use alloc::vec::Vec;

use crate::{core::config::Config, network::error::Error};

/// Trait for connector config constants that can be used to build RedStone configs.
/// This allows connectors to define simple constant configs without implementing
/// the full RedStoneConfig trait themselves.
pub trait ConfigFactory<X, C: Crypto> {
    /// The minimum number of signers required for validation.
    fn signer_count_threshold(&self) -> u8;
    
    /// Converts the signers to a vector of SignerAddress.
    fn redstone_signers(&self) -> Vec<SignerAddress>;
    
    /// Maximum delay of the package against the current block timestamp (in milliseconds).
    fn max_timestamp_delay_ms(&self) -> u64;
    
    /// Maximum ahead of time of the package against current block timestamp (in milliseconds).
    fn max_timestamp_ahead_ms(&self) -> u64;

    fn crypto(x: X) -> C;

    fn build_config<E: Environment>(&self, x: X, feeds: Vec<FeedId>, block_timestamp: TimestampMillis) -> Result<RedStoneConfigImpl<C, E>, Error> {
        let config = Config::try_new(
            self.signer_count_threshold(),
            self.redstone_signers(),
            feeds,
            block_timestamp,
            Some(self.max_timestamp_delay_ms().into()),
            Some(self.max_timestamp_ahead_ms().into()),
        )?;
        let crypto = Self::crypto(x);
        Ok((config, crypto).into())
    }
}

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
