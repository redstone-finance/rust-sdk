/// Performs lookup for repeated value in the slice.
///
/// # Arguments
///
/// * `slice` - A `&[T]` slice where `T` implements `PartialEq`, `Eq`, `Copy` traits.
///
/// # Returns
///
/// Returns a `Result<(), T>` which contains first repeated element found in the slice or if there is no repeated
/// element then Ok(()) otherwise.
pub fn check_no_duplicates<T>(slice: &[T]) -> Result<(), T>
where
    T: PartialEq + Eq + Copy,
{
    if slice.len() < 2 {
        return Ok(());
    }

    if slice.len() == 2 {
        if slice[0] == slice[1] {
            return Err(slice[0]);
        }
        return Ok(());
    }

    for (i, a) in slice.iter().enumerate() {
        for b in slice.iter().skip(i + 1) {
            if a == b {
                return Err(*a);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn test_has_repetition_quadratic_lookup_for_u32_unique_lists() -> Result<(), u32> {
        let test_cases: [Vec<u32>; 6] = [
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![1, 3, 2, 4, 5, 7, 6, 9, 8],
            vec![
                94, 218, 60, 212, 192, 42, 177, 209, 232, 95, 127, 89, 41, 133, 251, 130, 53, 84,
                3, 46, 123, 175, 152, 143, 57, 38, 139, 132, 171, 118, 147, 105, 166, 124, 215,
                233, 44, 160, 237, 149, 163, 162, 96, 70, 161, 1, 191, 78, 67, 231, 30, 35, 244,
                145, 47, 99, 186, 0, 158, 247, 128, 154, 214, 194, 223, 37, 72, 169, 62, 227, 136,
                59, 129, 80, 235, 58, 222, 106, 23, 10, 24, 200, 178, 69, 252, 202, 198, 153, 52,
                142, 31, 195, 61, 181, 254, 190, 242, 112, 148, 64, 101, 167, 75, 114, 33, 168,
                224, 249, 164, 87, 174, 208, 108, 34, 117, 144, 245, 180, 119, 213, 65, 179, 115,
                126, 74, 63, 20, 196, 159, 16, 206, 243, 131, 157, 26, 103, 83, 79, 246, 116, 4,
                113, 187, 229, 219, 6, 54, 36, 86, 12, 207, 104, 250, 141, 109, 55, 45, 228, 27,
                43, 100, 110, 176, 156, 102, 85, 248, 146, 189, 32, 184, 140, 137, 66, 122, 97,
                221, 98, 225, 150, 236, 134, 199, 165, 76, 107, 170, 135, 182, 203, 19, 211, 239,
                220, 238, 71, 48, 234, 22, 88, 29, 172, 13, 21, 204, 205, 120, 226, 197, 77, 7,
                111, 151, 193, 8, 15, 240, 5, 91, 14, 39, 25, 125, 50, 155, 82, 253, 230, 92, 56,
                121, 201, 2, 93, 40, 217, 210, 18, 241, 185, 68, 28, 73, 188, 216, 173, 183, 90,
                51, 17, 138, 9, 255, 11, 49, 81,
            ],
            vec![],
            vec![1],
        ];

        for test_case in test_cases.iter() {
            check_no_duplicates(test_case)?;
        }

        Ok(())
    }

    #[test]
    fn test_has_repetition_quadratic_lookup_for_u32_not_unique_lists() {
        let test_cases: [(Vec<u32>, u32); 6] = [
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 1], 1),
            (vec![1, 2, 3, 4, 5, 6, 2, 8, 9], 2),
            (vec![1, 2, 2, 1], 1),
            (
                vec![
                    94, 218, 60, 212, 192, 42, 177, 209, 232, 95, 127, 89, 41, 133, 251, 130, 53,
                    84, 3, 46, 123, 175, 152, 143, 57, 38, 139, 132, 171, 118, 147, 105, 166, 124,
                    215, 233, 44, 160, 237, 149, 163, 162, 96, 70, 161, 1, 191, 78, 67, 231, 30,
                    35, 244, 145, 47, 99, 186, 0, 158, 247, 128, 154, 214, 194, 223, 37, 72, 169,
                    62, 227, 136, 59, 129, 80, 235, 58, 222, 106, 23, 10, 24, 200, 178, 69, 252,
                    202, 198, 153, 52, 142, 31, 195, 61, 181, 254, 190, 242, 112, 148, 64, 101,
                    167, 75, 114, 33, 168, 224, 249, 164, 87, 174, 208, 108, 34, 117, 144, 245,
                    180, 119, 213, 65, 179, 115, 126, 74, 63, 20, 196, 159, 16, 206, 243, 131, 157,
                    26, 103, 83, 79, 246, 116, 4, 113, 187, 229, 219, 6, 54, 36, 86, 12, 207, 104,
                    250, 141, 109, 55, 45, 228, 27, 43, 100, 110, 176, 156, 102, 85, 248, 146, 189,
                    32, 184, 140, 137, 66, 122, 97, 221, 98, 225, 150, 236, 134, 199, 165, 76, 107,
                    170, 135, 182, 203, 19, 211, 239, 220, 238, 71, 48, 234, 22, 88, 29, 172, 13,
                    21, 204, 205, 120, 226, 197, 77, 7, 111, 151, 193, 8, 15, 240, 5, 91, 14, 39,
                    25, 125, 50, 155, 82, 253, 230, 92, 56, 121, 201, 2, 93, 40, 217, 210, 18, 241,
                    185, 68, 28, 73, 188, 216, 173, 183, 90, 51, 17, 138, 9, 254, 11, 49, 81,
                ],
                254,
            ),
            (vec![9, 8, 7, 1, 2, 3, 1, 2, 3], 1),
            (vec![1, 2, 3, 4, 5, 5, 6, 7, 8, 9], 5),
        ];

        for test_case in test_cases.iter() {
            let result = check_no_duplicates(&test_case.0);
            assert_eq!(result, Err(test_case.1))
        }
    }

    #[test]
    fn test_has_repetition_quadratic_lookup_for_str_unique_lists<'a>() -> Result<(), &'a str> {
        let test_cases: [Vec<&str>; 5] = [
            vec!["a", "b", "c", "d", "e"],
            vec!["e", "c", "b", "d", "a"],
            vec!["aaa", "aab", "aac", "aad", "aae"],
            vec![],
            vec!["a"],
        ];

        for test_case in test_cases.iter() {
            check_no_duplicates(test_case)?;
        }

        Ok(())
    }

    #[test]
    fn test_has_repetition_quadratic_lookup_for_str_nor_unique_lists() {
        let test_cases: [(Vec<&str>, &str); 5] = [
            (vec!["a", "b", "c", "d", "a"], "a"),
            (vec!["a", "b", "c", "b", "e"], "b"),
            (vec!["a", "b", "c", "a", "b"], "a"),
            (vec!["a", "b", "b", "b", "a"], "a"),
            (vec!["a", "b", "c", "d", "b"], "b"),
        ];

        for test_case in test_cases.iter() {
            let result = check_no_duplicates(&test_case.0);
            assert_eq!(result, Err(test_case.1));
        }
    }
}
