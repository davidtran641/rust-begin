struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn build_user(email: String, username: String) -> User {
  return User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  };
}

fn demo_user() {
  let mut user1 = User {
    email: String::from("abc@example.com"),
    username: String::from("user1"),
    active: true,
    sign_in_count: 1,
  };

  println!("UserName: {}", user1.username);

  user1.username = String::from("New name");

  println!("UserName: {}", user1.username);


  let user2 = User {
    email: String::from("abc@ex.com"),
    username: String::from("name2"),
    ..user1
  };
  println!("Active: {}", user2.active);

}


fn tuple_struct() {
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0,0,0);
  let origin = Point(0,0,0);

  println!("black: {}", black.0);
  println!("origin: {}", origin.0);
}

fn main() {
    demo_user();
    tuple_struct();
}
