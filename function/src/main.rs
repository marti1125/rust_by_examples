// Line comments are anything after '//' and extend to the end of the line.

fn main() {
    print_number(5);
    print_sum(5, 6);
    hello("Willy");
}

// If you have a long explanation for something, you can put line comments next
// to each other. Put a space between the // and your comment so that it's
// more readable.

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn foo(x: i32) -> i32 {
    if x < 5 {
        x
    } else {
        x + 1
    }
}

fn diverges() -> ! {
    panic!("This function never returns!");
}
