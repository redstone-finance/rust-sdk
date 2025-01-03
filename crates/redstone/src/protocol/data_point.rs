use core::fmt::{Debug, Formatter};

use crate::{
    network::as_str::{AsAsciiStr, AsHexStr},
    types::Value,
    FeedId,
};

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct DataPoint {
    pub(crate) feed_id: FeedId,
    pub(crate) value: Value,
}

impl Debug for DataPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "DataPoint {{\n      feed_id: {:?} (0x{}), value: {:?}\n   }}",
            self.feed_id.as_ascii_str(),
            self.feed_id.as_hex_str(),
            self.value,
        )
    }
}
