fn main(){
    let thor = ("Thor", true, 3500u32);
    println!("{:?}", thor); // ("Thor", true, 3500)
    println!("{} - {} - {}", thor.0, thor.1, thor.2);
    let (name, _, power) = thor;
    println!("{} has {} points of power", name, power);
    let (god, strength) = increase_power(thor.0, thor.2);
    println!("This god {} has now {} strength", god, strength);
}

fn increase_power(name: &str, power: u32) -> (&str, u32) {
  if power > 1000 {
    return (name, power * 3);
  } else {
    return (name, power * 2);
  }
}
