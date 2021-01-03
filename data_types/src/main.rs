fn main() {
    let c = 'z';
    println!("c is: {}", c);

    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat is: {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, _) = tup;
    println!("x, y is: {}, {}", x, y);

    println!("tup is: {}, {}, {}", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3, 4, 5];
    println!("arr is: {}", arr[0]);

    let arr = [3;5];
    println!("arr[4] is: {}", arr[4]);
}
