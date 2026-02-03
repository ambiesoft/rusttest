// Rectangle struct has Debug trait implemented
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // Print debug info to stderr as we create rect1
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("---------------");
    dbg!(&rect1);

    println!("---------------");

    println!("{:?}", rect1);
}