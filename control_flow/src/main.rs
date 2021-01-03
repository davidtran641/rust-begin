fn main() {
    control_ifelse();
    declare(true);

    loop_return(35);
    while_loop(35);
    for_loop();
    range();
}

fn control_ifelse() {
    let number = 30;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}

fn declare(condition: bool) {
    let number = if condition  { 5 } else { 6 };
    println!("Number is: {}", number);
}

fn loop_return(limit: i32) {
    let mut counter = 1;

    let result = loop {
        counter *= 2;
        if counter >= limit {
            break counter / 2;
        }
    };

    println!("Result is: {}", result);
}

fn while_loop(limit: i32) {
    let mut counter = 1;
    while counter < limit {
        counter *= 2;
    }

    println!("Counter is: {}", counter);
}

fn for_loop() {
    let a = [2, 3, 5, 7, 11];
    for e in a.iter() {
        println!("The value is: {}", e);
    }
}

fn range() {
    println!(">> Range");
    for n in (1..=4).rev() {
        println!("n is: {}", n);
    }
}