use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_placeholder(c: &mut Criterion) {
    c.bench_function("benchmark_placeholder", |b| {
        b.iter(|| {
            let _a: Vec<u8> = Vec::with_capacity(256);
        })
    });
}
criterion_group!(benches, benchmark_placeholder,);

criterion_main!(benches);
