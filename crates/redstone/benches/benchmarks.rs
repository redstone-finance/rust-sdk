use std::collections::HashSet;

use criterion::{criterion_group, criterion_main, Criterion};
use redstone::{helpers::trie::Trie, FeedId};

const TEST_FEED_IDS: &[&str; 540] = &[
    "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA",
    "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA",
    "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB",
    "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT",
    "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE",
    "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB",
    "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC",
    "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ",
    "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC",
    "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA",
    "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB",
    "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD",
    "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD",
    "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC",
    "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD",
    "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA",
    "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA",
    "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB",
    "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT",
    "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE",
    "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB",
    "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC",
    "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ",
    "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC",
    "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA",
    "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB",
    "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD",
    "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD",
    "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC",
    "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD",
    "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA",
    "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA",
    "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB",
    "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT",
    "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE",
    "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB",
    "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC",
    "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ",
    "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA", "ABA", "ADC",
    "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB", "ACB", "AAA",
    "ABA", "ADC", "DCA", "XYZ", "XYZD", "ABC", "ACC", "ABB", "ABCD", "CDE", "BCD", "0RT", "ZAB",
    "ACB", "AAA", "ABA", "ADC", "DCA", "XYZ", "XYZD",
];

fn benchmark_trie(c: &mut Criterion) {
    c.bench_function("benchmark_trie", |b| {
        let feeds: Vec<FeedId> = TEST_FEED_IDS
            .iter()
            .map(|f| f.as_bytes().to_vec().into())
            .collect();
        b.iter(|| {
            let mut trie = Trie::default();
            for feed in feeds.iter() {
                let _ = trie.store(feed);
            }
        });
    });
}

fn benchmark_hashset(c: &mut Criterion) {
    c.bench_function("benchmark_hashset", |b| {
        let feeds: Vec<FeedId> = TEST_FEED_IDS
            .iter()
            .map(|f| f.as_bytes().to_vec().into())
            .collect();
        b.iter(|| {
            let mut hs = HashSet::new();
            for feed in feeds.iter() {
                hs.insert(feed);
            }
        });
    });
}

criterion_group!(benches, benchmark_trie, benchmark_hashset);
criterion_main!(benches);
