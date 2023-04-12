fn main() {
  let result: String = count_sheep(3);
  println!("{result}");
}
fn count_sheep(n: u32) -> String {
  let mut res: String = String::new();
  for i in 1..=n {
    let sheep = format!("{} sheep...", i);
      res.push_str(&sheep)
  }
  res
}