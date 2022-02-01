fn main() {
    // String reference
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);

    // str literal is a slice by default
    let string_literal = "hello, world!";
    let word2 = first_word(string_literal);
    println!("{}", word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
