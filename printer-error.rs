fn main() {
  println!("{}", printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"))
}

fn printer_error(s: &str) -> String {
  let slen: usize = s.len();
  let errornum: u32 = s.chars().fold(0, |a, x| match x {
    'n'..='z' => a + 1,
    _ => a,
  });
  format!("{}/{}", errornum, slen)
}
// let mut errornum = 0u32;
// for char in s.chars() {
//   match char {
//     'n'..='z' => errornum += 1,
//     _ => (),
//   }
// }