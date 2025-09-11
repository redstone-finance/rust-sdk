use core::ops::Sub;

use crate::{types::VALUE_SIZE, Value};

macro_rules! impl_from_uint {
    ($($t:ty),* $(,)?) => {
        $(
            impl From<$t> for Value {
                fn from(value: $t) -> Self {
                    let mut buff = [0; VALUE_SIZE];
                    buff[VALUE_SIZE - size_of::<$t>()..].copy_from_slice(&value.to_be_bytes());

                    Value(buff)
                }
            }
        )*
    };
}

impl_from_uint!(u8, u16, u32, u64, u128);

impl Sub<u8> for Value {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        let mut result = self.0;
        let mut borrow = false;
        let mut remaining = rhs;

        for i in (0..VALUE_SIZE).rev() {
            let (intermediate, overflow1) = result[i].overflowing_sub(remaining);
            let (final_byte, overflow2) = intermediate.overflowing_sub(borrow as u8);

            result[i] = final_byte;
            borrow = overflow1 || overflow2;
            remaining = 0;
        }

        Value(result)
    }
}
