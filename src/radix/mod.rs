//! Radix extension
//!
//! Implementation of the config suited for the radix network.

use alloc::vec::Vec;
use alloc::string::String;

use scrypto::{
    crypto::{keccak256_hash, Hash, IsHash, Secp256k1Signature},
    prelude::{CryptoUtils, info},
};

use crate::{Bytes, Crypto, CryptoError};
use crate::Environment;
use crate::RedStoneConfigImpl;


/// Implementation of `RedstoneConfig` specialized for operations on the radix.
pub type RadixRedStoneConfig = RedStoneConfigImpl<RadixCrypto, RadixEnv>;

pub struct RadixCrypto;
pub struct RadixEnv;

impl Environment for RadixEnv {
    fn print<F: FnOnce() -> String>(_print_content: F) {
        info!("{}", _print_content());
    }
}

impl Crypto for RadixCrypto {
    type KeccakOutput = [u8; 32];

    fn keccak256(input: impl AsRef<[u8]>) -> Self::KeccakOutput {
        keccak256_hash(input.as_ref()).into_bytes()
    }

    fn recover_public_key(
        recovery_byte: u8,
        signature_bytes: impl AsRef<[u8]>,
        message_hash: Self::KeccakOutput,
    ) -> Result<Bytes, CryptoError> {
        let hash = Hash::from_bytes(message_hash);
        let mut sig_vec = Vec::with_capacity(65);
        sig_vec.push(recovery_byte);
        sig_vec.extend(signature_bytes.as_ref());
        let signature = Secp256k1Signature::try_from(sig_vec.as_slice())
            .map_err(|_| CryptoError::Signature(sig_vec))?;

        let pk = CryptoUtils::secp256k1_ecdsa_verify_and_key_recover_uncompressed(hash, signature)
            .0
            .to_vec();

        Ok(pk.into())
    }
}
