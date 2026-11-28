// ==================================================================
// PRACTICE 0305: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0305_collapse_repeated_values.md
// Run the tests:          cargo test practice::p0305
// ==================================================================

//! # Problem 0305: Collapse Repeated Values
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Vec::dedup`, `dedup_by_key`, `dedup_by`, sort + dedup
//! - **Source:** Original · [`Vec::dedup`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup)
//!
//! ## Problem
//!
//! Implement four functions. "A run" means a group of **consecutive** elements; only neighbours
//! are compared unless the statement says otherwise.
//!
//! 1. `collapse_runs(values: &mut Vec<i32>)` replaces every run of equal values by a single copy.
//! 2. `unique_sorted(values: Vec<i32>) -> Vec<i32>` returns every distinct value once, in
//!    increasing order (equal values do not have to be neighbours in the input).
//! 3. `collapse_ignore_case(words: &mut Vec<String>)` replaces every run of words that are equal
//!    when ASCII case is ignored by its **first** word (keep that word's spelling).
//! 4. `collapse_by_sign(values: &mut Vec<i32>)` keeps only the first value of every run of values
//!    with the same sign (negative, zero, positive).
//!
//! ## Examples
//!
//! ```text
//! collapse_runs:        [1, 1, 2, 2, 2, 1, 3, 3] -> [1, 2, 1, 3]
//! unique_sorted:        [3, 1, 3, 2, 1]          -> [1, 2, 3]
//! collapse_ignore_case: ["Hi", "hi", "HI", "yo", "Hi"] -> ["Hi", "yo", "Hi"]
//! collapse_by_sign:     [3, 5, -1, -7, 0, 0, 2]  -> [3, -1, 0, 2]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= len <= 10^5` for every input.
//!
//! ## Hints
//!
//! 1. `dedup()` removes **consecutive** duplicates only. To remove all duplicates, sort first.
//! 2. `dedup_by_key(|x| key(x))` compares neighbours by a computed key.
//! 3. `dedup_by(|a, b| ...)` receives the current element `a` and the previous kept element `b`;
//!    returning `true` removes `a`.

/// Replaces each run of equal values by one copy.
pub fn collapse_runs(values: &mut Vec<i32>) {
    todo!("Write your solution here")
}

/// Returns the distinct values in increasing order.
pub fn unique_sorted(values: Vec<i32>) -> Vec<i32> {
    todo!("Write your solution here")
}

/// Replaces each run of case-insensitively equal words by its first word.
pub fn collapse_ignore_case(words: &mut Vec<String>) {
    todo!("Write your solution here")
}

/// Keeps the first value of every run of values with the same sign.
pub fn collapse_by_sign(values: &mut Vec<i32>) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn collapse_runs_example() {
        let mut v = vec![1, 1, 2, 2, 2, 1, 3, 3];
        collapse_runs(&mut v);
        assert_eq!(v, vec![1, 2, 1, 3]);
        let mut empty: Vec<i32> = vec![];
        collapse_runs(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn unique_sorted_example() {
        assert_eq!(unique_sorted(vec![3, 1, 3, 2, 1]), vec![1, 2, 3]);
        assert_eq!(unique_sorted(vec![]), Vec::<i32>::new());
        assert_eq!(unique_sorted(vec![-5, -5, -5]), vec![-5]);
    }

    #[test]
    fn collapse_ignore_case_keeps_first_spelling() {
        let mut words = strings(&["Hi", "hi", "HI", "yo", "Hi"]);
        collapse_ignore_case(&mut words);
        assert_eq!(words, strings(&["Hi", "yo", "Hi"]));
    }

    #[test]
    fn collapse_by_sign_example() {
        let mut v = vec![3, 5, -1, -7, 0, 0, 2];
        collapse_by_sign(&mut v);
        assert_eq!(v, vec![3, -1, 0, 2]);
    }

    #[test]
    fn large_unique_sorted() {
        let values: Vec<i32> = (0..100_000).map(|i| (i * 7919) % 1000).collect();
        assert_eq!(unique_sorted(values), (0..1000).collect::<Vec<i32>>());
    }
}
