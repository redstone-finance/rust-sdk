use alloc::vec::Vec;
use core::convert::TryInto;
use core::marker::PhantomData;

use crate::{
    crypto::Crypto,
    network::{error::Error, Environment},
    protocol::{
        constants::{
            DATA_FEED_ID_BS, DATA_PACKAGES_COUNT_BS, DATA_POINTS_COUNT_BS,
            DATA_POINT_COUNT_MAX_VALUE, DATA_POINT_VALUE_BYTE_SIZE_BS, SIGNATURE_BS, TIMESTAMP_BS,
            UNSIGNED_METADATA_BYTE_SIZE_BS,
        },
        data_package::DataPackage,
        data_point::DataPoint,
        marker::trim_redstone_marker,
        payload::Payload,
    },
    utils::trim::{Trim, TryTrim},
    TimestampMillis,
};

pub struct PayloadDecoder<Env: Environment, C: Crypto>(PhantomData<(Env, C)>);

impl<Env: Environment, C: Crypto> PayloadDecoder<Env, C> {
    pub fn make_payload(payload_bytes: &mut Vec<u8>) -> Result<Payload, Error> {
        trim_redstone_marker(payload_bytes)?;
        let payload = Self::trim_payload(payload_bytes)?;

        if !payload_bytes.is_empty() {
            return Err(Error::NonEmptyPayloadRemainder(payload_bytes.len()));
        }

        Ok(payload)
    }

    fn trim_payload(payload: &mut Vec<u8>) -> Result<Payload, Error> {
        let data_package_count = Self::trim_metadata(payload)?;
        let data_packages = Self::trim_data_packages(payload, data_package_count)?;

        Ok(Payload { data_packages })
    }

    fn trim_metadata(payload: &mut Vec<u8>) -> Result<usize, Error> {
        let unsigned_metadata_size = payload
            .try_trim_end(UNSIGNED_METADATA_BYTE_SIZE_BS)?
            .try_into()?;
        let _: Vec<u8> = payload.trim_end(unsigned_metadata_size);

        let data_package_count = payload.try_trim_end(DATA_PACKAGES_COUNT_BS)?.try_into()?;

        Ok(data_package_count)
    }

    fn trim_data_packages(payload: &mut Vec<u8>, count: usize) -> Result<Vec<DataPackage>, Error> {
        let mut data_packages = Vec::with_capacity(count);

        for _ in 0..count {
            let data_package = Self::trim_data_package(payload)?;
            data_packages.push(data_package);
        }

        Ok(data_packages)
    }

    fn trim_data_package(payload: &mut Vec<u8>) -> Result<DataPackage, Error> {
        let signature: Vec<u8> = payload.trim_end(SIGNATURE_BS);
        let mut tmp = payload.clone();

        let data_point_count = payload.try_trim_end(DATA_POINTS_COUNT_BS)?;
        let value_size = payload.try_trim_end(DATA_POINT_VALUE_BYTE_SIZE_BS)?;
        let timestamp = payload.try_trim_end(TIMESTAMP_BS)?;

        let size: u64 = data_point_count
            * (value_size + TryInto::<u64>::try_into(DATA_FEED_ID_BS)?)
            + TryInto::<u64>::try_into(DATA_POINT_VALUE_BYTE_SIZE_BS)?
            + TryInto::<u64>::try_into(TIMESTAMP_BS)?
            + TryInto::<u64>::try_into(DATA_POINTS_COUNT_BS)?;

        let signable_bytes: Vec<_> = tmp.trim_end(size.try_into()?);
        let signer_address = C::recover_address(signable_bytes, signature)?;

        let data_points = Self::trim_data_points(
            payload,
            data_point_count.try_into()?,
            value_size.try_into()?,
        )?;

        Ok(DataPackage {
            data_points,
            timestamp: TimestampMillis::from_millis(timestamp),
            signer_address,
        })
    }

    fn trim_data_points(
        payload: &mut Vec<u8>,
        count: usize,
        value_size: usize,
    ) -> Result<Vec<DataPoint>, Error> {
        Self::check_data_point_count(count)?;

        let mut data_points = Vec::with_capacity(count);

        for _ in 0..count {
            let data_point = Self::trim_data_point(payload, value_size);
            data_points.push(data_point);
        }

        Ok(data_points)
    }

    fn trim_data_point(payload: &mut Vec<u8>, value_size: usize) -> DataPoint {
        let value: Vec<_> = payload.trim_end(value_size);
        let feed_id = payload.trim_end(DATA_FEED_ID_BS);

        DataPoint {
            value: value.into(),
            feed_id,
        }
    }

    #[inline(always)]
    fn check_data_point_count(count: usize) -> Result<(), Error> {
        if count > DATA_POINT_COUNT_MAX_VALUE || count == 0 {
            return Err(Error::SizeNotSupported(count));
        }
        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "helpers")]
