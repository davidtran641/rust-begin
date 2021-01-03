fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS const is: {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let x = x * 2;
    println!("The value of x is: {}", x);

    let guess: u32 = "42".parse().expect("Nan");
    println!("Guess is: {}", guess);
}
