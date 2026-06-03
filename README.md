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
|        1 |  23.76 us |  17.53 us |  11.43 us |
|        4 |  28.82 us |  22.73 us |  74.44 us |
|       10 |  32.49 us |  33.26 us |  71.02 us |
|       16 |  43.78 us |  34.78 us |  95.41 us |
|       30 |  43.47 us |  48.15 us |  88.26 us |
|       60 |  50.70 us |  72.70 us | 100.46 us |
|      100 |  63.42 us | 121.51 us | 106.96 us |
|      250 |  76.68 us | 320.88 us | 105.76 us |
|    1,000 | 190.63 us |   3.19 ms | 126.21 us |
|    5,000 | 551.41 us |  23.60 ms | 309.53 us |
|   10,000 | 999.52 us |  48.96 ms | 485.24 us |
|   20,000 |   1.92 ms |  99.50 ms | 817.25 us |

Run the benchmark:

```bash
cargo bench --bench lfu
```

Regenerate the SVG and CSV report from the latest Criterion JSON output:

```bash
./scripts/render_benchmark_chart.py
```
