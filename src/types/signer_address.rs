use crate::types::Sanitized;

/// Type describing address of signer. Typically pubkey of length 20 bytes;
/// As of right now we dont expect larger keys than 32 bytes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SignerAddress(pub [u8; 32]);

impl From<Vec<u8>> for SignerAddress {
    fn from(value: Vec<u8>) -> Self {
        let value = value.sanitized();

        let mut buff = [0; 32];
        buff[0..value.len()].copy_from_slice(&value);

        Self(buff)
    }
}
