#!/bin/sh
''''command -v uv >/dev/null 2>&1 && exec uv run --script "$0" "$@" || exec python3 "$0" "$@" # '''
# /// script
# requires-python = ">=3.10"
# dependencies = [
#   "matplotlib>=3.10",
# ]
# ///

import csv
import json
import subprocess
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter


BENCHMARK_GROUP = "lfu"
OUTPUT_DIR = Path("assets")
SVG_OUTPUT = OUTPUT_DIR / "lfu-benchmark.svg"
CSV_OUTPUT = OUTPUT_DIR / "lfu-benchmark.csv"
CAPACITIES = [
    1,
    2,
    4,
    8,
    16,
    32,
    64,
    128,
    256,
    512,
    1_024,
    2_048,
    4_096,
    8_192,
    16_384,
    32_768,
    65_536,
]

SERIES = {
    "btree": {"label": "BTree index", "color": "#2563eb"},
    "vec": {"label": "Sorted Vec", "color": "#dc2626"},
    "heap": {"label": "Indexed heap", "color": "#16a34a"},
    "linked": {"label": "Linked list", "color": "#9333ea"},
}


def cargo_target_dir() -> Path:
    metadata = subprocess.run(
        ["cargo", "metadata", "--format-version=1", "--no-deps"],
        check=True,
        capture_output=True,
        text=True,
    )
    return Path(json.loads(metadata.stdout)["target_directory"])


def read_mean_microseconds(estimates_path: Path) -> tuple[float, float, float]:
    estimates = json.loads(estimates_path.read_text())
    mean = estimates["mean"]
    interval = mean["confidence_interval"]

    return (
        interval["lower_bound"] / 1_000,
        mean["point_estimate"] / 1_000,
        interval["upper_bound"] / 1_000,
    )


def load_results() -> tuple[list[int], dict[str, dict[int, tuple[float, float, float]]]]:
    benchmark_dir = cargo_target_dir() / "criterion" / BENCHMARK_GROUP
    results: dict[str, dict[int, tuple[float, float, float]]] = {
        implementation: {} for implementation in SERIES
    }

    for benchmark_json in benchmark_dir.glob("*/*/new/benchmark.json"):
        benchmark = json.loads(benchmark_json.read_text())
        implementation = benchmark.get("function_id")
        value = benchmark.get("value_str")

        if implementation not in SERIES or value is None:
            continue

        capacity = int(value)
        estimates_path = benchmark_json.with_name("estimates.json")
        results[implementation][capacity] = read_mean_microseconds(estimates_path)

    capacities = CAPACITIES
    missing = [
        f"{implementation}/{capacity}"
        for implementation, rows in results.items()
        for capacity in capacities
        if capacity not in rows
    ]
    if missing:
        raise RuntimeError(f"Missing Criterion results: {', '.join(missing)}")

    if not capacities:
        raise RuntimeError(f"No Criterion results found in {benchmark_dir}")

    return capacities, results


def write_csv(capacities: list[int], results: dict[str, dict[int, tuple[float, float, float]]]) -> None:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    with CSV_OUTPUT.open("w", newline="") as file:
        writer = csv.writer(file)
        writer.writerow(["capacity", "implementation", "lower_µs", "mean_µs", "upper_µs"])
        for capacity in capacities:
            for implementation in SERIES:
                low, mean, high = results[implementation][capacity]
                writer.writerow([capacity, implementation, f"{low:.6f}", f"{mean:.6f}", f"{high:.6f}"])


def time_tick(value: float, _position: int) -> str:
    if value >= 1_000:
        return f"{value / 1_000:g} ms"
    return f"{value:g} µs"


def main() -> None:
    capacities, results = load_results()
    write_csv(capacities, results)

    plt.rcParams.update(
        {
            "font.family": "DejaVu Sans",
            "axes.spines.top": False,
            "axes.spines.right": False,
            "axes.edgecolor": "#d0d5dd",
            "axes.labelcolor": "#344054",
            "xtick.color": "#475467",
            "ytick.color": "#475467",
            "grid.color": "#eaecf0",
            "grid.linewidth": 1,
            "figure.facecolor": "white",
            "axes.facecolor": "white",
        }
    )

    fig, ax = plt.subplots(figsize=(14.5, 8.0))
    fig.subplots_adjust(left=0.08, right=0.98, bottom=0.16, top=0.72)
    fig.suptitle("LFU Cache Benchmark", x=0.08, y=0.96, ha="left", fontsize=24, fontweight="bold")
    fig.text(
        0.08,
        0.86,
        "Criterion mean estimates from 250 mixed operations per iteration.",
        color="#667085",
        fontsize=12,
    )
    fig.text(
        0.08,
        0.815,
        "Lower is better. Error bands show the measured Criterion confidence interval from the latest local run.",
        color="#667085",
        fontsize=11,
    )

    for implementation, metadata in SERIES.items():
        values = [results[implementation][capacity] for capacity in capacities]
        low = [value[0] for value in values]
        mean = [value[1] for value in values]
        high = [value[2] for value in values]

        ax.plot(
            capacities,
            mean,
            marker="o",
            markersize=7,
            markeredgecolor="white",
            markeredgewidth=1.5,
            linewidth=3,
            color=metadata["color"],
            label=metadata["label"],
        )
        ax.fill_between(capacities, low, high, color=metadata["color"], alpha=0.10)

    ax.set_xscale("log")
    ax.set_yscale("log")
    shown_ticks = [1, 4, 16, 64, 256, 1_024, 4_096, 16_384, 65_536]
    ax.set_xticks(shown_ticks)
    ax.set_xticklabels([f"{capacity:,}" for capacity in shown_ticks])
    ax.yaxis.set_major_formatter(FuncFormatter(time_tick))
    ax.margins(x=0.04)
    max_high = max(value[2] for rows in results.values() for value in rows.values())
    ax.set_ylim(2, max_high * 1.4)
    ax.grid(True, which="major")
    ax.grid(True, which="minor", alpha=0.25)
    ax.set_xlabel("Cache capacity")
    ax.set_ylabel("Mean time per iteration, log scale")
    ax.legend(loc="upper left", frameon=True, framealpha=1, edgecolor="#eaecf0")

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    fig.savefig(SVG_OUTPUT, format="svg")


if __name__ == "__main__":
    main()
