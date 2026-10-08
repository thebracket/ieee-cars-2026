# It's Just Enums

Here's the slide I promised: **`Option` and `Result` are not special.** They're
ordinary enums from the standard library. You could have written them yourself:

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

That's really it. The whole safety mechanism boils down to two tiny data types and
a compiler that enforces their use.

## Sum Types

Regular C enums can only tag a value. Rust enums can *contain* data — one variant's
worth, and only one at a time. This is a *tagged union*, also known as a *sum type*:

```rust
enum OptionInt {
    Some(i32),
    None,
}
```

C can emulate this — badly:

```c
enum tag { SOME, NONE };

struct option_int {
    enum tag tag;
    union { int some; } data;
};

int get(struct option_int o) {
    return o.data.some; // Nothing stops you when tag == NONE!
}
```

In C, the tag is advisory. You can read `data` even when there's no data — garbage
in, garbage out. In Rust, the **only** way to reach the `i32` is to `match` (or use
a helper that matches internally), and the compiler will not let a match finish
without covering every variant:

```rust
fn get(o: OptionInt) -> i32 {
    match o {
        OptionInt::Some(n) => n,
        OptionInt::None => 0, // The compiler made you decide.
    }
}
```

> Add a new variant to that enum next year, and *every* `match` on it stops
> compiling until you've decided what to do with the new case. That sounds
> annoying until the day it saves you — that's the compiler finding the bug
> before your users do.

## Make the Impossible Unrepresentable

That's the deeper idea, and it's why Rust sum types are awesome: if "a value that
might not be there" is a *type*, then "forgot to check for null" is a *compile
error*. If "a call that might fail" is a type, then "ignored the error" is at best
a very loud warning. `Option` and `Result` are just the two sum types you'll use
every day — the same trick scales up to your own domain types (a `LoggedOutUser`
that structurally cannot call `post_comment()`, and so on).

Let's use them.
