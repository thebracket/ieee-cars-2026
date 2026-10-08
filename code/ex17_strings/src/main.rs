fn main() {
    let text = String::from("Hello, 🌎!");

    // len() is BYTES, not characters.
    println!("Bytes: {}", text.len());
    println!("Chars: {}", text.chars().count());

    // Safe slicing: .get() returns Option<&str> -- None if the range
    // would split a character.
    println!("0..7: {:?}", text.get(0..7));
    println!("0..8: {:?}", text.get(0..8)); // would split the emoji

    // Iterate by character, not by byte.
    println!("char #7: {:?}", text.chars().nth(7));

    // The emoji's UTF-8 encoding is four bytes long.
    let emoji_bytes: Vec<u8> = text.bytes().skip(7).take(4).collect();
    println!("emoji bytes: {emoji_bytes:?}");
}
