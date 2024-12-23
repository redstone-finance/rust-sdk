use crate::{network::error::Error, FeedId, Value};
use alloc::vec::Vec;

pub trait Trim<T>
where
    Self: Sized,
{
    fn trim_end(&mut self, len: usize) -> T;
}

pub trait TryTrim<T>
where
    Self: Sized,
{
    fn try_trim_end(&mut self, len: usize) -> Result<T, Error>;
}

impl Trim<Vec<u8>> for Vec<u8> {
    fn trim_end(&mut self, len: usize) -> Self {
        if len >= self.len() {
            core::mem::take(self)
        } else {
            self.split_off(self.len() - len)
        }
    }
}

impl Trim<FeedId> for Vec<u8> {
    fn trim_end(&mut self, len: usize) -> FeedId {
        let v: Vec<_> = self.trim_end(len);

        v.into()
    }
}

impl TryTrim<usize> for Vec<u8> {
    fn try_trim_end(&mut self, len: usize) -> Result<usize, Error> {
        let y: u64 = self.try_trim_end(len)?;

        y.try_into()
            .map_err(|_| Error::NumberOverflow(Value::from(y)))
    }
}

impl TryTrim<u64> for Vec<u8> {
    fn try_trim_end(&mut self, len: usize) -> Result<u64, Error> {
        let y: Vec<u8> = self.trim_end(len);
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
    use crate::{
        protocol::constants::{REDSTONE_MARKER, REDSTONE_MARKER_BS},
        utils::trim::Trim,
        FeedId,
    };

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    use crate::{network::error::Error, utils::trim::TryTrim};

    const MARKER_DECIMAL: u64 = 823907890102272;

    fn redstone_marker_bytes() -> Vec<u8> {
        REDSTONE_MARKER.into()
    }

    #[test]
    fn test_trim_end_number() {
        let (rest, result): (_, FeedId) = test_trim_end(3);
        assert_eq!(result, REDSTONE_MARKER[6..].to_vec().into());
        assert_eq!(rest.as_slice(), &REDSTONE_MARKER[..6]);

        let (_, result) = test_try_trim_end(3);
        assert_eq!(result, Ok(256u64.pow(2) * 30));

        let (_, result) = test_try_trim_end(3);
        assert_eq!(result, Ok(256usize.pow(2) * 30));

        let (_, result): (_, Vec<u8>) = test_trim_end(3);
        assert_eq!(result.as_slice(), &REDSTONE_MARKER[6..]);
    }

    #[test]
    fn test_trim_end_number_null() {
        let (rest, result): (_, FeedId) = test_trim_end(0);
        assert_eq!(result, vec![0].into());
        assert_eq!(rest.as_slice(), &REDSTONE_MARKER);

        let (_, result) = test_try_trim_end(0);
        assert_eq!(result, Ok(0_usize));

        let (_, result) = test_try_trim_end(0);
        assert_eq!(result, Ok(0_usize));

        let (_, result): (_, Vec<u8>) = test_trim_end(0);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_trim_end_whole() {
        test_trim_end_whole_size(REDSTONE_MARKER_BS);
        test_trim_end_whole_size(REDSTONE_MARKER_BS - 1);
        test_trim_end_whole_size(REDSTONE_MARKER_BS - 2);
        test_trim_end_whole_size(REDSTONE_MARKER_BS + 1);
    }

    fn test_trim_end_whole_size(size: usize) {
        let (rest, result): (_, FeedId) = test_trim_end(size);
        assert_eq!(result, REDSTONE_MARKER.to_vec().into());
        assert_eq!(
            rest.as_slice().len(),
            REDSTONE_MARKER_BS - size.min(REDSTONE_MARKER_BS)
        );

        let (_, result) = test_try_trim_end(size);
        assert_eq!(result, Ok(MARKER_DECIMAL));

        #[cfg(not(target_arch = "wasm32"))]
        {
            let (_, result) = test_try_trim_end(size);
            assert_eq!(result, Ok(823907890102272usize));
        }

        let (_rest, result): (_, Vec<u8>) = test_trim_end(size);
        assert_eq!(result.as_slice().len(), size.min(REDSTONE_MARKER_BS));
    }

    #[test]
    fn test_trim_end_u64() {
        let mut bytes = vec![255, 255, 255, 255, 255, 255, 255, 255, 255];
        let x: u64 = bytes.try_trim_end(8).unwrap();

        let expected_bytes = vec![255];

        assert_eq!(bytes, expected_bytes);
        assert_eq!(x, 18446744073709551615);
    }

    #[cfg(target_arch = "wasm32")]
    #[test]
    fn test_trim_end_u64_overflow_usize_wasm32() {
        let (_, output): (_, Result<usize, _>) = test_try_trim_end(REDSTONE_MARKER_BS);

        assert_eq!(
            output,
            Err(Error::NumberOverflow(823907890102272_u64.into()))
        );
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

    fn test_trim_end<T>(size: usize) -> (Vec<u8>, T)
    where
        Vec<u8>: Trim<T>,
    {
        let mut bytes = redstone_marker_bytes();
        let rest = bytes.trim_end(size);
        (bytes, rest)
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
