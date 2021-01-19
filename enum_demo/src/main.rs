fn main() {
  enum_ip_demo();
  demo_option();
  demo_coin();
  demo_match_option();
  demo_if_let();
}

#[derive(Debug)]
struct Ipv4Addr {
  a: u8,
  b: u8,
  c: u8,
  d: u8,
}

impl Ipv4Addr {
  fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
    Ipv4Addr{ a,b,c,d }
  }
} 

#[derive(Debug)]
enum IPAddr {
  V4(Ipv4Addr),
  V6(String),
}

impl IPAddr {
  fn route(&self) {
    
  }
}

fn route(ip_kind: IPAddr) {
  
}

fn enum_ip_demo() {
  let home = IPAddr::V4(Ipv4Addr::new(127,0,0,1));
  let loopback = IPAddr::V6(String::from("::1"));
  
  println!("IP: {:?}, {:?}", home, loopback);
}

fn demo_option() {
  let x: i8 = 5;
  let y: Option<i8> = Some(10);
  let sum = x + y.unwrap_or(0);
  
  println!("{} + {:?} = {}", x, y, sum);
}

fn demo_coin() {
  #[derive(Debug)]
  enum UsState {
    Alabama,
    Alaska
  }
  
  #[derive(Debug)]
  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
  }
  
  fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
      Coin::Penny => {
        println!("Lucky penny!");
        1
      },
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quater(state) => {
        println!("State quarter from {:?}!", state);
        25
      },
    }
  }
  
  let coin = Coin::Penny;
  println!("Value of {:?} is {}", coin, value_in_cents(&coin));
  
  let coin2 = Coin::Quater(UsState::Alaska);
  println!("Value of {:?} is {}", coin2, value_in_cents(&coin2));
}

fn demo_match_option() {
  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
      None => None,
      Some(i) => Some(i+1),
    }
  }
  
  let five = Some(5);
  println!("five + 1 = {:?}", plus_one(five));
  
  println!("None + 1 = {:?}", plus_one(None));
}

fn demo_if_let() {
  fn plus_one(x: Option<i32>) -> Option<i32> {
    if let Some(v) = x {
      Some(v+1)
    } else {
      None
    }
  }


  let five = Some(5);
  println!("If-let: five + 1 = {:?}", plus_one(five));

  println!("If-let: None + 1 = {:?}", plus_one(None));
}