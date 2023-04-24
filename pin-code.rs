fn main() {
  println!("{}", validate_pin("a234"));
}

fn validate_pin(pin: &str) -> bool {
    pin.len() == 4 || 6;
    
}