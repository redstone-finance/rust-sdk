use core::time::Duration;

use redstone::Value;

use crate::{
    env::{PriceAdapterRunEnv, Signer},
    sample::Sample,
};

enum Action {
    WritePrice {
        feed_id: String,
        payload: String,
        signer: Signer,
    },
    WritePrices {
        feed_ids: Vec<String>,
        payload: String,
        signer: Signer,
    },
    GetPrices {
        feed_ids: Vec<String>,
        payload: String,
        signer: Signer,
        expected_values: Vec<Value>,
        expected_timestamp: u64,
    },
    CheckPricesAndTimestamp {
        feed_ids: Vec<String>,
        expected_values: Vec<Value>,
        expected_timestamp: u64,
    },
    AdvanceClock {
        by: Duration,
    },
    SetClock {
        to: Duration,
    },
}

pub enum InitTime {
    No,
    SetToSampleTime,
}

#[derive(Default)]
pub struct Scenario {
    actions: Vec<Action>,
}

impl Scenario {
    pub fn then_set_clock(mut self, to: Duration) -> Self {
        self.actions.push(Action::SetClock { to });

        self
    }

    pub fn then_write_price(mut self, feed_id: &str, payload: &str, signer: Signer) -> Self {
        self.actions.push(Action::WritePrice {
            feed_id: feed_id.to_string(),
            payload: payload.to_string(),
            signer,
        });

        self
    }

    pub fn then_advance_clock(mut self, by: Duration) -> Self {
        self.actions.push(Action::AdvanceClock { by });

        self
    }

    pub fn then_write_prices(mut self, feed_ids: Vec<&str>, payload: &str, signer: Signer) -> Self {
        self.actions.push(Action::WritePrices {
            feed_ids: feed_ids.iter().map(|feed| feed.to_string()).collect(),
            payload: payload.to_string(),
            signer,
        });
        self
    }

    pub fn then_check_prices(
        mut self,
        feed_ids: Vec<&str>,
        expected_values: Vec<Value>,
        expected_timestamp: u64,
    ) -> Self {
        self.actions.push(Action::CheckPricesAndTimestamp {
            feed_ids: feed_ids.iter().map(|feed| feed.to_string()).collect(),
            expected_values,
            expected_timestamp,
        });

        self
    }

    pub fn then_get_prices(
        mut self,
        feed_ids: Vec<&str>,
        payload: &str,
        signer: Signer,
        expected_values: Vec<Value>,
        expected_timestamp: u64,
    ) -> Self {
        self.actions.push(Action::GetPrices {
            feed_ids: feed_ids.iter().map(|feed| feed.to_string()).collect(),
            payload: payload.to_string(),
            signer,
            expected_values,
            expected_timestamp,
        });
        self
    }

    pub fn run<P: PriceAdapterRunEnv>(self, mut price_adapter: P) {
        for action in self.actions {
            match action {
                Action::WritePrice {
                    feed_id,
                    payload,
                    signer,
                } => {
                    price_adapter.process_payload(
                        hex::decode(payload).unwrap(),
                        vec![feed_id.as_bytes().to_vec()],
                        signer,
                    );
                }
                Action::WritePrices {
                    feed_ids,
                    payload,
                    signer,
                } => {
                    price_adapter.process_payload(
                        hex::decode(payload).unwrap(),
                        feed_ids
                            .iter()
                            .map(|feed| feed.as_bytes().to_vec())
                            .collect(),
                        signer,
                    );
                }
                Action::CheckPricesAndTimestamp {
                    feed_ids,
                    expected_values,
                    expected_timestamp,
                } => {
                    let (values, timestamp) = price_adapter.read_prices_and_timestamp(
                        feed_ids
                            .iter()
                            .map(|feed| feed.as_bytes().to_vec())
                            .collect(),
                    );
                    assert_eq!(timestamp, expected_timestamp);
                    assert_eq!(values, expected_values);
                }
                Action::AdvanceClock { by } => {
                    price_adapter.increase_time_by(by);
                }
                Action::SetClock { to } => {
                    price_adapter.set_time_to(to);
                }
                Action::GetPrices {
                    feed_ids,
                    payload,
                    signer,
                    expected_values,
                    expected_timestamp,
                } => {
                    let (values, timestamp) = price_adapter.process_payload_get(
                        hex::decode(payload).unwrap(),
                        feed_ids
                            .iter()
                            .map(|feed| feed.as_bytes().to_vec())
                            .collect(),
                        signer,
                    );
                    assert_eq!(timestamp, expected_timestamp);
                    assert_eq!(values, expected_values);
                }
            };
        }
    }

    pub fn scenario_steps_from_sample(
        self,
        sample: Sample,
        init_time: InitTime,
        signer: Signer,
        feeds_overwrite: Option<Vec<&str>>,
    ) -> Self {
        let feeds: Vec<_> = sample.values.keys().map(std::ops::Deref::deref).collect();

        let scenario = match init_time {
            InitTime::No => self,
            InitTime::SetToSampleTime => {
                self.then_set_clock(Duration::from_millis(sample.system_timestamp))
            }
        };

        let feeds = match feeds_overwrite {
            Some(feeds) => feeds,
            None => feeds,
        };

        scenario
            .then_write_prices(feeds.clone(), sample.content, signer)
            .then_check_prices(
                feeds,
                sample.values.values().cloned().collect(),
                sample.timestamp,
            )
    }
}
