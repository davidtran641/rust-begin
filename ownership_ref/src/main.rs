fn main() {
    string_ref();
    
    string_ref_modify();
    multiple_ref(); 
}

fn string_ref() {
    let s = String::from("Hello world!");
    let len = calculate_len(&s);

    println!("Len of '{}' is: {}", s, len);
}

fn calculate_len(s: &String) -> usize {
    return s.len();
}

fn string_ref_modify() {
    let mut s = String::from("Hello");
    change(&mut s);

    println!("New s: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn multiple_ref() {
    let mut s = String::from("Hello");
    let mut r1 = &mut s;
    // let mut r2 = &mut s;

    println!("{}", r1);
    // println!("{} , {}", r1, r2);
}

// Error: this function's return type contains a borrowed value, but there is no value for it to be borrowed from

// fn dangle_ref() {
//     let ref_to_nothing = dangle();
//     println!("{}", ref_to_nothing);
// }

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }

