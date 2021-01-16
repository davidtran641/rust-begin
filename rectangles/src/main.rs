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

fn demo_rectangle_struct() {
  let rect = Rectangle{
    width: 30,
    height: 50,
  };

  println!("Area of {:#?} is: {}", rect, area2(&rect));
}

fn area2(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}