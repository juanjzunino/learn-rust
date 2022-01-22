use std::{io, usize};

fn main() {
    // Create array
    let a: [u32; 5] = [1, 2, 3, 4, 5];

    // Users input
    println!("Provide an array index:");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered is not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    )
}
