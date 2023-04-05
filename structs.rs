fn main() {
  #[derive(Debug)]
    struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }

  let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};

  user1.email = String::from("aliyahgwinter@gmail.com");

  fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

  let user2 = build_user(String::from("oilgrace@hotmail.com"), String::from("oilgrace"));

  let user3 = User {
    email: String::from("another@example.com"),
    ..user1
  };

  println!("{:?}", &user2);
  println!("{:?}", &user3);
  // println!("{:?}", user1);
}