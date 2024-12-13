use crate::{
    network::{assert::Assert, error::Error, Environment},
    protocol::{
        constants::{
            DATA_FEED_ID_BS, DATA_PACKAGES_COUNT_BS, DATA_POINTS_COUNT_BS,
            DATA_POINT_VALUE_BYTE_SIZE_BS, SIGNATURE_BS, TIMESTAMP_BS,
            UNSIGNED_METADATA_BYTE_SIZE_BS,
        },
        data_package::DataPackage,
        data_point::DataPoint,
        marker,
        payload::Payload,
    },
    utils::trim::Trim,
    RecoverPublicKey, TimestampMillis,
};
use core::marker::PhantomData;

use self::marker::trim_redstone_marker;

pub struct PayloadDecoder<Env: Environment, RPK: RecoverPublicKey>(PhantomData<(Env, RPK)>);

impl<Env: Environment, RPK: RecoverPublicKey> PayloadDecoder<Env, RPK> {
    pub fn make_payload(payload_bytes: &mut Vec<u8>) -> Payload {
        trim_redstone_marker(payload_bytes);
        let payload = Self::trim_payload(payload_bytes);

        payload_bytes.assert_or_revert(
            |payload_bytes| payload_bytes.is_empty(),
            |payload_bytes| Error::NonEmptyPayloadRemainder(payload_bytes.as_slice().to_vec()),
        );

        payload
    }

    fn trim_payload(payload: &mut Vec<u8>) -> Payload {
        let data_package_count = Self::trim_metadata(payload);
        let data_packages = Self::trim_data_packages(payload, data_package_count);

        Payload { data_packages }
    }

    fn trim_metadata(payload: &mut Vec<u8>) -> usize {
        let unsigned_metadata_size = payload.trim_end(UNSIGNED_METADATA_BYTE_SIZE_BS);
        let _: Vec<u8> = payload.trim_end(unsigned_metadata_size);

        payload.trim_end(DATA_PACKAGES_COUNT_BS)
    }

    fn trim_data_packages(payload: &mut Vec<u8>, count: usize) -> Vec<DataPackage> {
        let mut data_packages = Vec::with_capacity(count);

        for _ in 0..count {
            let data_package = Self::trim_data_package(payload);
            data_packages.push(data_package);
        }

        data_packages
    }

    fn trim_data_package(payload: &mut Vec<u8>) -> DataPackage {
        let signature: Vec<u8> = payload.trim_end(SIGNATURE_BS);
        let mut tmp = payload.clone();

        let data_point_count = payload.trim_end(DATA_POINTS_COUNT_BS);
        let value_size = payload.trim_end(DATA_POINT_VALUE_BYTE_SIZE_BS);
        let timestamp = payload.trim_end(TIMESTAMP_BS);
        let size = data_point_count * (value_size + DATA_FEED_ID_BS)
            + DATA_POINT_VALUE_BYTE_SIZE_BS
            + TIMESTAMP_BS
            + DATA_POINTS_COUNT_BS;

        let signable_bytes: Vec<_> = tmp.trim_end(size);
        let signer_address = RPK::recover_address(signable_bytes, signature)
            .expect("Todo: result like error handling");

        let data_points = Self::trim_data_points(payload, data_point_count, value_size);

        DataPackage {
            data_points,
            timestamp: TimestampMillis::from_millis(timestamp),
            signer_address,
        }
    }

    fn trim_data_points(payload: &mut Vec<u8>, count: usize, value_size: usize) -> Vec<DataPoint> {
        count.assert_or_revert(|&count| count == 1, |&count| Error::SizeNotSupported(count));

        let mut data_points = Vec::with_capacity(count);

        for _ in 0..count {
            let data_point = Self::trim_data_point(payload, value_size);
            data_points.push(data_point);
        }

        data_points
    }

    fn trim_data_point(payload: &mut Vec<u8>, value_size: usize) -> DataPoint {
        let value: Vec<_> = payload.trim_end(value_size);
        let feed_id = payload.trim_end(DATA_FEED_ID_BS);

        DataPoint {
            value: value.into(),
            feed_id,
        }
    }
}

