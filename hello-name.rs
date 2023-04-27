fn main() {
  println!("{}", hello("aliyah"));
}

fn hello(name: &str) -> String {
  if name == "" {
    String::from("Hello, World!")
  } else {
    format!("Hello, {}{}!", (&name[..1].to_string()).to_uppercase(), (&name[1..].to_string()).to_lowercase())
  }
}