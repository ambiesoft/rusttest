
#![allow(unused)]
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // &String is type-converted to &str
    // s1の所有権を奪い、s2の中身をコピー追加し、結果の所有権を返す
    let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意

    // value borrowed here after move
    // println!("s1 is {}", s1);
    // + is definded as
    // fn add(self, s: &str) -> String
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
}
