use tokio::time;
use std::time::{Instant, Duration};

async fn task_one() {
  println!("Task one starting");
  time::sleep(Duration::from_secs(2)).await;
  println!("Task one completed");
}

async fn task_two() {
  println!("Task two starting");
  time::sleep(Duration::from_secs(3)).await;
  println!("Task two completed");
}

#[tokio::main]
async fn main() {
  let start_time = Instant::now();

  let result_one = task_one();
  let result_two = task_two();

  tokio::join!(result_one, result_two);

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
}


