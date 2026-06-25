# LFU Cache Implementations in Rust

This repository compares several least-frequently-used cache implementations:

- `btree_lfu`: `HashMap` storage with a `BTreeSet` frequency index.
- `vec_lfu`: `HashMap` index with a frequency-sorted `Vec`.
- `heap_lfu`: `HashMap` storage with a custom indexed tuple min-heap.
- `linked_lfu`: `HashMap` index with an indexed doubly linked list.

## Benchmark

```bash
cargo bench
```

![LFU cache benchmark report](/assets/lfu-benchmark.svg)

Mean time per Criterion iteration:

| Capacity |     BTree |       Vec |      Heap |    Linked |
| -------: | --------: | --------: | --------: | --------: |
|        1 |   5.97 µs |   4.33 µs |   3.79 µs |   3.46 µs |
|        2 |   6.28 µs |   4.48 µs |   3.91 µs |   3.49 µs |
|        4 |   7.43 µs |   5.77 µs |  17.81 µs |   5.77 µs |
|        8 |   8.07 µs |   7.78 µs |  17.64 µs |   5.75 µs |
|       16 |  11.87 µs |   9.75 µs |  20.64 µs |   8.04 µs |
|       32 |  12.13 µs |  14.42 µs |  22.11 µs |  12.95 µs |
|       64 |  14.27 µs |  26.48 µs |  22.73 µs |  27.81 µs |
|      128 |  18.70 µs |  64.10 µs |  22.10 µs |  63.95 µs |
|      256 |  32.11 µs | 226.76 µs |  28.99 µs | 231.15 µs |
|      512 |  50.09 µs | 560.88 µs |  43.63 µs | 621.48 µs |
|    1,024 |  92.47 µs |   1.22 ms |  64.88 µs |   1.91 ms |
|    2,048 | 172.15 µs |   2.49 ms |  98.92 µs |   6.48 ms |
|    4,096 | 341.22 µs |   5.21 ms | 164.05 µs |  23.35 ms |
|    8,192 | 683.61 µs |  10.60 ms | 287.12 µs |  92.47 ms |
|   16,384 |   1.39 ms |  21.19 ms | 530.94 µs | 363.02 ms |
|   32,768 |   2.89 ms |  43.93 ms |   1.03 ms |    1.42 s |
|   65,536 |   5.94 ms |  94.23 ms |   2.11 ms |    4.69 s |

Run the benchmark:

```bash
cargo bench --bench lfu
```

Regenerate the SVG and CSV report from the latest Criterion JSON output:

```bash
./scripts/render_benchmark_chart.py
```
