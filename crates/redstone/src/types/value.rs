use core::fmt::Display;

use alloc::vec::Vec;

#[cfg(feature = "radix")]
use scrypto::prelude::*;

use crate::{
    types::{Sanitized, VALUE_SIZE},
    u256::U256,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
#[cfg_attr(feature = "radix", derive(ScryptoSbor))]
pub struct Value(pub [u8; VALUE_SIZE]);

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        let mut buff = [0; VALUE_SIZE];
        buff[VALUE_SIZE - 1] = value;

        Value(buff)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        let mut buff = [0; VALUE_SIZE];
        buff[VALUE_SIZE - 2..].copy_from_slice(&value.to_be_bytes());

        Value(buff)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        let mut buff = [0; VALUE_SIZE];
        buff[VALUE_SIZE - 4..].copy_from_slice(&value.to_be_bytes());

        Value(buff)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        let mut buff = [0; VALUE_SIZE];
        buff[VALUE_SIZE - 8..].copy_from_slice(&value.to_be_bytes());

        Value(buff)
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        let mut buff = [0; VALUE_SIZE];
        buff[VALUE_SIZE - 16..].copy_from_slice(&value.to_be_bytes());

        Value(buff)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x")?;
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl Value {
    pub fn le_bytes(&self) -> [u8; 32] {
        let mut le = self.0;
        le.reverse();

        le
    }

    pub fn as_be_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == [0; 32]
    }

    pub fn from_u256<U: U256>(value: U) -> Self {
        Self(value.into_be_bytes())
    }

    pub fn to_u256<U: U256>(self) -> U {
        U::from_be_bytes(self.0)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        let value = value.sanitized();

        let mut buff = [0; VALUE_SIZE];
        buff[VALUE_SIZE - value.len()..].copy_from_slice(&value);

        Self(buff)
    }
}
