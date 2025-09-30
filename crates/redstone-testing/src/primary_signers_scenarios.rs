use core::{ops::Add, time::Duration};

use crate::{
    sample::{
        sample_btc_5sig, sample_btc_5sig_newer, sample_btc_eth_3sig, sample_btc_eth_3sig_newer,
        sample_eth_2sig, sample_eth_3sig, sample_eth_3sig_newer, Sample, DEFAULT_SIGNERS_THRESHOLD,
    },
    scenario::{InitTime, Scenario},
    signer::ContractUpdateSigner,
};

pub fn scenario_trusted_updates_twice_without_waiting_for_threshold(
    threshold: Duration,
) -> Scenario {
    let less_than_threshold_duration = threshold.div_f32(2_f32);
    let first_sample = sample_eth_3sig();
    let second_sample = sample_eth_3sig_newer();

    Scenario::default()
        .scenario_steps_from_sample_with_initialization(
            first_sample,
            InitTime::SetToSampleTime,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
        .then_advance_clock(less_than_threshold_duration)
        .scenario_steps_from_sample(
            second_sample,
            InitTime::No,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
}

pub fn scenario_untrusted_updates_twice_waiting_for_threshold(threshold: Duration) -> Scenario {
    let more_than_threshold = threshold.add(Duration::from_secs(1));
    let first_sample = sample_eth_3sig();
    let second_sample = sample_eth_3sig_newer();

    Scenario::default()
        .scenario_steps_from_sample_with_initialization(
            first_sample,
            InitTime::SetToSampleTime,
            ContractUpdateSigner::Untrusted,
            None,
            None,
            None,
        )
        .then_advance_clock(more_than_threshold)
        .scenario_steps_from_sample(
            second_sample,
            InitTime::No,
            ContractUpdateSigner::Untrusted,
            None,
            None,
            None,
        )
}

pub fn scenario_updating_twice_with_the_same_timestamp() -> Scenario {
    let sample = sample_eth_3sig();

    let feeds = sample.feeds();
    let write_time = sample.system_timestamp;

    Scenario::default()
        .scenario_steps_from_sample_with_initialization(
            sample.clone(),
            InitTime::SetToSampleTime,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
        .then_advance_clock(Duration::from_secs(1))
        .scenario_steps_from_sample(
            sample,
            InitTime::No,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
        .then_check_write_timestamp(feeds, write_time)
}

pub fn scenario_updating_with_only_2_signers() -> Scenario {
    let sample = sample_eth_2sig();

    Scenario::default().scenario_steps_from_sample_with_initialization(
        sample.clone(),
        InitTime::SetToSampleTime,
        ContractUpdateSigner::Trusted,
        None,
        None,
        None,
    )
}

pub fn scenario_untrusted_updates_twice_without_waiting_for_threshold(
    threshold: Duration,
) -> Scenario {
    let less_than_threshold_duration = threshold.div_f32(2_f32);
    let first_sample = sample_eth_3sig();
    let second_sample = sample_eth_3sig_newer();

    let timestamp_overwrite = Some(first_sample.timestamp);

    let values_after_second_update = Some(first_sample.values.values().cloned().collect());

    Scenario::default()
        .scenario_steps_from_sample_with_initialization(
            first_sample,
            InitTime::SetToSampleTime,
            ContractUpdateSigner::Untrusted,
            None,
            None,
            None,
        )
        .then_advance_clock(less_than_threshold_duration)
        .scenario_steps_from_sample(
            second_sample,
            InitTime::No,
            ContractUpdateSigner::Untrusted,
            None,
            values_after_second_update,
            timestamp_overwrite,
        )
}

pub fn scenario_missing_feed_in_payload() -> Scenario {
    let sample = sample_eth_3sig();

    Scenario::default().scenario_steps_from_sample(
        sample,
        InitTime::SetToSampleTime,
        ContractUpdateSigner::Trusted,
        Some(vec!["BTC"]),
        None,
        None,
    )
}

pub fn scenario_one_missing_feed_in_payload() -> Scenario {
    let sample = sample_eth_3sig();

    Scenario::default().scenario_steps_from_sample(
        sample,
        InitTime::SetToSampleTime,
        ContractUpdateSigner::Trusted,
        Some(vec!["ETH", "BTC"]),
        None,
        None,
    )
}

pub fn scenario_2_feed_update(threshold: Duration) -> Scenario {
    let more_than_threshold = threshold.add(Duration::from_secs(1));
    let first_sample = sample_btc_eth_3sig();
    let second_sample = sample_btc_eth_3sig_newer();

    Scenario::default()
        .scenario_steps_from_sample_with_initialization(
            first_sample,
            InitTime::SetToSampleTime,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
        .then_advance_clock(more_than_threshold)
        .scenario_steps_from_sample(
            second_sample,
            InitTime::No,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
}

pub fn scenario_payload_with_multiple_feed_update_one(threshold: Duration) -> Scenario {
    let more_than_threshold = threshold.add(Duration::from_secs(1));
    let first_sample = sample_btc_eth_3sig();
    let second_sample = sample_btc_eth_3sig_newer();

    Scenario::default()
        .scenario_steps_from_sample_with_initialization(
            first_sample,
            InitTime::SetToSampleTime,
            ContractUpdateSigner::Trusted,
            Some(vec!["ETH"]),
            None,
            None,
        )
        .then_advance_clock(more_than_threshold)
        .scenario_steps_from_sample(
            second_sample,
            InitTime::No,
            ContractUpdateSigner::Trusted,
            Some(vec!["ETH"]),
            None,
            None,
        )
}

pub fn scenario_with_5_signers(threshold: Duration) -> Scenario {
    let more_than_threshold = threshold.add(Duration::from_secs(1));
    let first_sample = sample_btc_5sig();
    let second_sample = sample_btc_5sig_newer();

    Scenario::default()
        .scenario_steps_from_sample_with_initialization(
            first_sample,
            InitTime::SetToSampleTime,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
        .then_advance_clock(more_than_threshold)
        .scenario_steps_from_sample(
            second_sample,
            InitTime::No,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
}

pub fn scenario_adapter_update_with_old_timestamp(max_timestamp_delay: Duration) -> Scenario {
    let sample = Sample::any_valid();

    let system_time =
        Duration::from_millis(sample.timestamp) + max_timestamp_delay + Duration::from_secs(1);

    Scenario::default()
        .then_set_clock(system_time)
        .scenario_steps_from_sample_with_initialization(
            sample.clone(),
            InitTime::No,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
}

pub fn scenario_adapter_update_with_future_timestamp(max_timestamp_ahead_ms: Duration) -> Scenario {
    let sample = Sample::any_valid();

    let system_time =
        Duration::from_millis(sample.timestamp) - max_timestamp_ahead_ms - Duration::from_secs(1);

    Scenario::default()
        .then_set_clock(system_time)
        .scenario_steps_from_sample_with_initialization(
            sample,
            InitTime::No,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
}

pub fn scenario_adapter_update_with_almost_old_timestamp(
    max_timestamp_delay: Duration,
) -> Scenario {
    let sample = Sample::any_valid();

    let system_time = Duration::from_millis(sample.timestamp) + max_timestamp_delay;

    Scenario::default()
        .then_set_clock(system_time)
        .scenario_steps_from_sample_with_initialization(
            sample,
            InitTime::No,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
}

pub fn scenario_adapter_update_with_almost_future_timestamp(
    max_timestamp_ahead_ms: Duration,
) -> Scenario {
    let sample = Sample::any_valid();

    let system_time = Duration::from_millis(sample.timestamp) - max_timestamp_ahead_ms;

    Scenario::default()
        .then_set_clock(system_time)
        .scenario_steps_from_sample_with_initialization(
            sample,
            InitTime::No,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
}

pub fn scenario_check_initalization() -> Scenario {
    let sample = Sample::any_valid();

    Scenario::default()
        .then_initialize(sample.signers, DEFAULT_SIGNERS_THRESHOLD)
        .then_check_unique_threshold_count(DEFAULT_SIGNERS_THRESHOLD)
}

pub fn scenario_read_stale_data(data_ttl: Duration) -> Scenario {
    let sample = sample_eth_3sig();

    Scenario::default()
        .scenario_steps_from_sample_with_initialization(
            sample.clone(),
            InitTime::SetToSampleTime,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
        .then_advance_clock(data_ttl)
        .then_read_prices(sample.feeds().iter().map(|s| s.as_str()).collect())
}

pub fn scenario_read_data(data_ttl: Duration) -> Scenario {
    let sample = sample_eth_3sig();

    Scenario::default()
        .scenario_steps_from_sample_with_initialization(
            sample.clone(),
            InitTime::SetToSampleTime,
            ContractUpdateSigner::Trusted,
            None,
            None,
            None,
        )
        .then_advance_clock(data_ttl - Duration::from_secs(1))
        .then_read_prices(sample.feeds().iter().map(|s| s.as_str()).collect())
}
