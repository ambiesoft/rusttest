fn main() {
    let alphabets = ['a','E','Z','0','x','9','Y'];

    for ab in alphabets {
        assert!(match ab {
            'a'..='z' => true,
            'A'..='Z' => true,
            '0'..='9' => true,
            _ => false,
        });

        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'));
    }

    println!("All tests passed!");
}