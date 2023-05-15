fn main() {
  println!("{}", nb_year(1500, 5.0, 100, 5000))
}

fn nb_year(mut p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
  let mut year_count: i32 = 0;
 while p0 < p { 
  p0 = p0 + (p0 as f64 * percent / 100.0) as i32 + aug;
  year_count += 1;
}
  year_count

}