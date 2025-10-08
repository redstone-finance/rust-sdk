use alloc::vec::Vec;

#[cfg(feature = "radix")]
use scrypto::prelude::*;

use crate::types::{Sanitized, VALUE_SIZE};

/// Type describing address of signer. Typically pubkey of length 20 bytes;
/// As of right now we dont expect larger keys than 32 bytes.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug)]
#[cfg_attr(feature = "radix", derive(ScryptoSbor))]
pub struct SignerAddress([u8; VALUE_SIZE]);

impl AsRef<[u8]> for SignerAddress {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl SignerAddress {
    pub fn new(raw_address: [u8; VALUE_SIZE]) -> Self {
        Self(raw_address)
    }
}

impl From<Vec<u8>> for SignerAddress {
    fn from(value: Vec<u8>) -> Self {
        let value = value.sanitized();

        let mut buff = [0; VALUE_SIZE];
        buff[0..value.len()].copy_from_slice(&value);

        Self::new(buff)
    }
}
