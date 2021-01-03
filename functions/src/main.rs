fn main() {
    another_function(5, 6);
    expression();

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(x);
    println!("The value of x is: {}", x);

    let x = plus_one_return(x);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn expression() {
    let a = 5;
    let b = {
        let a = 8;
        a + 1
    };
    println!("The value of a, b are: {}, {}", a, b);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_one_return(x: i32) -> i32 {
    return x + 1;
}