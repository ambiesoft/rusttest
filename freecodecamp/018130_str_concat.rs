fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");

    // expected `&str`, found `String`
    // let s3 = s1 + s2;

    // s1 is moved here and can no longer be used
    let s3 = s1 + &s2;

    println!("{}", s3);

    println!("{}", s2);

    // value borrowed here after move
    // println!("{}", s1);
}
