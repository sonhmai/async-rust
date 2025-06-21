use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

pub struct CounterFuture {
    count: u32,
    max_count: u32,
}

impl CounterFuture {
    pub fn new(max_count: u32) -> Self {
        Self {
            count: 0,
            max_count,
        }
    }
}

impl Future for CounterFuture {
    type Output = u32;
    
    /// poll function is not async. 
    /// - because an async poll function would return a circular dependency, as you would be
    /// - sending a future to be polled in order to resolve a future being polled.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("polling with result: {}", self.count);
        std::thread::sleep(Duration::from_secs(1));
        if self.count < self.max_count {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

pub struct FastCounterFuture {
    count: u32,
    max_count: u32,
}

impl FastCounterFuture {
    pub fn new(max_count: u32) -> Self {
        Self {
            count: 0,
            max_count,
        }
    }
}

impl Future for FastCounterFuture {
    type Output = u32;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("fast polling with result: {}", self.count);
        std::thread::sleep(Duration::from_millis(100));
        if self.count < self.max_count {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}