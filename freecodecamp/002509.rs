fn main() {
    let (x, y);

    // double dots, ignore '4'
    (x, ..) = (3, 4);

    // double dots before y, ignore '1'
    [.., y] = [1, 2];

    assert_eq!([x, y], [3, 2]);

    // x: 3, y: 2
    println!("x: {}, y: {}", x, y);
}
