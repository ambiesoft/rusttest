#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn main() {
    let f = File {
        name: String::from("hello.txt"),
        data: String::from("Hello, world!"),
    };

    let _name = f.name; // move occurs here

    // can't use f.name anymore, but can still use f.data
    println!("Name: {}, Data: {}", _name, f.data);
}
