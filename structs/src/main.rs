
struct Player {
   nname: &'static str, // nickname
   health: i32,
   level: u8
}

fn main(){
    // let Score(h, l) = score1; // destructure the tuple
    // println!("Health {} - Level {}", h, l);
    struct Kilograms(u32);
    let weight = Kilograms(250);
    let Kilograms(kgm) = weight; // extracting kgm
    println!("weight is {} kilograms", kgm);
    let mut pl1 = Player{ nname: "Dzenan", health: 73, level: 2 };
    println!("Player {} is at level {}", pl1.nname, pl1.level);
    pl1.level = 3;
    let Player{ health: ht, nname: nn, .. } = pl1;
    println!("Player {} has health {}", nn, ht);
    let ps = &Player{ nname: "John", health: 95, level: 1 };
    println!("{} == {}", ps.nname, (*ps).nname);
}
