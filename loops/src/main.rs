fn main() {
  let max_power = 10;
  let mut power = 1;
  while power < max_power {
        print!("{} ", power); // prints without newline
        power += 1;           // increment counter
  }

  loop {
      power += 1;
      if power == 42 {
        // Skip the rest of this iteration
        continue;
      }
      print!("{}  ", power);
      if power == 50 {
        print!("OK, that's enough for today");
        break;  // exit the loop
      }
  }

  'outer: loop {
      println!("Entered the outer dungeon - ");
      'inner: loop {
          println!("Entered the inner dungeon - ");
          // break;    // this would break out of the inner loop
          break 'outer; // breaks to the outer loop
      }
      println!("This treasure can sadly never be reached - ");
    }
    println!("Exited the outer dungeon!");

    for n in 1..12 {
      println!("The square of {} is {}", n, n * n);
    }

}
