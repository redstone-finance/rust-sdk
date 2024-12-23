use alloc::vec::Vec;

use scrypto::{
    crypto::{keccak256_hash, Hash, IsHash, Secp256k1Signature},
    prelude::CryptoUtils,
};

use crate::{Bytes, Crypto, CryptoError};

pub struct RadixCrypto;

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

#[cfg(test)]
#[cfg(feature = "helpers")]
mod tests {
    use crate::{crypto::recovery_key_tests::run_all_testcases, radix::RadixCrypto};

    #[test]
    fn test_default_crypto_impl() {
        run_all_testcases::<RadixCrypto>();
    }
}
