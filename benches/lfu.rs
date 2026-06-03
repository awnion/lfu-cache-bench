use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use lfu_cache_rs::{btree_lfu, heap_lfu, vec_lfu};
use std::hint::black_box;

const CAPACITIES: [usize; 12] = [1, 4, 10, 16, 30, 60, 100, 250, 1_000, 5_000, 10_000, 20_000];
const OPERATIONS: u64 = 1_000;

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

    for capacity in CAPACITIES {
        group.bench_with_input(
            BenchmarkId::new("btree", capacity),
            &capacity,
            |b, &capacity| {
                b.iter(|| run_btree_workload(black_box(capacity), black_box(OPERATIONS)))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("vec", capacity),
            &capacity,
            |b, &capacity| b.iter(|| run_vec_workload(black_box(capacity), black_box(OPERATIONS))),
        );

        group.bench_with_input(
            BenchmarkId::new("heap", capacity),
            &capacity,
            |b, &capacity| b.iter(|| run_heap_workload(black_box(capacity), black_box(OPERATIONS))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_lfu);
criterion_main!(benches);
