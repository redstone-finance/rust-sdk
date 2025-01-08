/// Performs lookup for repeated value in the list.
///
/// # Arguments
///
/// * `list` - A `&[T]` list where `T` implements `PartialEq`, `Eq` and `Copy` traits.
///
/// # Returns
///
/// Returns a `Option<T>` which contains first repeated element found in the list or if there is no repeated
/// element then None otherwise.
pub fn has_repetition<T>(list: &[T]) -> Option<T>
where
    T: PartialEq + Eq + Copy + Ord,
{
    if list.len() < 2 {
        return None;
    }
    if list.len() == 2 {
        if list[0] == list[1] {
            return Some(list[0]);
        }
        return None;
    }
    let mut list: Vec<T> = list.to_vec();
    list.sort();
    for i in 1..list.len() {
        if list[i - 1] == list[i] {
            return Some(list[i]);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_repetition_quadratic_lookup_for_u32_unique_lists() {
        let mut test_cases: [Vec<u32>; 5] = [
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![1, 3, 2, 4, 5, 7, 6, 9, 8],
            vec![],
            vec![1],
        ];

        for test_case in test_cases.iter_mut() {
            let result = has_repetition(test_case);
            assert_eq!(result, None)
        }
    }

    #[test]
    fn test_has_repetition_quadratic_lookup_for_u32_not_unique_lists() {
        let mut test_cases: [(Vec<u32>, Option<u32>); 5] = [
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 1], Some(1)),
            (vec![1, 2, 3, 4, 5, 6, 2, 8, 9], Some(2)),
            (vec![1, 2, 2, 1], Some(1)),
            (vec![9, 8, 7, 1, 2, 3, 1, 2, 3], Some(1)),
            (vec![1, 2, 3, 4, 5, 5, 6, 7, 8, 9], Some(5)),
        ];

        for test_case in test_cases.iter_mut() {
            let result = has_repetition(&mut test_case.0);
            assert_eq!(result, test_case.1)
        }
    }

    #[test]
    fn test_has_repetition_quadratic_lookup_for_str_unique_lists() {
        let mut test_cases: [Vec<&str>; 5] = [
            vec!["a", "b", "c", "d", "e"],
            vec!["e", "c", "b", "d", "a"],
            vec!["aaa", "aab", "aac", "aad", "aae"],
            vec![],
            vec!["a"],
        ];

        for test_case in test_cases.iter_mut() {
            let result = has_repetition(test_case);
            assert_eq!(result, None)
        }
    }

    #[test]
    fn test_has_repetition_quadratic_lookup_for_str_nor_uniqie_lists() {
        let mut test_cases: [(Vec<&str>, Option<&str>); 5] = [
            (vec!["a", "b", "c", "d", "a"], Some("a")),
            (vec!["a", "b", "c", "b", "e"], Some("b")),
            (vec!["a", "b", "c", "a", "b"], Some("a")),
            (vec!["a", "b", "b", "b", "a"], Some("a")),
            (vec!["a", "b", "c", "d", "b"], Some("b")),
        ];

        for test_case in test_cases.iter_mut() {
            let result = has_repetition(&mut test_case.0);
            assert_eq!(result, test_case.1)
        }
    }
}
