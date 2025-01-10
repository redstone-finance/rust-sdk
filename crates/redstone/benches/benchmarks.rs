use criterion::{criterion_group, criterion_main, Criterion};
use redstone::utils::slice::check_no_duplicates;

const ELEM_COUNT: usize = 256;

fn benchmark_has_duplicates_unique_reversed(c: &mut Criterion) {
    let mut slice = vec![];
    for i in (0..ELEM_COUNT).rev() {
        slice.push(i);
    }

    c.bench_function("benchmark_has_duplicates_unique_reversed", |b| {
        b.iter(|| {
            if check_no_duplicates(&slice).is_err() {
                panic!("Shouldn't find any repetition in benchmark");
            };
        })
    });
}

fn benchmark_has_duplicates_unique_sorted(c: &mut Criterion) {
    let mut slice = vec![];
    for i in 0..ELEM_COUNT {
        slice.push(i);
    }

    c.bench_function("benchmark_has_duplicates_unique_sorted", |b| {
        b.iter(|| {
            if check_no_duplicates(&slice).is_err() {
                panic!("Shouldn't find any repetition in benchmark");
            };
        })
    });
}

fn benchmark_has_duplicates_unique_shuffled(c: &mut Criterion) {
    let slice = vec![
        94, 218, 60, 212, 192, 42, 177, 209, 232, 95, 127, 89, 41, 133, 251, 130, 53, 84, 3, 46,
        123, 175, 152, 143, 57, 38, 139, 132, 171, 118, 147, 105, 166, 124, 215, 233, 44, 160, 237,
        149, 163, 162, 96, 70, 161, 1, 191, 78, 67, 231, 30, 35, 244, 145, 47, 99, 186, 0, 158,
        247, 128, 154, 214, 194, 223, 37, 72, 169, 62, 227, 136, 59, 129, 80, 235, 58, 222, 106,
        23, 10, 24, 200, 178, 69, 252, 202, 198, 153, 52, 142, 31, 195, 61, 181, 254, 190, 242,
        112, 148, 64, 101, 167, 75, 114, 33, 168, 224, 249, 164, 87, 174, 208, 108, 34, 117, 144,
        245, 180, 119, 213, 65, 179, 115, 126, 74, 63, 20, 196, 159, 16, 206, 243, 131, 157, 26,
        103, 83, 79, 246, 116, 4, 113, 187, 229, 219, 6, 54, 36, 86, 12, 207, 104, 250, 141, 109,
        55, 45, 228, 27, 43, 100, 110, 176, 156, 102, 85, 248, 146, 189, 32, 184, 140, 137, 66,
        122, 97, 221, 98, 225, 150, 236, 134, 199, 165, 76, 107, 170, 135, 182, 203, 19, 211, 239,
        220, 238, 71, 48, 234, 22, 88, 29, 172, 13, 21, 204, 205, 120, 226, 197, 77, 7, 111, 151,
        193, 8, 15, 240, 5, 91, 14, 39, 25, 125, 50, 155, 82, 253, 230, 92, 56, 121, 201, 2, 93,
        40, 217, 210, 18, 241, 185, 68, 28, 73, 188, 216, 173, 183, 90, 51, 17, 138, 9, 255, 11,
        49, 81,
    ];
    c.bench_function("benchmark_has_duplicates_unique_shuffled", |b| {
        b.iter(|| {
            if check_no_duplicates(&slice).is_err() {
                panic!("Shouldn't find any repetition in benchmark");
            };
        })
    });
}

