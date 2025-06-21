use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::task::JoinHandle;

struct CounterWithoutWaker {
    count: u32,
    max_count: u32,
}

impl CounterWithoutWaker {
    pub fn new(max_count: u32) -> Self {
        Self {
            count: 0,
            max_count,
        }
    }
}

impl Future for CounterWithoutWaker {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("fast polling with result: {}", self.count);
        std::thread::sleep(Duration::from_millis(100));
        if self.count < self.max_count {
            // removing waker here means Future is not waken hence no progress will be made.
            // cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

#[tokio::main]
async fn main() {
    let counter_one = CounterWithoutWaker::new(5);
    let counter_two = CounterWithoutWaker::new(5);

    let handle_one: JoinHandle<u32> = tokio::task::spawn(counter_one);
    let handle_two: JoinHandle<u32> = tokio::task::spawn(counter_two);

    let (result_one, result_two) = tokio::join!(handle_one, handle_two);

    println!("Counter one result: {:?}", result_one);
    println!("Counter two result: {:?}", result_two);
}