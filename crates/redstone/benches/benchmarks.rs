use criterion::{criterion_group, criterion_main, Criterion};
use redstone::helpers::slice::has_duplicates;

const ELEM_COUNT: usize = 256;

fn benchmark_has_repetition(c: &mut Criterion) {
    let mut nums = vec![];
    for i in (0..ELEM_COUNT).rev() {
        nums.push(i);
    }

    c.bench_function("benchmark_has_repetition", |b| {
        b.iter(|| {
            if let Some(_) = has_duplicates(&nums) {
                panic!("Shouldn't find any repetition in benchmark");
            };
        })
    });
}

criterion_group!(benches, benchmark_has_repetition,);

criterion_main!(benches);
