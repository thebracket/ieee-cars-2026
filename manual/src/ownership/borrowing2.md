# Borrows in Rust

Borrows in Rust do a bit more than a C++ reference. In addition to handy syntax sugar, they also track a *lifetime* of the borrow. Take the following modern C++ (it compiles, but any
checker worth running will fail it):

```cpp
#include <string>
#include <iostream>
#include <memory>

struct Test {
    int n;
    Test(int n) : n(n) {} ;
};

int main() {
    auto tmp = std::make_unique<Test>(1);
    auto n_ptr = &tmp->n;
    
    // All good
    std::cout << tmp->n << '\n';
    std::cout << *n_ptr << '\n';
    
    tmp.reset();
    
    // Oh dear
    std::cout << tmp->n << '\n';
    std::cout << *n_ptr << '\n';
}
```

On `cpp.sh` at least, this prints "1 1 1668509029 1". It could legally print just about anything for the last two! Note that `Box` is `unique_ptr` wearing a crab outfit.

Rust *will not let you do that*. So this is fine (in both languages!):

```rust
struct Test {
    n: i32
}

fn main() {
    let tmp = Box::new(Test { n: 1 });
    let n_ptr = &tmp.n;
    
    println!("{}", tmp.n);
    println!("{}", *n_ptr);
}
```

But when you try and use after free:

```rust
struct Test {
    n: i32
}

fn main() {
    let tmp = Box::new(Test { n: 1 });
    let n_ptr = &tmp.n;
    
    println!("{}", tmp.n);
    println!("{}", *n_ptr);
    drop(tmp);
    println!("{}", *n_ptr);
    println!("{}", tmp.n);
}
```

You get a compilation error:

```
  error[E0505]: cannot move out of `tmp` because it is borrowed
  --> src/main.rs:11:10
   |
 6 |     let tmp = Box::new(Test { n: 1 });
   |         --- binding `tmp` declared here
 7 |     let n_ptr = &tmp.n;
   |                 ------ borrow of `tmp.n` occurs here
...
11 |     drop(tmp);
   |          ^^^ move out of `tmp` occurs here
12 |     println!("{}", *n_ptr);
   |                    ------ borrow later used here
   |
help: consider cloning the value if the performance cost is acceptable
   |
 7 |     let n_ptr = &tmp.n.clone();
   |                       ++++++++

error[E0382]: borrow of moved value: `tmp`
  --> src/main.rs:13:20
   |
 6 |     let tmp = Box::new(Test { n: 1 });
   |         --- move occurs because `tmp` has type `Box<Test>`, which does not implement the `Copy` trait
...
11 |     drop(tmp);
   |          --- value moved here
12 |     println!("{}", *n_ptr);
13 |     println!("{}", tmp.n);
   |                    ^^^^^ value borrowed here after move
   |
note: if `Test` implemented `Clone`, you could clone the value
  --> src/main.rs:1:1
   |
 1 | struct Test {
   | ^^^^^^^^^^^ consider implementing `Clone` for this type
...
11 |     drop(tmp);
   |          --- you could clone this value

Some errors have detailed explanations: E0382, E0505.
For more information about an error, try `rustc --explain E0382`.
error: could not compile `playground` (bin "playground") due to 2 previous errors
```

Quite a big error, really. It even shows you where the drop occurred, and suggests that you could make a copy if you really want
to keep it around. And note that this is transitive and deep - not just the `Box` (unique_ptr equivalent) but the reference/pointer
to the contents fail to compile.

So what's really happening when you borrow (with `&`)? Rust sets up a *lifetime* for the borrow. The compiler will find the last time
the borrowed value is used, and then check that the borrow is valid throughout the *lifetime* of the borrow. In this case, it isn't -
you kill the target.

This also shows why move-by-default is so important. Dropping a value *moves* it, placing `tmp` into a "moved from" state. Since Rust
tracks *ownership*, using the pointer after it is moved (freed in this case) will not compile.
