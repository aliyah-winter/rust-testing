fn main() {
  println!("{}", shark(7.0, 55.0, 4.0, 16.0, true))
}

fn shark(pontoon_distance: f64, shark_distance: f64, you_speed: f64, shark_speed: f64, dolphin: bool) -> String {
    if pontoon_distance / you_speed < shark_distance / (if dolphin { shark_speed / 2.0 } else {shark_speed}) {
      return String::from("Alive!");
    }
  String::from("Shark Bait!")
}