#[cfg(feature = "default-crypto")]
mod tests {
    use alloc::{borrow::ToOwned, string::ToString, vec::Vec};
    use core::ops::Shr;

    use crate::{
        default_ext::DefaultCrypto,
        helpers::hex::{hex_to_bytes, sample_payload_bytes, sample_payload_hex},
        network::{error::Error, StdEnv},
        protocol::{
            constants::{
                DATA_FEED_ID_BS, DATA_POINTS_COUNT_BS, DATA_POINT_VALUE_BYTE_SIZE_BS,
                REDSTONE_MARKER_BS, SIGNATURE_BS, TIMESTAMP_BS,
            },
            data_package::DataPackage,
            data_point::DataPoint,
            PayloadDecoder,
        },
        types::VALUE_SIZE,
        Value,
    };

    type TestProcessor = PayloadDecoder<StdEnv, DefaultCrypto>;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    const PAYLOAD_METADATA_BYTES: &str = "000f000000";
    const PAYLOAD_METADATA_WITH_UNSIGNED_BYTE: &str = "000f55000001";
    const PAYLOAD_METADATA_WITH_UNSIGNED_BYTES: &str = "000f11223344556677889900aabbccddeeff000010";

    #[test]
    fn test_trim_metadata() {
        let prefix = "9e0294371c";

        for &bytes_str in &[
            PAYLOAD_METADATA_BYTES,
            PAYLOAD_METADATA_WITH_UNSIGNED_BYTE,
            PAYLOAD_METADATA_WITH_UNSIGNED_BYTES,
        ] {
            let mut bytes = hex_to_bytes(prefix.to_owned() + bytes_str);
            let result = TestProcessor::trim_metadata(&mut bytes);

            assert_eq!(bytes, hex_to_bytes(prefix.into()));
            assert_eq!(result, Ok(15));
        }
    }

    #[test]
    fn test_trim_payload() {
        let payload_hex = sample_payload_bytes();

        let mut bytes = payload_hex[..payload_hex.len() - REDSTONE_MARKER_BS].into();
        let payload = TestProcessor::trim_payload(&mut bytes).unwrap();

        assert_eq!(bytes, Vec::<u8>::new());
        assert_eq!(payload.data_packages.len(), 15);
    }

    #[test]
    fn test_make_payload() {
        let mut payload_hex = sample_payload_bytes();
        let payload = TestProcessor::make_payload(&mut payload_hex).unwrap();

        assert_eq!(payload.data_packages.len(), 15);
    }

    #[test]
    fn test_make_payload_with_prefix() {
        let payload_hex = sample_payload_hex();
        let mut bytes = hex_to_bytes("12".to_owned() + &payload_hex);
        let res = TestProcessor::make_payload(&mut bytes);

        assert!(matches!(res, Err(Error::NonEmptyPayloadRemainder(1))));
    }

    const DATA_PACKAGE_BYTES_1: &str = "4554480000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000360cafc94e018d79bf0ba00000002000000151afa8c5c3caf6004b42c0fb17723e524f993b9ecbad3b9bce5ec74930fa436a3660e8edef10e96ee5f222de7ef5787c02ca467c0ec18daa2907b43ac20c63c11c";
    const DATA_PACKAGE_BYTES_2: &str = "4554480000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000360cdd851e018d79bf0ba000000020000001473fd9dc72e6814a7de719b403cf4c9eba08934a643fd0666c433b806b31e69904f2226ffd3c8ef75861b11b5e32a1fda4b1458e0da4605a772dfba2a812f3ee1b";

    const SIGNER_ADDRESS_1: &str = "1ea62d73edf8ac05dfcea1a34b9796e937a29eff";
    const SIGNER_ADDRESS_2: &str = "109b4a318a4f5ddcbca6349b45f881b4137deafb";

    const VALUE_1: u128 = 232141080910;
    const VALUE_2: u128 = 232144078110;

    const DATA_PACKAGE_SIZE: usize = 32
        + DATA_FEED_ID_BS
        + DATA_POINT_VALUE_BYTE_SIZE_BS
        + TIMESTAMP_BS
        + SIGNATURE_BS
        + DATA_POINTS_COUNT_BS;

    const DATA_POINT_BYTES_TAIL: &str = "4554480000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000360cafc94e";
    const DATA_POINTS_500_COUNT: usize = 500;
    const DATA_POINTS_50_COUNT: usize = 50;
    const DATA_POINTS_BYTES_ARRAY_50_PACKED_TAIL: &str =
        include_str!("../../../../test_data/payload_50_datapoints.hex");
    const DATA_POINTS_BYTES_ARRAY_500_PACKED_TAIL: &str =
        include_str!("../../../../test_data/payload_500_datapoints.hex");
    const VALUE: u128 = 232141080910;

