use alloc::{format, string::String, vec::Vec};

use crate::{utils::trim_zeros::TrimZeros, FeedId, SignerAddress};

pub trait AsHexStr {
    fn as_hex_str(&self) -> String;
}

impl AsHexStr for &[u8] {
    #[allow(clippy::format_collect)]
    fn as_hex_str(&self) -> String {
        self.iter().map(|byte| format!("{:02x}", byte)).collect()
    }
}

impl AsAsciiStr for FeedId {
    fn as_ascii_str(&self) -> String {
        self.as_ref().trim_zeros().as_ascii_str()
    }
}

impl AsHexStr for Vec<u8> {
    fn as_hex_str(&self) -> String {
        self.as_slice().as_hex_str()
    }
}

pub trait AsAsciiStr {
    fn as_ascii_str(&self) -> String;
}

impl AsAsciiStr for &[u8] {
    fn as_ascii_str(&self) -> String {
        self.iter().map(|&code| code as char).collect()
    }
}

impl AsAsciiStr for Vec<u8> {
    fn as_ascii_str(&self) -> String {
        self.as_slice().as_ascii_str()
    }
}

impl AsHexStr for FeedId {
    fn as_hex_str(&self) -> String {
        self.as_ref().trim_zeros().as_hex_str()
    }
}

impl AsHexStr for SignerAddress {
    fn as_hex_str(&self) -> String {
        self.as_ref().trim_zeros().as_hex_str()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use crate::{
        network::as_str::{AsAsciiStr, AsHexStr},
        types::FeedId,
    };

    const ETH: u32 = 4543560u32;

    #[test]
    fn test_as_hex_str() {
        let u256: FeedId = ETH.to_be_bytes().to_vec().into();
        let result = u256.as_hex_str();

        assert_eq!(result, "455448");
    }

    #[test]
    fn test_as_ascii_str() {
        let u256: FeedId = ETH.to_be_bytes().to_vec().into();
        let result = u256.as_ascii_str();

        assert_eq!(result, "ETH");
    }
}
