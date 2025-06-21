use chap2basic::CounterFuture;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let counter_one = CounterFuture::new(5);
    let counter_two = CounterFuture::new(5);
    
    let handle_one: JoinHandle<u32> = tokio::task::spawn(counter_one);
    let handle_two: JoinHandle<u32> = tokio::task::spawn(counter_two);
    
    let (result_one, result_two) = tokio::join!(handle_one, handle_two);
    
    println!("Counter one result: {:?}", result_one);
    println!("Counter two result: {:?}", result_two);
}