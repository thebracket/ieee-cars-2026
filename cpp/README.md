# C++ Comparison Demos

Runnable versions of the C++ examples used in the workshop manual. Each demo is
a standalone `main.cpp` in its own directory, and one CMake build produces all
six binaries in `cpp/build/`.

## macOS prerequisites

- Xcode Command Line Tools: `xcode-select --install`
- CMake 3.21 or newer: `brew install cmake`

Verify with `cmake --version`.

## Build everything

From this directory:

```sh
cmake --preset debug
cmake --build --preset debug --parallel
```

Or without presets:

```sh
cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug
cmake --build build --parallel
```

The `Debug` build is deliberate: it is unoptimized, which keeps the data-race
and missing-lock behavior easy to observe.

## Run the demos

```sh
./build/hello_world
./build/move_by_default
./build/borrowing
./build/data_race
./build/cpp_atomic
./build/cpp_mutex
```

| Demo | Manual page | What to expect |
|---|---|---|
| `hello_world` | `ownership/intro.md` | Prints `Hello, World.` |
| `move_by_default` | `ownership/move_by_default.md` | Prints `hello world`, then a blank line for the moved-from string. The moved-from value is valid but unspecified. |
| `borrowing` | `ownership/borrowing2.md` | Prints `1` twice, then demonstrates use-after-free. It may crash — that is the point. |
| `data_race` | `concurrency/data_race.md` | The total is below 2,000,000 and changes on every run. |
| `cpp_atomic` | `concurrency/cpp_atomic.md` | Prints `Counter: 300000` every time. |
| `cpp_mutex` | `concurrency/cpp_mutex.md` | The total is usually below 20,000 and changes on every run, because `t2` forgot to lock. The manual shows this binary as `./mutex_spot`. |

`data_race`, `borrowing`, and `cpp_mutex` intentionally contain undefined
behavior. Run `data_race` and `cpp_mutex` several times during the demo so the
variation is visible. Clang also warns that the `lock` capture in `cpp_mutex`
is unused — that warning is part of the lesson.

## Other commands

```sh
# Rebuild after an edit
cmake --build --preset debug --parallel

# Build one demo
cmake --build --preset debug --target data_race

# Remove all generated files
rm -rf build
```

Build output lives in `build/`, which is git-ignored.
