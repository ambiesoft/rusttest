fn main() {
    let mut s: String = String::from("hello, ");

    let r1: &mut String = &mut s;
    r1.push_str("world!");

    let r2: &mut String = &mut s;
    r2.push_str("!");

    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    // println!("{}", r1);

    println!("{}", r2);
}
