use criterion::{criterion_group, Criterion};
use redstone::{core::decode_payload, default_ext::DefaultCrypto};
use redstone_utils::hex::hex_to_bytes;
use samples::*;

fn benchmark_all_samples(c: &mut Criterion) {
    let samples = [
        ("eth_btc_avax_5sig_old", sample_eth_btc_avax_5sig_old()),
        ("eth_btc_avax_5sig", sample_eth_btc_avax_5sig()),
        ("eth_btc_avax_5sig_2", sample_eth_btc_avax_5sig_2()),
        ("eth_3sig", sample_eth_3sig()),
        ("eth_2sig", sample_eth_2sig()),
        ("eth_3sig_newer", sample_eth_3sig_newer()),
        ("btc_eth_3sig", sample_btc_eth_3sig()),
        ("btc_eth_3sig_newer", sample_btc_eth_3sig_newer()),
        ("btc_5sig", sample_btc_5sig()),
        ("btc_5sig_newer", sample_btc_5sig_newer()),
    ];

    let mut group = c.benchmark_group("decode_payload");

    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(3));

    for (name, sample) in samples {
        let payload = hex_to_bytes(sample.content.to_string());
        let mut crypto = DefaultCrypto;

        group.bench_function(name, |b| {
            b.iter(|| {
                return;
                // let test_payload = payload.clone();
                // decode_payload(&mut crypto, test_payload).unwrap()
            })
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_all_samples,);
