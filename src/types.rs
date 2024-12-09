use core::fmt::Debug;

/// We dont expect value to be larger from u256. Adjust this once this no longer hold :)
pub const VALUE_SIZE: usize = 32;

pub trait Sanitized {
    fn sanitized(self) -> Self;
}

impl Sanitized for Vec<u8> {
    fn sanitized(self) -> Self {
        if self.len() <= VALUE_SIZE {
            return self;
        }

        let index = self.len().max(VALUE_SIZE) - VALUE_SIZE;
        let remainder = &self[0..index];

        if remainder != vec![0; index] {
            panic!("Number to big: {:?} digits", self.len())
        }

        self[index..].into()
    }
}

/// Type describing feed ids.
/// We expect FeedId to be byte string like b"EUR"
/// converted to bytearray and padded with zeroes to the right.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FeedId(pub [u8; 32]);

/// Type describing address of signer. Typically pubkey of length 20 bytes;
/// As of right now we dont expect larger keys than 32 bytes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SignerAddress(pub [u8; 32]);

/// Type describing values we are getting from and to network.
///  We expect it to be at most u256 and reserve that many bytes for it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Value(pub [u8; 32]);

macro_rules! from_number {
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

from_number!(u8, u16, u32, u64, u128);

impl Value {
    pub fn to_u256(self) -> primitive_types::U256 {
        primitive_types::U256::from_big_endian(&self.0)
    }

    pub fn from_u256(value: primitive_types::U256) -> Self {
        value.to_big_endian().to_vec().into()
    }
}

/// Type describing timpestamp, we use to directly show we expect milliseconds.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockTimestampMillis(u64);

impl Debug for BlockTimestampMillis {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<u64> for BlockTimestampMillis {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl BlockTimestampMillis {
    pub fn from_millis(millis: u64) -> Self {
        Self(millis)
    }

    pub fn as_millis(&self) -> u64 {
        self.0
    }

    pub fn add(&self, other: impl Into<Self>) -> Self {
        Self(self.0 + other.into().0)
    }

    pub fn is_before(&self, other: Self) -> bool {
        self.0 <= other.0
    }

    pub fn is_after(&self, other: Self) -> bool {
        self.0 >= other.0
    }
}

fn trim_zeros(v: Vec<u8>) -> Vec<u8> {
    if v.is_empty() {
        return v;
    }
    let l_index = {
        let mut i = 0;
        while i < v.len() && v[i] == 0 {
            i += 1;
        }
        i
    };

    let r_index = {
        let mut i = v.len() - 1;
        while i != 0 && v[i] == 0 {
            i -= 1;
        }
        i
    };

    v[l_index..=r_index].into()
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Bytes(pub Vec<u8>);

impl From<Vec<u8>> for FeedId {
    fn from(value: Vec<u8>) -> Self {
        let value = trim_zeros(value);
        let value = value.sanitized();

        let mut buff = [0; 32];
        buff[0..value.len()].copy_from_slice(&value);

        Self(buff)
    }
}

impl From<Vec<u8>> for SignerAddress {
    fn from(value: Vec<u8>) -> Self {
        let value = value.sanitized();

        let mut buff = [0; 32];
        buff[0..value.len()].copy_from_slice(&value);

        Self(buff)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        let value = value.sanitized();

        let mut buff = [0; 32];
        buff[32 - value.len()..].copy_from_slice(&value);

        Self(buff)
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
