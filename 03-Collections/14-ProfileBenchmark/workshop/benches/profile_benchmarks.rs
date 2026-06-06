use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use profile_benchmark::{count_btreemap, count_hashmap, count_vec, top_n};

const SAMPLE: &str = "the quick brown fox jumps over the lazy dog \
                      the quick brown fox jumps over the lazy dog \
                      the quick brown fox jumps over the lazy dog \
                      Sphinx of black quartz judge my vow \
                      Pack my box with five dozen liquor jugs \
                      How vexingly quick daft zebras jump ";

fn make_text(words: usize) -> String {
    let base_words: Vec<&str> = SAMPLE.split_whitespace().collect();
    let mut out = String::with_capacity(words * 6);
    for i in 0..words {
        out.push_str(base_words[i % base_words.len()]);
        out.push(' ');
    }
    out
}

fn bench_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_count");
    for size in [1_000usize, 10_000].iter() {
        let text = make_text(*size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &text, |b, t| {
            b.iter(|| count_vec(t));
        });
    }
    group.finish();
}

fn bench_hashmap(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_count");
    for size in [1_000usize, 10_000].iter() {
        let text = make_text(*size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &text, |b, t| {
            b.iter(|| count_hashmap(t));
        });
    }
    group.finish();
}

fn bench_btreemap(c: &mut Criterion) {
    let mut group = c.benchmark_group("btreemap_count");
    for size in [1_000usize, 10_000].iter() {
        let text = make_text(*size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &text, |b, t| {
            b.iter(|| count_btreemap(t));
        });
    }
    group.finish();
}

fn bench_top_n(c: &mut Criterion) {
    let text = make_text(10_000);
    let counts = count_hashmap(&text);
    c.bench_function("top_n/10", |b| {
        b.iter(|| top_n(&counts, 10));
    });
}

criterion_group!(
    benches,
    bench_vec,
    bench_hashmap,
    bench_btreemap,
    bench_top_n
);
criterion_main!(benches);
