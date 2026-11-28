//! # Problem 0867: Spawn and Join
//!
//! - **Difficulty:** Easy
//! - **Topics:** `std::thread::spawn`, `JoinHandle<T>::join`, `move` closures, collecting handles
//!   before joining
//! - **Source:** Original · [The Rust Book 16.1](https://doc.rust-lang.org/book/ch16-01-threads.html) ·
//!   [`thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html)
//!
//! ## Problem
//!
//! This problem starts the concurrency part of the level. Write two small functions that run work
//! on new operating-system threads and bring the results back with `join`:
//!
//! 1. `squares_in_threads(nums: Vec<i64>) -> Vec<i64>` spawns **one thread per number**; each
//!    thread computes the square of its number. Return the squares in the same order as `nums`.
//!    All threads must be started before the first one is joined, so they can run at the same time.
//! 2. `shout_in_thread(message: String) -> String` moves `message` into a new thread, which returns
//!    the message in upper case followed by `"!"`. The calling thread waits for the result and
//!    returns it.
//!
//! ## Examples
//!
//! ```text
//! squares_in_threads(vec![1, 2, 3])        -> [1, 4, 9]
//! squares_in_threads(vec![])               -> []
//! shout_in_thread("hello".to_string())     -> "HELLO!"
//! ```
//!
//! ## Constraints
//!
//! - `0 <= nums.len() <= 200`; `|nums[i]| <= 10^9`, so every square fits in an `i64`.
//!
//! ## Hints
//!
//! 1. `thread::spawn(move || x * x)` returns a `JoinHandle<i64>`; `handle.join()` waits for the
//!    thread and returns `Result<i64, _>` (`Err` only if the thread panicked).
//! 2. Iterators are lazy: `nums.iter().map(spawn).map(join)` would join each thread right after
//!    spawning it, one at a time. Collect the handles into a `Vec` first, then join them.
//! 3. A closure passed to `thread::spawn` must own what it uses (`'static`), hence `move`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** spawn all threads and keep their `JoinHandle`s in a `Vec` (this starts them
//! concurrently), then join the handles in order; joining in order keeps the results in input
//! order no matter which thread finishes first. For the second function, `move` transfers
//! ownership of the `String` into the thread, and the result comes back through `join`.
//!
//! **Rust concepts:**
//! - `thread::spawn<F, T>(f: F) -> JoinHandle<T>` requires `F: FnOnce() -> T + Send + 'static`
//!   and `T: Send + 'static`: the thread may outlive the caller, so it cannot borrow local data.
//! - `JoinHandle::join` returns `std::thread::Result<T>`. These threads cannot panic (squares fit,
//!   uppercase cannot fail), which justifies `expect` with an explanation.
//! - After `move`, `message` is no longer usable in the calling thread; the compiler enforces it.
// @solution-end

use std::thread;

pub fn squares_in_threads(nums: Vec<i64>) -> Vec<i64> {
    // @solution-start
    let handles: Vec<thread::JoinHandle<i64>> = nums
        .into_iter()
        .map(|x| thread::spawn(move || x * x))
        .collect();
    handles
        .into_iter()
        .map(|h| {
            h.join()
                .expect("squaring cannot panic within the constraints")
        })
        .collect()
    // @solution-end
}

pub fn shout_in_thread(message: String) -> String {
    // @solution-start
    let handle = thread::spawn(move || format!("{}!", message.to_uppercase()));
    handle.join().expect("uppercasing a string cannot panic")
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_numbers() {
        assert_eq!(squares_in_threads(vec![1, 2, 3]), vec![1, 4, 9]);
    }

    #[test]
    fn example_empty_input() {
        assert_eq!(squares_in_threads(vec![]), Vec::<i64>::new());
    }

    #[test]
    fn example_shout() {
        assert_eq!(shout_in_thread("hello".to_string()), "HELLO!");
    }

    #[test]
    fn negative_and_large_numbers() {
        let got = squares_in_threads(vec![-5, 0, 1_000_000_000]);
        assert_eq!(got, vec![25, 0, 1_000_000_000_000_000_000]);
    }

    #[test]
    fn order_is_preserved_with_many_threads() {
        let nums: Vec<i64> = (0..200).collect();
        let want: Vec<i64> = nums.iter().map(|x| x * x).collect();
        assert_eq!(squares_in_threads(nums), want);
    }

    #[test]
    fn shout_handles_empty_and_mixed_text() {
        assert_eq!(shout_in_thread(String::new()), "!");
        assert_eq!(shout_in_thread("Rust 1.87".to_string()), "RUST 1.87!");
    }
}
