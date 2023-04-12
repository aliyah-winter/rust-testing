fn main() {
  println!("{}", odd_or_even([0, -1, -5].to_vec()))
}
fn odd_or_even(numbers: Vec<i32>) -> String {
  let res: i32 = numbers.iter().fold(0, |acc, num| acc + num);
  match res {
    _ if res % 2 == 0 => String::from("even"),
    _ => String::from("odd")
  }
}