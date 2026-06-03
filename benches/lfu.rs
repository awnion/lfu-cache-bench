use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use workato::{btree_lfu, heap_lfu, vec_lfu};

fn run_btree_workload(capacity: usize, operations: u64) {
    let mut cache = btree_lfu::LFU::new(capacity);

    for key in 0..capacity as u64 {
        black_box(cache.insert(key, key));
    }

    let key_space = capacity as u64 * 2;
    for i in 0..operations {
        let key = i % key_space;

        if i % 4 == 0 {
            black_box(cache.insert(key, i));
        } else {
            black_box(cache.get(&key));
        }
    }
}

fn run_vec_workload(capacity: usize, operations: u64) {
    let mut cache = vec_lfu::LFU::new(capacity);

    for key in 0..capacity as u64 {
        black_box(cache.insert(key, key));
    }

    let key_space = capacity as u64 * 2;
    for i in 0..operations {
        let key = i % key_space;

        if i % 4 == 0 {
            black_box(cache.insert(key, i));
        } else {
            black_box(cache.get(&key));
        }
    }
}

fn run_heap_workload(capacity: usize, operations: u64) {
    let mut cache = heap_lfu::LFU::new(capacity);

    for key in 0..capacity as u64 {
        black_box(cache.insert(key, key));
    }

    let key_space = capacity as u64 * 2;
    for i in 0..operations {
        let key = i % key_space;

        if i % 4 == 0 {
            black_box(cache.insert(key, i));
        } else {
            black_box(cache.get(&key));
        }
    }
}

fn bench_lfu(c: &mut Criterion) {
    let mut group = c.benchmark_group("lfu");

    group.bench_function("btree", |b| {
        b.iter(|| run_btree_workload(black_box(1024 << 4), black_box(1_000)))
    });

    group.bench_function("vec", |b| {
        b.iter(|| run_vec_workload(black_box(1024 << 4), black_box(1_000)))
    });

    group.bench_function("heap", |b| {
        b.iter(|| run_heap_workload(black_box(1024 << 4), black_box(1_000)))
    });

    group.finish();
}

criterion_group!(benches, bench_lfu);
criterion_main!(benches);
