// ==================================================================
// PRACTICE 0959: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0959_maximum_frequency_stack.md
// Run the tests:          cargo test practice::p0959
// ==================================================================

//! # Problem 0959: Maximum Frequency Stack
//!
//! - **Difficulty:** Hard
//! - **Topics:** one stack per frequency (`Vec<Vec<i32>>`), `HashMap` counters, `entry` API, amortised `O(1)` operations
//! - **Source:** [LeetCode 895. Maximum Frequency Stack](https://leetcode.com/problems/maximum-frequency-stack/)
//!
//! ## Problem
//!
//! Design a stack-like structure whose `pop` removes the **most frequent** element; among the most
//! frequent elements it removes the one pushed **most recently**.
//!
//! Implement the `FreqStack` struct:
//!
//! - `FreqStack::new() -> Self`.
//! - `push(&mut self, val: i32)`.
//! - `pop(&mut self) -> i32`: removes and returns the element described above (`pop` is only called
//!   when the stack is not empty).
//!
//! ## Examples
//!
//! ```text
//! push 5, 7, 5, 7, 4, 5          // stack bottom -> top: 5 7 5 7 4 5
//! pop() => 5      (5 appears 3 times)
//! pop() => 7      (5 and 7 now appear twice; 7 was pushed more recently)
//! pop() => 5
//! pop() => 4      (5, 7, 4 appear once; 4 is the most recent)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= val <= 10^9`
//! - At most `2 * 10^4` calls in total.
//!
//! ## Hints
//!
//! 1. Track `count[val]`, the current frequency of each value.
//! 2. Keep `groups[f - 1]`: a stack of the values in the order in which they **reached** frequency
//!    `f`. A value with frequency 3 appears in groups 1, 2 and 3.
//! 3. `pop` takes from the last (highest-frequency) group, and drops that group when it is empty.

use std::collections::HashMap;

pub struct FreqStack {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl FreqStack {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn push(&mut self, val: i32) {
        todo!("Write your solution here")
    }

    pub fn pop(&mut self) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut s = FreqStack::new();
        for v in [5, 7, 5, 7, 4, 5] {
            s.push(v);
        }
        let popped: Vec<i32> = (0..4).map(|_| s.pop()).collect();
        assert_eq!(popped, vec![5, 7, 5, 4]);
    }

    #[test]
    fn behaves_like_a_stack_without_repeats() {
        let mut s = FreqStack::new();
        for v in 1..=5 {
            s.push(v);
        }
        let popped: Vec<i32> = (0..5).map(|_| s.pop()).collect();
        assert_eq!(popped, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn interleaved_pushes_and_pops() {
        let mut s = FreqStack::new();
        s.push(1);
        s.push(1);
        s.push(2);
        assert_eq!(s.pop(), 1);
        s.push(2);
        assert_eq!(s.pop(), 2);
        assert_eq!(s.pop(), 2);
        assert_eq!(s.pop(), 1);
        s.push(1_000_000_000);
        assert_eq!(s.pop(), 1_000_000_000);
    }

    #[test]
    fn matches_a_naive_model() {
        let mut s = FreqStack::new();
        let mut model: Vec<i32> = Vec::new();
        for step in 0..3_000_usize {
            if step % 3 == 2 && !model.is_empty() {
                // Naive pop: highest count, then the latest position among those values.
                let mut counts: HashMap<i32, usize> = HashMap::new();
                for &v in &model {
                    *counts.entry(v).or_insert(0) += 1;
                }
                let best = counts.values().copied().max().unwrap();
                let index = model.iter().rposition(|v| counts[v] == best).unwrap();
                let expected = model.remove(index);
                assert_eq!(s.pop(), expected, "step {step}");
            } else {
                let v = ((step * 7) % 11) as i32;
                s.push(v);
                model.push(v);
            }
        }
    }
}
