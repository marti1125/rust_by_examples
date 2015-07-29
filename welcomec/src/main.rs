static MAX_HEALTH: i32 = 100;
static GAME_NAME: &'static str = "Monster Attack";

type MagicPower = u16;

// warning: incorrect code!
// static GAME_NAME: &str = "Monster Attack";

fn main() {
    let run: MagicPower= 7800;
    let energy = 5; // value 5 is bound to variable energy
    let copy_energy = energy;
    println!("Your energy is {}", energy);
    let level_title = "Level 1";
    let dead = false;
    let magic_number = 3.14f32;
    let empty = ();  // the value of the unit type ()
    let mut fuel = 34;
    fuel = 60;

    let outer = 42;
    { // start of code block
        let inner = 3.14;
        println!("block variable: {}", inner);
        let outer = 99; // shadows the first outer variable
        println!("block variable outer: {}", outer);
    } // end of code block
    println!("outer variable: {}", outer);

    let player1 = "Rob";
    let player2 = "Jane";
    let player3 = format!("{} {}", player1, player2);

    // Here starts the execution of the Game.
    // We begin with printing a welcome message:
    println!("Welcome to the Game!");
    const PI: f32 = 3.14;
    println!("The Game you are playing is called {}.", GAME_NAME);
    println!("You start with {} health points.", MAX_HEALTH);
    println!("In the Game {0} you start with {1} % health, yes you read it correctly: {1} points!", GAME_NAME, MAX_HEALTH);
    println!("You have {points} % health", points=70);
    println!("MAX_HEALTH is {:x} in hexadecimal", MAX_HEALTH); // 64
    println!("MAX_HEALTH is {:b} in binary", MAX_HEALTH);  // 1100100
    println!("pi is {:e} in floating point notation", PI); // 3.14e0

    let n1 = {
        let a = 2;
        let b = 5;
        a + b   // <-- no semicolon!
    };
    println!("n1 is: {}", n1);  // prints: n1 is 7

    let n2 = {
        let a = 2;
        let b = 5;
        a + b;
    };
    println!("n2 is: {:?}", n2);  // prints: n2 is ()

    let health = 32;
    let mut game = "Space Invaders";
    println!("address of health-value: {:p}", &health);
    // prints 0x23fba4
    println!("address of game-value: {:p}", &game); // prints 0x23fb90
    println!("game-value: {}", game); // prints "Space Invaders"

    let game2 = &game;
    println!("{:p}", game2); // prints 0x23fb90
    println!("{}", *game2); // prints "Space Invaders"
    
}
