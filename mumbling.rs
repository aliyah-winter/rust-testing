fn main() {
  println!("{}", accum("abcd"));
}

fn accum(s:&str)->String {
  //s-tolowercase
  //map over string
  //return number of letters with length as idx
  //dash in between
  let mut idx = 0u32;
  let mut new_s = String::new();
  for c in s.to_lowercase().chars() {
    let upper_c = c.to_ascii_uppercase();
    new_s.push(c);
    idx += 1
  }
  s.chars().enumerate()
  new_s
}