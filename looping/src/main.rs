fn main() {
    let a: [usize; 5] = [1, 2, 3, 4, 5];

    let mut index: usize = 0;

    while index < a.len() {
        println!("A[{}]: {}", index, a[index]);
        index += 1;
    }

    println!("");

    for (idx, val) in a.iter().enumerate() {
        println!("A[{}]: {}", idx, val);
    }

    println!("");

    for idx in (0..a.len()).rev() {
        println!("A[{}]: {}", idx, a[idx]);
    }
}
