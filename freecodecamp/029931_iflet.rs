fn main() {
    let o: Option<i32> = Some(7);

    match o {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            println!("Success!");
        },
        _ => {}
    };

    // Same as above but using if let syntax
    if let Some(i) = o {
        println!("if let and `{:?}`", i);
        println!("Success!");
    }
}