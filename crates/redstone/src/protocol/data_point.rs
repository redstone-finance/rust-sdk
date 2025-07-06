use crate::{types::Value, FeedId};

#[cfg(feature = "extra")]
use {
    crate::network::as_str::{AsAsciiStr, AsHexStr},
    core::fmt::{Debug, Formatter},
};

#[cfg_attr(feature = "extra", derive(Clone, PartialEq, Eq))]
pub struct DataPoint {
    pub(crate) feed_id: FeedId,
    pub(crate) value: Value,
}

#[cfg(feature = "extra")]
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
