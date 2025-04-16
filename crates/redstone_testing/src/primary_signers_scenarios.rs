use crate::{
    env::Signer,
    sample::{sample_eth_2sig, sample_eth_3sig, sample_eth_3sig_newer},
    scenario::Scenario,
};
use core::{ops::Add, time::Duration};

pub fn scenario_trusted_updates_twice_without_waiting_for_threshold(
    threshold: Duration,
) -> Scenario {
    let less_than_threshold_duration = threshold.div_f32(2_f32);
    let first_sample = sample_eth_3sig();
    let second_sample = sample_eth_3sig_newer();

    let feed = "ETH";

    Scenario::new()
        .then_set_clock(Duration::from_millis(first_sample.system_timestamp))
        .then_write_price(feed, first_sample.content, Signer::Trusted)
        .then_check_prices(
            vec![feed],
            vec![*first_sample.values.get(feed).unwrap()],
            first_sample.timestamp,
        )
        .then_advance_clock(less_than_threshold_duration)
        .then_write_price(feed, second_sample.content, Signer::Trusted)
        .then_check_prices(
            vec![feed],
            vec![*second_sample.values.get(feed).unwrap()],
            second_sample.timestamp,
        )
}

pub fn scenario_untrusted_updates_twice_waiting_for_threshold(threshold: Duration) -> Scenario {
    let more_than_threshold = threshold.add(Duration::from_secs(1));
    let first_sample = sample_eth_3sig();
    let second_sample = sample_eth_3sig_newer();

    let feed = "ETH";

    Scenario::new()
        .then_set_clock(Duration::from_millis(first_sample.system_timestamp))
        .then_write_price(feed, first_sample.content, Signer::Untrusted)
        .then_check_prices(
            vec![feed],
            vec![*first_sample.values.get(feed).unwrap()],
            first_sample.timestamp,
        )
        .then_advance_clock(more_than_threshold)
        .then_write_price(feed, second_sample.content, Signer::Untrusted)
        .then_check_prices(
            vec![feed],
            vec![*second_sample.values.get(feed).unwrap()],
            second_sample.timestamp,
        )
}

pub fn scenario_updating_twice_with_the_same_timestamp() -> Scenario {
    let first_sample = sample_eth_3sig();

    let feed = "ETH";

    Scenario::new()
        .then_set_clock(Duration::from_millis(first_sample.system_timestamp))
        .then_write_price(feed, first_sample.content, Signer::Trusted)
        .then_check_prices(
            vec![feed],
            vec![*first_sample.values.get(feed).unwrap()],
            first_sample.timestamp,
        )
        .then_advance_clock(Duration::from_millis(1))
        .then_write_price(feed, first_sample.content, Signer::Trusted)
}

pub fn scenario_updating_with_only_2_signers() -> Scenario {
    let sample = sample_eth_2sig();

    let feed = "ETH";

    Scenario::new()
        .then_set_clock(Duration::from_millis(sample.system_timestamp))
        .then_write_price(feed, sample.content, Signer::Trusted)
        .then_check_prices(
            vec![feed],
            vec![*sample.values.get(feed).unwrap()],
            sample.timestamp,
        )
        .then_advance_clock(Duration::from_millis(1))
        .then_write_price(feed, sample.content, Signer::Trusted)
}

pub fn scenario_untrusted_updates_twice_without_waiting_for_threshold(
    threshold: Duration,
) -> Scenario {
    let less_than_threshold_duration = threshold.div_f32(2_f32);
    let first_sample = sample_eth_3sig();
    let second_sample = sample_eth_3sig_newer();

    let feed = "ETH";

    Scenario::new()
        .then_set_clock(Duration::from_millis(first_sample.system_timestamp))
        .then_write_price(feed, first_sample.content, Signer::Untrusted)
        .then_check_prices(
            vec![feed],
            vec![*first_sample.values.get(feed).unwrap()],
            first_sample.timestamp,
        )
        .then_advance_clock(less_than_threshold_duration)
        .then_write_price(feed, second_sample.content, Signer::Untrusted)
        .then_check_prices(
            vec![feed],
            vec![*second_sample.values.get(feed).unwrap()],
            second_sample.timestamp,
        )
}
