fn main() {
    let n = 42u32;
    let n2 = n; // a copy of the value from n to n2
    life(n);
    println!("{}", m);  // error: unresolved name `m`.
    println!("{}", o);  // error: unresolved name `o`.
}

fn life(m: u32) -> u32 {
    let o = m;
    o
}
