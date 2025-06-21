use chap2basic::{CounterFuture, FastCounterFuture};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    let max_count = if args.len() > 1 {
        args[1].parse::<u32>().unwrap_or(3)
    } else {
        3
    };
    
    let use_fast = args.len() > 2 && args[2] == "fast";
    
    println!("Running advanced counter with max_count: {}, fast: {}", max_count, use_fast);
    
    if use_fast {
        let counter = FastCounterFuture::new(max_count);
        let result = counter.await;
        println!("Fast counter completed with result: {}", result);
    } else {
        let counter = CounterFuture::new(max_count);
        let result = counter.await;
        println!("Regular counter completed with result: {}", result);
    }
}