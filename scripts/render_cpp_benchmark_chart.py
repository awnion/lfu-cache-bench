#!/bin/sh
''''command -v uv >/dev/null 2>&1 && exec uv run --script "$0" "$@" || exec python3 "$0" "$@" # '''
# /// script
# requires-python = ">=3.10"
# dependencies = [
#   "matplotlib>=3.10",
# ]
# ///

import csv
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter


OUTPUT_DIR = Path("assets")
CSV_INPUT = OUTPUT_DIR / "cpp-lfu-benchmark.csv"
SVG_OUTPUT = OUTPUT_DIR / "cpp-lfu-benchmark.svg"

SERIES = {
    "btree": {"label": "Tree index", "color": "#2563eb"},
    "vec": {"label": "Sorted vector", "color": "#dc2626"},
    "heap": {"label": "Indexed heap", "color": "#16a34a"},
    "linked": {"label": "Bucketed linked list", "color": "#9333ea"},
}


def load_results() -> tuple[list[int], dict[str, dict[int, float]]]:
    results: dict[str, dict[int, float]] = {implementation: {} for implementation in SERIES}
    with CSV_INPUT.open(newline="") as file:
        for row in csv.DictReader(file):
            implementation = row["implementation"]
            if implementation in results:
                results[implementation][int(row["capacity"])] = float(row["mean_us"])

    capacities = sorted({capacity for rows in results.values() for capacity in rows})
    return capacities, results


def time_tick(value: float, _position: int) -> str:
    if value >= 1_000_000:
        return f"{value / 1_000_000:g} s"
    if value >= 1_000:
        return f"{value / 1_000:g} ms"
    return f"{value:g} µs"


def main() -> None:
    capacities, results = load_results()

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
    fig.suptitle("C++ LFU Cache Benchmark", x=0.08, y=0.96, ha="left", fontsize=24, fontweight="bold")
    fig.text(
        0.08,
        0.86,
        "Mean of 7 samples from 250 mixed operations per iteration.",
        color="#667085",
        fontsize=12,
    )
    fig.text(
        0.08,
        0.815,
        "Lower is better. Same power-of-two capacities as the Rust benchmark.",
        color="#667085",
        fontsize=11,
    )

    for implementation, metadata in SERIES.items():
        mean = [results[implementation][capacity] for capacity in capacities]
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

    ax.set_xscale("log")
    ax.set_yscale("log")
    shown_ticks = [1, 4, 16, 64, 256, 1_024, 4_096, 16_384, 32_768]
    ax.set_xticks(shown_ticks)
    ax.set_xticklabels([f"{capacity:,}" for capacity in shown_ticks])
    ax.yaxis.set_major_formatter(FuncFormatter(time_tick))
    ax.margins(x=0.04)
    max_mean = max(value for rows in results.values() for value in rows.values())
    ax.set_ylim(0.1, max_mean * 1.4)
    ax.grid(True, which="major")
    ax.grid(True, which="minor", alpha=0.25)
    ax.set_xlabel("Cache capacity")
    ax.set_ylabel("Mean time per iteration, log scale")
    ax.legend(loc="upper left", frameon=True, framealpha=1, edgecolor="#eaecf0")

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    fig.savefig(SVG_OUTPUT, format="svg")


if __name__ == "__main__":
    main()
