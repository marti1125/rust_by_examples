fn main() {
  let mut klaatu = Alien{ planet: "Venus".to_string(), n_tentacles: 15 };
  {
    let kl2 = &mut klaatu;
    kl2.n_tentacles = 14;
    println!("{} - {}", kl2.planet, kl2.n_tentacles);
    // prints: Venus - 14

  }
}
