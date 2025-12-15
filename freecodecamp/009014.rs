fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {

        }
        _ => {

        }
    };

    // Rather than returning a None, we use a diverging function
    never_return_fn();
}

fn never_return_fn() -> ! {
    panic!();
}