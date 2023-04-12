fn main() {
  println!("{}", odd_or_even([0, -1, -5].to_vec()))
}
fn odd_or_even(numbers: Vec<i32>) -> String {
  match numbers.iter().sum::<i32>() % 2 == 0 {
    true => String::from("even"),
    false => String::from("odd")
  }
}