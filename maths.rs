fn main() {
  let result = basic_op('-', 15, 18);
  println!("{}", result);
}
fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
  match operator {
    '+' => value1 + value2,
    '-' => value1 - value2,
    '*' => value1 * value2,
    '/' => value1 / value2,
    _ => 0
  }
}