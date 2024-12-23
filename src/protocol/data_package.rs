use crate::{protocol::data_point::DataPoint, SignerAddress, TimestampMillis};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter};
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct DataPackage {
    pub(crate) signer_address: SignerAddress,
    pub(crate) timestamp: TimestampMillis,
    pub(crate) data_points: Vec<DataPoint>,
}

impl Debug for DataPackage {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            // todo: fix hex display
            "DataPackage {{\n   signer_address: 0x{:?}, timestamp: {:?},\n   data_points: {:?}\n}}",
            self.signer_address, self.timestamp, self.data_points
        )
    }
}
