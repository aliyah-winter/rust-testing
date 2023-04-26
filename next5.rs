fn main() {
  println!("{}", round_to_next_5(22))
}

fn round_to_next_5(n: i32) -> i32 {
  if n % 5 == 0 {
    n
  } else {
    let mut num = n;
    while num % 5 != 0 {
      num += 1
    }
    num
  }
}