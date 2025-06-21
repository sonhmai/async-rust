/**
Polling directly is not the most efficient way as our executor will be busy polling
futures that are not ready. To explain how we can prevent busy polling, we will move
onto waking futures remotely.
**/
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::sync::{Arc, Mutex};
use std::future::Future;
use tokio::sync::mpsc;
use tokio::task;

/// simulate external calls with channels
struct MyFuture {
    state: Arc<Mutex<MyFutureState>>,
}

impl MyFuture {
    fn new() -> (Self, Arc<Mutex<MyFutureState>>) {
        let state = Arc::new(Mutex::new(MyFutureState {
            data: None,
            waker: None,
        }));
        (
            MyFuture {
                state: state.clone(),
            },
            // also return a ref to state so we can access waker outside of future
            state,
        )
    }
}

struct MyFutureState {
    data: Option<Vec<u8>>,
    waker: Option<Waker>,
}

impl Future for MyFuture {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling the future");
        let mut state = self.state.lock().unwrap();
        if state.data.is_some() {
            let data = state.data.take().unwrap();
            Poll::Ready(String::from_utf8(data).unwrap())
        } else {
            // we pass the waker into the state so we can wake the future from outside of the future
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let (my_future, state) = MyFuture::new();
    let (tx, mut rx) = mpsc::channel::<()>(1);
    let task_handle = task::spawn(async {
        my_future.await
    });
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    
    println!("spawning trigger task");
    let trigger_task = task::spawn(async move {
        // once our trigger task receives the message in the channel, it gets
        // the state of our future, and populates the data. We then check to see if the waker is
        // present. Once we get hold of the waker, we wake the future.
        rx.recv().await;
        let mut state = state.lock().unwrap();
        state.data = Some(b"Hello from the outside".to_vec());
        loop {
            if let Some(waker) = state.waker.take() {
                waker.wake();
                break;
            }
        }
    });
    tx.send(()).await.unwrap();

    let outcome = task_handle.await.unwrap();
    println!("Task completed with outcome: {}", outcome);
    trigger_task.await.unwrap();
    
/*
Output:
    Polling the future
    spawning trigger task
    Polling the future
    Task completed with outcome: Hello from the outside
        
Polling only happens once on the initial setup and then happens
one more time when we wake the future with the data. Async runtimes set up
efficient ways to listen to OS events so they do not have to blindly poll futures.

For instance, Tokio has an event loop that listens to OS events and then handles them
so the event wakes up the right task. 

We will be calling the waker directly in the poll function. 
This is because we want to reduce the amount of unnecessary code when
focusing on other areas of async programming.
*/
    
}