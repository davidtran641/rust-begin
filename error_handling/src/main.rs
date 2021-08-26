use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::fs;

fn main() {
    println!("Hello, world!");
    // panic1();
    // panic2();
    // error1();
    // error_cleaner();
    // error_expect();
    // read_username_demo();
    read_username_demo2();
}

fn panic1() {
  panic!("CRASH AND BURN");
}

// RUST_BACKTRACE=1 cargo run
fn panic2() {
  let v = vec![1,2,3];
  v[99];
  
}

fn error1() {
  let f = File::open("Hello.txt");
  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("Hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error)
      },
    },
  };
}

fn error_cleaner() {
  let f = File::open("Hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("Hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error)
    }
  });
}

fn error_expect() {
  let f = File::open("Hello2.txt").expect("Failed to open Hello2.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("name.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

fn read_username_demo() {
  let name = read_username_from_file().expect("Failed to read username from file");
  println!("Username: {}", name)
}

fn read_username_demo2() {
  let name = fs::read_to_string("name2.txt").expect("Failed to read username from file");
  println!("Username: {}", name)
}
