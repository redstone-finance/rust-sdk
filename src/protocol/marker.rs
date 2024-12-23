use crate::{
    network::error::Error,
    protocol::constants::{REDSTONE_MARKER, REDSTONE_MARKER_BS},
    utils::trim::Trim,
};
use alloc::vec::Vec;

pub fn trim_redstone_marker(payload: &mut Vec<u8>) -> Result<(), Error> {
    let marker: Vec<u8> = payload.trim_end(REDSTONE_MARKER_BS);

    if marker != REDSTONE_MARKER {
        return Err(Error::WrongRedStoneMarker(marker));
    }

    Ok(())
}

#[cfg(feature = "helpers")]
#[cfg(test)]
mod tests {
    use crate::{
        helpers::hex::hex_to_bytes,
        network::error::Error,
        protocol::{constants::REDSTONE_MARKER_BS, marker::trim_redstone_marker},
    };

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    const PAYLOAD_TAIL: &str = "1c000f000000000002ed57011e0000";

    #[test]
    fn test_trim_redstone_marker() {
        let mut bytes = hex_to_bytes(PAYLOAD_TAIL.into());
        trim_redstone_marker(&mut bytes).unwrap();

        assert_eq!(
            bytes,
            hex_to_bytes(PAYLOAD_TAIL[..PAYLOAD_TAIL.len() - 2 * REDSTONE_MARKER_BS].into())
        );
    }

    #[test]
    fn test_trim_redstone_marker_wrong() {
        let res = trim_redstone_marker(&mut hex_to_bytes(PAYLOAD_TAIL.replace('1', "2")));
        assert_eq!(
            res,
            Err(Error::WrongRedStoneMarker(vec![
                0, 0, 2, 237, 87, 2, 46, 0, 0
            ]))
        )
    }

    #[test]
    fn test_trim_redstone_marker_wrong_ending() {
        let res = trim_redstone_marker(&mut hex_to_bytes(
            PAYLOAD_TAIL[..PAYLOAD_TAIL.len() - 2].into(),
        ));
        assert_eq!(
            res,
            Err(Error::WrongRedStoneMarker(vec![
                0, 0, 0, 2, 237, 87, 1, 30, 0
            ]))
        )
    }

    #[test]
    fn test_trim_redstone_marker_wrong_beginning() {
        let res = trim_redstone_marker(&mut hex_to_bytes(
            PAYLOAD_TAIL.replace("0000000", "1111111"),
        ));
        assert_eq!(
            res,
            Err(Error::WrongRedStoneMarker(vec![
                16, 0, 2, 237, 87, 1, 30, 0, 0
            ]))
        )
    }

    #[test]
    fn test_trim_redstone_marker_too_short() {
        let res = trim_redstone_marker(&mut hex_to_bytes(
            PAYLOAD_TAIL[PAYLOAD_TAIL.len() - 2 * (REDSTONE_MARKER_BS - 1)..].into(),
        ));
        assert_eq!(
            res,
            Err(Error::WrongRedStoneMarker(vec![0, 2, 237, 87, 1, 30, 0, 0]))
        )
    }
}
