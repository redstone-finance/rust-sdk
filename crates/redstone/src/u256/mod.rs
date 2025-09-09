use crate::{types::VALUE_SIZE, Avg};

#[cfg(feature = "alloy")]
pub mod alloy;

#[cfg(feature = "primitive_types")]
pub mod primitive_types;

pub trait FromBeBytes {
    fn from_be_bytes(value: [u8; VALUE_SIZE]) -> Self;
}
pub trait IntoBeBytes {
    fn into_be_bytes(self) -> [u8; VALUE_SIZE];
}

pub trait U256: Ord + Copy + FromBeBytes + IntoBeBytes + Avg {}

impl<T> U256 for T where T: Ord + Copy + FromBeBytes + IntoBeBytes + Avg {}
