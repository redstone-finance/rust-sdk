use alloc::vec::Vec;

use super::hex::{hex_to_bytes, make_feed_id};
use crate::{FeedId, SignerAddress};

pub trait IterInto<U> {
    fn iter_into(&self) -> U;
}

impl<U, T: Copy + Into<U>> IterInto<Vec<U>> for Vec<T> {
    fn iter_into(&self) -> Vec<U> {
        self.iter().map(|&value| value.into()).collect()
    }
}

pub trait OptIterIntoOpt<U> {
    fn opt_iter_into_opt(&self) -> U;
}

impl<U, T: Copy + Into<U>> OptIterIntoOpt<Vec<Option<U>>> for Vec<Option<T>> {
    fn opt_iter_into_opt(&self) -> Vec<Option<U>> {
        self.iter().map(|&value| value.map(|v| v.into())).collect()
    }
}

pub trait IterIntoOpt<U> {
    fn iter_into_opt(&self) -> U;
}

impl<U: Copy, T: Copy + Into<U>> IterIntoOpt<Vec<Option<U>>> for Vec<T> {
    fn iter_into_opt(&self) -> Vec<Option<U>> {
        self.iter_into().iter_into()
    }
}

macro_rules! impl_iter_into_with_converter {
    ($(
        ($receiver:ident,
        $converter:expr)
    ),*) => {
        $(
            impl IterInto<Vec<$receiver>> for Vec<&str> {
                fn iter_into(&self) -> Vec<$receiver> {
                    self
                        .into_iter()
                        .map(|v| $converter((*v).into()).into())
                        .collect()
                }
            }
        )*
    };
}
impl_iter_into_with_converter!((SignerAddress, hex_to_bytes), (FeedId, make_feed_id));

#[cfg(test)]
mod iter_into_tests {
    use alloc::vec::Vec;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use crate::{
        helpers::iter_into::{IterInto, IterIntoOpt, OptIterIntoOpt},
        Value,
    };

    #[test]
    fn test_iter_into() {
        let values = vec![23u128, 12, 12, 23];

        assert_eq!(
            values.iter_into() as Vec<Value>,
            vec![23u8.into(), 12u8.into(), 12u8.into(), 23u8.into()]
        )
    }

    #[test]
    fn test_iter_into_opt() {
        let values: Vec<u8> = vec![23u8, 12, 12, 23];

        assert_eq!(
            values.iter_into_opt(),
            vec![Some(23u8), 12u8.into(), 12u8.into(), 23u8.into()]
        )
    }

    #[test]
    fn test_opt_iter_into_opt() {
        let values: Vec<Option<u128>> =
            vec![Some(23u128), 12.into(), 12.into(), None, 23.into(), None];

        assert_eq!(
            values.opt_iter_into_opt() as Vec<Option<Value>>,
            vec![
                Some(Value::from(23u8)),
                Value::from(12u8).into(),
                Value::from(12u8).into(),
                None,
                Value::from(23u8).into(),
                None
            ]
        )
    }
}
