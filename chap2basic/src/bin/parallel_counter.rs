use chap2basic::{CounterFuture, FastCounterFuture};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    println!("Running parallel counters with different speeds...");
    
    let regular_counter1 = CounterFuture::new(3);
    let regular_counter2 = CounterFuture::new(4);
    let fast_counter1 = FastCounterFuture::new(8);
    let fast_counter2 = FastCounterFuture::new(6);
    
    let handle1: JoinHandle<u32> = tokio::task::spawn(async move {
        println!("Starting regular counter 1");
        regular_counter1.await
    });
    
    let handle2: JoinHandle<u32> = tokio::task::spawn(async move {
        println!("Starting regular counter 2");
        regular_counter2.await
    });
    
    let handle3: JoinHandle<u32> = tokio::task::spawn(async move {
        println!("Starting fast counter 1");
        fast_counter1.await
    });
    
    let handle4: JoinHandle<u32> = tokio::task::spawn(async move {
        println!("Starting fast counter 2");
        fast_counter2.await
    });
    
    let (result1, result2, result3, result4) = tokio::join!(handle1, handle2, handle3, handle4);
    
    println!("Results:");
    println!("Regular counter 1: {:?}", result1);
    println!("Regular counter 2: {:?}", result2);
    println!("Fast counter 1: {:?}", result3);
    println!("Fast counter 2: {:?}", result4);
}