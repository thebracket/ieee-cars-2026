use std::collections::HashMap;

fn main() {
    // Arrays live on the stack and know their length.
    let array = [1, 2, 3, 4];

    // Direct indexing checks bounds at runtime -- this panics rather than
    // reading past the end like C:
    // for index in 2..6 { println!("{}", array[index]); }

    // get() turns a bounds risk into an Option -- the errors section theme:
    match array.get(5) {
        Some(value) => println!("array[5] = {value}"),
        None => println!("No element 5 -- and no reading past the end"),
    }

    // Vectors: heap-allocated and growable.
    let mut my_vec = Vec::new();
    my_vec.push(1);
    my_vec.push(2);
    println!("vector: {my_vec:?}");

    // HashMap: key -> value; lookup hands you an Option.
    let mut my_map = HashMap::new();
    my_map.insert("Hello".to_string(), 5);
    my_map.insert("World".to_string(), 6);

    if let Some(count) = my_map.get("Hello") {
        println!("Hello appears {count} times");
    }
}
