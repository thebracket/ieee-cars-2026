// Run with: cargo run -p ex03_errors_part3 -- 5
//
// Safe-math mini-exercise: read a divisor from the command line, parse it,
// and divide 100 by it -- handling every Option and Result along the way.

fn main() {
    let Some(arg) = std::env::args().nth(1) else {
        println!("Usage: cargo run -p ex03_errors_part3 -- <number>");
        return;
    };

    let divisor = match arg.parse::<i32>() {
        Ok(number) => number,
        Err(error) => {
            println!("That isn't a number: {error}");
            return;
        }
    };

    match 100i32.checked_div(divisor) {
        Some(result) => println!("100 / {divisor} = {result}"),
        None => println!("Cannot divide 100 by {divisor}"),
    }
}
