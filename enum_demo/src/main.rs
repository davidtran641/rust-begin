fn main() {
  enum_ip_demo();
  demo_option();
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
