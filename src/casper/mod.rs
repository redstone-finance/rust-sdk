//! Casper extension
//!
//! Contains helper implementations of conversion between types used in Casper and this library.
//! Implementation of the config suited for the casper network.

use crate::{network::StdEnv, Bytes, DefaultCrypto, RedStoneConfigImpl};

impl From<casper_types::bytesrepr::Bytes> for Bytes {
    fn from(value: casper_types::bytesrepr::Bytes) -> Self {
        value.to_vec().into()
    }
}

pub type CasperRedStoneConfig = RedStoneConfigImpl<DefaultCrypto, StdEnv>;

#[cfg(feature = "casper-test")]
pub mod casper_test {
    use crate::{network::Environment, DefaultCrypto, RedStoneConfigImpl};
    use casper_contract::contract_api::runtime::print;

    /// Config for casper tests, not to be used in the production
    pub type CasperTestRedStoneConfig = RedStoneConfigImpl<DefaultCrypto, CasperTestEnvironment>;

    /// Casper test environment.
    pub struct CasperTestEnvironment;

    impl Environment for CasperTestEnvironment {
        fn print<F: FnOnce() -> String>(print_content: F) {
            print(&print_content());
        }
    }
}
