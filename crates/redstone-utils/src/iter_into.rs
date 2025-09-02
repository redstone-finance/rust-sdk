use alloc::vec::Vec;

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

#[cfg(test)]
mod iter_into_tests {
    #[cfg(test)]
    mod tests {
        use crate::iter_into::{IterInto, IterIntoOpt, OptIterIntoOpt};

        #[test]
        fn test_iter_into_basic_conversion() {
            let input: Vec<i32> = vec![1, 2, 3, 4];
            let result: Vec<i64> = input.iter_into();
            let expected = vec![1i64, 2i64, 3i64, 4i64];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_iter_into_empty_vec() {
            let input: Vec<i32> = vec![];
            let result: Vec<i64> = input.iter_into();
            let expected: Vec<i64> = vec![];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_iter_into_same_type() {
            let input = vec![1, 2, 3];
            let result: Vec<i32> = input.iter_into();

            assert_eq!(result, input);
            assert_ne!(result.as_ptr(), input.as_ptr());
        }

        #[test]
        fn test_iter_into_float_conversion() {
            let input = vec![1, 2, 3];
            let result: Vec<f64> = input.iter_into();
            let expected = vec![1.0, 2.0, 3.0];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_opt_iter_into_opt_all_some() {
            let input = vec![Some(1i32), Some(2i32), Some(3i32)];
            let result: Vec<Option<i64>> = input.opt_iter_into_opt();
            let expected = vec![Some(1i64), Some(2i64), Some(3i64)];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_opt_iter_into_opt_mixed_values() {
            let input = vec![Some(1i32), None, Some(3i32), None];
            let result: Vec<Option<i64>> = input.opt_iter_into_opt();
            let expected = vec![Some(1i64), None, Some(3i64), None];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_opt_iter_into_opt_all_none() {
            let input: Vec<Option<i32>> = vec![None, None, None];
            let result: Vec<Option<i64>> = input.opt_iter_into_opt();
            let expected: Vec<Option<i64>> = vec![None, None, None];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_opt_iter_into_opt_empty_vec() {
            let input: Vec<Option<i32>> = vec![];
            let result: Vec<Option<i64>> = input.opt_iter_into_opt();
            let expected: Vec<Option<i64>> = vec![];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_iter_into_opt_basic_conversion() {
            let input = vec![1i32, 2i32, 3i32];
            let result: Vec<Option<i64>> = input.iter_into_opt();
            let expected = vec![Some(1i64), Some(2i64), Some(3i64)];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_iter_into_opt_empty_vec() {
            let input: Vec<i32> = vec![];
            let result: Vec<Option<i64>> = input.iter_into_opt();
            let expected: Vec<Option<i64>> = vec![];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_iter_into_opt_chain_conversion() {
            let input = vec![1u8, 2u8, 3u8];
            let result: Vec<Option<u64>> = input.iter_into_opt();
            let expected = vec![Some(1u64), Some(2u64), Some(3u64)];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_all_traits_work_together() {
            let original = vec![1i16, 2i16, 3i16];

            let step1: Vec<i32> = original.iter_into();
            let step2: Vec<Option<i64>> = step1.iter_into_opt();
            let expected = vec![Some(1i64), Some(2i64), Some(3i64)];

            assert_eq!(step2, expected);
        }

        #[test]
        fn test_trait_bounds_with_custom_type() {
            #[derive(Debug, Copy, Clone, PartialEq)]
            struct CustomInt(i32);

            impl From<CustomInt> for i64 {
                fn from(val: CustomInt) -> Self {
                    val.0 as i64
                }
            }

            let input = vec![CustomInt(10), CustomInt(20), CustomInt(30)];
            let result: Vec<i64> = input.iter_into();
            let expected = vec![10i64, 20i64, 30i64];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_large_vector_performance() {
            let input: Vec<i32> = (0..10000).collect();
            let result: Vec<i64> = input.iter_into();

            assert_eq!(result.len(), 10000);
            assert_eq!(result[0], 0i64);
            assert_eq!(result[9999], 9999i64);
        }

        #[test]
        fn test_option_vector_with_custom_type() {
            #[derive(Debug, Copy, Clone, PartialEq)]
            struct Point(i32, i32);

            impl From<Point> for (i64, i64) {
                fn from(p: Point) -> Self {
                    (p.0 as i64, p.1 as i64)
                }
            }

            let input = vec![Some(Point(1, 2)), None, Some(Point(3, 4))];
            let result: Vec<Option<(i64, i64)>> = input.opt_iter_into_opt();
            let expected = vec![Some((1i64, 2i64)), None, Some((3i64, 4i64))];

            assert_eq!(result, expected);
        }

        #[test]
        fn test_iter_into_opt_implementation_detail() {
            let input = vec![1i8, 2i8, 3i8];
            let intermediate: Vec<i32> = input.iter_into();
            let final_result: Vec<Option<i32>> = intermediate.iter_into();
            let direct_result: Vec<Option<i32>> = input.iter_into_opt();

            assert_eq!(final_result, direct_result);
        }
    }
}
