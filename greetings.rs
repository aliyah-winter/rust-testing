fn main() {
  println!("{}", greet("welsh"))
}

fn greet(language: &str) -> &str {
  let data: [(&str, &str); 17] = [("english", "Welcome"),
("czech", "Vitejte"),
("danish", "Velkomst"),
("dutch", "Welkom"),
("estonian", "Tere tulemast"),
("finnish", "Tervetuloa"),
("flemish", "Welgekomen"),
("french", "Bienvenue"),
("german", "Willkommen"),
("irish", "Failte"),
("italian", "Benvenuto"),
("latvian", "Gaidits"),
("lithuanian", "Laukiamas"),
("polish", "Witamy"),
("spanish", "Bienvenido"),
("swedish", "Valkommen"),
("welsh", "Croeso")];
  let result = data.iter().find(|(lang, greeting)| -> _ {lang == language});
  println!("{:?}", result);
  "Welcome"
}