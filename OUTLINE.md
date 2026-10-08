IEEE-CARS 2026 — Grand Forks, Oct 27
"Foundational Rust for Secure Systems & Accelerating Python with Rust"
Herbert Wolverson — Ardan Labs / LibreQoS — Ballroom

ACTUAL schedule (verified live on ieee-cars.org/program/schedule, Tue Oct 6 2026)

10:00 – 10:15 AM
• Block: Break
• Content: (Ballroom)

**10:15 AM – 12:15 PM**
• Block: Rust Workshop Session 1
• Content: Foundational Rust: Core Rust, Ownership, Borrowing, Safe Error Handling, Cargo & Dependencies

**12:15 – 1:00 PM**
• Block: Lunch (45 min)
• Content: Lunch, Networking and Interaction with Dr. Andy Williams (AFRL); moderated by Dr. Mark Askelson & Dr. Prakash Ranganathan — Ballroom

**1:00 – 3:00 PM**
• Block: Rust Workshop Session 2
• Content: Foundational Rust: Safe Parsing, Data Handling, Fearless Concurrency, Lifetimes & Smart Pointers

3:00 – 3:15 PM
• Block: Break
• Content: 

**3:15 – 5:15 PM**
• Block: Rust Workshop Session 3
• Content: Accelerating Python with Rust: Bridging Python & Rust with PyO3, Maturin, Project Structure & Packaging

Total teaching time: 6 hours (2h + 2h + 2h), 45-min lunch with AFRL networking.

Original outline (from the docx you sent Sep 30 — structure preserved in PLAN.md)

Part A — "IEEE-Cars Rust Outline" (4h Foundational → now S1+S2):
1. Welcome + environment check (rustup verify, PASS/FAIL script, TAs circulate)
2. Hello World + Cargo project anatomy (cargo new/build/run/test)
3. Ownership & borrowing — move/borrow/mutable-borrow demo; why this eliminates bug classes in secure systems
4. Safe error handling — Option/Result, ?, thiserror-style enums, safe arithmetic/overflow
5. Cargo & dependencies — ends Part A on a natural boundary ("you now have a Cargo project with deps")
6. Safe parsing & data handling — Serde/JSON event-log lab (core); Clap CLI lab (compressible)
7. Fearless concurrency — C++ vs Rust data-race comparison, threads, Mutex/Arc, Rayon
8. Lifetimes & smart pointers — RAII, Box/Rc/Arc, read-only lifetime depth

Part B — "IEEE-Cars Rust Python Workshop" (4h PyO3 → compressed to S3's 2h):
1. PyO3 intro — #[pyfunction]/#[pymodule], PyResult, safe errors across FFI
2. The wordstats demo (ieee_cars_pyo3_demo/) — word_count, word_frequencies (Rayon-parallel), top_words; GIL handling; cdylib
3. Maturin — project structure, packaging, wheel building
4. Labs 2–3 from the original (cut in the 2h compression)

Differences from the earlier assumed schedule: S1 starts 10:15 (not 10:30), lunch is 45 min (not 30), S3 is 2 hours (not ~1) — the demo-led arc gets its full length back.
