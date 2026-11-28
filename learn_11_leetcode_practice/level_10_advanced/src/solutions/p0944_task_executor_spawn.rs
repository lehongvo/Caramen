//! # Problem 0944: A Task Executor with `spawn`
//!
//! - **Difficulty:** Hard
//! - **Topics:** tasks as `Arc<Task>`, a run queue over `std::sync::mpsc`, `Wake` that reschedules, `Pin<Box<dyn Future + Send>>`, spawning from inside tasks, termination
//! - **Source:** Original · [std: `task::Wake`](https://doc.rust-lang.org/std/task/trait.Wake.html) · [std: `sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/index.html)
//!
//! ## Problem
//!
//! `block_on` runs **one** future. Real runtimes run many **tasks**: each spawned future is wrapped in
//! a task, and waking a task puts it back on a run queue. Build a small single-threaded executor:
//!
//! ```text
//! type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;
//! struct Task { future: Mutex<Option<BoxFuture>>, queue: mpsc::Sender<Arc<Task>> }
//! pub struct Spawner  { queue: mpsc::Sender<Arc<Task>>, live: Arc<AtomicUsize> }   // Clone
//! pub struct Executor { queue: mpsc::Receiver<Arc<Task>>, spawner: Spawner, polls: usize }
//! ```
//!
//! - `Wake for Task`: send a clone of the task's `Arc` to the run queue (ignore a send error).
//! - `Executor::new()` (and `Default`), `Executor::spawner(&self) -> Spawner`,
//!   `Executor::spawn(&self, future)`.
//! - `Spawner::spawn(&self, future: impl Future<Output = ()> + Send + 'static)`: box the future,
//!   count it as live, and queue the new task.
//! - `Executor::run(&mut self) -> usize`: while some task is still live, take the next task from the
//!   queue (blocking with `recv` if it is empty: an outside thread may still wake a task), and poll
//!   its future with a waker made from the task itself. A finished future is removed from its task
//!   (`None`) and the live count decremented; a task found with no future (woken twice and already
//!   finished) is skipped. Returns the total number of polls made by this executor so far.
//!
//! Tasks are run in FIFO order: the first queued task runs first, and a task that wakes itself goes to
//! the back of the queue. `yield_now()` (given) makes a task yield once.
//!
//! ## Examples
//!
//! ```text
//! executor.spawn(task "a": log a0, yield, log a1)
//! executor.spawn(task "b": log b0, yield, log b1)
//! executor.run()   => 4 polls, log: [a0, b0, a1, b1]
//! ```
//!
//! ## Constraints
//!
//! - Every spawned future eventually completes (otherwise `run` would wait forever).
//!
//! ## Hints
//!
//! 1. `Waker::from(Arc::clone(&task))` builds the waker; a `Context` borrows it during the poll.
//! 2. Lock the task's `future` slot, `take()` the future out (or `as_mut()` it), poll it, and put it
//!    back only if it is still pending.
//! 3. The executor keeps its own `Spawner`, so the channel never disconnects while `run` is waiting.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a task owns its future behind a mutex and a handle to the queue. Waking it means "send
//! me to the queue". `run` is a loop over the queue: poll the task's future with a waker that points
//! back to the same task; completed tasks lower the live counter, and the loop ends when it reaches
//! zero. Because `yield_now` wakes its task during the poll, the task is re-queued behind the others,
//! which produces the FIFO interleaving.
//!
//! **Complexity:** `O(1)` per poll plus the cost of the futures themselves.
//!
//! **Rust concepts:**
//! - A `Waker` built from `Arc<Task>` keeps the task alive while some future holds the waker.
//! - `Pin<Box<dyn Future<Output = ()> + Send>>` erases each spawned future's type; `Send` lets tasks
//!   be woken from other threads, which `Wake` requires (`Task: Send + Sync`).
//! - `mpsc::Sender` is `Sync` (since Rust 1.72), so it can live inside an `Arc`-shared task.
//! - Termination needs explicit bookkeeping: an empty queue does not mean all tasks are done.
// @solution-end

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::task::{Context, Poll, Wake, Waker};

type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

struct Task {
    future: Mutex<Option<BoxFuture>>,
    queue: mpsc::Sender<Arc<Task>>,
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        // @solution-start
        let queue = self.queue.clone();
        let _ = queue.send(self);
        // @solution-end
    }
}

#[derive(Clone)]
pub struct Spawner {
    queue: mpsc::Sender<Arc<Task>>,
    live: Arc<AtomicUsize>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        // @solution-start
        self.live.fetch_add(1, Ordering::SeqCst);
        let task = Arc::new(Task {
            future: Mutex::new(Some(Box::pin(future))),
            queue: self.queue.clone(),
        });
        let _ = self.queue.send(task);
        // @solution-end
    }
}

pub struct Executor {
    queue: mpsc::Receiver<Arc<Task>>,
    spawner: Spawner,
    polls: usize,
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    pub fn new() -> Self {
        // @solution-start
        let (sender, receiver) = mpsc::channel();
        Executor {
            queue: receiver,
            spawner: Spawner {
                queue: sender,
                live: Arc::new(AtomicUsize::new(0)),
            },
            polls: 0,
        }
        // @solution-end
    }

