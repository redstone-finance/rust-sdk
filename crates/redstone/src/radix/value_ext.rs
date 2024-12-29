use scrypto::math::{Decimal, ParseDecimalError, U256};

use crate::Value;

impl TryFrom<Value> for Decimal {
    type Error = ParseDecimalError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let le_bytes = value.le_bytes();

        let u256 = U256::from_le_bytes(&le_bytes);

        u256.try_into()
    }
}
