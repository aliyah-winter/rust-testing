fn main() {
 let result = find_short("bitcoin take over the world maybe who knows perhaps");
 println!("{}", result);
}

fn find_short(s: &str) -> u32 {
  let v: Vec<&str> = s.split(' ').collect();
  v.iter().fold(v[0], |acc: &str, &item| {
    if item.len() < acc.len() {
      item
    } else {
      acc
    }
  }).len() as u32
}