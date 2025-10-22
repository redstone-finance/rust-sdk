use alloc::vec::Vec;
use core::fmt::Debug;

use thiserror::Error;

use crate::{network::as_str::AsHexStr, Bytes, SignerAddress};

const ECDSA_N_DIV_2: [u8; 32] =
    hex_literal::hex!("7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0");

const ECDSA_N: [u8; 32] =
    hex_literal::hex!("ffffffffffffffffffffffffffffffffbaaedce6af48a03bbfd25e8cd0364141");

const SIGNATURE_SIZE_BS: usize = 65;
const SIGNATURE_COMPONENT_SIZE: usize = 32;
const SIGNATURE_S_OFFSET: usize = 32;
const SIGNATURE_RS_SIZE: usize = 64;

#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum CryptoError {
    #[error("Invalid recovery byte: {0}")]
    RecoveryByte(u8),
    #[error("Invalid signature: {}", .0.as_hex_str())]
    Signature(Vec<u8>),
    #[error("Could not recover from PreHash")]
    RecoverPreHash,
    #[error("Invalid signature length: {0}")]
    InvalidSignatureLen(usize),
}
impl CryptoError {
    pub fn code(&self) -> u16 {
        match self {
            CryptoError::RecoveryByte(byte) => *byte as u16,
            CryptoError::Signature(vec) => vec.len() as u16,
            CryptoError::RecoverPreHash => 0,
            CryptoError::InvalidSignatureLen(len) => *len as u16,
        }
    }
}

pub trait Crypto {
    type KeccakOutput: AsRef<[u8]>;

    fn keccak256(&mut self, input: impl AsRef<[u8]>) -> Self::KeccakOutput;

    fn recover_public_key(
        &mut self,
        recovery_byte: u8,
        signature_bytes: impl AsRef<[u8]>,
        message_hash: Self::KeccakOutput,
    ) -> Result<Bytes, CryptoError>;

    fn recover_address<A: AsRef<[u8]>, B: AsRef<[u8]>>(
        &mut self,
        message: A,
        signature: B,
    ) -> Result<SignerAddress, CryptoError> {
        let signature = signature.as_ref();
        if signature.len() != SIGNATURE_SIZE_BS {
            return Err(CryptoError::InvalidSignatureLen(signature.len()));
        }

        check_signature_bounds(signature)?;
        check_signature_malleability(signature)?;
        let recovery_byte = check_recovery_byte(signature[64])?; // 65-byte representation

        let msg_hash = self.keccak256(message);
        let key = self.recover_public_key(recovery_byte, &signature[..64], msg_hash)?;
        let key_hash = self.keccak256(&key.as_ref()[1..]); // skip first uncompressed-key byte

        Ok(key_hash.as_ref()[12..].to_vec().into()) // last 20 bytes
    }
}

fn check_signature_bounds(sig: &[u8]) -> Result<(), CryptoError> {
    let r: [u8; SIGNATURE_COMPONENT_SIZE] = sig[..SIGNATURE_S_OFFSET]
        .try_into()
        .expect("Slice is of length 32");
    let s: [u8; SIGNATURE_COMPONENT_SIZE] = sig[SIGNATURE_S_OFFSET..SIGNATURE_RS_SIZE]
        .try_into()
        .expect("Slice is of length 32");

    let r_is_zero = r == [0u8; SIGNATURE_COMPONENT_SIZE];
    let s_is_zero = s == [0u8; SIGNATURE_COMPONENT_SIZE];
    let r_exceeds_n = r >= ECDSA_N;

    if r_is_zero || s_is_zero || r_exceeds_n {
        return Err(CryptoError::Signature(sig.to_vec()));
    }

    Ok(())
}

