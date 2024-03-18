use std::collections::HashSet;
use poker::winning_hands;

fn test() {
  let value = "J";
  let j = "JQKA".find(value).unwrap() as u8;

  let value = "Q";
  let q = "JQKA".find(value).unwrap() as u8;

  let value = "K";
  let k = "JQKA".find(value).unwrap() as u8;

  let value = "A";
  let a = "JQKA".find(value).unwrap() as u8;

  let value = "B";
  let b = "JQKAB".find(value).unwrap();

  let value = "L";
  let l = "JQKABL".find(value).unwrap();

  let value = "x";
  let x = "JQKABLx".find(value).unwrap();

  println!("J -->>> {}", j);
  println!("Q -->>> {}", q);
  println!("K -->>> {}", k);
  println!("A -->>> {}", a);
  println!("B -->>> {:?}", b);
  println!("L -->>> {:?}", l);
  println!("x -->>> {:?}", x);
}

fn main() {
  test();

  // /*
  let input = &[
    // "4H 4S AH JC 3D",
    // "4C 4D AS 5D 6C",

    // "5H 4S AH JC 3D",
    // "7C 4D AS 5D 6C",

    "7C 4D AS 5D 6C",
    "2H 3S 4H 5C 6D",

    // "5C 5D KS 6D 7C"
  ];
  println!("input -->>> {:?}", input);

  let output = winning_hands(input).into_iter().collect::<HashSet<_>>();

  println!("output -->>> {:?}", output);
  // let expected = ["4H 4S AH JC 3D"].into_iter().collect::<HashSet<_>>();

  println!("------------- for unit test -------------");
  // let suits = [72, 83, 72, 67, 68];
  // let suits = [72, 72, 72, 72, 72];
  let suits = [72, 72, 72, 72, 68];
  let is_flush = suits[1..].iter().all(|&x| x == suits[0]);
  println!("is_flush -->>> {}", is_flush);
   // */
}


