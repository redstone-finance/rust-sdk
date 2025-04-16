use core::{ops::Add, time::Duration};

use crate::{
    env::Signer,
    sample::{
        sample_btc_5sig, sample_btc_5sig_newer, sample_btc_eth_3sig, sample_btc_eth_3sig_newer,
        sample_eth_2sig, sample_eth_3sig, sample_eth_3sig_newer, Sample,
    },
    scenario::{InitTime, Scenario},
};

pub fn scenario_trusted_updates_twice_without_waiting_for_threshold(
    threshold: Duration,
) -> Scenario {
    let less_than_threshold_duration = threshold.div_f32(2_f32);
    let first_sample = sample_eth_3sig();
    let second_sample = sample_eth_3sig_newer();

    Scenario::default()
        .scenario_steps_from_sample(
            first_sample,
            InitTime::SetToSampleTime,
            Signer::Trusted,
            None,
        )
        .then_advance_clock(less_than_threshold_duration)
        .scenario_steps_from_sample(second_sample, InitTime::No, Signer::Trusted, None)
}

pub fn scenario_untrusted_updates_twice_waiting_for_threshold(threshold: Duration) -> Scenario {
    let more_than_threshold = threshold.add(Duration::from_secs(1));
    let first_sample = sample_eth_3sig();
    let second_sample = sample_eth_3sig_newer();

    Scenario::default()
        .scenario_steps_from_sample(
            first_sample,
            InitTime::SetToSampleTime,
            Signer::Untrusted,
            None,
        )
        .then_advance_clock(more_than_threshold)
        .scenario_steps_from_sample(second_sample, InitTime::No, Signer::Untrusted, None)
}

pub fn scenario_updating_twice_with_the_same_timestamp() -> Scenario {
    let sample = sample_eth_3sig();

    Scenario::default()
        .scenario_steps_from_sample(
            sample.clone(),
            InitTime::SetToSampleTime,
            Signer::Trusted,
            None,
        )
        .then_advance_clock(Duration::from_secs(1))
        .scenario_steps_from_sample(sample, InitTime::No, Signer::Trusted, None)
}

pub fn scenario_updating_with_only_2_signers() -> Scenario {
    let sample = sample_eth_2sig();

    Scenario::default().scenario_steps_from_sample(
        sample.clone(),
        InitTime::SetToSampleTime,
        Signer::Trusted,
        None,
    )
}

pub fn scenario_untrusted_updates_twice_without_waiting_for_threshold(
    threshold: Duration,
) -> Scenario {
    let less_than_threshold_duration = threshold.div_f32(2_f32);
    let first_sample = sample_eth_3sig();
    let second_sample = sample_eth_3sig_newer();

    Scenario::default()
        .scenario_steps_from_sample(
            first_sample,
            InitTime::SetToSampleTime,
            Signer::Untrusted,
            None,
        )
        .then_advance_clock(less_than_threshold_duration)
        .scenario_steps_from_sample(second_sample, InitTime::No, Signer::Untrusted, None)
}

pub fn scenario_missing_feed_in_payload() -> Scenario {
    let sample = sample_eth_3sig();

    Scenario::default().scenario_steps_from_sample(
        sample,
        InitTime::SetToSampleTime,
        Signer::Trusted,
        Some(vec!["BTC"]),
    )
}

pub fn scenario_2_feed_update(threshold: Duration) -> Scenario {
    let more_than_threshold = threshold.add(Duration::from_secs(1));
    let first_sample = sample_btc_eth_3sig();
    let second_sample = sample_btc_eth_3sig_newer();

    Scenario::default()
        .scenario_steps_from_sample(
            first_sample,
            InitTime::SetToSampleTime,
            Signer::Trusted,
            None,
        )
        .then_advance_clock(more_than_threshold)
        .scenario_steps_from_sample(second_sample, InitTime::No, Signer::Trusted, None)
}

pub fn scenario_payload_with_multiple_feed_update_one(threshold: Duration) -> Scenario {
    let more_than_threshold = threshold.add(Duration::from_secs(1));
    let first_sample = sample_btc_eth_3sig();
    let second_sample = sample_btc_eth_3sig_newer();

    Scenario::default()
        .scenario_steps_from_sample(
            first_sample,
            InitTime::SetToSampleTime,
            Signer::Trusted,
            Some(vec!["ETH"]),
        )
        .then_advance_clock(more_than_threshold)
        .scenario_steps_from_sample(
            second_sample,
            InitTime::No,
            Signer::Trusted,
            Some(vec!["ETH"]),
        )
}

pub fn scenario_with_5_signers(threshold: Duration) -> Scenario {
    let more_than_threshold = threshold.add(Duration::from_secs(1));
    let first_sample = sample_btc_5sig();
    let second_sample = sample_btc_5sig_newer();

    Scenario::default()
        .scenario_steps_from_sample(
            first_sample,
            InitTime::SetToSampleTime,
            Signer::Trusted,
            None,
        )
        .then_advance_clock(more_than_threshold)
        .scenario_steps_from_sample(second_sample, InitTime::No, Signer::Trusted, None)
}

pub fn scenario_adapter_update_with_old_timestamp(max_timestamp_delay: Duration) -> Scenario {
    let sample = Sample::any_valid();

    let system_time =
        Duration::from_millis(sample.timestamp) + max_timestamp_delay + Duration::from_secs(1);

    Scenario::default()
        .then_set_clock(system_time)
        .scenario_steps_from_sample(sample.clone(), InitTime::No, Signer::Trusted, None)
        .then_check_prices(
            sample.values.keys().map(std::ops::Deref::deref).collect(),
            sample.values.values().cloned().collect(),
            0,
        )
}

pub fn scenario_adapter_update_with_future_timestamp(max_timestamp_ahead_ms: Duration) -> Scenario {
    let sample = Sample::any_valid();

    let system_time =
        Duration::from_millis(sample.timestamp) - max_timestamp_ahead_ms - Duration::from_secs(1);

    Scenario::default()
        .then_set_clock(system_time)
        .scenario_steps_from_sample(sample, InitTime::No, Signer::Trusted, None)
}

pub fn scenario_adapter_update_with_almost_old_timestamp(
    max_timestamp_delay: Duration,
) -> Scenario {
    let sample = Sample::any_valid();

    let system_time = Duration::from_millis(sample.timestamp) + max_timestamp_delay;

    Scenario::default()
        .then_set_clock(system_time)
        .scenario_steps_from_sample(sample, InitTime::No, Signer::Trusted, None)
}

pub fn scenario_adapter_update_with_almost_future_timestamp(
    max_timestamp_ahead_ms: Duration,
) -> Scenario {
    let sample = Sample::any_valid();

    let system_time = Duration::from_millis(sample.timestamp) - max_timestamp_ahead_ms;

    Scenario::default()
        .then_set_clock(system_time)
        .scenario_steps_from_sample(sample, InitTime::No, Signer::Trusted, None)
}
