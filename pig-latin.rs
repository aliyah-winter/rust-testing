use std::io;

fn main() {


  println!("Please input any word to convert to pig latin!");

  let mut word = String::new();

  io::stdin()
      .read_line(&mut word)
      .expect("Failed to read the word.");

  let mut word: String = word.trim().to_lowercase();

  println!("{}", pigify(&mut word));

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
