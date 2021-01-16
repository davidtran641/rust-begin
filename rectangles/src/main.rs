fn main() {
  demo_area();
  demo_rectangle_struct();
}

fn demo_area() {
  let width = 30;
  let height = 50;
  println!("Area: {}, {} is: {}", width, height, area(width, height));
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size
    }
  }
}

fn demo_rectangle_struct() {
  let rect = Rectangle{
    width: 30,
    height: 50,
  };

  println!("Area of {:#?} is: {}", rect, rect.area());

  let rect2 = Rectangle{
    width: 20,
    height: 40,
  };
  println!("Can hold: {}", rect.can_hold(&rect2));

  let square = Rectangle::square(20);
  println!("square: {:?}", square);
}
