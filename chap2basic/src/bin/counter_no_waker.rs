/**
Futures need the Waker::wake() function so it can be called when the
future should be polled again. The process takes the following steps:

1. The poll function for a future is called, and the result is that the future needs to
wait for an async operation to complete before the future is able to return a value.

2. The future registers its interest in being notified of the operation’s completion by
calling a method that references the waker.

3. The executor takes note of the interest in the future’s operation and stores the
waker in a queue.

4. At some later time, the operation completes, and the executor is notified. The
executor retrieves the wakers from the queue and calls wake_by_ref on each one,
waking up the futures.

5. The wake_by_ref function signals the associated task that should be scheduled
for execution. The way this is done can vary depending on the runtime.

6. When the future is executed, the executor will call the poll method of the
future again, and the future will determine whether the operation has completed,
returning a value if completion is achieved.
**/
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