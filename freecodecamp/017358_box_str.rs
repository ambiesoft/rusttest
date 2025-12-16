fn main() {
    // allocate on the heap
    let s: Box<str> = "hello, world".into();
    greetings(&s);
}

fn greetings(s: &str) {
    println!("{}", s)
}
