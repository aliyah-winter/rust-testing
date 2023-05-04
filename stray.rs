fn main() {
  println!("{}", stray(&[1, 1, 1, 1, 1, 1, 2]));
  println!("{}", stray(&[2, 3, 2, 2, 2]));
  println!("{}", stray(&[3, 2, 2, 2, 2]));

}

fn stray(arr: &[u32]) -> u32 {
  let stray = arr[2];
  let mut result: u32 = 0;
  for num in arr.iter() {
    if stray != *num {
      result += num
    }
  }
  result
}