    pub fn spawner(&self) -> Spawner {
        // @solution-start
        self.spawner.clone()
        // @solution-end
    }

    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        // @solution-start
        self.spawner.spawn(future);
        // @solution-end
    }

    pub fn run(&mut self) -> usize {
        // @solution-start
        while self.spawner.live.load(Ordering::SeqCst) > 0 {
            // The executor holds a sender itself, so `recv` cannot fail while it is alive.
            let Ok(task) = self.queue.recv() else {
                break;
            };
            let mut slot = task.future.lock().unwrap_or_else(|e| e.into_inner());
            let Some(future) = slot.as_mut() else {
                // Woken again after it finished: nothing to do.
                continue;
            };
            let waker = Waker::from(Arc::clone(&task));
            let mut cx = Context::from_waker(&waker);
            self.polls += 1;
            if future.as_mut().poll(&mut cx).is_ready() {
                *slot = None;
                self.spawner.live.fetch_sub(1, Ordering::SeqCst);
            }
        }
        self.polls
        // @solution-end
    }
}

// ---------------------------------------------------------------- given helpers

/// Pending (after waking itself) on the first poll, ready on the second.
pub struct YieldNow {
    yielded: bool,
}

pub fn yield_now() -> YieldNow {
    YieldNow { yielded: false }
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            return Poll::Ready(());
        }
        self.yielded = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    type Log = Arc<Mutex<Vec<String>>>;

    fn new_log() -> Log {
        Arc::new(Mutex::new(Vec::new()))
    }

    async fn logged(name: &'static str, yields: usize, log: Log) {
        for k in 0..=yields {
            log.lock().unwrap().push(format!("{name}{k}"));
            if k < yields {
                yield_now().await;
            }
        }
    }

    #[test]
    fn tasks_interleave_in_fifo_order() {
        let log = new_log();
        let mut executor = Executor::new();
        executor.spawn(logged("a", 1, Arc::clone(&log)));
        executor.spawn(logged("b", 1, Arc::clone(&log)));
        assert_eq!(executor.run(), 4);
        assert_eq!(*log.lock().unwrap(), ["a0", "b0", "a1", "b1"]);
    }

    #[test]
    fn uneven_tasks() {
        let log = new_log();
        let mut executor = Executor::default();
        executor.spawn(logged("x", 2, Arc::clone(&log)));
        executor.spawn(logged("y", 0, Arc::clone(&log)));
        executor.spawn(logged("z", 1, Arc::clone(&log)));
        assert_eq!(executor.run(), 6);
        assert_eq!(*log.lock().unwrap(), ["x0", "y0", "z0", "x1", "z1", "x2"]);
    }

    #[test]
    fn tasks_can_spawn_tasks() {
        let log = new_log();
        let mut executor = Executor::new();
        let spawner = executor.spawner();
        let inner_log = Arc::clone(&log);
        executor.spawn(async move {
            inner_log.lock().unwrap().push("parent".to_string());
            for i in 0..3 {
                let child_log = Arc::clone(&inner_log);
                spawner.spawn(async move {
                    child_log.lock().unwrap().push(format!("child {i}"));
                });
            }
        });
        assert_eq!(executor.run(), 4);
        assert_eq!(
            *log.lock().unwrap(),
            ["parent", "child 0", "child 1", "child 2"]
        );
    }

    /// Completes once another thread has called `wake` after setting the flag.
    struct WokenByThread {
        started: bool,
        done: Arc<Mutex<bool>>,
    }

    impl Future for WokenByThread {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if *self.done.lock().unwrap() {
                return Poll::Ready(());
            }
            if !self.started {
                self.started = true;
                let done = Arc::clone(&self.done);
                let waker = cx.waker().clone();
                thread::spawn(move || {
                    *done.lock().unwrap() = true;
                    waker.wake();
                });
            }
            Poll::Pending
        }
    }

    #[test]
    fn run_waits_for_wakeups_from_other_threads() {
        let mut executor = Executor::new();
        let finished = Arc::new(AtomicUsize::new(0));
        for _ in 0..5 {
            let finished = Arc::clone(&finished);
            executor.spawn(async move {
                WokenByThread {
                    started: false,
                    done: Arc::new(Mutex::new(false)),
                }
                .await;
                finished.fetch_add(1, Ordering::SeqCst);
            });
        }
        let polls = executor.run();
        assert_eq!(finished.load(Ordering::SeqCst), 5);
        assert!(
            polls >= 10,
            "each task needs at least two polls, got {polls}"
        );
    }

    #[test]
    fn running_again_after_more_spawns() {
        let mut executor = Executor::new();
        assert_eq!(executor.run(), 0);
        executor.spawn(async {});
        assert_eq!(executor.run(), 1);
        for _ in 0..1_000 {
            executor.spawn(async {
                yield_now().await;
            });
        }
        assert_eq!(executor.run(), 1 + 2_000);
    }
}
