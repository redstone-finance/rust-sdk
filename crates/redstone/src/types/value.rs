use alloc::vec::Vec;

#[cfg(feature = "radix")]
use scrypto::prelude::*;

use crate::types::{Sanitized, VALUE_SIZE};
/// Type describing values we are getting from and to network.
/// We expect it to be at most u256 and reserve that many bytes for it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "radix", derive(ScryptoSbor))]
pub struct Value(pub [u8; VALUE_SIZE]);

#[cfg(feature = "extra")]
macro_rules! impl_from_number {
    ($(
        $number_type:ident
    ),*) => {
        $(
            impl From<$number_type> for Value {
                fn from(value: $number_type) -> Self {
                    Value::from_u256(alloy_primitives::U256::from(value))
                }
            }
         )*
    };
}
#[cfg(feature = "extra")]
impl_from_number!(u8, u16, u32, u64, u128);

impl Value {
    pub fn to_u256(self) -> alloy_primitives::U256 {
        alloy_primitives::U256::from_be_bytes(self.0)
    }

    pub fn from_u256(value: alloy_primitives::U256) -> Self {
        Self(value.to_be_bytes())
    }

    pub fn le_bytes(&self) -> [u8; 32] {
        let mut le = self.0;
        le.reverse();

        le
    }

    pub fn as_be_bytes(&self) -> &[u8] {
        &self.0
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
