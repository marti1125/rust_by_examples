use std::cmp::Ordering::{self, Equal, Less, Greater};

fn cmp(a: i32, b: i32) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

fn main() {
    let x = 5;
    let y = 10;

    let ordering = cmp(x, y); // ordering: Ordering

    if ordering == Less { println!("less"); }
    else if ordering == Greater { println!("greater"); }
    else if ordering == Equal { println!("equal"); }
}
