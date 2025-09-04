use criterion::{criterion_group, Criterion};
use redstone::{
    core::{config::Config, process_payload},
    default_ext::DefaultCrypto,
    network::Dummy,
    RedStoneConfigImpl,
};
use redstone_utils::hex::hex_to_bytes;
use redstone_utils::hex::make_hex_value_from_string;

use samples::*;

fn create_config(sample: &Sample) -> RedStoneConfigImpl<DefaultCrypto, Dummy> {
    let config = Config::try_new(
        1,
        sample
            .signers
            .get_signers()
            .iter()
            .map(|signer| hex_to_bytes(signer.to_string()).into())
            .collect(),
        sample
            .feeds
            .iter()
            .map(|feed| make_hex_value_from_string(feed))
            .collect(),
        sample.system_timestamp.into(),
        None,
        None,
    )
    .unwrap();

    RedStoneConfigImpl::from((config, DefaultCrypto))
}

fn benchmark_validation_all_samples(c: &mut Criterion) {
    let samples = [
        (
            "validation_eth_btc_avax_5sig_old",
            sample_eth_btc_avax_5sig_old(),
        ),
        ("validation_eth_btc_avax_5sig", sample_eth_btc_avax_5sig()),
        (
            "validation_eth_btc_avax_5sig_2",
            sample_eth_btc_avax_5sig_2(),
        ),
        ("validation_eth_3sig", sample_eth_3sig()),
        ("validation_eth_2sig", sample_eth_2sig()),
        ("validation_eth_3sig_newer", sample_eth_3sig_newer()),
        ("validation_btc_eth_3sig", sample_btc_eth_3sig()),
        ("validation_btc_eth_3sig_newer", sample_btc_eth_3sig_newer()),
        ("validation_btc_5sig", sample_btc_5sig()),
        ("validation_btc_5sig_newer", sample_btc_5sig_newer()),
    ];

    let mut group = c.benchmark_group("process_payload");

    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(3));

    for (name, sample) in samples {
        let payload = hex_to_bytes(sample.content.to_string());
        let mut config = create_config(&sample);

        group.bench_function(name, |b| {
            b.iter(|| {
                let test_payload = payload.clone();
                let _ = process_payload(&mut config, test_payload);
            })
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_validation_all_samples,);
