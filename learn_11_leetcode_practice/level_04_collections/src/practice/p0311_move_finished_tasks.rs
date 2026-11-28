// ==================================================================
// PRACTICE 0311: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0311_move_finished_tasks.md
// Run the tests:          cargo test practice::p0311
// ==================================================================

//! # Problem 0311: Move Finished Tasks
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Vec::drain`, `extend`, `append`, range arguments, `saturating_sub`
//! - **Source:** Original · [`Vec::drain`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain) · [`Vec::append`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append)
//!
//! ## Problem
//!
//! A to-do app moves tasks between vectors. Implement these four functions; none of them may panic
//! when `n` is larger than the number of tasks.
//!
//! 1. `take_first(queue: &mut Vec<String>, n: usize) -> Vec<String>` removes the first `n` tasks
//!    from `queue` (or all of them if there are fewer) and returns them in order.
//! 2. `move_done(pending: &mut Vec<String>, done: &mut Vec<String>, n: usize)` moves the first `n`
//!    tasks of `pending` to the **end** of `done`, keeping their order.
//! 3. `merge_batch(target: &mut Vec<i32>, batch: &mut Vec<i32>)` moves **every** element of
//!    `batch` to the end of `target`; `batch` ends up empty (but still usable).
//! 4. `keep_last(log: &mut Vec<String>, n: usize)` keeps only the last `n` entries of `log`.
//!
//! ## Examples
//!
//! ```text
//! queue = ["a", "b", "c"]
//! take_first(queue, 2) => ["a", "b"], queue becomes ["c"]
//!
//! pending = ["write", "test", "ship"], done = ["plan"]
//! move_done(pending, done, 2) => pending = ["ship"], done = ["plan", "write", "test"]
//!
//! target = [1, 2], batch = [3, 4]
//! merge_batch(target, batch) => target = [1, 2, 3, 4], batch = []
//!
//! log = ["e1", "e2", "e3", "e4"]
//! keep_last(log, 2) => log becomes ["e3", "e4"]
//! ```
//!
//! ## Constraints
//!
//! - Every vector has at most `10^5` elements; `n` can be any `usize`.
//!
//! ## Hints
//!
//! 1. `drain(range)` removes a range of elements and returns an iterator over them. It panics if
//!    the range goes past the end, so clamp with `n.min(len)`.
//! 2. `extend` accepts any iterator, including the one returned by `drain`.
//! 3. `append(&mut other)` moves all elements of `other` without cloning them.

/// Removes and returns the first `n` tasks (or all of them).
pub fn take_first(queue: &mut Vec<String>, n: usize) -> Vec<String> {
    todo!("Write your solution here")
}

/// Moves the first `n` tasks of `pending` to the end of `done`.
pub fn move_done(pending: &mut Vec<String>, done: &mut Vec<String>, n: usize) {
    todo!("Write your solution here")
}

/// Moves every element of `batch` to the end of `target`.
pub fn merge_batch(target: &mut Vec<i32>, batch: &mut Vec<i32>) {
    todo!("Write your solution here")
}

/// Keeps only the last `n` entries of `log`.
pub fn keep_last(log: &mut Vec<String>, n: usize) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn take_first_examples() {
        let mut queue = strings(&["a", "b", "c"]);
        assert_eq!(take_first(&mut queue, 2), strings(&["a", "b"]));
        assert_eq!(queue, strings(&["c"]));
        assert_eq!(take_first(&mut queue, 10), strings(&["c"]));
        assert!(queue.is_empty());
        assert_eq!(take_first(&mut queue, 1), Vec::<String>::new());
    }

    #[test]
    fn move_done_example() {
        let mut pending = strings(&["write", "test", "ship"]);
        let mut done = strings(&["plan"]);
        move_done(&mut pending, &mut done, 2);
        assert_eq!(pending, strings(&["ship"]));
        assert_eq!(done, strings(&["plan", "write", "test"]));
        move_done(&mut pending, &mut done, 99);
        assert!(pending.is_empty());
        assert_eq!(done.len(), 4);
    }

    #[test]
    fn merge_batch_empties_the_batch() {
        let mut target = vec![1, 2];
        let mut batch = vec![3, 4];
        merge_batch(&mut target, &mut batch);
        assert_eq!(target, vec![1, 2, 3, 4]);
        assert!(batch.is_empty());
        batch.push(5);
        merge_batch(&mut target, &mut batch);
        assert_eq!(target, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn keep_last_examples() {
        let mut log = strings(&["e1", "e2", "e3", "e4"]);
        keep_last(&mut log, 2);
        assert_eq!(log, strings(&["e3", "e4"]));
        keep_last(&mut log, 5);
        assert_eq!(log, strings(&["e3", "e4"]));
        keep_last(&mut log, 0);
        assert!(log.is_empty());
    }

    #[test]
    fn large_move() {
        let mut pending: Vec<String> = (0..100_000).map(|i| i.to_string()).collect();
        let mut done = Vec::new();
        move_done(&mut pending, &mut done, 60_000);
        assert_eq!(pending.len(), 40_000);
        assert_eq!(pending[0], "60000");
        assert_eq!(done.len(), 60_000);
        assert_eq!(done[59_999], "59999");
    }
}
