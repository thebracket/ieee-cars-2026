# Iterators: Owning and Borrowing

> The code for this is in `code/ex19_iterators`.

`for` loops in Rust *are* iterators — and which iterator you get decides whether
you're borrowing your collection or consuming it.

## Three Ways to Iterate

```rust
let my_vec = vec![1, 2, 3, 4];

for n in &my_vec { ... }         // .iter()        -- borrows: you keep my_vec
for n in &mut my_vec { ... }     // .iter_mut()    -- mutable borrows
for n in my_vec { ... }          // .into_iter()   -- CONSUMES: my_vec is gone
```

`for n in my_vec` is syntactic sugar for `for n in my_vec.into_iter()`, and it
*moves* the vector into the loop. Try to use `my_vec` afterward and the compiler
stops you — that's the ownership section, applied to loops.

## Chaining

Iterators chain into pipelines, like C++20 ranges:

```rust
let my_new_vec: Vec<String> = my_vec
    .iter()
    .map(|n| n.to_string()) // convert each entry to another type
    .collect();             // note: the target type comes from the annotation

let sum: u32 = my_vec.iter().sum();
let max = my_vec.iter().max();
let sum_with_fold = my_vec.iter().fold(0, |acc, x| acc + x);

my_vec
    .iter()
    .filter(|n| **n > 2) // two layers of reference: deref twice
    .for_each(|n| println!("{n}"));
```

`collect()` reads beautifully but needs to know *what* to collect into — that's
what the `Vec<String>` annotation is for.

Run it:

```
[(1, "1"), (2, "2"), (3, "3"), (4, "4")]
3
4
max=Some(4) min=Some(1) sum=10 count=4 fold=10
```

> `max()` and `min()` return `Option` — because an empty iterator has no maximum.
> The compiler is consistent about this; every `get`-shaped operation is
> fallible or optional and *says so in its type*.

Iterators are *zero-cost abstractions*: the compiler flattens these chains into
the same machine code as a hand-written loop. And when we meet Rayon later this
session, `.iter()` becomes `.par_iter()` and the *same chain* runs across every
CPU core.
