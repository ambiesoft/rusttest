fn main() {
    // let x = 1; // Imuutable by default
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}