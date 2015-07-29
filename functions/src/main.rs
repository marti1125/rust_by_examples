fn main() {
  let hero1 = "Pac Man";
  let hero2 = "Riddick";
  greet(hero2);
  greet_both(hero1, hero2);
  let power = increment_power(1); // function is called
  println!("My power level is now: {}", power);
  println!("The cube of 4 is {}", cube(4));
  //on_windows();
}

fn greet(name: &str) {
  println!("Hi mighty {}, what brings you here?", name);
}

fn greet_both(name1: &str, name2: &str) {
  greet(name1);
  greet(name2);
}

fn increment_power(power: i32) -> i32 {
  println!("My power is going to increase:");
  power + 1
}

/// Calculates the cube `val * val * val`.
///
/// # Examples
///
/// ```
/// let cube = cube(val);
/// ```
pub fn cube(val: u32) -> u32 {
    val * val * val
}

#[cfg(target_os = "windows")]
fn on_windows() {
    println!("This machine has Windows as its OS.");
}
