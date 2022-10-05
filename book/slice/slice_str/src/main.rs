fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // clearはStringを切り詰める必要があるので、可変な参照を得る必要があります。
    // Rustはこれを認めないので、コンパイルが失敗します。 
    s.clear(); // error! （エラー！）

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}