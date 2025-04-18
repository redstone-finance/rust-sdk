use alloc::vec::Vec;
pub trait FilterSome<Output> {
    fn filter_some(&self) -> Output;
}

impl<T: Copy> FilterSome<Vec<T>> for [Option<T>] {
    fn filter_some(&self) -> Vec<T> {
        self.iter().filter_map(|&opt| opt).collect()
    }
}

#[cfg(test)]
mod filter_some_tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use crate::utils::filter::FilterSome;

    #[test]
    fn test_filter_some() {
        let values = [None, Some(23u64), None, Some(12), Some(12), None, Some(23)];

        assert_eq!(values.filter_some(), vec![23, 12, 12, 23])
    }
}
