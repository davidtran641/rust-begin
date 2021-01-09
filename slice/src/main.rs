fn main() {
    slice();
    test_first_word();
}

fn slice() {
    let s = String::from("Hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}-{}", hello, world);
}

fn test_first_word() {
    let s = String::from("Hello world");

    let ret = first_word(&s);
    
    // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
    //s.clear();

    println!("ret = {}", ret);
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]
}
