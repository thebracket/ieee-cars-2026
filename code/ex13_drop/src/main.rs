struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("I was dropped");
    }
}

fn do_something() {
    let a = MyStruct{};
}

fn main() {
    println!("Calling function");
    do_something();
    println!("Returned");
}
