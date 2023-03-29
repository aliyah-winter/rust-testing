fn main() {
  let months = ["January", "February", "March", "April", "May", "June", "July",
  "August", "September", "October", "November", "December"];
  println!("{}", months[0]);

  another_function(months[2]);

  fn another_function(m: &str) {
    println!("The month is {}", m);
  }
}