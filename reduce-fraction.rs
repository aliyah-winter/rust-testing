use goose::util;

fn main() {
  println!("{:?}", reduce_fraction((60, 20)));
}

fn reduce_fraction(fraction: (u32, u32)) -> (u32, u32) {
  let gcd = util::gcd(fraction.0, fraction.1)
}