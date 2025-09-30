use core::time::Duration;

use redstone::Value;

use crate::signer::ContractUpdateSigner;

pub trait PriceAdapterRunEnv {
    fn set_time_to(&mut self, to: Duration);
    fn unique_signer_threshold(&self) -> u8;
    fn initialize(&mut self, _signers: Vec<Vec<u8>>, _unique_signer_threshold: u8) {}
    fn read_timestamp(&mut self, feed_id: Option<&str>) -> u64;
    fn read_write_timestamp(&mut self, feed_id: Vec<u8>) -> u64;
    fn read_prices(&mut self, feed_ids: Vec<Vec<u8>>) -> Option<Vec<Value>>;
    fn read_prices_and_timestamp(&mut self, feed_ids: Vec<Vec<u8>>) -> (Vec<Value>, u64);
    fn process_payload(
        &mut self,
        payload: Vec<u8>,
        feed_ids: Vec<Vec<u8>>,
        signer: ContractUpdateSigner,
    );
    fn process_payload_get(
        &mut self,
        payload: Vec<u8>,
        feed_ids: Vec<Vec<u8>>,
        signer: ContractUpdateSigner,
    ) -> (Vec<Value>, u64);
    fn increase_time_by(&mut self, by: Duration);
}
