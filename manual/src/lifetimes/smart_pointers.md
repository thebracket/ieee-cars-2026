# Smart Pointers: Box, Rc, Arc

The C++ translations:

| Rust | C++ equivalent | Ownership |
|------|----------------|-----------|
| `Box<T>` | `std::unique_ptr<T>` | One owner |
| `Rc<T>` | `std::shared_ptr<T>` | Shared, single-threaded |
| `Arc<T>` | `std::shared_ptr<T>` (atomic) | Shared, thread-safe |

## Box: The Unique Pointer

```rust
struct MyStruct { n: i32 }

fn main() {
    let boxed = Box::new(MyStruct { n: 12 });
    println!("{}", boxed.n);
}
```

> The code for this is in `code/ex14_box`.

A `Box` puts its contents on the *heap* and owns them. You reach for it when:

* **Recursive types** need indirection (a tree node holding a `Box<Node>`) — an
  inline `Node` would be infinitely sized.
* **Large values** shouldn't be copied around the stack.
* You need a **trait object**: `Box<dyn Error>` (that's how the generic error type
  in `ex03_errors` worked, before `thiserror`).

## Rc: Shared Ownership

```rust
use std::rc::Rc;

let shared = Rc::new(MyStruct {});
move_it(shared.clone());  // clone -> count 2 for the call
ref_it(&shared);          // move_it returned, its Rc dropped -> count 1
// shared goes out of scope -> "Dropping", exactly once
```

> The code for this is in `code/ex15_rc_arc`.

`clone()` on an `Rc` doesn't copy the data — it bumps a reference counter and
hands you another pointer to the same value. `move_it` takes that clone by
value, so the count is 2 only while the function runs; its parameter is dropped
on return, leaving the count back at 1 before `ref_it` borrows `shared`. When
the **last** `Rc` dies, the value is dropped. The data is shared between as many
owners as you like; the counter is not thread-safe, so `Rc` stays on one thread.

## Arc: Shared Across Threads

`Arc` ("atomic reference count") is `Rc` with an atomic counter — safe to send
between threads, slightly more expensive per clone.

The pattern worth remembering is **`Arc<Mutex<T>>`**: the `Arc` shares
ownership of the lock across threads, and the `Mutex` makes every access safe.
`Arc` on its own gives you shared read access; you need the `Mutex` when the
shared data has to be mutated.
