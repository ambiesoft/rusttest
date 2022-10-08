fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable    
    v.push(6);
    println!("The first element is: {}", first);
}
