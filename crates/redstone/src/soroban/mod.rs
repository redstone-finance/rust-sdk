//! Soroban (Stellar) extension
//!
//! Implementation of the config suited for the stellar network,
//! with the crypto operations using soroban specific operations

pub mod helpers;

use soroban_sdk::{crypto::Hash, Bytes as SorobanBytes, BytesN as SorobanBytesN, Env};

use crate::{
    crypto::{Crypto, CryptoError},
    network::StdEnv,
    types::Bytes,
    RedStoneConfigImpl,
};

/// Implementation of `RedstoneConfig` specialized for operations on the soroban (stellar).
pub type SorobanRedStoneConfig<'a> =
    RedStoneConfigImpl<SorobanCrypto<'a>, SorobanEnv, alloy_primitives::U256>;

pub type SorobanEnv = StdEnv;

pub struct SorobanCrypto<'a> {
    env: &'a Env,
}

impl<'a> SorobanCrypto<'a> {
    pub fn new(env: &'a Env) -> Self {
        Self { env }
    }
}

pub struct Keccak256Output {
    hash: Hash<32>,
    data: [u8; 32],
}

impl Keccak256Output {
    fn new(hash: Hash<32>) -> Self {
        let data = hash.to_array();
        Self { hash, data }
    }
}

impl AsRef<[u8]> for Keccak256Output {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Crypto for SorobanCrypto<'_> {
    type KeccakOutput = Keccak256Output;

    fn keccak256(&mut self, input: impl AsRef<[u8]>) -> Self::KeccakOutput {
        let soroban_bytes = SorobanBytes::from_slice(self.env, input.as_ref());
        Keccak256Output::new(self.env.crypto().keccak256(&soroban_bytes))
    }

    fn recover_public_key(
        &mut self,
        recovery_byte: u8,
        signature_bytes: impl AsRef<[u8]>,
        message_hash: Self::KeccakOutput,
    ) -> Result<Bytes, CryptoError> {
        let sig_bytes = signature_bytes.as_ref();

        let Ok(sig_array) = sig_bytes.try_into() else {
            return Err(CryptoError::InvalidSignatureLen(sig_bytes.len()));
        };
        let signature = SorobanBytesN::<64>::from_array(self.env, &sig_array);
        let public_key = self.env.crypto().secp256k1_recover(
            &message_hash.hash,
            &signature,
            recovery_byte.into(),
        );

        let mut bytes = vec![0u8; public_key.len() as usize];
        public_key.as_ref().copy_into_slice(&mut bytes);

        Ok(Bytes::from(bytes))
    }
}

#[cfg(test)]
mod tests {
    use soroban_sdk::Env;

    use super::SorobanCrypto;
    use crate::crypto::recovery_key_tests::{
        test_recover_address_1b, test_recover_address_1c, test_signature_malleability,
    };

    #[test]
    fn test_default_crypto_impl() {
        let env = Env::default();
        let mut crypto = SorobanCrypto::new(&env);
        // Soroban SDK doesn't provide any method to construct Hash
        // from raw bytes.  So recover_public_key() is untestable.
        // test_recover_public_key_v27();
        // test_recover_public_key_v28();
        test_recover_address_1b(&mut crypto);
        test_recover_address_1c(&mut crypto);
        test_signature_malleability(&mut crypto);
    }
}
