fn main() {
    let mut rng = 0..7;
    println!("> {:?}", rng.next()); // prints Some(0)
    println!("> {:?}", rng.next()); // prints Some(1)
    for n in rng {
      print!("{} - ", n);
    } // prints 2 - 3 - 4 - 5 - 6 -
    let aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
    for alien in aliens.iter() {
        print!("{} / ", alien)
        // process alien
    }
}
