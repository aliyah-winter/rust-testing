fn main() {
  let words: String = String::from("hello my name is aliyah");
  let other_word = "aliyah lol";

  let result = first_word(&words);
  let result2 = first_word(other_word);

  println!("{}, {}", result, result2);

  fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

}