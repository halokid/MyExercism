use std::collections::HashSet;

fn main() {
  let s = "hello world";
  let char_vec: Vec<char> = s.chars().collect();
  println!("{:?}", char_vec);

  println!("---------------------------------");

  let inputs = ["hello", "world", "zombies", "pants"];
  let res: HashSet<&str> = inputs.iter().filter(|item| {
    println!("item -->>> {}", item);
    let item_lower = item.to_lowercase();
    // true
    item.len() > 5
  }).copied().collect();
  println!("res -->>> {:?}", res);
}

