fn main() {
    let my_vec = vec![1, 2, 3, 4];

    // Build a new vector of Strings:
    let my_new_vec: Vec<String> = my_vec
        .iter()
        .map(|n| n.to_string()) // map converts each entry to another type
        .collect();

    let max = my_vec.iter().max();
    let min = my_vec.iter().min();
    let sum: u32 = my_vec.iter().sum();
    let count = my_vec.iter().count();

    let sum_with_fold = my_vec.iter().fold(0, |acc, x| acc + x);

    let all_the_numbers: Vec<(&u32, &String)> = my_vec.iter().zip(my_new_vec.iter()).collect();
    println!("{all_the_numbers:?}");

    my_vec
        .iter()
        .filter(|n| **n > 2) // two layers of reference: deref twice
        .for_each(|n| println!("{n}"));

    println!("max={max:?} min={min:?} sum={sum} count={count} fold={sum_with_fold}");
}
