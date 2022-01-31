fn main() {
    // Stack: Clone directly
    let x = 5;
    let mut y = x;
    y += 1;
    println!("x: {}, y: {}", x, y);

    // Heap: Clone needs to be called
    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    s2.push_str(", world!");
    println!("s1: {}, s2: {}", s1, s2);

    // Ownership: Scope
    let s = String::from("Hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // Ownership: Transfer
    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);

    // Reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    // Mutable Reference
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{}", s1);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
