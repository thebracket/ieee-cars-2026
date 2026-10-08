// A directory module, declared in main.rs with `mod net;`.
// The directory's `mod.rs` is the module root: this is `net::` from outside.
// More files can live alongside it in the `net/` directory.

pub fn check_connection(host: &str) {
    println!("Checking connection to {host}... (imagine a socket here)");
}
