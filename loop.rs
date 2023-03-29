fn main() {
  let mut count = 0;

  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }
    count += 1;
  }
println!("End count = {count}");

for num in (1..4).rev() {
  println!("{num}!");
}

println!("LIFTOFF");

let a = [10, 20, 30, 40, 50];
let mut index = 0;

while index < 5 {
  println!("The value is : {}", a[index]);

  index += 1;
};

for num in a {
  println!("{num}");
}

}