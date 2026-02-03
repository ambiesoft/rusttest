struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let v = Point(10, 20, 30);
    check_color(v);
}

fn check_color(p: Point) {
    let Point(x, y, z) = p;
    println!("Point - X: {}, Y: {}, Z: {}", x, y, z);
}