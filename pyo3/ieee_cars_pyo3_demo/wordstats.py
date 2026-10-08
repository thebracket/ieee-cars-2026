#!/usr/bin/env python3
"""Benchmark: pure Python vs Rust (via PyO3) word stats.

Usage:
    python wordstats.py [--mb 2]

Requires the Rust module to be built: `maturin develop` (or `--release`).
"""

import argparse
import random
import time
from collections import Counter
from concurrent.futures import ThreadPoolExecutor

import ieee_cars_pyo3_demo as demo

WORDS = (
    "the quick brown fox jumps over lazy dog security systems rust python "
    "network packet analysis alert log monitor trace erlang beam tcp ip"
).split()


def generate_corpus(mb: int) -> str:
    """Deterministic pseudo-English text, ~mb megabytes."""
    random.seed(2026)
    target = mb * 1024 * 1024
    parts = []
    size = 0
    while size < target:
        line = " ".join(random.choices(WORDS, k=12))
        parts.append(line + "\n")
        size += len(line) + 1
    return "".join(parts)


def py_word_count(text: str) -> int:
    return len(text.split())


def py_word_frequencies(text: str) -> dict:
    return dict(Counter(word.lower() for word in text.split()))


def timed(label, fn):
    start = time.perf_counter()
    result = fn()
    elapsed = time.perf_counter() - start
    print(f"  {label:<30} {elapsed:8.3f}s")
    return result


def run_threaded(fn, text: str, workers: int = 4) -> float:
    start = time.perf_counter()
    with ThreadPoolExecutor(max_workers=workers) as pool:
        list(pool.map(fn, [text] * workers))
    return time.perf_counter() - start


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mb", type=int, default=2, help="corpus size in MB")
    args = parser.parse_args()

    text = generate_corpus(args.mb)
    words = demo.word_count(text)
    print(f"Corpus: ~{args.mb} MB, {words:,} words\n")

    # Correctness before performance: both implementations must agree.
    assert words == py_word_count(text)
    assert py_word_frequencies(text) == demo.word_frequencies(text)
    print("Correctness: Python and Rust agree\n")

    print("Word frequencies:")
    timed("Python", lambda: py_word_frequencies(text))
    timed("Rust", lambda: demo.word_frequencies(text))

    print("\nTop 10 words from Rust:")
    for word, count in demo.top_words(text, 10):
        print(f"  {word:<10} {count:,}")

    print("\n4 Python threads, each running the analysis:")
    timed("GIL held (default)", lambda: run_threaded(demo.word_frequencies, text))
    timed(
        "GIL released (detached)",
        lambda: run_threaded(demo.word_frequencies_detached, text),
    )


if __name__ == "__main__":
    main()
