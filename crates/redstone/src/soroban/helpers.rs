use soroban_sdk::String;

pub trait ToBytes {
    fn to_bytes<const N: usize>(&self) -> [u8; N];
}

impl ToBytes for String {
    fn to_bytes<const N: usize>(&self) -> [u8; N] {
        let mut bytes = [0u8; N];
        let len = self.len() as usize;
        assert!(len <= bytes.len());
        self.copy_into_slice(&mut bytes[..len]);
        bytes
    }
}
