fn main() {
    let mut strength = 26;
    println!("My tripled strength equals {}",triples(strength)); // 78
    println!("My strength is still {}", strength); // 26
    strength = triples(strength);
    println!("My strength is now {}", strength); // 78
    strength = 78;
    let triples = |n| { 3 * n };
    strength = again(triples, strength);
    println!("My strength is now {}", strength); // 702
    let m: i32 = 42;
    let print_add_move = move |s| {
        println!("m is {}", m);
        m + s
    };
    let res = print_add_move(strength); // strength == 702
    assert_eq!(res, 744); // 42 + 702
}
