use orcastrate::worker::Worker;

use orcastrate_macro::orca_task;
use tokio::time::Duration;

#[orca_task]
async fn my_async_task(url: String, count: i32) -> Result<String, String> {
    // No TaskContext
    println!("Running task: url={}, count={}", url, count);
    tokio::time::sleep(Duration::from_secs(1)).await;
    if count < 0 {
        Err("Count cannot be negative".to_string())
    } else {
        Ok(format!("Processed {} - Count: {}", url, count))
    }
}

#[tokio::main]
async fn main() {
    let worker = Worker::new("redis://localhost:6379".to_string())
        .run()
        .await;
    let async_task = my_async_task::register(worker.clone());
    let res = async_task
        .submit("https://example.com".to_string(), 10)
        .await;
    println!("Res: {:?}", res);

    println!("Main loop running. Tasks are executing asynchronously...");
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
