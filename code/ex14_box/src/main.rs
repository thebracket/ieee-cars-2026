struct MyStruct {
    n: i32,
}

fn main() {
    let boxed = Box::new(MyStruct { n: 12 });
    println!("{}", boxed.n);
}
