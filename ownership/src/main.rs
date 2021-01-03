fn main() {
    play_with_string();
    moving_ownership();
    clone_data();
    stack_only_copy();
    ownership_function();
    gives_ownership();
}

fn play_with_string() {
    {
        let mut s = String::from("Hello");
        s.push_str(", world!");
        println!("{}", s);
    }
    // s is out of scope, Rust will calls `drop` to cleanup memory from heap
}

fn moving_ownership() {
    let s1  = String::from("Hello-Move");
    let s2 = s1;

    // Error: value borrowed here after move
    // println!("s1 is: {}", s1);

    println!("s2 is: {}", s2);
}

fn clone_data() {
    let s1  = String::from("Hello-Clone");
    let s2 = s1.clone();

    println!("s1 is: {}", s1);
    println!("s2 is: {}", s2);
}

fn stack_only_copy() {
    let x = 5;
    let y = x;
    println!("x: {}", x);
    println!("y: {}", y);
}

fn ownership_function() {
    let s = String::from("Hello-function");
    take_ownership(s);

    // Error: value borrowed here after move
    // println!("s is: {}", s);

    let x = 5;
    make_copy(x);
    println!("x is: {}", x);
}

fn take_ownership(s: String) {
    println!("Take ownership: {}", s);
}

fn make_copy(x: i32) {
    println!("Make copy: {}", x);
}

fn gives_ownership() {
    let s1 = String::from("Hello-take-give-back");

    let s2 = take_and_gives_back(s1);

    // Error: value borrowed here after move
    // println!("s1: {}", s1);

    println!("s2: {}", s2);
}

fn take_and_gives_back(s: String) -> String {
    let mut s = s;
    s.push_str(": give back");
    s // return value and move the ownership out.
}