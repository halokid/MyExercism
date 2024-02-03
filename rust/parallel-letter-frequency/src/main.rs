use std::collections::HashMap;
use tokio::time;
use std::time::{Instant, Duration};

async fn task_one() -> i32 {
  println!("Task one starting");
  time::sleep(Duration::from_secs(2)).await;
  println!("Task one completed");

  8
}

async fn task_two() -> i32 {
  println!("Task two starting");
  time::sleep(Duration::from_secs(3)).await;
  println!("Task two completed");

  9
}


async fn worker_function(worker_id: u32) {
  // Your asynchronous worker function implementation here
  println!("Worker {} is running", worker_id);
}

#[tokio::main]
async fn main() {
  let start_time = Instant::now();

  let result_one = task_one();
  let result_two = task_two();

  // tokio::join!(result_one, result_two);
  // TODO: this way is async!!!
  let (a, b) = tokio::join!(result_one, result_two);

  // TODO: below way is not async!!!!!
  // let (a) = tokio::join!(result_one);
  // let (b) = tokio::join!(result_two);
  println!("a: {:?}, b: {:?}", a, b);

  let end_time = Instant::now();

  // Calculate the elapsed time
  let elapsed_time = end_time - start_time;

  // Print the elapsed time in milliseconds
  println!("Elapsed time: {} milliseconds", elapsed_time.as_millis());

  // Or you can print in seconds and nanoseconds
  println!(
    "Elapsed time: {}.{:03} seconds",
    elapsed_time.as_secs(),
    elapsed_time.subsec_nanos() / 1_000_000
  );

  // TODO: ---- same function in parallel for 3 workers
  let worker1 = worker_function(1);
  let worker2 = worker_function(2);
  let worker3 = worker_function(3);

  tokio::join!(worker1, worker2, worker3);
}

fn check_key() {
  let mut data = HashMap::new();

  data.insert("key1", 42);
  data.insert("key2", 100);

  let key_to_check = "key1";

  if data.contains_key(key_to_check) {
    println!("The key '{}' exists in the HashMap", key_to_check);
  } else {
    println!("The key '{}' does not exist in the HashMap", key_to_check);
  }
}






