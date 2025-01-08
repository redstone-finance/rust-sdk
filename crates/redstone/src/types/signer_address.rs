use core::fmt;

#[cfg(feature = "radix")]
use scrypto::prelude::*;

use crate::types::{Sanitized, VALUE_SIZE};
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
                .expect("We know the length eq 32"),
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

impl fmt::Display for SignerAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0.iter() {
            write!(f, "{:x}", byte)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_of_signer_address() {
        let test_address: SignerAddress = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ]
        .into();
        assert_eq!(
            "0x123456789abcdef101112131415161718191a1b1c1d1e1f20",
            format!("{test_address}")
        );
    }
}