#[cfg(test)]
#[cfg(feature = "helpers")]
mod tests {
    use crate::{
        crypto::DefaultCrypto,
        helpers::hex::{hex_to_bytes, sample_payload_bytes, sample_payload_hex},
        network::StdEnv,
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
    use std::ops::Shr;

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
            assert_eq!(result, 15);
        }
    }

    #[test]
    fn test_trim_payload() {
        let payload_hex = sample_payload_bytes();

        let mut bytes = payload_hex[..payload_hex.len() - REDSTONE_MARKER_BS].into();
        let payload = TestProcessor::trim_payload(&mut bytes);

        assert_eq!(bytes, Vec::<u8>::new());
        assert_eq!(payload.data_packages.len(), 15);
    }

    #[test]
    fn test_make_payload() {
        let mut payload_hex = sample_payload_bytes();
        let payload = TestProcessor::make_payload(&mut payload_hex);

        assert_eq!(payload.data_packages.len(), 15);
    }

    #[should_panic(expected = "Non empty payload remainder: 12")]
    #[test]
    fn test_make_payload_with_prefix() {
        let payload_hex = sample_payload_hex();
        let mut bytes = hex_to_bytes("12".to_owned() + &payload_hex);
        let payload = TestProcessor::make_payload(&mut bytes);

        assert_eq!(payload.data_packages.len(), 15);
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
        let data_packages = TestProcessor::trim_data_packages(&mut bytes, 1);
        assert_eq!(data_packages.len(), 1);
        assert_eq!(bytes, Vec::<u8>::new());

        verify_data_package(data_packages[0].clone(), VALUE_1, SIGNER_ADDRESS_1);
    }

    fn test_trim_data_packages_of(count: usize, prefix: &str) {
        let input: Vec<u8> =
            hex_to_bytes((prefix.to_owned() + DATA_PACKAGE_BYTES_1) + DATA_PACKAGE_BYTES_2);
        let mut bytes = input.clone();

        let data_packages = TestProcessor::trim_data_packages(&mut bytes, count);

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

    #[should_panic(expected = "index out of bounds")]
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
        let result = TestProcessor::trim_data_package(&mut bytes);
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
        let result = TestProcessor::trim_data_points(&mut bytes, 1, 32);

        assert_eq!(result.len(), 1);

        verify_rest_and_result(
            DATA_POINT_BYTES_TAIL,
            32,
            VALUE.into(),
            bytes,
            result[0].clone(),
        )
    }

    #[should_panic(expected = "Size not supported: 0")]
    #[test]
    fn test_trim_zero_data_points() {
        TestProcessor::trim_data_points(&mut hex_to_bytes(DATA_POINT_BYTES_TAIL.into()), 0, 32);
    }

    #[should_panic(expected = "Size not supported: 2")]
    #[test]
    fn test_trim_two_data_points() {
        TestProcessor::trim_data_points(&mut hex_to_bytes(DATA_POINT_BYTES_TAIL.into()), 2, 32);
    }

    #[test]
    fn test_trim_data_point() {
        test_trim_data_point_of(DATA_POINT_BYTES_TAIL, 32, VALUE.into());
    }

    #[test]
    fn test_trim_data_point_with_prefix() {
        test_trim_data_point_of(
            &("a2a812f3ee1b".to_owned() + DATA_POINT_BYTES_TAIL),
            32,
            VALUE.into(),
        );
    }

    #[test]
    fn test_trim_data_point_other_lengths() {
        for i in 1..VALUE_SIZE {
            test_trim_data_point_of(
                &DATA_POINT_BYTES_TAIL[..DATA_POINT_BYTES_TAIL.len() - 2 * i],
                32 - i,
                Value::from_u256(primitive_types::U256::from(VALUE).shr(8 * i as u32)),
            );
        }
    }

    fn test_trim_data_point_of(value: &str, size: usize, expected_value: Value) {
        let mut bytes = hex_to_bytes(value.into());
        let result = TestProcessor::trim_data_point(&mut bytes, size);

        verify_rest_and_result(value, size, expected_value, bytes, result);
    }

    fn verify_rest_and_result(
        value: &str,
        size: usize,
        expected_value: Value,
        rest: Vec<u8>,
        result: DataPoint,
    ) {
        assert_eq!(
            rest,
            hex_to_bytes(value[..value.len() - 2 * (size + DATA_FEED_ID_BS)].into())
        );

        let data_point = DataPoint {
            value: expected_value,
            feed_id: hex_to_bytes(DATA_POINT_BYTES_TAIL[..6].to_string()).into(),
        };

        assert_eq!(result, data_point);
    }
}
