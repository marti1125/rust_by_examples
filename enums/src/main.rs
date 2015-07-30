fn main(){
    enum Compass {
      North, South, East, West
    }
    let direction = Compass::West;
    type species = &'static str;

    enum PlanetaryMonster {
      VenusMonster(species, i32),
      MarsMonster(species, i32)
    }
    let martian = PlanetaryMonster::MarsMonster("Chela", 42);
}
