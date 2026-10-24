//! IEEE-CARS 2026 — Accelerating Python with Rust.
//!
//! Build with `maturin develop`, then `import ieee_cars_pyo3_demo` from Python.

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod ieee_cars_pyo3_demo {
    use pyo3::prelude::*;
    use std::collections::HashMap;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    /// Say hello, from Rust.
    #[pyfunction]
    fn hello_rust() -> PyResult<()> {
        println!("Hello World from Rust");
        Ok(())
    }

    /// Divide two numbers, raising a Python exception on failure.
    #[pyfunction]
    fn divide(a: i32, b: i32) -> PyResult<i32> {
        use pyo3::exceptions::PyArithmeticError;
        if b == 0 {
            Err(PyArithmeticError::new_err(
                "Dividing by zero will ruin your day",
            ))
        } else {
            // `i32::MIN / -1` doesn't fit in an i32, and plain `/` would panic.
            // `checked_div` returns `None` instead, so we can raise normally.
            a.checked_div(b).ok_or_else(|| {
                PyArithmeticError::new_err("The result doesn't fit in a 32-bit integer")
            })
        }
    }

    // --- Word stats ---

    /// Count the words in a document.
    #[pyfunction]
    fn word_count(text: &str) -> usize {
        text.split_whitespace().count()
    }

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

    /// The same analysis, but with the GIL released while Rust works.
    ///
    /// Copying into an owned `String` is a deliberate choice, not a `detach`
    /// requirement: the detached closure works on data it owns outright.
    #[pyfunction]
    fn word_frequencies_detached(py: Python<'_>, text: &str) -> HashMap<String, usize> {
        let owned = text.to_string();
        py.detach(move || word_frequencies(&owned))
    }

    /// The most common words, as (word, count) pairs sorted descending.
    #[pyfunction]
    fn top_words(text: &str, n: usize) -> Vec<(String, usize)> {
        let mut counts: Vec<(String, usize)> = word_frequencies(text).into_iter().collect();
        // Biggest count first; alphabetical tie-break keeps the output stable.
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        counts.truncate(n);
        counts
    }
}
