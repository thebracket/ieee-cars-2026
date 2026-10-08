fn printme(s: String) -> String {
    println!("{s}");
    s // Note that "return" isn't required, the last expression WITHOUT a semicolon is returned
}

fn main() {
    let s = "Hello world".to_string();
    let s = printme(s); // We're shadowing; technically, it's a whole new s!
    println!("{s}");
}
