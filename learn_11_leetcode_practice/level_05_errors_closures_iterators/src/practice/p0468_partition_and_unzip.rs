// ==================================================================
// PRACTICE 0468: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0468_partition_and_unzip.md
// Run the tests:          cargo test practice::p0468
// ==================================================================

//! # Problem 0468: Partition and Unzip Exam Results
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Iterator::partition`, `Iterator::unzip`, `zip`, `into_iter` vs `iter`, tuple destructuring in closures
//! - **Source:** Original · [std `Iterator::partition`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition) · [std `Iterator::unzip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip)
//!
//! ## Problem
//!
//! Two consumers split one iterator into **two** collections:
//!
//! - `partition(pred)` puts the items for which `pred` is true in the first collection and the
//!   others in the second;
//! - `unzip()` turns an iterator of pairs `(a, b)` into a pair of collections `(all a, all b)`.
//!
//! Exam results are `(name, score)` pairs. Implement:
//!
//! - `split_pass_fail(results: &[(&str, u32)], pass_mark: u32) -> (Vec<String>, Vec<String>)`:
//!   the names of the students with `score >= pass_mark`, and the names of the others, both in
//!   input order.
//! - `names_and_scores(results: &[(&str, u32)]) -> (Vec<String>, Vec<u32>)`: the two columns.
//! - `evens_and_odds(nums: Vec<i32>) -> (Vec<i32>, Vec<i32>)`: consume `nums` and split it.
//! - `report(names: &[&str], scores: &[u32]) -> Vec<String>`: `"NAME: SCORE"` for each pair taken
//!   from the two lists in parallel; extra elements of the longer list are ignored.
//!
//! ## Examples
//!
//! ```text
//! results = [("ann", 72), ("bob", 48), ("cy", 50), ("dee", 91)]
//! split_pass_fail(results, 50)  -> (["ann", "cy", "dee"], ["bob"])
//! names_and_scores(results)     -> (["ann", "bob", "cy", "dee"], [72, 48, 50, 91])
//! evens_and_odds(vec![1, 2, 3, 4, -6]) -> ([2, 4, -6], [1, 3])
//! report(&["ann", "bob"], &[72, 48, 99]) -> ["ann: 72", "bob: 48"]
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` results.
//!
//! ## Hints
//!
//! 1. Partition the result pairs first, then turn each half into names; or `partition` an iterator
//!    of `(name, score)` and map afterwards.
//! 2. `unzip` needs the target types: `let (names, scores): (Vec<String>, Vec<u32>) = ...`.
//! 3. `zip` stops at the end of the shorter iterator.

pub fn split_pass_fail(results: &[(&str, u32)], pass_mark: u32) -> (Vec<String>, Vec<String>) {
    todo!("Write your solution here")
}

pub fn names_and_scores(results: &[(&str, u32)]) -> (Vec<String>, Vec<u32>) {
    todo!("Write your solution here")
}

pub fn evens_and_odds(nums: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    todo!("Write your solution here")
}

pub fn report(names: &[&str], scores: &[u32]) -> Vec<String> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: [(&str, u32); 4] = [("ann", 72), ("bob", 48), ("cy", 50), ("dee", 91)];

    #[test]
    fn pass_and_fail() {
        let (passed, failed) = split_pass_fail(&RESULTS, 50);
        assert_eq!(passed, vec!["ann", "cy", "dee"]);
        assert_eq!(failed, vec!["bob"]);
        let (passed, failed) = split_pass_fail(&RESULTS, 100);
        assert!(passed.is_empty());
        assert_eq!(failed.len(), 4);
    }

    #[test]
    fn unzip_columns() {
        let (names, scores) = names_and_scores(&RESULTS);
        assert_eq!(names, vec!["ann", "bob", "cy", "dee"]);
        assert_eq!(scores, vec![72, 48, 50, 91]);
        assert_eq!(names_and_scores(&[]), (vec![], vec![]));
    }

    #[test]
    fn evens_odds() {
        assert_eq!(
            evens_and_odds(vec![1, 2, 3, 4, -6]),
            (vec![2, 4, -6], vec![1, 3])
        );
        assert_eq!(evens_and_odds(vec![-3]), (vec![], vec![-3]));
    }

    #[test]
    fn zipped_report() {
        assert_eq!(
            report(&["ann", "bob"], &[72, 48, 99]),
            vec!["ann: 72", "bob: 48"]
        );
        assert_eq!(report(&["ann", "bob", "cy"], &[1]), vec!["ann: 1"]);
        assert_eq!(report(&[], &[1, 2]), Vec::<String>::new());
    }
}
