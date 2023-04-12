fn main()  {
  println!("{}", get_grade(50, 50, 50));
}

fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let average: u16 = (s1 + s2 + s3) / 3;
    match average {
      0..=59 => 'F',
      60..=69 => 'D',
      70..=79 => 'C',
      80..=89 => 'B',
      90..=100 => 'A',
      _ => panic!("Not a valid average grade")
    }
}