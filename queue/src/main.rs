mod lib;

use async_task::Task;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

fn main() {
    let one = CounterFuture { count: 0 };
    let two = CounterFuture { count: 0 };

    let t_one = spawn_task(one);
    let t_two = spawn_task(two);
}

fn spawn_task<F, T>(future: F) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    todo!()
}

struct CounterFuture {
    count: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("polling with result: {}", self.count);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if self.count >= 3 {
            return Poll::Ready(self.count);
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}