use hex::{decode, encode};

use crate::{Bytes, FeedId};

const SAMPLE_PAYLOAD_HEX: &str = include_str!("../.././sample-data/payload.hex");

pub fn hex_to_bytes(hex_str: String) -> Vec<u8> {
    let trimmed_hex = hex_str.trim_start_matches("0x");

    decode(trimmed_hex).expect("Conversion error")
}

pub fn hex_from<T: AsRef<[u8]>>(bytes: T) -> String {
    encode(bytes)
}

pub fn make_bytes(vec: Vec<&str>, fun: fn(&str) -> String) -> Vec<Bytes> {
    vec.iter()
        .map(|addr| hex_to_bytes(fun(addr)).into())
        .collect()
}

pub fn make_feed_id(s: &str) -> FeedId {
    hex_to_bytes(encode(s)).into()
}

pub fn make_feed_ids(vec: Vec<&str>) -> Vec<FeedId> {
    vec.iter().map(|&s| make_feed_id(s)).collect()
}

pub fn sample_payload_hex() -> String {
    SAMPLE_PAYLOAD_HEX.to_string()
}

pub fn sample_payload_bytes() -> Vec<u8> {
    let contents = sample_payload_hex();

    hex_to_bytes(contents)
}
