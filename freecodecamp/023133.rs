struct Person {
    name: String,
    age: u8,
}

fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("Alice"),
        age,
    };

    p.age = 30;
    p.name = String::from("sunfei");

    let q = build_person(String::from("Bob"), 25);
    
    println!("{} is {} years old.", p.name, p.age);
    println!("{} is {} years old.", q.name, q.age);
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        name,
        age,
    }
}