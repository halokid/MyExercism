
use std::time::Duration;
use tokio::time;

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
    let result_one = task_one();
    let result_two = task_two();

    tokio::join!(result_one, result_two);
}
