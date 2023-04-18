fn main() {
  println!("{}", to_alternating_case("HeLLo WoRLD"))
}

fn to_alternating_case(s: &str) -> String {
  s.chars().map(|c| {
    if c.is_uppercase() {
      c.to_ascii_lowercase()
    } else {
      c.to_ascii_uppercase()
    }
  }).collect()
}