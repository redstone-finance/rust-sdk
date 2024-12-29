use redstone::Value;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RunMode {
    Get = 0,
    Write,
}


pub trait PriceAdapterRunEnv {
    type State;

    fn instantiate(
        unique_signer_count: u8,
        signers: Vec<Vec<u8>>,
        timestamp: Option<u64>,
    ) -> Self;

    fn state(&self) -> Self::State;
    
    fn read_timestamp(&mut self, feed_id: Option<&str>) -> u64;
    
    fn read_prices(&mut self, feed_ids: Vec<Vec<u8>>) -> Vec<Value>;
    
    fn process_payload(
        &mut self,
        run_mode: RunMode,
        payload: Vec<u8>,
        feed_ids: Vec<Vec<u8>>,
        timestamp: u64,
    ) -> (u64, Vec<Value>);
    
    fn increase_time(&mut self);
}
