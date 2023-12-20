fn main() {
  for i in 0..5 {
    println!("{}", i);
  }

  let list = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
  println!("{}", list[2 : 5]);
}