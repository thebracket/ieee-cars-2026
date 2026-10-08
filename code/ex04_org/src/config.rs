// A file-based module, declared in main.rs with `mod config;`.
// The FILE is the namespace: this is `config::` from outside.

pub fn load(path: &str) -> String {
    let raw = std::fs::read_to_string(path).unwrap_or_else(|_| String::new());
    if validate(&raw) {
        raw
    } else {
        "(empty or invalid config)".to_string()
    }
}

// No `pub` = private to this module. Callers can't reach it, and if you
// try, the compiler names the exact privacy boundary you violated.
fn validate(text: &str) -> bool {
    !text.trim().is_empty()
}
