use alloy_primitives::U256;
use core::ops::Shr;

use crate::{
    types::VALUE_SIZE,
    u256::{FromBeBytes, IntoBeBytes},
    Avg,
};

impl FromBeBytes for U256 {
    fn from_be_bytes(value: [u8; VALUE_SIZE]) -> Self {
        Self::from_be_bytes(value)
    }
}

impl IntoBeBytes for U256 {
    fn into_be_bytes(self) -> [u8; VALUE_SIZE] {
        self.to_be_bytes()
    }
}

impl Avg for U256 {
    fn avg(self, other: Self) -> Self {
        let one = Self::ONE;

        self.shr(one) + other.shr(one) + (self.bitand(one) + other.bitand(one)).shr(one)
    }
}