fn check_signature_malleability(sig: &[u8]) -> Result<(), CryptoError> {
    let s: [u8; SIGNATURE_COMPONENT_SIZE] = sig[SIGNATURE_S_OFFSET..SIGNATURE_RS_SIZE]
        .try_into()
        .expect("Slice is of length 32");

    if s > ECDSA_N_DIV_2 {
        return Err(CryptoError::Signature(sig.to_vec()));
    }

    Ok(())
}
fn check_recovery_byte(recovery_byte: u8) -> Result<u8, CryptoError> {
    let normalized = match recovery_byte {
        0 | 1 => recovery_byte,
        27 | 28 => recovery_byte - 27,
        _ => return Err(CryptoError::RecoveryByte(recovery_byte)),
    };

    Ok(normalized)
}

#[cfg(test)]
#[allow(dead_code)] // this is test template for crypto implementations
pub mod recovery_key_tests {
    use alloc::{borrow::ToOwned, string::ToString};

    use redstone_utils::hex::hex_to_bytes;

    use crate::{Crypto, CryptoError};

    const MESSAGE: &str = "415641580000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000d394303d018d79bf0ba000000020000001";
    const MESSAGE_HASH: &str = "f0805644755393876d0e917e553f0c206f8bc68b7ebfe73a79d2a9e7f5a4cea6";
    const SIG_V27: &str = "475195641dae43318e194c3d9e5fc308773d6fdf5e197e02644dfd9ca3d19e3e2bd7d8656428f7f02e658a16b8f83722169c57126cc50bec8fad188b1bac6d19";
    const SIG_V28: &str = "c88242d22d88252c845b946c9957dbf3c7d59a3b69ecba2898198869f9f146ff268c3e47a11dbb05cc5198aadd659881817a59ee37e088d3253f4695927428c1";
    const PUBLIC_KEY_V27: &str =
        "04f5f035588502146774d0ccfd62ee5bf1d7f1dbb96aae33a79765c636b8ec75a36f5121931b5cc37215a7d4280c5700ca92daaaf93c32b06ca9f98b1f4ece624e";
    const PUBLIC_KEY_V28: &str =
        "04626f2ad2cfb0b41a24276d78de8959bcf45fc5e80804416e660aab2089d15e98206526e639ee19d17c8f9ae0ce3a6ff1a8ea4ab773d0fb4214e08aad7ba978c8";
    const ADDRESS_V27: &str = "2c59617248994D12816EE1Fa77CE0a64eEB456BF";
    const ADDRESS_V28: &str = "12470f7aBA85c8b81D63137DD5925D6EE114952b";

    /// run testcases against implementation of the RecovePublicKey.
    pub fn run_all_testcases<T>(crypto: &mut T)
    where
        T: Crypto<KeccakOutput = [u8; 32]>,
    {
        test_recover_public_key_v27(crypto);
        test_recover_public_key_v28(crypto);
        test_recover_address_1b(crypto);
        test_recover_address_1c(crypto);
        test_signature_malleability(crypto);
        test_invalid_recovery_id(crypto);
        test_signature_r_zero(crypto);
        test_signature_s_zero(crypto);
        test_signature_r_exceeds_n(crypto);
    }

    fn test_recover_public_key_v27<T>(crypto: &mut T)
    where
        T: Crypto<KeccakOutput = [u8; 32]>,
    {
        let public_key =
            crypto.recover_public_key(0, hex_to_bytes(SIG_V27.into()), u8_slice(MESSAGE_HASH));

        assert_eq!(Ok(hex_to_bytes(PUBLIC_KEY_V27.into()).into()), public_key);
    }

    fn test_recover_public_key_v28<T>(crypto: &mut T)
    where
        T: Crypto<KeccakOutput = [u8; 32]>,
    {
        let public_key =
            crypto.recover_public_key(1, hex_to_bytes(SIG_V28.into()), u8_slice(MESSAGE_HASH));

        assert_eq!(Ok(hex_to_bytes(PUBLIC_KEY_V28.into()).into()), public_key);
    }

