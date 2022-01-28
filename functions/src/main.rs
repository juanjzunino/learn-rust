fn five() -> i32 {
    5
}
fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The measurement is {}{}", x, unit_label);
}

fn main() {
    print_labeled_measurement(five(), 'h');
}
