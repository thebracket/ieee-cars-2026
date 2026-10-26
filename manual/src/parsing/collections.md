# Collections and Bounds

> The code for this is in `code/ex18_collections`.

Rust's standard library ships the containers you'd expect from C++ — with the
safety defaults flipped.

## Arrays: Stack, Fixed Size, Checked

```rust
let array = [1, 2, 3, 4];            // type inferred
let array: [u32; 4] = [1, 2, 3, 4];  // length is part of the type
```

Unlike C, the length is part of the type and every access is bounds-checked.
This C reads whatever happens to be after the array:

```c
int my_array[4] = {1, 2, 3, 4};
printf("%d", my_array[5]); // "32767" on my machine, on a good day
```

The Rust equivalent panics — loudly, immediately, in the right place:

```
thread 'main' panicked at src/main.rs:4:24:
index out of bounds: the len is 4 but the index is 4
```

And when you'd rather *handle* the out-of-bounds case than crash — the errors
section again — `.get()` returns an `Option`:

```rust
match array.get(5) {
    Some(value) => println!("array[5] = {value}"),
    None => println!("No element 5 -- and no reading past the end"),
}
```

> `get_unchecked` exists for when you can prove the index is valid, but it
> requires `unsafe`. The default is *check*, and you must opt out deliberately.
> We'll talk more about `unsafe` this afternoon.

## Vectors: Heap, Growable

`Vec<T>` is C++'s `std::vector`: a heap buffer plus length and capacity, doubling
when it fills. `vec![1, 2, 3]` is the literal macro; `Vec::with_capacity(100)`
pre-allocates when you know the size.

```rust
let mut my_vec = Vec::new();
my_vec.push(1);
my_vec.push(2);
println!("vector: {my_vec:?}"); // vector: [1, 2]
```

Indexing is checked here too — `my_vec[3]` panics instead of printing whatever
`operator[]` found in C++.

## Slices: Borrowed Views

A slice is a borrowed window into a sequence — `&vec[1..3]`, or a `&str` into a
`String`. Slices carry a pointer *and* a length, so iterating them can't run off
the end. Functions that take `&[T]` accept arrays, vectors, and slices alike.

## HashMap and Friends

`HashMap<K, V>` is `std::unordered_map`. Lookup returns an `Option` — no "is this
key present?" two-step that can race:

```rust
let mut my_map = HashMap::new();
my_map.insert("Hello".to_string(), 5);
my_map.insert("World".to_string(), 6);

if let Some(count) = my_map.get("Hello") {
    println!("Hello appears {count} times");
}
```

The standard library also has `VecDeque` (a double-ended queue), `BTreeMap` /
`BTreeSet` (ordered), `HashSet`, `LinkedList` (yes, really — writing one in Rust is
notoriously harder than using the provided one), and `BinaryHeap`.

Run it:

```
No element 5 -- and no reading past the end
vector: [1, 2]
Hello appears 5 times
```

> Every "get" in this language hands you an `Option` — array bounds, map keys,
> even the first command-line argument. Once you internalize that, the shape of
> safe systems code starts to click.
