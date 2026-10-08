# Capstone: The Traffic Report

It's the last exercise of the day, and everything you need, you've already met:
JSON parsing with Serde, `HashMap`, iterators, and `Result` errors.

## The Brief

`traffic.json` is a log of network flows — one JSON object per connection:

```json
{"source_ip": "10.0.0.14", "destination_port": 443, "bytes": 8000}
```

Write a program that reads it and prints a **traffic report**:

1. The number of flows and the total bytes.
2. Bytes per source IP, sorted descending, and the loudest one:
   `Top talker: <ip> (<bytes> bytes)`.
3. The number of flows per destination port, sorted ascending by port.

Your output should look like this:

```
Flows: 12
Total bytes: 81850
Bytes by source IP:
  203.0.113.99: 50000
  10.0.0.14: 23000
  192.168.4.7: 8000
  10.0.0.31: 850
Top talker: 203.0.113.99 (50000 bytes)
Flows by destination port:
  port 22: 3
  port 80: 2
  port 443: 5
  port 8080: 2
```

## How to Work

**This isn't high school.** There's no exam and I'm not grading you. If copying
the walkthrough line-by-line is how you learn — do it. If you'd rather struggle
through it yourself first, also great. Do what's best for *your* learning style.

**Work together.** Grab the person next to you. Compare outputs. Argue about
whether `u64` was the right call. Pair up — it's the best way to learn what you
didn't know you didn't know.

**Ask questions.** That's what the TAs and I are here for. If you're stuck,
someone else in the room almost certainly is too.

## Getting Started

A starter is provided — the types (`Flow`, the error enum) and the file loading
are already done, with TODOs in `main`:

```bash
cargo run -p capstone_starter
```

And the full worked answer, if you want to peek (it doesn't count as cheating!):

```bash
cargo run -p capstone_solution
```

The walkthrough chapter explains every step of the solution. Try to make the
starter's output match before you read it — *then* read it and compare.

## Stretch Goals (If You Finish Early)

1. Also report the average bytes per flow, rounded down.
2. Take the input path from the command line — `std::env::args().nth(1)` is an
   `Option` (you've met this one).
3. Report bytes per *port* as well as flow counts.
4. Corrupt `traffic.json` on purpose and make sure the error message helps you
   find the problem.
5. Feeling brave: compute the per-IP totals in parallel with Rayon. The chains
   are the same ones you already wrote.