    #[test]
    fn test_trim_data_packages() {
        test_trim_data_packages_of(2, "");
        test_trim_data_packages_of(0, "");
        test_trim_data_packages_of(1, "");
    }

    #[test]
    fn test_trim_data_packages_with_prefix() {
        let prefix = "da4687f1914a1c";

        test_trim_data_packages_of(2, prefix);
    }

    #[test]
    fn test_trim_data_packages_single() {
        let mut bytes = hex_to_bytes(DATA_PACKAGE_BYTES_1.into());
        let data_packages = TestProcessor::trim_data_packages(&mut bytes, 1).unwrap();
        assert_eq!(data_packages.len(), 1);
        assert_eq!(bytes, Vec::<u8>::new());

        verify_data_package(data_packages[0].clone(), VALUE_1, SIGNER_ADDRESS_1);
    }

    fn test_trim_data_packages_of(count: usize, prefix: &str) {
        let input: Vec<u8> =
            hex_to_bytes((prefix.to_owned() + DATA_PACKAGE_BYTES_1) + DATA_PACKAGE_BYTES_2);
        let mut bytes = input.clone();

        let data_packages = TestProcessor::trim_data_packages(&mut bytes, count).unwrap();

        assert_eq!(data_packages.len(), count);
        assert_eq!(
            bytes.as_slice(),
            &input[..input.len() - count * DATA_PACKAGE_SIZE]
        );

        let values = &[VALUE_2, VALUE_1];
        let signers = &[SIGNER_ADDRESS_2, SIGNER_ADDRESS_1];

        for i in 0..count {
            verify_data_package(data_packages[i].clone(), values[i], signers[i]);
        }
    }

    #[should_panic(expected = "CryptographicError(InvalidSignatureLen(0))")]
    #[test]
    fn test_trim_data_packages_bigger_number() {
        test_trim_data_packages_of(3, "");
    }

    #[test]
    fn test_trim_data_package() {
        test_trim_data_package_of(DATA_PACKAGE_BYTES_1, VALUE_1, SIGNER_ADDRESS_1);
        test_trim_data_package_of(DATA_PACKAGE_BYTES_2, VALUE_2, SIGNER_ADDRESS_2);
    }

    #[test]
    fn test_trim_data_package_with_prefix() {
        test_trim_data_package_of(
            &("da4687f1914a1c".to_owned() + DATA_PACKAGE_BYTES_1),
            VALUE_1,
            SIGNER_ADDRESS_1,
        );
        test_trim_data_package_of(
            &("da4687f1914a1c".to_owned() + DATA_PACKAGE_BYTES_2),
            VALUE_2,
            SIGNER_ADDRESS_2,
        );
    }

    #[should_panic]
    #[test]
    fn test_trim_data_package_signature_only() {
        test_trim_data_package_of(
            &DATA_PACKAGE_BYTES_1[(DATA_PACKAGE_BYTES_1.len() - 2 * SIGNATURE_BS)..],
            0,
            "",
        );
    }

    #[should_panic]
    #[test]
    fn test_trim_data_package_shorter() {
        test_trim_data_package_of(
            &DATA_PACKAGE_BYTES_1
                [(DATA_PACKAGE_BYTES_1.len() - 2 * (SIGNATURE_BS + DATA_POINTS_COUNT_BS))..],
            0,
            "",
        );
    }

    fn test_trim_data_package_of(bytes_str: &str, expected_value: u128, signer_address: &str) {
        let mut bytes: Vec<u8> = hex_to_bytes(bytes_str.into());
        let result = TestProcessor::trim_data_package(&mut bytes).unwrap();
        assert_eq!(
            bytes,
            hex_to_bytes(bytes_str[..bytes_str.len() - 2 * (DATA_PACKAGE_SIZE)].into())
        );

        verify_data_package(result, expected_value, signer_address);
    }

    fn verify_data_package(result: DataPackage, expected_value: u128, signer_address: &str) {
        let data_package = DataPackage {
            data_points: vec![DataPoint {
                feed_id: hex_to_bytes(DATA_PACKAGE_BYTES_1[..6].into()).into(),
                value: Value::from(expected_value),
            }],
            timestamp: 1707144580000.into(),
            signer_address: hex_to_bytes(signer_address.into()).into(),
        };

        assert_eq!(result, data_package);
    }

    #[test]
    fn test_trim_data_points() {
        let mut bytes = hex_to_bytes(DATA_POINT_BYTES_TAIL.into());
        let result = TestProcessor::trim_data_points(&mut bytes, 1, 32).unwrap();

        assert_eq!(result.len(), 1);

        verify_rest_and_result(
            DATA_POINT_BYTES_TAIL,
            32,
            1,
            VALUE.into(),
            bytes,
            result[0].clone(),
        )
    }

