use core::time::Duration;

use redstone::Value;

use crate::{
    env::PriceAdapterRunEnv,
    package_signers::Signers,
    sample::{Sample, DEFAULT_SIGNERS_THRESHOLD},
    signer::ContractUpdateSigner,
};

struct Initialize {
    signers: Signers,
    unique_signer_threshold: u8,
}

enum Action {
    Initialize(Initialize),
    WritePrice {
        feed_id: String,
        payload: String,
        signer: ContractUpdateSigner,
    },
    WritePrices {
        feed_ids: Vec<String>,
        payload: String,
        signer: ContractUpdateSigner,
    },
    GetPrices {
        feed_ids: Vec<String>,
        payload: String,
        signer: ContractUpdateSigner,
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
    CheckUniqueContractUpdateSignerCount {
        expected: u8,
    },
    ReadPrices {
        feed_ids: Vec<String>,
    },
    CheckWriteTime {
        feed_ids: Vec<String>,
        expected_timestamp: u64,
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

    pub fn then_write_price(
        mut self,
        feed_id: &str,
        payload: &str,
        signer: ContractUpdateSigner,
    ) -> Self {
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

    pub fn then_write_prices(
        mut self,
        feed_ids: Vec<&str>,
        payload: &str,
        signer: ContractUpdateSigner,
    ) -> Self {
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
        signer: ContractUpdateSigner,
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

    pub fn then_initialize(mut self, signers: Signers, unique_signer_threshold: u8) -> Self {
        self.actions.push(Action::Initialize(Initialize {
            signers,
            unique_signer_threshold,
        }));

        self
    }

    pub fn then_check_unique_threshold_count(mut self, expected: u8) -> Self {
        self.actions
            .push(Action::CheckUniqueContractUpdateSignerCount { expected });

        self
    }

    pub fn then_read_prices(mut self, feed_ids: Vec<&str>) -> Self {
        self.actions.push(Action::ReadPrices {
            feed_ids: feed_ids.iter().map(|feed| feed.to_string()).collect(),
        });

        self
    }

    pub fn then_check_write_timestamp(
        mut self,
        feed_ids: Vec<String>,
        expected_timestamp: u64,
    ) -> Self {
        self.actions.push(Action::CheckWriteTime {
            feed_ids,
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
                Action::Initialize(Initialize {
                    signers,
                    unique_signer_threshold,
                }) => {
                    price_adapter.initialize(
                        signers
                            .get_signers()
                            .into_iter()
                            .map(|signer| hex::decode(signer).unwrap())
                            .collect(),
                        unique_signer_threshold,
                    );
                }
                Action::CheckUniqueContractUpdateSignerCount { expected } => {
                    let unique_signer_count = price_adapter.unique_signer_threshold();

                    assert_eq!(unique_signer_count, expected);
                }
                Action::ReadPrices { feed_ids } => {
                    assert!(price_adapter
                        .read_prices(
                            feed_ids
                                .iter()
                                .map(|feed| feed.as_bytes().to_vec())
                                .collect(),
                        )
                        .is_some());
                }
                Action::CheckWriteTime {
                    feed_ids,
                    expected_timestamp,
                } => {
                    for feed in feed_ids {
                        assert_eq!(
                            price_adapter.read_write_timestamp(feed.as_bytes().to_vec()),
                            expected_timestamp
                        );
                    }
                }
            };
        }
    }

    pub fn scenario_steps_from_sample(
        self,
        sample: Sample,
        init_time: InitTime,
        signer: ContractUpdateSigner,
        feeds_overwrite: Option<Vec<&str>>,
        values_overwrite: Option<Vec<Value>>,
    ) -> Self {
        let scenario = match init_time {
            InitTime::No => self,
            InitTime::SetToSampleTime => {
                self.then_set_clock(Duration::from_millis(sample.system_timestamp))
            }
        };

        let (values, feeds) = match feeds_overwrite {
            Some(feeds) => (
                feeds
                    .iter()
                    .filter_map(|f| sample.values.get(*f).cloned())
                    .collect(),
                feeds,
            ),
            None => (
                sample.values.values().cloned().collect(),
                sample.values.keys().map(std::ops::Deref::deref).collect(),
            ),
        };

        let values = match values_overwrite {
            Some(v) => v,
            _ => values,
        };

        scenario
            .then_write_prices(feeds.clone(), sample.content, signer)
            .then_check_prices(feeds, values, sample.timestamp)
    }

    pub fn scenario_steps_from_sample_with_initialization(
        self,
        sample: Sample,
        init_time: InitTime,
        signer: ContractUpdateSigner,
        feeds_overwrite: Option<Vec<&str>>,
        values_overwrite: Option<Vec<Value>>,
    ) -> Self {
        self.then_initialize(sample.signers, DEFAULT_SIGNERS_THRESHOLD)
            .scenario_steps_from_sample(
                sample,
                init_time,
                signer,
                feeds_overwrite,
                values_overwrite,
            )
    }
}
