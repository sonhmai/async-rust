use std::time::Duration;
use tokio::time::timeout;

async fn slow_task() -> &'static str {
    tokio::time::sleep(Duration::from_secs(10)).await;
    "slow task completed"
}

#[tokio::main]
async fn main() {
    let duration = Duration::from_secs(3);
    // Cancel safety ensures that when a future is canceled, any state or resources it was
    // using are handled correctly. If a task is in the middle of an operation when it’s
    // canceled, it shouldn’t leave the system in a bad state, like holding onto locks, leaving
    // files open, or partially modifying data.
    // In Rust’s async ecosystem, most operations are cancel-safe by default; they can be
    // safely interrupted without causing issues. However, it’s still a good practice to be
    // aware of how your tasks interact with external resources or state and ensure that
    // those interactions are cancel-safe.
    // if the slow_task() is canceled due to the timeout, the task itself is
    // simply stopped, and the timeout returns an error indicating the task didn’t complete
    // in time. Since tokio::time::sleep is a cancel-safe operation, there’s no risk of
    // resource leaks or inconsistent states. However, if the task involves more complex
    // operations, such as network communication or file I/O, additional care might be
    // needed to ensure that the cancellation is handled appropriately.
    let result = timeout(duration, slow_task()).await;
    match result {
        Ok(value) => println!("Task completed successfully: {}", value),
        Err(_) => println!("Task timed out"),
    }
}