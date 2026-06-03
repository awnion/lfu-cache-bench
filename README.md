# LFU Cache Implementations in Rust

This repository compares several least-frequently-used cache implementations:

- `btree_lfu`: `HashMap` storage with a `BTreeSet` frequency index.
- `vec_lfu`: `HashMap` index with a frequency-sorted `Vec`.
- `heap_lfu`: `HashMap` storage with a custom indexed tuple min-heap.

## Benchmark

```bash
cargo bench
```

![LFU cache benchmark report](/assets/lfu-benchmark.svg)

Mean time per Criterion iteration:

| Capacity |     BTree |       Vec |      Heap |
| -------: | --------: | --------: | --------: |
|        1 |  23.76 µs |  17.53 µs |  11.43 µs |
|        4 |  28.82 µs |  22.73 µs |  74.44 µs |
|       10 |  32.49 µs |  33.26 µs |  71.02 µs |
|       16 |  43.78 µs |  34.78 µs |  95.41 µs |
|       30 |  43.47 µs |  48.15 µs |  88.26 µs |
|       60 |  50.70 µs |  72.70 µs | 100.46 µs |
|      100 |  63.42 µs | 121.51 µs | 106.96 µs |
|      250 |  76.68 µs | 320.88 µs | 105.76 µs |
|    1,000 | 190.63 µs |   3.19 ms | 126.21 µs |
|    5,000 | 551.41 µs |  23.60 ms | 309.53 µs |
|   10,000 | 999.52 µs |  48.96 ms | 485.24 µs |
|   20,000 |   1.92 ms |  99.50 ms | 817.25 µs |

Run the benchmark:

```bash
cargo bench --bench lfu
```

Regenerate the SVG and CSV report from the latest Criterion JSON output:

```bash
./scripts/render_benchmark_chart.py
```
