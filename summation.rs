fn main() {
  println!("{}", summation(8));
}
fn summation(n: i32) -> i32 {
  (1..=n).fold(0, |acc, num| acc + num)
}