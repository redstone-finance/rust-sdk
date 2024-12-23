use alloc::vec::Vec;

use crate::types::{Sanitized, VALUE_SIZE};
/// Type describing values we are getting from and to network.
/// We expect it to be at most u256 and reserve that many bytes for it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Value(pub [u8; VALUE_SIZE]);

macro_rules! impl_from_number {
    ($(
        $number_type:ident
    ),*) => {
        $(
            impl From<$number_type> for Value {
                fn from(value: $number_type) -> Self {
                    Value::from_u256(primitive_types::U256::from(value))
                }
            }
         )*
    };
}
impl_from_number!(u8, u16, u32, u64, u128);

impl Value {
    pub fn to_u256(self) -> primitive_types::U256 {
        primitive_types::U256::from_big_endian(&self.0)
    }

    pub fn from_u256(value: primitive_types::U256) -> Self {
        value.to_big_endian().to_vec().into()
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
