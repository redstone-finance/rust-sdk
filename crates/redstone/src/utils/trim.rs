use core::cmp::Ordering;

use alloc::vec::Vec;

use crate::{network::error::Error, FeedId};

pub trait TryTrim<T>
where
    Self: Sized,
{
    fn try_trim_end(&mut self, len: usize) -> Result<T, Error>;
}

impl TryTrim<Vec<u8>> for Vec<u8> {
    fn try_trim_end(&mut self, len: usize) -> Result<Self, Error> {
        Ok(match len.cmp(&self.len()) {
            Ordering::Less => self.split_off(self.len() - len),
            Ordering::Equal => core::mem::take(self),
            Ordering::Greater => return Err(Error::BufferOverflow),
        })
    }
}

impl TryTrim<FeedId> for Vec<u8> {
    fn try_trim_end(&mut self, len: usize) -> Result<FeedId, Error> {
        let v: Vec<_> = self.try_trim_end(len)?;

        Ok(v.into())
    }
}

impl TryTrim<u64> for Vec<u8> {
    fn try_trim_end(&mut self, len: usize) -> Result<u64, Error> {
        let y: Vec<u8> = self.try_trim_end(len)?;
        let y: Vec<u8> = y.into_iter().skip_while(|&b| b == 0).collect();

        if y.len() > 8 {
            return Err(Error::NumberOverflow(y.into()));
        }
        let mut buff = [0; 8];
        buff[8 - y.len()..].copy_from_slice(&y);

        Ok(u64::from_be_bytes(buff))
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use crate::{
        network::error::Error,
        protocol::constants::{REDSTONE_MARKER, REDSTONE_MARKER_BS},
        utils::trim::TryTrim,
        FeedId,
    };

    const MARKER_DECIMAL: u64 = 823907890102272;

    fn redstone_marker_bytes() -> Vec<u8> {
        REDSTONE_MARKER.into()
    }

    #[test]
    fn test_trim_end_number() {
        let (rest, result): (_, Result<FeedId, _>) = test_try_trim_end(3);
        assert_eq!(result, Ok(REDSTONE_MARKER[6..].to_vec().into()));
        assert_eq!(rest.as_slice(), &REDSTONE_MARKER[..6]);

        let (_, result) = test_try_trim_end(3);
        assert_eq!(result, Ok(256u64.pow(2) * 30));
    }

    #[test]
    fn test_trim_end_number_null() {
        let (rest, result): (_, Result<FeedId, _>) = test_try_trim_end(0);
        assert_eq!(result, Ok(vec![0].into()));
        assert_eq!(rest.as_slice(), &REDSTONE_MARKER);

        let (_, result) = test_try_trim_end(0);
        assert_eq!(result, Ok(0));

        let (_, result) = test_try_trim_end(0);
        assert_eq!(result, Ok(0));

        let (_, result): (_, Result<Vec<u8>, _>) = test_try_trim_end(0);
        assert_eq!(result, Ok(Vec::<u8>::new()));
    }

    #[test]
    fn test_trim_end_whole() {
        test_trim_end_whole_size(REDSTONE_MARKER_BS);
        test_trim_end_whole_size(REDSTONE_MARKER_BS - 1);
        test_trim_end_whole_size(REDSTONE_MARKER_BS - 2);
    }

    fn test_trim_end_whole_size(size: usize) {
        let (rest, result): (_, Result<FeedId, _>) = test_try_trim_end(size);
        assert_eq!(result.unwrap(), REDSTONE_MARKER.to_vec().into());
        assert_eq!(
            rest.as_slice().len(),
            REDSTONE_MARKER_BS - size.min(REDSTONE_MARKER_BS)
        );

        let (_, result) = test_try_trim_end(size);
        assert_eq!(result, Ok(MARKER_DECIMAL));

        #[cfg(not(target_arch = "wasm32"))]
        {
            let (_, result) = test_try_trim_end(size);
            assert_eq!(result, Ok(823907890102272));
        }

        let (_rest, result): (_, Result<Vec<u8>, _>) = test_try_trim_end(size);
        assert_eq!(
            result.unwrap().as_slice().len(),
            size.min(REDSTONE_MARKER_BS)
        );
    }

    #[test]
    fn test_trim_buffer_overflow() {
        let mut buffer = vec![0, 1, 2];

        let res: Result<Vec<_>, _> = buffer.try_trim_end(buffer.len() + 1);

        assert_eq!(res, Err(Error::BufferOverflow));
    }

    #[test]
    fn test_trim_end_u64() {
        let mut bytes = vec![255, 255, 255, 255, 255, 255, 255, 255, 255];
        let x: u64 = bytes.try_trim_end(8).unwrap();

        let expected_bytes = vec![255];

        assert_eq!(bytes, expected_bytes);
        assert_eq!(x, 18446744073709551615);
    }

    #[test]
    fn test_trim_end_u64_overflow_u64() {
        let mut bytes = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9];

        let output: Result<u64, _> = bytes.try_trim_end(9);

        assert_eq!(
            output,
            Err(Error::NumberOverflow(
                vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9].into()
            ))
        );
    }

    fn test_try_trim_end<T>(size: usize) -> (Vec<u8>, Result<T, Error>)
    where
        Vec<u8>: TryTrim<T>,
    {
        let mut bytes = redstone_marker_bytes();
        let rest = bytes.try_trim_end(size);
        (bytes, rest)
    }
}
