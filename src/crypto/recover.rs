use crate::crypto::{
    keccak256, recover::crypto256::recover_public_key, EcdsaUncompressedPublicKey, Keccak256Hash,
    Secp256SigRs,
};

pub fn recover_address(message: Vec<u8>, signature: Vec<u8>) -> Vec<u8> {
    //TODO: check malleability support by the libraries
    let recovery_byte = signature[64]; // 65-byte representation
    let msg_hash = keccak256::keccak256(message.as_slice());
    let key = recover_public_key(
        msg_hash,
        signature[..64].try_into().unwrap(),
        recovery_byte - (if recovery_byte >= 27 { 27 } else { 0 }),
    );
    let key_hash = keccak256::keccak256(&key[1..]); // skip first uncompressed-key byte

    key_hash[12..].into() // last 20 bytes
}

#[cfg(feature = "crypto_secp256k1")]
pub(crate) mod crypto256 {
    use super::{EcdsaUncompressedPublicKey, Keccak256Hash, Secp256SigRs};
    use crate::network::{assert::Unwrap, error::Error};
    use secp256k1::{ecdsa::RecoverableSignature, Message, Secp256k1 as Secp256k1Curve};

    pub(crate) fn recover_public_key(
        message_hash: Keccak256Hash,
        signature_bytes: Secp256SigRs,
        recovery_byte: u8,
    ) -> EcdsaUncompressedPublicKey {
        let msg = Message::from_digest(message_hash);

        let recovery_id = secp256k1::ecdsa::RecoveryId::from_i32(recovery_byte.into())
            .unwrap_or_revert(|_| Error::CryptographicError(recovery_byte.into()));

        let sig: RecoverableSignature =
            RecoverableSignature::from_compact(signature_bytes.as_slice(), recovery_id)
                .unwrap_or_revert(|_| Error::CryptographicError(signature_bytes.len()));

        let public_key = Secp256k1Curve::new().recover_ecdsa(&msg, &sig);

        public_key.unwrap().serialize_uncompressed()
    }
}

#[cfg(feature = "crypto_k256")]
pub(crate) mod crypto256 {
    use super::{EcdsaUncompressedPublicKey, Keccak256Hash, Secp256SigRs};
    use crate::network::{assert::Unwrap, error::Error};
    use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

    pub(crate) fn recover_public_key(
        message_hash: Keccak256Hash,
        signature_bytes: Secp256SigRs,
        recovery_byte: u8,
    ) -> EcdsaUncompressedPublicKey {
        let recovery_id = RecoveryId::from_byte(recovery_byte)
            .unwrap_or_revert(|_| Error::CryptographicError(recovery_byte.into()));

        let signature = Signature::try_from(signature_bytes.as_slice())
            .unwrap_or_revert(|_| Error::CryptographicError(signature_bytes.len()));

        let recovered_key =
            VerifyingKey::recover_from_prehash(message_hash.as_ref(), &signature, recovery_id)
                .map(|key| key.to_encoded_point(false).to_bytes())
                .unwrap_or_revert(|_| Error::CryptographicError(0));

        recovered_key.as_ref().try_into().unwrap()
    }
}

#[cfg(all(feature = "crypto_radix", target_arch = "wasm32"))]
pub(crate) mod crypto256 {
    use super::{EcdsaUncompressedPublicKey, Keccak256Hash, Secp256SigRs};
    use crate::network::assert::Unwrap;
    use crate::network::error::Error;
    use radix_common::crypto::{Hash, IsHash, Secp256k1Signature};
    use scrypto::crypto_utils::CryptoUtils;

    pub(crate) fn recover_public_key(
        message_hash: Keccak256Hash,
        signature_bytes: Secp256SigRs,
        recovery_byte: u8,
    ) -> EcdsaUncompressedPublicKey {
        let hash = Hash::from_bytes(message_hash);

        let mut sig_vec = Vec::with_capacity(65);
        sig_vec.push(recovery_byte);
        sig_vec.extend(signature_bytes);
        let signature = Secp256k1Signature::try_from(sig_vec.as_slice())
            .unwrap_or_revert(|_| Error::CryptographicError(signature_bytes.len()));

        CryptoUtils::secp256k1_ecdsa_verify_and_key_recover_uncompressed(hash, signature).0
    }
}

#[cfg(feature = "crypto_solana")]
pub(crate) mod crypto256 {
    use super::{EcdsaUncompressedPublicKey, Keccak256Hash, Secp256SigRs};
    use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;

    pub(crate) fn recover_public_key(
        message_hash: Keccak256Hash,
        signature_bytes: Secp256SigRs,
        recovery_byte: u8,
    ) -> EcdsaUncompressedPublicKey {
        let key = secp256k1_recover(&message_hash, recovery_byte, &signature_bytes)
            .unwrap().0;
        let mut uncompressed_key = [0u8; 65];
        uncompressed_key[0] = 0x04;
        uncompressed_key[1..].copy_from_slice(&key);

        uncompressed_key
    }
}

#[cfg(all(
    not(feature = "crypto_solana"),
    not(feature = "crypto_k256"),
    not(feature = "crypto_secp256k1"),
    not(all(feature = "crypto_radix", target_arch = "wasm32"))
))]
pub(crate) mod crypto256 {
    use super::{EcdsaUncompressedPublicKey, Keccak256Hash, Secp256SigRs};

    pub(crate) fn recover_public_key(
        _message_hash: Keccak256Hash,
        _signature_bytes: Secp256SigRs,
        _recovery_byte: u8,
    ) -> EcdsaUncompressedPublicKey {
        panic!("Not implemented!")
    }
}

#[cfg(not(all(feature = "crypto_radix", target_arch = "wasm32")))]
#[cfg(feature = "helpers")]
#[cfg(test)]
mod tests {
    use crate::{
        crypto::recover::{crypto256::recover_public_key, recover_address},
        helpers::hex::hex_to_bytes,
    };

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

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

    #[test]
    fn test_recover_public_key_v27() {
        let public_key = recover_public_key(u8_slice(MESSAGE_HASH), u8_slice(SIG_V27), 0);

        assert_eq!(u8_slice(PUBLIC_KEY_V27), public_key);
    }

    #[test]
    fn test_recover_public_key_v28() {
        let public_key = recover_public_key(u8_slice(MESSAGE_HASH), u8_slice(SIG_V28), 1);

        assert_eq!(u8_slice(PUBLIC_KEY_V28), public_key);
    }

    #[test]
    fn test_recover_address_1b() {
        let address = recover_address(
            hex_to_bytes(MESSAGE.into()),
            hex_to_bytes(SIG_V27.to_owned() + "1b"),
        );

        assert_eq!(hex_to_bytes(ADDRESS_V27.into()), address);
    }

    #[test]
    fn test_recover_address_1c() {
        let address = recover_address(
            hex_to_bytes(MESSAGE.into()),
            hex_to_bytes(SIG_V28.to_owned() + "1c"),
        );

        assert_eq!(hex_to_bytes(ADDRESS_V28.into()), address);
    }

    fn u8_slice<const N: usize>(str: &str) -> [u8; N] {
        hex_to_bytes(str.into()).as_slice().try_into().unwrap()
    }
}
