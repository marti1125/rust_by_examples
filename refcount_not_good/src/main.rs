struct Alien {
    name: String,
    n_tentacles: u8
}

struct Tentacle {
    poison: u8,
    owner: Alien
}

fn main() {
    let dhark = Alien { name: "Dharkalen".to_string(), n_tentacles: 7 };
    // defining dhark's tentacles:
    for i in 1u8..dhark.n_tentacles {
        Tentacle { poison: i * 3, owner: dhark }; // <- error!
    }
}
