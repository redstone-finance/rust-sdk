//! Solana extension
//!
//! Implementation of the config suited for the solana network, with the crypto operations using anchor_lang (solana) specific operations

use alloc::string::ToString;

use anchor_lang::{
    error::{AnchorError, Error as AnchorLangError},
    solana_program::{
        keccak::hash,
        secp256k1_recover::{secp256k1_recover, Secp256k1RecoverError},
    },
};

use crate::{
    core::config::Config as RedstoneConfig,
    crypto::{Crypto, CryptoError},
    network::{error::Error, StdEnv},
    ConfigConstants, FeedId, RedStoneConfigImpl, TimestampMillis,
};

impl From<Error> for AnchorLangError {
    fn from(value: Error) -> Self {
        AnchorError {
            error_name: "rust-sdk".to_string(),
            error_code_number: value.code() as u32,
            error_msg: value.to_string(),
            error_origin: None,
            compared_values: None,
        }
        .into()
    }
}

/// Implementation of `RedstoneConfig` specialized for operations on the solana.
pub type SolanaRedStoneConfig = RedStoneConfigImpl<SolanaCrypto, SolanaEnv>;

pub type SolanaEnv = StdEnv;
pub struct SolanaCrypto;

impl Crypto for SolanaCrypto {
    type KeccakOutput = [u8; 32];

    fn keccak256(&mut self, input: impl AsRef<[u8]>) -> Self::KeccakOutput {
        hash(input.as_ref()).to_bytes()
    }

    fn recover_public_key(
        &mut self,
        recovery_byte: u8,
        signature_bytes: impl AsRef<[u8]>,
        message_hash: Self::KeccakOutput,
    ) -> Result<crate::Bytes, crate::CryptoError> {
        let key = secp256k1_recover(
            message_hash.as_ref(),
            recovery_byte,
            signature_bytes.as_ref(),
        )
        .map_err(|error| match error {
            Secp256k1RecoverError::InvalidHash => CryptoError::RecoverPreHash,
            Secp256k1RecoverError::InvalidRecoveryId => CryptoError::RecoveryByte(recovery_byte),
            Secp256k1RecoverError::InvalidSignature => {
                CryptoError::Signature(signature_bytes.as_ref().to_vec())
            }
        })?
        .to_bytes();

        let mut uncompressed_key = vec![0x04];
        uncompressed_key.extend_from_slice(&key);

        Ok(uncompressed_key.into())
    }
}

/// Helper function to build a SolanaRedStoneConfig from a ConfigConstants implementation.
/// This simplifies the creation of RedStone configs in connector code.
pub fn build_solana_config(
    config_constants: &impl ConfigConstants,
    feed_id: FeedId,
    block_timestamp: TimestampMillis,
) -> Result<SolanaRedStoneConfig, Error> {
    let config = RedstoneConfig::try_new(
        config_constants.signer_count_threshold(),
        config_constants.redstone_signers(),
        alloc::vec![feed_id],
        block_timestamp,
        Some(config_constants.max_timestamp_delay_ms().into()),
        Some(config_constants.max_timestamp_ahead_ms().into()),
    )?;

    Ok((config, SolanaCrypto).into())
}

#[cfg(test)]
mod tests {
    use crate::{crypto::recovery_key_tests::run_all_testcases, solana::SolanaCrypto};

    #[test]
    fn test_default_crypto_impl() {
        run_all_testcases(&mut SolanaCrypto);
    }
}
