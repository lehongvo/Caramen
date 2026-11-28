# Problem 0875: Shared Counter with Arc and Mutex

- **Difficulty:** Easy
- **Topics:** shared mutable state with `Arc<Mutex<T>>`, `lock()` and `MutexGuard`,
  getting the value back with `Arc::into_inner` and `Mutex::into_inner`
- **Source:** Original · [The Rust Book 16.3](https://doc.rust-lang.org/book/ch16-03-shared-state.html) ·
  [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) ·
  [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)

## Problem

Several threads need to update the same value. `Mutex<T>` makes sure only one thread at a time
can touch the value, and `Arc<T>` (atomically reference-counted pointer) lets several threads
own the mutex.

Implement:

1. `parallel_increments(threads: usize, per_thread: usize) -> usize`: spawn `threads` threads
   that each add `1` to one shared counter, `per_thread` times (locking once per increment).
   Return the final counter after all threads have finished.
2. `parallel_push(threads: usize, per_thread: usize) -> Vec<usize>`: spawn `threads` threads;
   thread `t` pushes the value `t` into one shared `Vec<usize>`, `per_thread` times. When all
   threads are done, take the vector **out of** the `Arc<Mutex<_>>` (without cloning it), sort
   it and return it.

## Examples

```text
parallel_increments(4, 1000) -> 4000
parallel_push(3, 2)          -> [0, 0, 1, 1, 2, 2]
parallel_push(0, 5)          -> []
```

## Constraints

- `0 <= threads <= 64`, `0 <= per_thread <= 10^4`

## Hints

1. `let counter = Arc::new(Mutex::new(0));` then, for each thread, `let counter =
   Arc::clone(&counter);` and move the clone into the thread.
2. `*counter.lock().unwrap() += 1;` locks, updates and unlocks (the guard is dropped at the end
   of the statement).
3. After joining every thread, the original `Arc` is the last one: `Arc::into_inner(arc)` returns
   `Some(mutex)`, and `mutex.into_inner()` returns the value.

## Starter code

Write your code in [`src/practice/p0875_shared_counter_with_arc_and_mutex.rs`](../src/practice/p0875_shared_counter_with_arc_and_mutex.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0875
```

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_increments(threads: usize, per_thread: usize) -> usize {
    todo!("Write your solution here")
}

pub fn parallel_push(threads: usize, per_thread: usize) -> Vec<usize> {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_increments() {
        assert_eq!(parallel_increments(4, 1000), 4000);
    }

    #[test]
    fn example_push() {
        assert_eq!(parallel_push(3, 2), vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn example_no_threads() {
        assert!(parallel_push(0, 5).is_empty());
        assert_eq!(parallel_increments(0, 5), 0);
    }

    #[test]
    fn many_threads_no_lost_update() {
        assert_eq!(parallel_increments(64, 10_000), 640_000);
    }

    #[test]
    fn push_counts_per_thread() {
        let v = parallel_push(16, 500);
        assert_eq!(v.len(), 8000);
        for t in 0..16 {
            assert_eq!(v.iter().filter(|&&x| x == t).count(), 500);
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** create the shared value, give each thread an `Arc` clone, lock for every update, join
all threads, then unwrap the `Arc` and the `Mutex` to get the value. The final result is the
same on every run: the mutex serializes the updates, and joining waits for all of them.

**Rust concepts:**
- `Rc` is not thread-safe (its counter is not atomic), so it is not `Send`; `Arc` is the
  thread-safe version.
- `lock()` returns `LockResult<MutexGuard<T>>`; the `Err` case (poisoning) only happens if a
  thread panicked while holding the lock (problem 0877 handles it). Here no thread can panic,
  which is why `expect` is acceptable.
- `Arc::into_inner` (stable since 1.70) returns the inner value only when called on the last
  strong reference; it never clones. Joining every thread first guarantees that.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_increments(threads: usize, per_thread: usize) -> usize {
    let counter = Arc::new(Mutex::new(0_usize));
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..per_thread {
                    *counter
                        .lock()
                        .expect("no thread panics while holding the lock") += 1;
                }
            })
        })
        .collect();
    for h in handles {
        h.join().expect("workers cannot panic");
    }
    // Edition 2024 drops the guard of this tail expression before `counter` itself.
    *counter
        .lock()
        .expect("no thread panics while holding the lock")
}

pub fn parallel_push(threads: usize, per_thread: usize) -> Vec<usize> {
    let shared = Arc::new(Mutex::new(Vec::with_capacity(threads * per_thread)));
    let handles: Vec<_> = (0..threads)
        .map(|t| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || {
                for _ in 0..per_thread {
                    shared
                        .lock()
                        .expect("no thread panics while holding the lock")
                        .push(t);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().expect("workers cannot panic");
    }
    let mut values = Arc::into_inner(shared)
        .expect("all workers have finished, so this is the last Arc")
        .into_inner()
        .expect("no thread panicked while holding the lock");
    values.sort_unstable();
    values
}
```

Full tested solution: [`src/solutions/p0875_shared_counter_with_arc_and_mutex.rs`](../src/solutions/p0875_shared_counter_with_arc_and_mutex.rs)

</details>
