fn main() {
let result = xo("xxOo");
println!("{}", result);
}

fn xo(string: &'static str) -> bool {
  if string.to_lowercase().matches('x').collect::<Vec<&str>>().len() == string.to_lowercase().matches('o').collect::<Vec<&str>>().len() {true} else {false}
}