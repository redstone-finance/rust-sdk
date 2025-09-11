use core::ops::Sub;

use crate::{types::VALUE_SIZE, Value};

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        let mut buff = [0; VALUE_SIZE];
        buff[VALUE_SIZE - size_of::<u8>()] = value;

        Value(buff)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        let mut buff = [0; VALUE_SIZE];
        buff[VALUE_SIZE - size_of::<u16>()..].copy_from_slice(&value.to_be_bytes());

        Value(buff)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        let mut buff = [0; VALUE_SIZE];
        buff[VALUE_SIZE - size_of::<u32>()..].copy_from_slice(&value.to_be_bytes());

        Value(buff)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        let mut buff = [0; VALUE_SIZE];
        buff[VALUE_SIZE - size_of::<u64>()..].copy_from_slice(&value.to_be_bytes());

        Value(buff)
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        let mut buff = [0; VALUE_SIZE];
        buff[VALUE_SIZE - size_of::<u128>()..].copy_from_slice(&value.to_be_bytes());

        Value(buff)
    }
}

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
