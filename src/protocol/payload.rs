use crate::protocol::data_package::DataPackage;

#[derive(Clone, Debug)]
pub(crate) struct Payload {
    pub(crate) data_packages: Vec<DataPackage>,
}
