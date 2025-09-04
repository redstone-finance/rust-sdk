use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::payload_decoding::benches,
    benchmarks::payload_processing::benches,
}
