use std::collections::{LinkedList, VecDeque};

use criterion::{criterion_group, criterion_main, Criterion};

fn vector_insertion_bencmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_insertion_benchmarks");
    let mut v = Vec::new();
    group.bench_function("vector_insertion", |b| {
        b.iter(|| {
            for i in 0..1000 {
                v.push(i);
            }
        })
    });
    group.finish();
}

fn vector_lookup_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_lookup_benchmarks");
    let mut v = Vec::new();
    group.bench_function("vector_lookup", |b| {
        b.iter(|| {
            for i in 0..1000 {
                v.push(i);
            }
            for i in 0..1000 {
                let _v = v[i];
            }
        })
    });
    group.finish();
}

fn vecdeque_insertion_bencmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("vecdeque_insertion_benchmarks");
    let mut v = VecDeque::new();
    group.bench_function("vecdeque_insertion", |b| {
        b.iter(|| {
            for i in 0..1000 {
                v.push_back(i);
            }
        })
    });
    group.finish();
}

fn vecdeque_lookup_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("vecdeque_lookup_benchmarks");
    let mut v = VecDeque::new();
    group.bench_function("vecdeque_lookup", |b| {
        b.iter(|| {
            for i in 0..1000 {
                v.push_back(i);
            }
            for i in 0..1000 {
                let _ = v[i];
            }
        })
    });
    group.finish();
}

fn linked_list_insertion_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("linked_list_insertion_benchmarks");
    let mut v = LinkedList::new();
    group.bench_function("linked_list_insertion", |b| {
        b.iter(|| {
            for i in 0..1000 {
                v.push_back(i);
            }
        })
    });
    group.finish();
}

fn linked_list_lookup_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("linked_list_lookup_benchmarks");
    let mut v = LinkedList::new();
    group.bench_function("linked_list_lookup", |b| {
        b.iter(|| {
            for i in 0..1000 {
                v.push_back(i);
            }
            for i in 0..1000 {
                v.iter().any(|&x| x == i);
            }
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    vector_insertion_bencmarks,
    vector_lookup_benchmarks,
    vecdeque_insertion_bencmarks,
    vecdeque_lookup_benchmarks,
    linked_list_insertion_benchmarks,
    linked_list_lookup_benchmarks
);
criterion_main!(benches);