fn benchmark_has_duplicates_not_unique_shuffled(c: &mut Criterion) {
    let slice = vec![
        94, 218, 60, 212, 192, 42, 177, 209, 232, 95, 127, 89, 41, 133, 251, 130, 53, 84, 3, 46,
        123, 175, 152, 143, 57, 38, 139, 132, 171, 118, 147, 105, 166, 124, 215, 233, 44, 160, 237,
        149, 163, 162, 96, 70, 161, 1, 191, 78, 67, 231, 30, 35, 244, 145, 47, 99, 186, 0, 158,
        247, 128, 154, 214, 194, 223, 37, 72, 169, 62, 227, 136, 59, 129, 80, 235, 58, 222, 106,
        23, 10, 24, 200, 178, 69, 252, 202, 198, 153, 52, 142, 31, 195, 61, 181, 254, 190, 242,
        112, 148, 64, 101, 167, 75, 114, 33, 168, 224, 249, 164, 87, 174, 208, 108, 34, 117, 144,
        245, 180, 119, 213, 65, 179, 115, 126, 74, 63, 20, 196, 159, 16, 206, 243, 131, 157, 26,
        103, 83, 79, 246, 116, 4, 113, 187, 229, 219, 6, 54, 36, 86, 12, 207, 104, 250, 141, 109,
        55, 45, 228, 27, 43, 100, 110, 176, 156, 102, 85, 248, 146, 189, 32, 184, 140, 137, 66,
        122, 97, 221, 98, 225, 150, 236, 134, 199, 165, 76, 107, 170, 135, 182, 203, 19, 211, 239,
        220, 238, 71, 48, 234, 22, 88, 29, 172, 13, 21, 204, 205, 120, 226, 197, 77, 7, 111, 151,
        193, 8, 15, 240, 5, 91, 14, 39, 25, 125, 50, 155, 82, 253, 230, 92, 56, 121, 201, 2, 93,
        40, 217, 210, 18, 241, 185, 68, 28, 73, 188, 216, 173, 183, 90, 51, 17, 138, 9, 254, 11,
        49, 81,
    ];

    c.bench_function("benchmark_has_duplicates_unique_shuffled", |b| {
        b.iter(|| {
            let Err(_) = check_no_duplicates(&slice) else {
                panic!("Shouldn't find any repetition in benchmark");
            };
        })
    });
}

fn benchmark_has_duplicates_unique_shuffled_extra_small(c: &mut Criterion) {
    let slice = vec![94, 218, 60, 212];

    c.bench_function(
        "benchmark_has_duplicates_unique_shuffled_extra_small",
        |b| {
            b.iter(|| {
                if check_no_duplicates(&slice).is_err() {
                    panic!("Shouldn't find any repetition in benchmark");
                };
            })
        },
    );
}
fn benchmark_has_duplicates_unique_shuffled_medium(c: &mut Criterion) {
    let slice = vec![
        94, 218, 60, 212, 192, 42, 177, 209, 232, 95, 127, 89, 41, 133, 251, 130, 53, 84, 3, 46,
    ];

    c.bench_function(
        "benchmark_has_duplicates_unique_shuffled_quite_small",
        |b| {
            b.iter(|| {
                if check_no_duplicates(&slice).is_err() {
                    panic!("Shouldn't find any repetition in benchmark");
                };
            })
        },
    );
}

fn benchmark_has_duplicates_unique_shuffled_small(c: &mut Criterion) {
    let slice = vec![94, 218, 60, 212, 192, 42, 177, 209, 232, 95, 127, 89];

    c.bench_function(
        "benchmark_has_duplicates_unique_shuffled_quite_small",
        |b| {
            b.iter(|| {
                if check_no_duplicates(&slice).is_err() {
                    panic!("Shouldn't find any repetition in benchmark");
                };
            })
        },
    );
}

criterion_group!(
    benches,
    benchmark_has_duplicates_unique_sorted,
    benchmark_has_duplicates_unique_reversed,
    benchmark_has_duplicates_unique_shuffled,
    benchmark_has_duplicates_not_unique_shuffled,
    benchmark_has_duplicates_unique_shuffled_extra_small,
    benchmark_has_duplicates_unique_shuffled_medium,
    benchmark_has_duplicates_unique_shuffled_small
);

criterion_main!(benches);
