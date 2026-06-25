use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use lfu_cache_rs::{btree_lfu, heap_lfu, linked_lfu, vec_lfu};
use std::hint::black_box;

const CAPACITIES: [usize; 16] = [
    1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1_024, 2_048, 4_096, 8_192, 16_384, 32_768,
];
const OPERATIONS: u64 = 250;

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

fn run_linked_workload(capacity: usize, operations: u64) {
    let mut cache = linked_lfu::LFU::new(capacity);

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
    group.sample_size(10);
    group.warm_up_time(std::time::Duration::from_millis(250));
    group.measurement_time(std::time::Duration::from_millis(500));

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

        group.bench_with_input(
            BenchmarkId::new("linked", capacity),
            &capacity,
            |b, &capacity| {
                b.iter(|| run_linked_workload(black_box(capacity), black_box(OPERATIONS)))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_lfu);
criterion_main!(benches);
