fn main() {
  println!("{}", nb_year(1500, 5.0, 100, 5000))
}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
  let mut total: i32 = p0;
  let mut years: i32 = 0;
  
 while total < p { 
  total = total + (total as f64 * percent / 100.0) as i32 + aug;
  years += 1;
}
  years
}