fn main() {
  let dead = false;
  let health = 48;
  if dead {
    println!("Game over!");
    return;
  }
  if dead {
    println!("Game over!");
    return;
  } else {
    println!("You still have a chance to win!");
  }
  if health >= 50 {
    println!("Continue to fight!");
  } else if health >= 20  {
    println!("Stop the battle and gain strength!");
  } else {
    println!("Hide and try to recover!");
  }

  let adult = true;
  let age = if adult { "+18" } else { "-18" };
  println!("Age is {}", age);  // Age is +18
  verbose(9);
  // let result = if health <=0 { "Game over man!" };

}

fn verbose(x: i32) -> &'static str {
  let mut result: &'static str;
  if x < 10 {
    result = "less than 10";
  } else {
    result = "10 or more";
  }
  return result;
}
