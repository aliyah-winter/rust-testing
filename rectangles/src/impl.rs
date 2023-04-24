pub impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn width(&self) -> bool {
    self.width > 0
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}