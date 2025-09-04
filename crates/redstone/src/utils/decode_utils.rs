use crate::network::error::Error;
use alloc::vec::Vec;

pub fn decode_u64(bytes: &[u8]) -> Result<u64, Error> {
    let significant_bytes: Vec<u8> = bytes.iter().skip_while(|&&b| b == 0).copied().collect();

    if significant_bytes.len() > size_of::<u64>() {
        return Err(Error::NumberOverflow(significant_bytes.into()));
    }

    let mut buffer = [0; size_of::<u64>()];
    buffer[size_of::<u64>() - significant_bytes.len()..].copy_from_slice(&significant_bytes);

    Ok(u64::from_be_bytes(buffer))
}

#[cfg(test)]
mod tests {
    use crate::utils::decode_utils::decode_u64;

    #[test]
    fn test_decode_usize_1() {
        let usize_bs = [0, 0, 1];

        let v = decode_u64(&usize_bs).unwrap();

        assert_eq!(1, v);
    }

    #[test]
    fn test_decode_usize_2() {
        let usize_bs = [1];

        let v = decode_u64(&usize_bs).unwrap();

        assert_eq!(1, v);
    }

    #[test]
    fn test_decode_usize_3() {
        let usize_bs = [];

        let v = decode_u64(&usize_bs).unwrap();

        assert_eq!(0, v);
    }

    #[test]
    fn test_decode_usize_4() {
        let usize_bs = [1, 1];

        let v = decode_u64(&usize_bs).unwrap();

        assert_eq!(257, v);
    }
}
