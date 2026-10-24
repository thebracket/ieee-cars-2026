# The Word Stats Demo

Time for the real thing. The task: given a big block of text, produce word
statistics — how many words, how often each appears, and which are most common.

The classic Python approach is pleasantly short:

```python
from collections import Counter

def py_word_frequencies(text):
    return dict(Counter(word.lower() for word in text.split()))
```

> Fair warning before any benchmarking: Python's `Counter` is *C code* under the
> hood. It is genuinely fast. This demo is about seeing the Rust pipeline in
> action, and measuring a useful speedup.

Everything here lives in `pyo3/ieee_cars_pyo3_demo/src/lib.rs`, and the
benchmark that produced the numbers below is `wordstats.py`.

> **Scope of the comparison:** The Python and Rust versions agree on the supplied
> benchmark corpus, not necessarily every possible input. Python's `str.split()`
> and Rust's `split_whitespace()` differ on some control characters: for example,
> `"a\u001cb"` splits into two words in Python but one in Rust. For production use,
> define the intended word-splitting rules and test both implementations against them.

## Step 1: `word_count`

```rust
    /// Count the words in a document.
    #[pyfunction]
    fn word_count(text: &str) -> usize {
        text.split_whitespace().count()
    }
```

`&str` in the signature means Python hands us a string *without copying it* —
PyO3 knows how to borrow a Python `str` as Rust's `&str` (UTF-8, of course).

## Step 2: `word_frequencies` — the Sequential Shape

Frequency counting is the `entry().or_insert(0) += 1` idiom — the same one from
this morning's event-log lab and the capstone. If we wrote it as a plain loop,
it would look like this (we won't keep this exact function — but it's the shape
Rayon is about to parallelize):

```rust
    fn count_sequentially(text: &str) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for word in text.split_whitespace() {
            *counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }
        counts
    }
```

## Step 3: `word_frequencies` — Actually Parallel

But we have Rayon in `Cargo.toml`, and twenty cores in the machine. The
parallel version is a *map-reduce*: give each worker its own map, then merge the
maps pairwise.

```rust
    /// Frequency of each word (case-insensitive), counted in parallel with Rayon.
    #[pyfunction]
    fn word_frequencies(text: &str) -> HashMap<String, usize> {
        use rayon::prelude::*;

        text.split_whitespace()
            .collect::<Vec<_>>()
            .par_iter()
            .fold(HashMap::new, |mut counts, word| {
                *counts.entry(word.to_lowercase()).or_insert(0) += 1;
                counts
            })
            .reduce(HashMap::new, |mut a, b| {
                for (word, count) in b {
                    *a.entry(word).or_insert(0) += count;
                }
                a
            })
    }
```

Read it as prose: *split into words → `par_iter()` → each Rayon worker folds a
local `HashMap` → reduce merges the maps.* No locks, no shared mutable state, no
data races — each worker owns its accumulator, and ownership makes that a
compile-time fact.

## Step 4: `top_words`

```rust
    /// The most common words, as (word, count) pairs sorted descending.
    #[pyfunction]
    fn top_words(text: &str, n: usize) -> Vec<(String, usize)> {
        let mut counts: Vec<(String, usize)> = word_frequencies(text).into_iter().collect();
        // Biggest count first; alphabetical tie-break keeps the output stable.
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        counts.truncate(n);
        counts
    }
```

`Vec<(String, usize)>` crosses the boundary as a Python list of tuples. The
tie-break isn't decoration: `HashMap` iteration order is effectively random, so
without it two runs could disagree — annoying in a demo, unacceptable in a
report.

## The Benchmark

```bash
cd pyo3/ieee_cars_pyo3_demo
source .venv/bin/activate
python wordstats.py --mb 16
```

Real numbers from the workshop machine (20 cores, CPython 3.14):

| Corpus | Python | Rust | Note |
|--------|--------|------|------|
| 1 MB | 0.016s | 0.036s | **debug build — Rust LOST!** |
| 2 MB | 0.033s | 0.007s | release, ~4.7× faster |
| 16 MB | 0.322s | 0.078s | release, ~4.1× faster |

**The first row is the important one.** `maturin develop` builds a *debug*
build — no optimization — and debug Rust can absolutely lose to C-accelerated
Python. When you care about speed:

```bash
maturin develop --release
```

## A Working Extension, a Measured Speedup

Python called our Rust extension, Rayon spread the work across cores, and the
release build delivered a meaningful speedup on this corpus. That's the core
outcome: keep Python for orchestration and move the measured hot path into Rust.

There is always more to explore. The [optional afterword](afterword.md) covers
the GIL, `detach`, and calls from multiple Python threads — further reading,
not another requirement for the live session. The supplied benchmark includes
that threaded comparison too; we can leave its explanation for the afterword.

## Next

One thing remains between this demo and something your colleagues can
`pip install`: **[packaging it as a wheel](packaging.md)**.
