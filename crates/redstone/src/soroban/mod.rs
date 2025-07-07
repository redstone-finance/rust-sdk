//! Soroban (Stellar) extension
//!
//! Implementation of the config suited for the stellar network,
//! with the crypto operations using soroban specific operations

use soroban_sdk::{crypto::Hash, Bytes as SorobanBytes, BytesN as SorobanBytesN, Env};

use crate::{
    crypto::{Crypto, CryptoError},
    network::StdEnv,
    types::Bytes,
    RedStoneConfigImpl,
};

/// Implementation of `RedstoneConfig` specialized for operations on the soroban (stellar).
pub type SorobanRedStoneConfig = RedStoneConfigImpl<SorobanCrypto, SorobanEnv>;

pub type SorobanEnv = StdEnv;

pub struct SorobanCrypto;

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

static mut GLOBAL_ENV: Option<Env> = None;

impl SorobanCrypto {
    pub unsafe fn set_env(env: &Env) {
        GLOBAL_ENV = Some(env.clone());
    }

    pub unsafe fn clear_env() {
        GLOBAL_ENV = None;
    }

    pub fn with_env<F, R>(env: &Env, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        unsafe {
            Self::set_env(env);
            let result = f();
            Self::clear_env();
            result
        }
    }

    fn get_env() -> &'static Env {
        unsafe {
            GLOBAL_ENV
                .as_ref()
                .expect("Env not set! Use SorobanCrypto::with_env()")
        }
    }
}

impl Crypto for SorobanCrypto {
    type KeccakOutput = Keccak256Output;

    fn keccak256(input: impl AsRef<[u8]>) -> Self::KeccakOutput {
        let env = Self::get_env();
        let soroban_bytes = SorobanBytes::from_slice(env, input.as_ref());
        Keccak256Output::new(env.crypto().keccak256(&soroban_bytes))
    }

    fn recover_public_key(
        recovery_byte: u8,
        signature_bytes: impl AsRef<[u8]>,
        message_hash: Self::KeccakOutput,
    ) -> Result<Bytes, CryptoError> {
        let env = Self::get_env();
        let sig_bytes = signature_bytes.as_ref();

        let Ok(sig_array) = sig_bytes.try_into() else {
            return Err(CryptoError::InvalidSignatureLen(sig_bytes.len()));
        };
        let signature = SorobanBytesN::<64>::from_array(env, &sig_array);
        let public_key =
            env.crypto()
                .secp256k1_recover(&message_hash.hash, &signature, recovery_byte.into());

        Ok(Bytes::from(public_key.as_ref().to_alloc_vec()))
    }
}

#[cfg(test)]
#[cfg(feature = "helpers")]
mod tests {
    use super::{Keccak256Output, SorobanCrypto};
    use crate::crypto::recovery_key_tests::{
        test_recover_address_1b, test_recover_address_1c, test_signature_malleability,
    };
    use soroban_sdk::Env;

    #[test]
    fn test_default_crypto_impl() {
        let env = Env::default();
        SorobanCrypto::with_env(&env, || {
            // Soroban SDK doesn't provide any method to construct Hash
            // from raw bytes.  So recover_public_key() is untestable.
            // test_recover_public_key_v27();
            // test_recover_public_key_v28();
            test_recover_address_1b::<SorobanCrypto, Keccak256Output>();
            test_recover_address_1c::<SorobanCrypto, Keccak256Output>();
            test_signature_malleability::<SorobanCrypto, Keccak256Output>();
        });
    }
}
