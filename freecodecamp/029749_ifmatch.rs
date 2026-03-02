enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let mut count=0;

    let v: Vec<MyEnum> = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];

    for e in v {
        // if-conditional can not be used to match enum variants, but match can be used instead
        // if e == MyEnum::Foo {
        if matches!(e, MyEnum::Foo){
            count+=1;
        }
    }

    assert_eq!(count,2);

    println!("All tests passed!");
}