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

| Capacity |     BTree |       Vec |      Heap |     Linked |
| -------: | --------: | --------: | --------: | ---------: |
|        1 |   6.12 µs |   4.75 µs |   2.81 µs |    3.07 µs |
|        2 |   6.45 µs |   4.73 µs |   3.18 µs |    3.24 µs |
|        4 |   7.68 µs |   5.91 µs |  17.12 µs |    5.32 µs |
|        8 |   8.27 µs |   8.36 µs |  18.79 µs |    5.86 µs |
|       16 |  12.44 µs |  10.06 µs |  20.44 µs |    7.13 µs |
|       32 |  12.59 µs |  14.22 µs |  22.56 µs |    9.83 µs |
|       64 |  15.07 µs |  26.32 µs |  24.05 µs |   17.15 µs |
|      128 |  19.92 µs |  62.24 µs |  23.66 µs |   33.04 µs |
|      256 |  32.94 µs | 220.65 µs |  30.70 µs |  105.88 µs |
|      512 |  53.00 µs | 550.13 µs |  44.62 µs |  279.21 µs |
|    1,024 |  95.00 µs |   1.20 ms |  66.07 µs |  813.25 µs |
|    2,048 | 179.73 µs |   2.48 ms | 101.17 µs |    3.79 ms |
|    4,096 | 359.83 µs |   5.00 ms | 166.15 µs |   19.74 ms |
|    8,192 | 698.90 µs |   9.67 ms | 288.57 µs |   68.26 ms |
|   16,384 |   1.42 ms |  19.18 ms | 542.05 µs |  228.44 ms |
|   32,768 |   3.02 ms |  40.10 ms |   1.06 ms |  900.11 ms |

Run the benchmark:

```bash
cargo bench --bench lfu
```

Regenerate the SVG and CSV report from the latest Criterion JSON output:

```bash
./scripts/render_benchmark_chart.py
```
