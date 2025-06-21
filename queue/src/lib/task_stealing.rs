use std::sync::LazyLock;
use async_task::Runnable;
/// Why use flume over std lib channels? To be able to send channels over to other threads.
use flume::{Sender, Receiver};

// LazyLock: a thread-safe wrapper for singleton - only initialized once, on first access. 
// (Sender<Runnable>, Receiver<Runnable>) - tuple for both channel ends of Runnable.
// || flume::unbounded::<Runnable>() -> closure of creating unbounded flume channel of Runnable.
static HIGH_CHANNEL: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
    LazyLock::new(|| flume::unbounded::<Runnable>());
static LOW_CHANNEL: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
    LazyLock::new(|| flume::unbounded::<Runnable>());

/// Loop:
/// 1. Check the high channel for a message.
/// 2. If no message, check the low channel.
/// 3. If not message, wait 100ms before next iteration.
static HIGH_QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
    todo!()
});
