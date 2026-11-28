// ==================================================================
// PRACTICE 0873: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0873_multi_stage_pipeline.md
// Run the tests:          cargo test practice::p0873
// ==================================================================

//! # Problem 0873: Multi-Stage Pipeline
//!
//! - **Difficulty:** Medium
//! - **Topics:** producer-consumer pipelines, a generic stage function with `Send + 'static`
//!   bounds, `FnMut` stages that keep state, shutdown that flows downstream
//! - **Source:** Original · [The Rust Book 16.2](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
//!
//! ## Problem
//!
//! A *pipeline* is a chain of threads connected by channels: each stage receives items from the
//! previous stage, transforms them and sends the results to the next one. All stages work at the
//! same time on different items.
//!
//! 1. Implement the generic building block
//!
//!    ```text
//!    pub fn spawn_stage<T, U, F>(input: Receiver<T>, f: F) -> Receiver<U>
//!    where T: Send + 'static, U: Send + 'static, F: FnMut(T) -> Option<U> + Send + 'static
//!    ```
//!
//!    It spawns one thread that applies `f` to every item received from `input`, in order. When
//!    `f` returns `Some(u)`, `u` is sent downstream; `None` drops the item. The stage stops when
//!    `input` is closed (all its senders dropped) or when nobody listens downstream any more, and
//!    stopping closes the returned receiver.
//!
//! 2. Use it to implement `parse_and_square(lines: Vec<String>) -> Vec<i64>`: a source thread sends
//!    the lines; stage 1 trims each line and parses it as an `i64` (lines that do not parse are
//!    dropped); stage 2 squares the number (dropping results that would overflow); the calling
//!    thread collects the results, which keep the input order.
//!
//! ## Examples
//!
//! ```text
//! parse_and_square(["1", "x", "3", " 4 "]) -> [1, 9, 16]
//! parse_and_square([])                     -> []
//!
//! // A stage that keeps only even numbers:
//! spawn_stage(rx_of_0_to_9, |x| (x % 2 == 0).then_some(x)) yields 0, 2, 4, 6, 8
//! ```
//!
//! ## Constraints
//!
//! - `0 <= lines.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. Inside `spawn_stage`: create `(tx, rx)`, move `input`, `tx` and `f` into a new thread running
//!    `for item in input { ... }`, and return `rx`.
//! 2. When the stage thread ends, `tx` is dropped, so the next stage's loop ends too: shutdown
//!    propagates down the pipeline by itself.
//! 3. `FnMut` (not just `Fn`) lets a stage keep state between items, for example a running total.

use std::sync::mpsc::{self, Receiver};
use std::thread;

pub fn spawn_stage<T, U, F>(input: Receiver<T>, f: F) -> Receiver<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: FnMut(T) -> Option<U> + Send + 'static,
{
    todo!("Write your solution here")
}

pub fn parse_and_square(lines: Vec<String>) -> Vec<i64> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_mixed_lines() {
        assert_eq!(
            parse_and_square(strings(&["1", "x", "3", " 4 "])),
            vec![1, 9, 16]
        );
    }

    #[test]
    fn example_empty_input() {
        assert!(parse_and_square(Vec::new()).is_empty());
    }

    #[test]
    fn example_filter_stage() {
        let (tx, rx) = mpsc::channel();
        for x in 0..10 {
            tx.send(x).unwrap();
        }
        drop(tx);
        let evens: Vec<i32> = spawn_stage(rx, |x: i32| (x % 2 == 0).then_some(x))
            .iter()
            .collect();
        assert_eq!(evens, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn stateful_stage_and_type_change() {
        let (tx, rx) = mpsc::channel();
        for x in [3, 1, 4, 1, 5] {
            tx.send(x).unwrap();
        }
        drop(tx);
        let mut total = 0;
        let running = spawn_stage(rx, move |x: i32| {
            total += x;
            Some(total)
        });
        let labels = spawn_stage(running, |t: i32| Some(format!("={t}")));
        let got: Vec<String> = labels.iter().collect();
        assert_eq!(got, strings(&["=3", "=4", "=8", "=9", "=14"]));
    }

    #[test]
    fn overflow_and_negative_numbers() {
        let lines = strings(&["-3", "9223372036854775807", "3037000499", "+2"]);
        assert_eq!(
            parse_and_square(lines),
            vec![9, 3_037_000_499 * 3_037_000_499, 4]
        );
    }

    #[test]
    fn long_input_keeps_order() {
        let lines: Vec<String> = (0..100_000).map(|i| i.to_string()).collect();
        let got = parse_and_square(lines);
        assert_eq!(got.len(), 100_000);
        assert!(got.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(got[99_999], 99_999 * 99_999);
    }
}
