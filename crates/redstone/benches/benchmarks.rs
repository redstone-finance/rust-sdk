use criterion::{criterion_group, criterion_main, Criterion};
use redstone::helpers::validate::has_repetition_quadratic_lookup;

const ELEM_COUNT: usize = 10_000;

fn benchmark_quadratic_lookup(c: &mut Criterion) {
    let mut nums = vec![];
    for i in (0..ELEM_COUNT).rev() {
        nums.push(i);
    }

    c.bench_function("benchmark_quadratic_lookup", |b| {
        b.iter(|| {
            has_repetition_quadratic_lookup(&nums);
        })
    });
}

criterion_group!(benches, benchmark_quadratic_lookup);

criterion_main!(benches);
