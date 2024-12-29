use redstone::{
    helpers::{hex::make_bytes, iter_into::IterIntoOpt},
    Value,
};

use crate::{
    env::run_env::{PriceAdapterRunEnv, RunMode},
    sample::{Sample, SIGNERS},
};

fn make_feed_ids(feeds: &[&str]) -> Vec<Vec<u8>> {
    feeds.iter().map(|&s| s.as_bytes().to_vec()).collect()
}

fn signers() -> Vec<Vec<u8>> {
    make_bytes(SIGNERS.to_vec(), |s| s.to_string())
        .into_iter()
        .map(|x| x.0)
        .collect()
}

impl Sample {
    pub fn instantiate_price_adapter<PriceAdapter: PriceAdapterRunEnv>(&self) -> PriceAdapter {
        PriceAdapter::instantiate(1, signers(), self.system_timestamp.into())
    }

    pub fn verify_written_values<PriceAdapter: PriceAdapterRunEnv>(
        &self,
        price_adapter: &mut PriceAdapter,
        override_feed_ids: Option<Vec<&str>>,
    ) {
        let feed_ids = override_feed_ids.unwrap_or(self.feed_ids());

        let values = price_adapter.read_prices(make_feed_ids(&feed_ids));
        let timestamp = price_adapter.read_timestamp(Some(feed_ids.first().unwrap()));

        self.verify_results(feed_ids, values.iter_into_opt(), timestamp);
    }

    pub fn test_write_prices<PriceAdapter: PriceAdapterRunEnv>(
        &self,
        price_adapter: &mut PriceAdapter,
        override_feed_ids: Option<Vec<&str>>,
    ) {
        self.test_process_payload(RunMode::Write, price_adapter, override_feed_ids.clone());
        self.verify_written_values(price_adapter, override_feed_ids);
        price_adapter.increase_time();
    }

    pub fn test_get_prices<PriceAdapter: PriceAdapterRunEnv>(
        &self,
        price_adapter: &mut PriceAdapter,
        override_feed_ids: Option<Vec<&str>>,
    ) {
        self.test_process_payload(RunMode::Get, price_adapter, override_feed_ids);
    }

    fn test_process_payload<PriceAdapter: PriceAdapterRunEnv>(
        &self,
        run_mode: RunMode,
        price_adapter: &mut PriceAdapter,
        override_feed_ids: Option<Vec<&str>>,
    ) {
        let feed_ids = override_feed_ids.clone().unwrap_or(self.feed_ids());
        let (timestamp, values) = price_adapter.process_payload(
            run_mode,
            redstone::helpers::hex::hex_to_bytes(self.content.to_string()),
            make_feed_ids(&feed_ids),
            self.system_timestamp,
        );

        self.verify_results(feed_ids, values.iter_into_opt(), timestamp);
    }

    pub fn feed_ids(&self) -> Vec<&str> {
        self.values.keys().map(|feed_id| feed_id.as_str()).collect()
    }

    pub fn verify_results(&self, feed_ids: Vec<&str>, values: Vec<Option<Value>>, timestamp: u64) {
        assert_eq!(self.timestamp, timestamp);
        assert_eq!(
            values,
            feed_ids
                .iter()
                .map(|&feed_id| self.values.get(feed_id).cloned())
                .collect::<Vec<Option<Value>>>()
        );
    }
}
