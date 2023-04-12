fn main() {
  let result: Vec<u32> = count_by(2, 5);
  println!("{:?}", result)
}
//first 5 multiples of 2
//loop to n, push multiples into vec
fn count_by(x: u32, n: u32) -> Vec<u32> {
  let mut multiples = Vec::new();
  for i in 1..=n {
    multiples.push(x * i)
  }
  multiples
}