# Problem 0900: Thread Pool

- **Difficulty:** Hard
- **Topics:** a fixed set of worker threads sharing one job queue (`Arc<Mutex<Receiver<Job>>>`),
  boxed `FnOnce` jobs, graceful shutdown in `Drop`, `thread::Builder` names,
  `catch_unwind` to survive panicking jobs
- **Source:** Original · [The Rust Book 21.2](https://doc.rust-lang.org/book/ch21-02-multithreaded.html) ·
  [The Rust Book 21.3](https://doc.rust-lang.org/book/ch21-03-graceful-shutdown-and-cleanup.html)

## Problem

Spawning a thread per task is expensive when there are many small tasks. A **thread pool**
starts a fixed number of workers once; tasks are sent to a shared queue, and whichever worker
is free takes the next one. Build a pool like the one at the end of The Rust Book, with two extra
requirements.

```text
pub struct ThreadPool { /* your fields */ }
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool;      // panics if size == 0
    pub fn size(&self) -> usize;                // number of workers
    pub fn execute<F>(&self, job: F) where F: FnOnce() + Send + 'static;
}
impl Drop for ThreadPool { /* finish every queued job, then stop and join all workers */ }
```

Requirements:

1. `new(0)` panics with a message containing `"at least one worker"`.
2. Jobs run concurrently on up to `size` workers.
3. Dropping the pool waits until **every job already submitted** has run, then joins all
   workers (so tests can check results right after `drop(pool)`).
4. Worker threads are named `"pool-worker-0"`, `"pool-worker-1"`, ...
5. A job that panics must not kill its worker: the worker keeps serving later jobs.

## Examples

```text
let pool = ThreadPool::new(4);
for i in 0..8 { pool.execute(move || println!("job {i}")); }
drop(pool);   // returns after all 8 jobs have printed
```

## Constraints

- `1 <= size <= 32`; up to `10^4` jobs.
- (Panicking jobs print a message on the test output; that is expected.)

## Hints

1. Use one `mpsc::channel::<Job>()` with `type Job = Box<dyn FnOnce() + Send + 'static>;`.
   The single receiver is shared by the workers as `Arc<Mutex<Receiver<Job>>>`.
2. A worker loops: lock the receiver, `recv()` one job, **unlock**, run the job. If the lock were
   still held while the job runs, the other workers could not take jobs and nothing would run in
   parallel.
3. In `Drop`, drop the `Sender` first (store it as `Option<Sender<Job>>` and `take()` it): the
   workers' `recv()` then returns `Err` once the queue is empty, and their loops end. Then join
   them.
4. `std::panic::catch_unwind(AssertUnwindSafe(job))` runs a job and turns a panic into an `Err`.

## Starter code

Write your code in [`src/practice/p0900_thread_pool.rs`](../src/practice/p0900_thread_pool.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0900
```

```rust
use std::panic::{self, AssertUnwindSafe};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::{self, JoinHandle};

/// A fixed-size pool of worker threads.
pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        todo!("Write your solution here")
    }

    pub fn size(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!("Write your solution here")
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        todo!("Write your solution here")
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
    use std::sync::Barrier;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn example_all_jobs_finish_before_drop_returns() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.size(), 4);
        let done = Arc::new(AtomicUsize::new(0));
        for _ in 0..100 {
            let done = Arc::clone(&done);
            pool.execute(move || {
                done.fetch_add(1, Ordering::SeqCst);
            });
        }
        drop(pool);
        assert_eq!(done.load(Ordering::SeqCst), 100);
    }

    #[test]
    #[should_panic(expected = "at least one worker")]
    fn zero_workers_is_rejected() {
        let _ = ThreadPool::new(0);
    }

    #[test]
    fn results_through_a_channel() {
        let pool = ThreadPool::new(3);
        let (tx, rx) = mpsc::channel();
        for i in 0..1000_u64 {
            let tx = tx.clone();
            pool.execute(move || tx.send(i * i).unwrap());
        }
        drop(tx);
        drop(pool);
        assert_eq!(rx.iter().sum::<u64>(), (0..1000).map(|i| i * i).sum());
    }

    #[test]
    fn workers_have_names() {
        let pool = ThreadPool::new(2);
        let (tx, rx) = mpsc::channel();
        for _ in 0..20 {
            let tx = tx.clone();
            pool.execute(move || tx.send(thread::current().name().map(String::from)).unwrap());
        }
        drop(tx);
        drop(pool);
        let names: Vec<Option<String>> = rx.iter().collect();
        assert_eq!(names.len(), 20);
        assert!(
            names
                .iter()
                .all(|n| matches!(n.as_deref(), Some("pool-worker-0" | "pool-worker-1")))
        );
    }

    #[test]
    fn jobs_run_in_parallel() {
        // Four jobs wait for each other. They can only all finish if four workers run them at the
        // same time; a pool that holds the queue lock while running a job would hang here.
        let pool = ThreadPool::new(4);
        let barrier = Arc::new(Barrier::new(4));
        let done = Arc::new(AtomicUsize::new(0));
        for _ in 0..4 {
            let (barrier, done) = (Arc::clone(&barrier), Arc::clone(&done));
            pool.execute(move || {
                barrier.wait();
                done.fetch_add(1, Ordering::SeqCst);
            });
        }
        drop(pool);
        assert_eq!(done.load(Ordering::SeqCst), 4);
    }

    #[test]
    fn a_panicking_job_does_not_kill_its_worker() {
        let pool = ThreadPool::new(1);
        let done = Arc::new(AtomicUsize::new(0));
        pool.execute(|| panic!("this job fails"));
        for _ in 0..10 {
            let done = Arc::clone(&done);
            pool.execute(move || {
                done.fetch_add(1, Ordering::SeqCst);
            });
        }
        drop(pool);
        assert_eq!(done.load(Ordering::SeqCst), 10);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `new` creates the channel, wraps the receiver in `Arc<Mutex<_>>` and spawns `size`
named workers with `thread::Builder`. Each worker repeatedly takes the lock only for `recv()`,
runs the job inside `catch_unwind`, and exits when `recv` reports a closed, empty channel.
`execute` boxes the closure and sends it. `Drop` drops the sender and joins every worker; queued
jobs are still delivered before `recv` reports the disconnection, so no submitted job is lost.

**Rust concepts:**
- `Box<dyn FnOnce() + Send + 'static>` erases the type of each closure so jobs of different types
  fit in one channel; calling `job()` on the box consumes it.
- In `let job = receiver.lock().unwrap().recv();` the temporary guard is dropped at the end of
  the statement, so the lock is *not* held while the job runs (a `while let` on the same
  expression would hold it for the whole loop body).
- `Option::take` moves the sender out of `&mut self` inside `drop`.
- `AssertUnwindSafe` tells the compiler we accept the risk of observing state left behind by a
  panicking job; the pool itself holds no state the job could corrupt.

```rust
use std::panic::{self, AssertUnwindSafe};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

/// A fixed-size pool of worker threads.
pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: Option<Sender<Job>>,
}

fn worker_loop(receiver: Arc<Mutex<Receiver<Job>>>) {
    loop {
        // The guard is a temporary of this statement: the lock is released before the job runs.
        let message = receiver
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .recv();
        match message {
            Ok(job) => {
                // A panicking job is reported on stderr by the panic hook; the worker survives.
                let _ = panic::catch_unwind(AssertUnwindSafe(job));
            }
            Err(_) => return, // the pool was dropped and the queue is empty
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "a thread pool needs at least one worker");
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..size)
            .map(|id| {
                let receiver = Arc::clone(&receiver);
                thread::Builder::new()
                    .name(format!("pool-worker-{id}"))
                    .spawn(move || worker_loop(receiver))
                    .expect("the operating system refused to create a worker thread")
            })
            .collect();
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn size(&self) -> usize {
        self.workers.len()
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(sender) = &self.sender {
            // Workers only stop after the sender is dropped, so the receiver is alive here.
            let _ = sender.send(Box::new(job));
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // close the queue: workers exit once it is empty
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }
}
```

Full tested solution: [`src/solutions/p0900_thread_pool.rs`](../src/solutions/p0900_thread_pool.rs)

</details>
