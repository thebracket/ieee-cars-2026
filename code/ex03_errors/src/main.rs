use std::collections::HashMap;
use std::path::Path;

// Like C's fopen() + fgets(), except the "did it work?" information travels
// inside the return value -- and the compiler insists that you deal with it.
fn load_config() -> Result<String, std::io::Error> {
    let path = Path::new("config.json");
    let text = std::fs::read_to_string(path)?; // <-- bail out early on error
    Ok(text)
}

fn main() {
    // --- Errors you can't ignore ------------------------------------
    // Uncomment this. It COMPILES -- but you get a compiler warning:
    // "unused `Result` that must be used".
    // load_config();

    // The normal way: match. Both variants MUST be handled.
    match load_config() {
        Ok(text) => println!("Config: {text}"),
        Err(e) => println!("Couldn't read config: {e}"),
    }

    // Ignoring the result is possible -- but you have to say so:
    let _ = load_config();

    // --- No more null -----------------------------------------------
    let mut users = HashMap::new();
    users.insert("herbert", 47);

    // get() returns Option<&i32> -- Some(age) or None. No null in sight.
    match users.get("root") {
        Some(age) => println!("root is {age}"),
        None => println!("No user named root"), // You MUST decide what happens
    }

    // One-variant pattern matching, for when you only care about Some:
    if let Some(age) = users.get("herbert") {
        println!("herbert is {age}");
    }
}
