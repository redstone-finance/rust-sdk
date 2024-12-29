use crate::types::{Sanitized, VALUE_SIZE};
#[cfg(feature = "radix")]
use scrypto::prelude::*;
/// Type describing address of signer. Typically pubkey of length 20 bytes;
/// As of right now we dont expect larger keys than 32 bytes.
/// The address is normalized to contain only lowercase letters (A-F) -> (a-f).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]

#[cfg_attr(feature = "radix", derive(ScryptoSbor))]
pub struct SignerAddress([u8; VALUE_SIZE]);

impl SignerAddress {
    pub fn new(raw_address: [u8; VALUE_SIZE]) -> Self {
        Self(
            raw_address
                .to_ascii_lowercase()
                .try_into()
                .expect("We know the lenght eq 32"),
        )
    }
}
use alloc::vec::Vec;
impl From<Vec<u8>> for SignerAddress {
    fn from(value: Vec<u8>) -> Self {
        let value = value.sanitized();

        let mut buff = [0; VALUE_SIZE];
        buff[0..value.len()].copy_from_slice(&value);

        Self::new(buff)
    }
}
