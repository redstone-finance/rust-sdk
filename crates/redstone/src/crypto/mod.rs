use alloc::vec::Vec;
use core::fmt::Debug;

use primitive_types::U256;

use crate::{Bytes, SignerAddress};

const ECDSA_N_DIV_2: U256 = U256([
    16134479119472337056,
    6725966010171805725,
    18446744073709551615,
    9223372036854775807,
]);

const SIGNATURE_SIZE_BS: usize = 65;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CryptoError {
    RecoveryByte(u8),
    Signature(Vec<u8>),
    RecoverPreHash,
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

    fn keccak256(input: impl AsRef<[u8]>) -> Self::KeccakOutput;

    fn recover_public_key(
        recovery_byte: u8,
        signature_bytes: impl AsRef<[u8]>,
        message_hash: Self::KeccakOutput,
    ) -> Result<Bytes, CryptoError>;

    fn recover_address<A: AsRef<[u8]>, B: AsRef<[u8]>>(
        message: A,
        signature: B,
    ) -> Result<SignerAddress, CryptoError> {
        let signature = signature.as_ref();
        if signature.len() != SIGNATURE_SIZE_BS {
            return Err(CryptoError::InvalidSignatureLen(signature.len()));
        }
        check_signature_malleability(signature)?;
        let recovery_byte = signature[64]; // 65-byte representation
        let msg_hash = Self::keccak256(message);
        let key = Self::recover_public_key(
            recovery_byte - (if recovery_byte >= 27 { 27 } else { 0 }),
            &signature[..64],
            msg_hash,
        )?;
        let key_hash = Self::keccak256(&key.as_ref()[1..]); // skip first uncompressed-key byte

        Ok(key_hash.as_ref()[12..].to_vec().into()) // last 20 bytes
    }
}

fn check_signature_malleability(sig: &[u8]) -> Result<(), CryptoError> {
    if U256::from_big_endian(&sig[32..64]) > ECDSA_N_DIV_2 {
        return Err(CryptoError::Signature(sig.to_vec()));
    }

    Ok(())
}

#[cfg(feature = "helpers")]
#[cfg(test)]
#[allow(dead_code)] // this is test template for crypto implementations
pub mod recovery_key_tests {
    use alloc::{borrow::ToOwned, string::ToString};

    use crate::{helpers::hex::hex_to_bytes, Crypto, CryptoError};

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
    pub fn run_all_testcases<T>()
    where
        T: Crypto<KeccakOutput = [u8; 32]>,
    {
        test_recover_public_key_v27::<T>();
        test_recover_public_key_v28::<T>();
        test_recover_address_1b::<T, [u8; 32]>();
        test_recover_address_1c::<T, [u8; 32]>();
        test_signature_malleability::<T, [u8; 32]>();
    }

    fn test_recover_public_key_v27<T>()
    where
        T: Crypto<KeccakOutput = [u8; 32]>,
    {
        let public_key =
            T::recover_public_key(0, hex_to_bytes(SIG_V27.into()), u8_slice(MESSAGE_HASH));

        assert_eq!(Ok(hex_to_bytes(PUBLIC_KEY_V27.into()).into()), public_key);
    }

    fn test_recover_public_key_v28<T>()
    where
        T: Crypto<KeccakOutput = [u8; 32]>,
    {
        let public_key =
            T::recover_public_key(1, hex_to_bytes(SIG_V28.into()), u8_slice(MESSAGE_HASH));

        assert_eq!(Ok(hex_to_bytes(PUBLIC_KEY_V28.into()).into()), public_key);
    }

    pub fn test_recover_address_1b<T, K>()
    where
        T: Crypto<KeccakOutput = K>,
        K: AsRef<[u8]>,
    {
        let address = T::recover_address(
            hex_to_bytes(MESSAGE.into()),
            hex_to_bytes(SIG_V27.to_owned() + "1b"),
        );

        assert_eq!(Ok(hex_to_bytes(ADDRESS_V27.into()).into()), address);
    }

    pub fn test_recover_address_1c<T, K>()
    where
        T: Crypto<KeccakOutput = K>,
        K: AsRef<[u8]>,
    {
        let address = T::recover_address(
            hex_to_bytes(MESSAGE.into()),
            hex_to_bytes(SIG_V28.to_owned() + "1c"),
        );

        assert_eq!(Ok(hex_to_bytes(ADDRESS_V28.into()).into()), address);
    }

    pub fn test_signature_malleability<T, K>()
    where
        T: Crypto<KeccakOutput = K>,
        K: AsRef<[u8]>,
    {
        let msg =
        hex_to_bytes("4254430000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000058f32c910a001924dc0bd5000000020000001".to_string());

        let signature =
        hex_to_bytes("6307247862e106f0d4b3cde75805ababa67325953145aa05bdd219d90a741e0eeba79b756bf3af6db6c26a8ed3810e3c584379476fd83096218e9deb95a7617e1b".to_string());

        let result = T::recover_address(&msg, &signature);
        assert_eq!(result, Err(CryptoError::Signature(signature)));
    }

    fn u8_slice<const N: usize>(str: &str) -> [u8; N] {
        hex_to_bytes(str.into()).as_slice().try_into().unwrap()
    }
}
