use crate::types::Sanitized;

/// Type describing feed ids.
/// We expect FeedId to be byte string like b"EUR"
/// converted to bytearray and padded with zeroes to the right.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FeedId(pub [u8; 32]);

fn trim_zeros(v: Vec<u8>) -> Vec<u8> {
    if v.is_empty() {
        return v;
    }
    let l_index = {
        let mut i = 0;
        while i < v.len() && v[i] == 0 {
            i += 1;
        }
        i
    };

    let r_index = {
        let mut i = v.len() - 1;
        while i != 0 && v[i] == 0 {
            i -= 1;
        }
        i
    };

    v[l_index..=r_index].into()
}

impl From<Vec<u8>> for FeedId {
    fn from(value: Vec<u8>) -> Self {
        let value = trim_zeros(value);
        let value = value.sanitized();

        let mut buff = [0; 32];
        buff[0..value.len()].copy_from_slice(&value);

        Self(buff)
    }
}
