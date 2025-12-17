fn main() {
    let mut s: String = String::from("hello world");

    let word: &str = first_word(&s);  // &String -> &str

    // Error: word is used in the following line
    // s.clear();

    println!("the first word is: {}", word);

    s.clear();
}

fn first_word(s: &str) -> &str {
    &s[..1]
}