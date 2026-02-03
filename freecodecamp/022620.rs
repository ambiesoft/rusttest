struct Person {
    name: String,
    age: u8,
    hobby: String,
}

fn main() {
    let age = 30;
    let p = Person {
        name:String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };

    println!("{} is {} years old, and likes {}.", p.name, p.age, p.hobby);
}
