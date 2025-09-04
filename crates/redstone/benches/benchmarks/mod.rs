#[cfg(feature = "bench")]
pub mod payload_decoding;
#[cfg(feature = "bench")]
pub mod payload_processing;

#[cfg(not(feature = "bench"))]
pub mod payload_decoding {
    use criterion::{criterion_group, Criterion};
    fn some_benchmark(_c: &mut Criterion) {}

    criterion_group!(benches, some_benchmark);
}

#[cfg(not(feature = "bench"))]
pub mod payload_processing {
    use criterion::{criterion_group, Criterion};
    fn some_benchmark(_c: &mut Criterion) {}

    criterion_group!(benches, some_benchmark);
}