    #[test]
    fn test_trim_medium_data_points() -> Result<(), Error> {
        let test_data_points_trimmed: String = DATA_POINTS_BYTES_ARRAY_50_PACKED_TAIL.trim().into();
        let mut bytes = hex_to_bytes(test_data_points_trimmed.clone());
        let res = TestProcessor::trim_data_points(&mut bytes, DATA_POINTS_50_COUNT, 32)?;
        assert_eq!(res.len(), DATA_POINTS_50_COUNT);
        verify_rest_and_result(
            DATA_POINTS_BYTES_ARRAY_50_PACKED_TAIL.trim(),
            32,
            DATA_POINTS_50_COUNT,
            VALUE.into(),
            bytes,
            res[0].clone(),
        );

        Ok(())
    }

    #[test]
    fn test_trim_large_data_points() -> Result<(), Error> {
        let test_data_points_trimmed: String =
            DATA_POINTS_BYTES_ARRAY_500_PACKED_TAIL.trim().into();
        let mut bytes = hex_to_bytes(test_data_points_trimmed.clone());
        let res = TestProcessor::trim_data_points(&mut bytes, DATA_POINTS_500_COUNT, 32)?;
        assert_eq!(res.len(), DATA_POINTS_500_COUNT);
        verify_rest_and_result(
            DATA_POINTS_BYTES_ARRAY_500_PACKED_TAIL.trim(),
            32,
            DATA_POINTS_500_COUNT,
            VALUE.into(),
            bytes,
            res[0].clone(),
        );

        Ok(())
    }

    #[test]
    fn test_trim_zero_data_points() {
        let res =
            TestProcessor::trim_data_points(&mut hex_to_bytes(DATA_POINT_BYTES_TAIL.into()), 0, 32);
        assert_eq!(res, Err(Error::SizeNotSupported(0)));
    }

    #[test]
    fn test_trim_above_max_available_data_points() {
        let res = TestProcessor::trim_data_points(
            &mut hex_to_bytes(DATA_POINT_BYTES_TAIL.trim().into()),
            u16::MAX as usize + 1,
            32,
        );
        assert_eq!(res, Err(Error::SizeNotSupported(u16::MAX as usize + 1)));
    }

    #[test]
    fn test_trim_data_point() -> Result<(), Error> {
        test_trim_data_point_of(DATA_POINT_BYTES_TAIL, 32, 1, VALUE.into())
    }

    #[test]
    fn test_trim_medium_slice_of_data_points() -> Result<(), Error> {
        test_trim_data_point_of(
            DATA_POINTS_BYTES_ARRAY_50_PACKED_TAIL.trim(),
            32,
            DATA_POINTS_50_COUNT,
            VALUE.into(),
        )
    }

    #[test]
    fn test_trim_large_slice_of_data_points() -> Result<(), Error> {
        test_trim_data_point_of(
            DATA_POINTS_BYTES_ARRAY_500_PACKED_TAIL.trim(),
            32,
            DATA_POINTS_500_COUNT,
            VALUE.into(),
        )
    }

    #[test]
    fn test_trim_data_point_with_prefix() -> Result<(), Error> {
        test_trim_data_point_of(
            &("a2a812f3ee1b".to_owned() + DATA_POINT_BYTES_TAIL),
            32,
            1,
            VALUE.into(),
        )
    }

    #[test]
    fn test_trim_data_point_other_lengths() -> Result<(), Error> {
        for i in 1..VALUE_SIZE {
            test_trim_data_point_of(
                &DATA_POINT_BYTES_TAIL[..DATA_POINT_BYTES_TAIL.len() - 2 * i],
                32 - i,
                1,
                Value::from_u256(primitive_types::U256::from(VALUE).shr(8 * i as u32)),
            )?;
        }
        Ok(())
    }

    fn test_trim_data_point_of(
        value: &str,
        size: usize,
        count: usize,
        expected_value: Value,
    ) -> Result<(), Error> {
        let mut bytes = hex_to_bytes(value.into());
        let result = TestProcessor::trim_data_points(&mut bytes, count, size)?;
        verify_rest_and_result(value, size, count, expected_value, bytes, result[0].clone());
        Ok(())
    }

    fn verify_rest_and_result(
        value: &str,
        size: usize,
        count: usize,
        expected_value: Value,
        rest: Vec<u8>,
        result: DataPoint,
    ) {
        assert_eq!(
            rest,
            hex_to_bytes(value[..value.len() - 2 * (size + DATA_FEED_ID_BS) * count].into())
        );

        let data_point = DataPoint {
            value: expected_value,
            feed_id: hex_to_bytes(DATA_POINT_BYTES_TAIL[..6].to_string()).into(),
        };

        assert_eq!(result, data_point);
    }
}
