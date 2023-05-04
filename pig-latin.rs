fn main() {

println!("{}", pigify(String::from("hello")));
println!("{}", pigify(String::from("epple")));
}

fn pigify(s: String) -> String {
  match &s[0..0] {
    "a" | "e" | "i" | "o" | "u" => format!("{s}ay"),
    _ => {let rest = &s[1..];
      let first = &s[0..0];
      format!("{rest}{first}ay")}
  }
  
}
