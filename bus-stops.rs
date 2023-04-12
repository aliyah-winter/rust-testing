fn main() {
  println!("{}", number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]))
}

fn number(bus_stops:&[(i32,i32)]) -> i32 {
  bus_stops.iter().fold(0, |acc, (x, y)| acc + (x - y))
}