use alloc::vec::Vec;
use core::{
    fmt::Display,
    ops::{Add, BitAnd, Shr},
};

#[cfg(feature = "radix")]
use scrypto::prelude::*;

use crate::types::{Sanitized, VALUE_SIZE};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
#[cfg_attr(feature = "radix", derive(ScryptoSbor))]
pub struct Value(pub [u8; VALUE_SIZE]);

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

    pub fn max() -> Self {
        Value([u8::MAX; VALUE_SIZE])
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

impl BitAnd<u8> for Value {
    type Output = Self;

    fn bitand(self, rhs: u8) -> Self::Output {
        let mut result = [0u8; VALUE_SIZE];

        result[VALUE_SIZE - 1] = self.0[VALUE_SIZE - 1] & rhs;

        Self(result)
    }
}

impl Shr<u32> for Value {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        if rhs >= (VALUE_SIZE as u32 * 8) {
            return Value([0u8; VALUE_SIZE]);
        }

        let byte_size = 8;

        let byte_shift = (rhs / byte_size as u32) as usize;
        let bit_shift = (rhs % byte_size as u32) as u8;
        let mut result = [0u8; VALUE_SIZE];

        if bit_shift == 0 {
            result[byte_shift..].copy_from_slice(&self.0[..VALUE_SIZE - byte_shift]);
        } else {
            #[allow(clippy::needless_range_loop)]
            for i in byte_shift..VALUE_SIZE {
                let source_idx = i - byte_shift;
                result[i] = self.0[source_idx] >> bit_shift;

                if source_idx > 0 {
                    result[i] |= self.0[source_idx - 1] << (byte_size - bit_shift);
                }
            }
        }

        Self(result)
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.0;
        let mut carry = 0u16;

        for i in (0..VALUE_SIZE).rev() {
            carry += result[i] as u16 + rhs.0[i] as u16;
            result[i] = (carry & 0xff) as u8;
            carry >>= 8;
        }

        Value(result)
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use crate::{types::VALUE_SIZE, Value};

    #[test]
    fn test_sub_u8_edge_cases() {
        let zero: Value = 0u8.into();
        let one: Value = 1u8.into();
        let ten: Value = 10u8.into();
        let max_u8: Value = 255u8.into();

        assert_eq!(zero - 0, zero);
        assert_eq!(one - 1, zero);
        assert_eq!(ten - 5, Value::from(5u8));
        assert_eq!(max_u8 - 255, zero);

        let val_256: Value = 256u16.into();
        let expected: Value = 255u8.into();
        assert_eq!(val_256 - 1, expected);

        let large_val: Value = 0x1000u16.into();
        let result = large_val - 0xff;
        let expected_large: Value = 0x0f01u16.into();
        assert_eq!(result, expected_large);
    }

    #[test]
    fn test_sub_u8_manual_u256() {
        let high_byte_set = Value([
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);

        let expected_borrow = Value([
            0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ]);

        assert_eq!(high_byte_set - 1, expected_borrow);

        let middle_byte = Value([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);

        let expected_middle_borrow = Value([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ]);

        assert_eq!(middle_byte - 1, expected_middle_borrow);
    }

    #[test]
    fn test_sub_u8_random() {
        let mut rng = thread_rng();

        for _ in 0..1000 {
            let base_u8 = rng.gen::<u8>();
            let sub_u8 = rng.gen_range(0..=base_u8);

            let base_value: Value = base_u8.into();
            let result = base_value - sub_u8;
            let expected: Value = (base_u8 - sub_u8).into();

            assert_eq!(result, expected);

            let base_u16 = rng.gen::<u16>();
            let sub_u8_small = rng.gen_range(0..=255u8);

            if base_u16 >= sub_u8_small as u16 {
                let base_value: Value = base_u16.into();
                let result = base_value - sub_u8_small;
                let expected: Value = (base_u16 - sub_u8_small as u16).into();

                assert_eq!(result, expected);
            }
        }
    }

    #[test]
    #[allow(clippy::erasing_op)]
    fn test_bitand_u8_edge_cases() {
        let zero: Value = 0u8.into();
        let max_u8: Value = 255u8.into();
        let pattern: Value = 0xaau8.into();

        assert_eq!(zero & 0xff, zero);
        assert_eq!(max_u8 & 0xff, max_u8);
        assert_eq!(max_u8 & 0x00, zero);
        assert_eq!(pattern & 0x55, zero);
        assert_eq!(pattern & 0xaa, pattern);

        let large_value: Value = 0x12345678u32.into();
        let result = large_value & 0xff;
        let expected: Value = 0x78u8.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitand_u8_manual_u256() {
        let complex_pattern = Value([
            0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
            0x11, 0x00, 0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78, 0x87, 0x96, 0xa5, 0xb4,
            0xc3, 0xd2, 0xe1, 0xf0,
        ]);

        let result = complex_pattern & 0x0f;
        let expected = Value([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);

        assert_eq!(result, expected);

        let result_aa = complex_pattern & 0xaa;
        let expected_aa = Value([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0xa0,
        ]);

        assert_eq!(result_aa, expected_aa);
    }

    #[test]
    fn test_bitand_u8_random() {
        let mut rng = thread_rng();

        for _ in 0..1000 {
            let base_u8 = rng.gen::<u8>();
            let mask_u8 = rng.gen::<u8>();

            let base_value: Value = base_u8.into();
            let result = base_value & mask_u8;
            let expected: Value = (base_u8 & mask_u8).into();

            assert_eq!(result, expected);

            let base_u32 = rng.gen::<u32>();
            let mask_u8_rand = rng.gen::<u8>();

            let base_value: Value = base_u32.into();
            let result = base_value & mask_u8_rand;
            let expected: Value = (base_u32 & mask_u8_rand as u32).into();

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_shr_u32_edge_cases() {
        let zero: Value = 0u8.into();
        let one: Value = 1u8.into();
        let max_u8: Value = 255u8.into();

        assert_eq!(zero >> 0, zero);
        assert_eq!(zero >> 100, zero);
        assert_eq!(zero >> 256, zero);

        assert_eq!(one >> 0, one);
        assert_eq!(one >> 1, zero);

        let two: Value = 2u8.into();
        assert_eq!(two >> 1, one);

        let val_256: Value = 256u16.into();
        assert_eq!(val_256 >> 8, one);

        assert_eq!(max_u8 >> 8, zero);

        let large_shift_test: Value = 0x8000u16.into();
        let expected_shift: Value = 0x0800u16.into();
        assert_eq!(large_shift_test >> 4, expected_shift);
    }

    #[test]
    fn test_shr_u32_manual_u256() {
        let high_bit_pattern = Value([
            0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01, 0x80, 0x40, 0x20, 0x10, 0x08, 0x04,
            0x02, 0x01, 0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01, 0x80, 0x40, 0x20, 0x10,
            0x08, 0x04, 0x02, 0x01,
        ]);

        let shifted_1 = high_bit_pattern >> 1;
        assert_eq!(shifted_1.0[0], 0x40);
        assert_eq!(shifted_1.0[1], 0x20);

        let shifted_8 = high_bit_pattern >> 8;
        assert_eq!(shifted_8.0[0], 0x00);
        assert_eq!(shifted_8.0[1], 0x80);
        assert_eq!(shifted_8.0[8], 0x01);
        assert_eq!(shifted_8.0[9], 0x80);

        let shifted_large = high_bit_pattern >> 256;
        let zero = Value([0u8; 32]);
        assert_eq!(shifted_large, zero);

        let mixed_pattern = Value([
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x01, 0x02, 0x03, 0x04,
            0x05, 0x06, 0x07, 0x08,
        ]);

        let shifted_4 = mixed_pattern >> 4;
        assert_eq!(shifted_4.0[0], 0x01);
        assert_eq!(shifted_4.0[1], 0x23);
        assert_eq!(shifted_4.0[2], 0x45);
    }

    #[test]
    fn test_shr_u32_random() {
        let mut rng = thread_rng();

        for _ in 0..1000 {
            let base_u8 = rng.gen::<u8>();
            let shift = rng.gen_range(0..16u32);

            let base_value: Value = base_u8.into();
            let result = base_value >> shift;
            let expected: Value = if shift >= 8 {
                0u8.into()
            } else {
                (base_u8 >> shift).into()
            };

            assert_eq!(result, expected, "{shift}");

            let base_u32 = rng.gen::<u32>();
            let shift_32 = rng.gen_range(0..40u32);

            let base_value: Value = base_u32.into();
            let result = base_value >> shift_32;

            if shift_32 >= 32 {
                let zero: Value = 0u8.into();
                assert_eq!(result, zero);
            } else {
                let expected: Value = (base_u32 >> shift_32).into();
                assert_eq!(result, expected);
            }
        }
    }

    #[test]
    fn test_add_edge_cases() {
        let zero: Value = 0u8.into();
        let one: Value = 1u8.into();

        assert_eq!(zero + zero, zero);
        assert_eq!(zero + one, one);
        assert_eq!(one + zero, one);

        let a = 127u8;
        let b = 128u8;
        let a_value: Value = a.into();
        let b_value: Value = b.into();
        assert_eq!(a_value + b_value, Value::from(a as u16 + b as u16));

        let carry_test_a = 255u8;
        let carry_test_b = 1u8;
        let a_value: Value = carry_test_a.into();
        let b_value: Value = carry_test_b.into();
        assert_eq!(a_value + b_value, Value::from(256u16));

        let large_a = u64::MAX;
        let large_b = 1u64;
        let a_value: Value = large_a.into();
        let b_value: Value = large_b.into();
        assert_eq!(a_value + b_value, Value::from(u128::from(u64::MAX) + 1));
    }

    #[test]
    fn test_add_manual_u256() {
        let max_low_half = Value([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ]);

        let one = Value([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ]);

        let expected_carry = Value([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);

        assert_eq!(max_low_half + one, expected_carry);

        let all_bytes_max = Value([0xff; 32]);
        let result = all_bytes_max + one;
        let expected_overflow = Value([0u8; 32]);
        assert_eq!(result, expected_overflow);

        let pattern_a = Value([
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
            0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x66, 0x77, 0x88, 0x99,
        ]);

        let pattern_b = Value([
            0xed, 0xcb, 0xa9, 0x87, 0x65, 0x43, 0x21, 0x0f, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99,
            0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00, 0xee, 0xdd, 0xcc, 0xbb, 0xaa,
            0x99, 0x88, 0x77, 0x66,
        ]);

        let sum = pattern_a + pattern_b;
        assert_eq!(sum.0, [0xff; VALUE_SIZE]);
    }

    #[test]
    fn test_add_random() {
        let mut rng = thread_rng();

        for _ in 0..1000 {
            let a = rng.gen::<u8>();
            let b = rng.gen::<u8>();
            let a_value: Value = a.into();
            let b_value: Value = b.into();
            assert_eq!(a_value + b_value, Value::from(a as u16 + b as u16));

            let a_u32 = rng.gen::<u32>();
            let b_u32 = rng.gen::<u32>();
            let a_value: Value = a_u32.into();
            let b_value: Value = b_u32.into();
            assert_eq!(a_value + b_value, Value::from(a_u32 as u64 + b_u32 as u64));

            let a_u64 = rng.gen::<u64>();
            let b_u64 = rng.gen::<u64>();
            let a_value: Value = a_u64.into();
            let b_value: Value = b_u64.into();
            assert_eq!(
                a_value + b_value,
                Value::from(a_u64 as u128 + b_u64 as u128)
            );

            let zero: Value = 0u8.into();
            assert_eq!(a_value + zero, a_value);
            assert_eq!(zero + a_value, a_value);
        }
    }

    #[test]
    fn test_operations_properties() {
        let mut rng = thread_rng();

        for _ in 0..100 {
            let a: Value = rng.gen::<u32>().into();
            let b: Value = rng.gen::<u32>().into();
            let c: Value = rng.gen::<u32>().into();

            assert_eq!((a + b) + c, a + (b + c));

            let shift_amount = rng.gen_range(0..32u32);
            let double_shift = (a >> (shift_amount / 2)) >> (shift_amount - shift_amount / 2);
            let single_shift = a >> shift_amount;
            assert_eq!(double_shift, single_shift);

            let mask = rng.gen::<u8>();
            let masked = a & mask;
            assert_eq!(masked.0[..31], [0u8; 31]);
        }
    }
}