    pub fn test_recover_address_1b<T, K>(crypto: &mut T)
    where
        T: Crypto<KeccakOutput = K>,
        K: AsRef<[u8]>,
    {
        let address = crypto.recover_address(
            hex_to_bytes(MESSAGE.into()),
            hex_to_bytes(SIG_V27.to_owned() + "1b"),
        );

        assert_eq!(Ok(hex_to_bytes(ADDRESS_V27.into()).into()), address);
    }

    pub fn test_invalid_recovery_id<T, K>(crypto: &mut T)
    where
        T: Crypto<KeccakOutput = K>,
        K: AsRef<[u8]>,
    {
        let result = crypto.recover_address(
            hex_to_bytes(MESSAGE.into()),
            hex_to_bytes(SIG_V27.to_owned() + "1d"),
        );

        assert_eq!(Err(CryptoError::RecoveryByte(29)), result);
    }

    pub fn test_recover_address_1c<T, K>(crypto: &mut T)
    where
        T: Crypto<KeccakOutput = K>,
        K: AsRef<[u8]>,
    {
        let address = crypto.recover_address(
            hex_to_bytes(MESSAGE.into()),
            hex_to_bytes(SIG_V28.to_owned() + "1c"),
        );

        assert_eq!(Ok(hex_to_bytes(ADDRESS_V28.into()).into()), address);
    }

    pub fn test_signature_malleability<T, K>(crypto: &mut T)
    where
        T: Crypto<KeccakOutput = K>,
        K: AsRef<[u8]>,
    {
        let msg =
        hex_to_bytes("4254430000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000058f32c910a001924dc0bd5000000020000001".to_string());

        let signature =
        hex_to_bytes("6307247862e106f0d4b3cde75805ababa67325953145aa05bdd219d90a741e0eeba79b756bf3af6db6c26a8ed3810e3c584379476fd83096218e9deb95a7617e1b".to_string());

        let result = crypto.recover_address(&msg, &signature);
        assert_eq!(result, Err(CryptoError::Signature(signature)));
    }

    pub fn test_signature_r_zero<T, K>(crypto: &mut T)
    where
        T: Crypto<KeccakOutput = K>,
        K: AsRef<[u8]>,
    {
        let msg = hex_to_bytes(MESSAGE.into());
        let signature = hex_to_bytes(
            "0000000000000000000000000000000000000000000000000000000000000000\
            475195641dae43318e194c3d9e5fc308773d6fdf5e197e02644dfd9ca3d19e3e1b"
                .to_string(),
        );

        let result = crypto.recover_address(&msg, &signature);

        assert_eq!(result, Err(CryptoError::Signature(signature)));
    }

    pub fn test_signature_s_zero<T, K>(crypto: &mut T)
    where
        T: Crypto<KeccakOutput = K>,
        K: AsRef<[u8]>,
    {
        let msg = hex_to_bytes(MESSAGE.into());
        let signature = hex_to_bytes(
            "475195641dae43318e194c3d9e5fc308773d6fdf5e197e02644dfd9ca3d19e3e\
            00000000000000000000000000000000000000000000000000000000000000001b"
                .to_string(),
        );

        let result = crypto.recover_address(&msg, &signature);

        assert_eq!(result, Err(CryptoError::Signature(signature)));
    }

    pub fn test_signature_r_exceeds_n<T, K>(crypto: &mut T)
    where
        T: Crypto<KeccakOutput = K>,
        K: AsRef<[u8]>,
    {
        let msg = hex_to_bytes(MESSAGE.into());
        let signature = hex_to_bytes(
            "ffffffffffffffffffffffffffffffffbaaedce6af48a03bbfd25e8cd0364141\
            475195641dae43318e194c3d9e5fc308773d6fdf5e197e02644dfd9ca3d19e3e1b"
                .to_string(),
        );

        let result = crypto.recover_address(&msg, &signature);

        assert_eq!(result, Err(CryptoError::Signature(signature)));
    }

    fn u8_slice<const N: usize>(str: &str) -> [u8; N] {
        hex_to_bytes(str.into()).as_slice().try_into().unwrap()
    }
}
