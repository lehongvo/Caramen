# Problem 0943: A Oneshot Channel Future

- **Difficulty:** Hard
- **Topics:** a future woken by another thread, storing a `Waker` in shared state, `Arc<Mutex<..>>`, `Drop` signalling cancellation, `Send` futures
- **Source:** Original · [std: `task::Waker`](https://doc.rust-lang.org/std/task/struct.Waker.html) · [The Rust Book 17.2: Applying Concurrency with Async](https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html)

## Problem

A **oneshot channel** carries a single value from a `Sender` to a `Receiver`, where the receiver
is a future. It is the basic building block of "await the result of some work done elsewhere".

```text
struct Shared<T> { value: Option<T>, waker: Option<Waker>, sender_alive: bool, receiver_alive: bool }
pub struct Sender<T>   { shared: Arc<Mutex<Shared<T>>> }
pub struct Receiver<T> { shared: Arc<Mutex<Shared<T>>> }
```

Implement:

- `oneshot<T>() -> (Sender<T>, Receiver<T>)`.
- `Sender::send(self, value: T) -> Result<(), T>`: if the receiver was dropped, give the value
  back as `Err(value)`; otherwise store it and **wake** the stored waker (if any).
- `Sender::is_closed(&self) -> bool`: `true` once the receiver has been dropped.
- `Drop for Sender<T>`: mark the sender as gone and wake the receiver, so a receiver waiting for a
  value that will never come can finish with an error.
- `Future for Receiver<T>` with `Output = Result<T, Canceled>`: a stored value gives `Ok(value)`
  (even if the sender is already gone); no value and no sender gives `Err(Canceled)`; otherwise
  store a clone of the current waker (replacing an older one) and return `Pending`.
- `Drop for Receiver<T>`: mark the receiver as gone.

A `block_on` executor is given.

## Examples

```text
let (tx, rx) = oneshot();
thread::spawn(move || tx.send(42));
block_on(rx)                    => Ok(42)

let (tx, rx) = oneshot::<u8>();
drop(tx);
block_on(rx)                    => Err(Canceled)

let (tx, rx) = oneshot();
drop(rx);
tx.send("late")                 => Err("late")
```

## Constraints

- Tests must be deterministic: real wake-ups only, no sleeping.

## Hints

1. Take the waker out of the shared state, **release the lock**, then call `waker.wake()`.
2. `send` consumes the sender, so `Drop for Sender` also runs right after a successful send; that
   is fine because the receiver checks for a value first.
3. A poisoned mutex can be recovered with `lock().unwrap_or_else(|e| e.into_inner())`.

## Starter code

Write your code in [`src/practice/p0943_oneshot_channel_future.rs`](../src/practice/p0943_oneshot_channel_future.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0943
```

```rust
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Canceled;

struct Shared<T> {
    value: Option<T>,
    waker: Option<Waker>,
    sender_alive: bool,
    receiver_alive: bool,
}

pub struct Sender<T> {
    shared: Arc<Mutex<Shared<T>>>,
}

pub struct Receiver<T> {
    shared: Arc<Mutex<Shared<T>>>,
}

pub fn oneshot<T>() -> (Sender<T>, Receiver<T>) {
    todo!("Write your solution here")
}

impl<T> Sender<T> {
    pub fn send(self, value: T) -> Result<(), T> {
        todo!("Write your solution here")
    }

    pub fn is_closed(&self) -> bool {
        todo!("Write your solution here")
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // TODO: mark the sender as gone and wake the receiver (never panic inside `drop`).
    }
}

impl<T> Future for Receiver<T> {
    type Output = Result<T, Canceled>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("Write your solution here")
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // TODO: mark the receiver as gone (never panic inside `drop`).
    }
}

// ---------------------------------------------------------------- given helpers

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Runs a future to completion on the current thread (see problem 0939).
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
            return output;
        }
        thread::park();
    }
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn value_sent_before_awaiting() {
        let (tx, rx) = oneshot();
        assert_eq!(tx.send("ready"), Ok(()));
        assert_eq!(block_on(rx), Ok("ready"));
    }

    #[test]
    fn value_sent_from_another_thread() {
        let (tx, rx) = oneshot();
        let worker = thread::spawn(move || tx.send(42).is_ok());
        assert_eq!(block_on(rx), Ok(42));
        assert!(worker.join().unwrap());
    }

    #[test]
    fn sender_waits_until_the_receiver_is_pending() {
        // The worker only sends after the receiver has been polled once and registered its waker.
        let (tx, rx) = oneshot::<String>();
        let (go_tx, go_rx) = mpsc::channel::<()>();
        let worker = thread::spawn(move || {
            go_rx.recv().unwrap();
            tx.send("woken".to_string()).unwrap();
        });
        let result = block_on(async move {
            let mut rx = rx;
            let mut first_poll = true;
            std::future::poll_fn(|cx| {
                let poll = Pin::new(&mut rx).poll(cx);
                if first_poll {
                    first_poll = false;
                    assert!(poll.is_pending());
                    go_tx.send(()).unwrap();
                }
                poll
            })
            .await
        });
        assert_eq!(result, Ok("woken".to_string()));
        worker.join().unwrap();
    }

    #[test]
    fn dropped_sender_cancels() {
        let (tx, rx) = oneshot::<u8>();
        drop(tx);
        assert_eq!(block_on(rx), Err(Canceled));
        let (tx, rx) = oneshot::<u8>();
        let worker = thread::spawn(move || drop(tx));
        assert_eq!(block_on(rx), Err(Canceled));
        worker.join().unwrap();
    }

    #[test]
    fn dropped_receiver_returns_the_value() {
        let (tx, rx) = oneshot();
        assert!(!tx.is_closed());
        drop(rx);
        assert!(tx.is_closed());
        assert_eq!(tx.send("late"), Err("late"));
    }

    #[test]
    fn many_channels_across_threads() {
        let receivers: Vec<Receiver<usize>> = (0..50)
            .map(|i| {
                let (tx, rx) = oneshot();
                thread::spawn(move || tx.send(i * i).unwrap());
                rx
            })
            .collect();
        let total: usize = receivers.into_iter().map(|rx| block_on(rx).unwrap()).sum();
        assert_eq!(total, (0..50).map(|i| i * i).sum());
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** both halves share one mutex-protected state. The receiver's `poll` checks, in order, for
a value, for a dead sender, and otherwise registers its waker. Any event that can change that
answer (a value arriving, the sender disappearing) takes the waker out and wakes it after the
lock is released, so the woken task can immediately lock the state again.

**Rust concepts:**
- The `Waker` stored by `poll` is the bridge between the executor and the other thread; it must
  be refreshed on every pending poll because a future may move between tasks.
- `Drop` is how cancellation is communicated: dropping either half changes what the other sees.
- `Receiver<T>` is `Send` when `T: Send`, so it can be awaited on another thread's executor.

```rust
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Canceled;

struct Shared<T> {
    value: Option<T>,
    waker: Option<Waker>,
    sender_alive: bool,
    receiver_alive: bool,
}

pub struct Sender<T> {
    shared: Arc<Mutex<Shared<T>>>,
}

pub struct Receiver<T> {
    shared: Arc<Mutex<Shared<T>>>,
}

fn lock<T>(shared: &Mutex<Shared<T>>) -> MutexGuard<'_, Shared<T>> {
    shared.lock().unwrap_or_else(|e| e.into_inner())
}

pub fn oneshot<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Mutex::new(Shared {
        value: None,
        waker: None,
        sender_alive: true,
        receiver_alive: true,
    }));
    (
        Sender {
            shared: Arc::clone(&shared),
        },
        Receiver { shared },
    )
}

impl<T> Sender<T> {
    pub fn send(self, value: T) -> Result<(), T> {
        let waker = {
            let mut state = lock(&self.shared);
            if !state.receiver_alive {
                return Err(value);
            }
            state.value = Some(value);
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        !lock(&self.shared).receiver_alive
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let waker = {
            let mut state = lock(&self.shared);
            state.sender_alive = false;
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

impl<T> Future for Receiver<T> {
    type Output = Result<T, Canceled>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = lock(&self.shared);
        if let Some(value) = state.value.take() {
            return Poll::Ready(Ok(value));
        }
        if !state.sender_alive {
            return Poll::Ready(Err(Canceled));
        }
        state.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        lock(&self.shared).receiver_alive = false;
    }
}

// ---------------------------------------------------------------- given helpers

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Runs a future to completion on the current thread (see problem 0939).
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
            return output;
        }
        thread::park();
    }
}
```

Full tested solution: [`src/solutions/p0943_oneshot_channel_future.rs`](../src/solutions/p0943_oneshot_channel_future.rs)

</details>
