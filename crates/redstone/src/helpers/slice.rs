/// Performs guadratic lookup for repeated value in the list.
///
/// # Arguments
///
/// * `list` - A `&[T]` list where `T` implements `PartialEq`, `Eq` and `Copy` traits.
///
/// # Returns
///
/// Returns a `Option<T>` which contains first repeated element found in the list or if there is no repeated
/// element then None otherwise.
pub fn has_repetition_quadratic_lookup<T>(list: &[T]) -> Option<T>
where
    T: PartialEq + Eq + Copy,
{
    for (i, a) in list.iter().enumerate() {
        for b in list.iter().skip(i + 1) {
            if a == b {
                return Some(*a);
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_repetition_quadratic_lookup_for_u32_unique_lists() {
        let test_cases: [Vec<u32>; 5] = [
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![1, 3, 2, 4, 5, 7, 6, 9, 8],
            vec![],
            vec![1],
        ];

        for test_case in test_cases.iter() {
            let result = has_repetition_quadratic_lookup(&test_case);
            assert_eq!(result, None)
        }
    }

    #[test]
    fn test_has_repetition_quadratic_lookup_for_u32_not_unique_lists() {
        let test_cases: [(Vec<u32>, Option<u32>); 5] = [
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 1], Some(1)),
            (vec![1, 2, 3, 4, 5, 6, 2, 8, 9], Some(2)),
            (vec![1, 2, 2, 1], Some(1)),
            (vec![9, 8, 7, 1, 2, 3, 1, 2, 3], Some(1)),
            (vec![1, 2, 3, 4, 5, 5, 6, 7, 8, 9], Some(5)),
        ];

        for test_case in test_cases.iter() {
            let result = has_repetition_quadratic_lookup(&test_case.0);
            assert_eq!(result, test_case.1)
        }
    }

    #[test]
    fn test_has_repetition_quadratic_lookup_for_str_unique_lists() {
        let test_cases: [Vec<&str>; 5] = [
            vec!["a", "b", "c", "d", "e"],
            vec!["e", "c", "b", "d", "a"],
            vec!["aaa", "aab", "aac", "aad", "aae"],
            vec![],
            vec!["a"],
        ];

        for test_case in test_cases.iter() {
            let result = has_repetition_quadratic_lookup(&test_case);
            assert_eq!(result, None)
        }
    }

    #[test]
    fn test_has_repetition_quadratic_lookup_for_str_nor_uniqie_lists() {
        let test_cases: [(Vec<&str>, Option<&str>); 5] = [
            (vec!["a", "b", "c", "d", "a"], Some("a")),
            (vec!["a", "b", "c", "b", "e"], Some("b")),
            (vec!["a", "b", "c", "a", "b"], Some("a")),
            (vec!["a", "b", "b", "b", "a"], Some("a")),
            (vec!["a", "b", "c", "d", "b"], Some("b")),
        ];

        for test_case in test_cases.iter() {
            let result = has_repetition_quadratic_lookup(&test_case.0);
            assert_eq!(result, test_case.1)
        }
    }
}
