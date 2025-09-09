use core::ops::{BitAnd, Shr};
use primitive_types::U256;

use crate::{
    types::VALUE_SIZE,
    u256::{FromBeBytes, IntoBeBytes},
    Avg,
};

impl FromBeBytes for U256 {
    fn from_be_bytes(value: [u8; VALUE_SIZE]) -> Self {
        Self::from_big_endian(&value)
    }
}

impl IntoBeBytes for U256 {
    fn into_be_bytes(self) -> [u8; VALUE_SIZE] {
        self.to_big_endian()
    }
}

impl Avg for U256 {
    fn avg(self, other: Self) -> Self {
        let one = Self::one();

        self.shr(one) + other.shr(one) + (self.bitand(one) + other.bitand(one)).shr(one)
    }
}
