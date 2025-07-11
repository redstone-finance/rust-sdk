use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use sha3::{Digest, Keccak256};

use crate::{crypto::Crypto, network::StdEnv, Bytes, CryptoError, RedStoneConfigImpl};

/// Standard nonspecialized implementation of the RedStoneConfig.
/// Constructible from the [crate::core::config::Config].
pub type StdRedStoneConfig = RedStoneConfigImpl<DefaultCrypto, StdEnv>;

/// Default crypto operations. Uses k256 and sha3 crates.
pub struct DefaultCrypto;

type CryptoResult<T> = Result<T, CryptoError>;

impl Crypto for DefaultCrypto {
    type KeccakOutput = [u8; 32];

    fn keccak256(&mut self, input: impl AsRef<[u8]>) -> Self::KeccakOutput {
        Keccak256::new_with_prefix(input).finalize().into()
    }

    fn recover_public_key(
        &mut self,
        recovery_byte: u8,
        signature_bytes: impl AsRef<[u8]>,
        message_hash: Self::KeccakOutput,
    ) -> CryptoResult<Bytes> {
        let recovery_id =
            RecoveryId::from_byte(recovery_byte).ok_or(CryptoError::RecoveryByte(recovery_byte))?;

        let signature = Signature::try_from(signature_bytes.as_ref())
            .map_err(|_| CryptoError::Signature(signature_bytes.as_ref().to_vec()))?;

        let recovered_key =
            VerifyingKey::recover_from_prehash(message_hash.as_ref(), &signature, recovery_id)
                .map(|key| key.to_encoded_point(false))
                .map_err(|_| CryptoError::RecoverPreHash)?;

        Ok(recovered_key.as_bytes().to_vec().into())
    }
}

#[cfg(test)]
#[cfg(feature = "helpers")]
mod test {
    use crate::{crypto::recovery_key_tests::run_all_testcases, default_ext::DefaultCrypto};

    #[test]
    fn test_default_crypto_impl() {
        run_all_testcases(&mut DefaultCrypto);
    }
}
