fn main(){
    match_number(1);
    match_number(2);
    match_number(3);
    match_number(4);
    match_number(5);
    match_number(6);
    match_number(10);
    match_number(11);
}

fn match_number(n: i32) {
    match n {
        1 => println!("{}: One!", n),
        2 | 3 | 4 | 5 => println!("{}: match 2 -> 5",n),
        6..=10 => println!("{}: match 6 -> 10",n),
        _ => println!("{}: match -infinite -> 0 or 11 -> +infinite", n),
    }
}