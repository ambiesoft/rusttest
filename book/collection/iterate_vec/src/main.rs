fn main() {
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }

    println!("------------------------------");
    
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        for i in &v {
            println!("{}", i);
        }

    }
}
