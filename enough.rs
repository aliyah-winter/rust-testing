fn main() {
  println!("{}", enough(100, 60, 50)) 
}

fn enough(cap: i32, on: i32, wait: i32) -> i32 {
  match on + wait > cap {
    true => (on + wait) - cap,
    false => 0
  }
}