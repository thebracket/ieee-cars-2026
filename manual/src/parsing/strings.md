# Rust's Many String Types

I once proof-read a book proposal that started with `String` as the *first* type
taught. I marked it as a "brave choice", because Rust has... so many string types:

* `&str` — a borrowed slice of characters. String literals are `&str`.
* `String` — an owned, growable string. `String::from("Hello")`.
* `Cow<str>` — "clone on write": can be either a borrowed `&str` or an owned
  `String`, only copying if it must.
* `OsString` / `&OsStr` — operating-system strings, which may *not* be valid UTF-8.
* `CString` / `&CStr` — C-compatible strings, guaranteed null-terminated (FFI land).
* `Path` / `PathBuf` — file paths, which aren't really strings on every platform.
* ...and plenty more when you add crates.

There's an entire chapter of FFI courses in there. Pollution of string types it may
be, but there's a reason: strings are harder than we pretend.

## The Two You'll Actually Use

* `&str` — an immutable *view* of characters in memory. Like C's `char *`, except
  it's length-prefixed rather than null-terminated — the length is stored, so no
  walking memory looking for a `\0` (and no buffer overruns when one escapes).
* `String` — an owned, growable string. You can `push_str`, `+=`, and hand
  ownership around. Coerce one to `&str` by borrowing: `&my_string`.

## The Key Point: UTF-8, Not ASCII

Rust strings are **UTF-8**, not arrays of 8-bit ASCII. Everything that implies:

* `.len()` is **bytes**, not characters. `.chars().count()` counts characters.
* You **cannot** index a `String` with `text[0]` — byte positions can fall in the
  middle of a multi-byte character. Slicing uses ranges, and `.get(range)` returns
  an `Option`: `None` when the range would split a character. (Another `Option` —
  this language is consistent.)
* Iterate with `.chars()` for characters, `.bytes()` for raw UTF-8 bytes.
* A Rust `char` is 4 bytes — a full Unicode scalar value. Its UTF-8 *encoding* is
  1 to 4 bytes; ASCII's guarantee of "one byte, one character" is gone.

> The code for this is in `code/ex17_strings`. Run it:

```
Bytes: 12
Chars: 9
0..7: Some("Hello, ")
0..8: None
char #7: Some('🌎')
emoji bytes: [240, 159, 140, 142]
```

`"Hello, 🌎!"` is 9 characters but 12 bytes — and byte range `0..8` isn't a string
at all, because it bisects the emoji. In C this is the kind of thing that "works"
in the demo and corrupts something in production the day a user pastes an emoji
into a form. Rust refuses at the boundary, and hands you an `Option` instead.

> Coming from C: your `char` is one byte and your strings are byte arrays. Rust's
> `str` is *text* — and if you genuinely need bytes, that's exactly why `[u8]`,
> `OsStr`, and `CStr` exist.
