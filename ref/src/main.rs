fn main() {
  let n = 42;
  match n {
      ref r => println!("Got a reference to {}", r),
  }
  let mut m = 42;
  match m {
      ref mut mr => {
        println!("Got a mutable reference to {}", mr);
        *mr = 43;
      },
  }
  println!("m has changed to {}!", m);
}
