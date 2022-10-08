
#![allow(unused)]
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // 所有権を奪わない
    let s4 = format!("{}-{}-{}", s1, s2, s3);

    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
    println!("s4 is {}", s4);
}
