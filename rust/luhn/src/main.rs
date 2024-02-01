
fn main() {

}

fn c1() {
  let numbers = vec![1, 2, 3, 4, 5];

  let result: Result<i32, &'static str> = numbers.into_iter().try_fold(0, |acc, x| {
    if x % 2 == 0 {
      Ok(acc + x)
    } else {
      Err("Odd number encountered")
    }
  });

  match result {
    Ok(sum) => println!("Sum of even numbers: {}", sum),
    Err(err) => println!("Error: {}", err),
  }
}
