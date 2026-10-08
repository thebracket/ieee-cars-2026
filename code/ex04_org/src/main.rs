// Demonstrates Rust's module system: inline modules, file-based modules
// (config.rs) and directory modules (net/mod.rs) -- plus "private by
// default" visibility.

mod config; // src/config.rs -- a file-based module
mod net; // src/net/mod.rs -- a directory module

// An inline module: a namespace, right here in this file.
pub mod greeting {
    // `pub` on the module exposes the module -- each item inside still
    // needs its own `pub` to be reachable.
    pub fn hello() {
        println!("Hello from an inline module!");
    }
}

fn main() {
    greeting::hello();

    // From src/config.rs (the public items, anyway):
    let cfg = config::load("app.conf");
    println!("Config: {cfg}");

    // From src/net/mod.rs:
    net::check_connection("ieee-cars.org");

    // Try uncommenting this. `validate` is private to the config module:
    // config::validate("nope"); // error[E0603]: function `validate` is private
}
