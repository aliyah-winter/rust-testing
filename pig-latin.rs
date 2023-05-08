fn main() {

  println!("{}", pigify(&mut String::from("hello")));
  println!("{}", pigify(&mut String::from("apple")));

}

fn pigify(s: &mut String) -> String {
  let first = s.remove(0);
  match first {
    'a' | 'e' | 'i' | 'o' | 'u' => format!("{}{}ay", first, &s[0..]),
    _ => format!("{}{}ay", &s[0..], first)
  }
}

// fn is_vowel(c: char) -> bool {
//   match c {
//     'a' | 'e' | 'i' | 'o' | 'u' => true,
//     _ => false
//   }
// }
