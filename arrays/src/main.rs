fn main(){
    let aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
    println!("{:?}", aliens);
    println!("The first item is: {}", aliens[0]); // Cherfer
    println!("The third item is: {}", aliens[2]); // Shirack
    let pa = &aliens;
    println!("Third item via pointer: {}", pa[2]);
    for ix in 0..aliens.len() {
        println!("Alien no {} is {}", ix, aliens[ix]);
    }
    for a in aliens.iter() {
        println!("The next alien is {}", a);
    }
    let mut numbers: Vec<i32> = Vec::new();
    let mut magic_numbers = vec![7i32, 42, 47, 45, 54];
    let rgvec: Vec<u32> = (0..7).collect();
    println!("Collected the range into: {:?}", rgvec);
    let values = vec![1, 2, 3];
    for n in values {
        println!("{}", n);
    }
    numbers.push(magic_numbers[1]);
    numbers.push(magic_numbers[4]);
    println!("{:?}", numbers); // [42, 54]
    let fifty_four = numbers.pop();// fifty_four now contains 54
    println!("{:?}", numbers); // [42]
    let slc = &magic_numbers[1..4]; // only the items 42, 47 and 45

    let location = "Middle-Earth";
    let part = &location[7..12];
    println!("{}", part); // Earth

    let magician = "Merlin";
    let mut chars: Vec<char> = magician.chars().collect();
    chars.sort();
    for c in chars.iter() {
        print!("{} ", c);
    }

}
