use alloc::vec::Vec;

use crate::{
    core::validator::Validator, network::error::Error, protocol::data_package::DataPackage,
    TimestampMillis,
};

#[derive(Clone, Debug)]
pub(crate) struct Payload {
    pub(crate) data_packages: Vec<DataPackage>,
}

impl Payload {
    pub fn get_min_validated_timestamp(
        &self,
        validator: &impl Validator,
    ) -> Result<TimestampMillis, Error> {
        self.data_packages
            .iter()
            .map(|package| package.timestamp)
            .enumerate()
            .map(|(index, timestamp)| validator.validate_timestamp(index, timestamp))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .min()
            .ok_or(Error::ArrayIsEmpty)
    }
}
