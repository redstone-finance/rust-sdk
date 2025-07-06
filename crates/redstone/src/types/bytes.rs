use alloc::vec::Vec;
/// Type wrapping bytes represantion.

#[cfg_attr(feature = "extra", derive(Clone, PartialEq, Eq, Debug, Default))]
pub struct Bytes(pub Vec<u8>);

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
