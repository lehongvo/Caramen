// ==================================================================
// PRACTICE 0496: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0496_split_array_into_fibonacci_sequence.md
// Run the tests:          cargo test practice::p0496
// ==================================================================

//! # Problem 0496: Split Array into Fibonacci Sequence
//!
//! - **Difficulty:** Medium
//! - **Topics:** trying candidates with `find_map`, `checked_add` + `?` inside an `Option`-returning helper, parsing with overflow detection, `starts_with`
//! - **Source:** [LeetCode 842. Split Array into Fibonacci Sequence](https://leetcode.com/problems/split-array-into-fibonacci-sequence/)
//!
//! ## Problem
//!
//! `num` is a string of digits. Split it into a *Fibonacci-like* sequence `f`:
//!
//! - `f` has at least 3 numbers, each `0 <= f[i] < 2^31` (it fits in an `i32`);
//! - `f[i] + f[i + 1] == f[i + 2]` for every valid `i`;
//! - writing the numbers one after another gives exactly `num`, and no number has a leading zero
//!   (except the number `0` itself).
//!
//! Return any such sequence, or an empty vector if none exists.
//!
//! Implement `Solution::split_into_fibonacci(num: String) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = "1101111"
//! Output: [11, 0, 11, 11]          ([110, 1, 111] would also be accepted)
//!
//! Input:  num = "112358130"
//! Output: []
//!
//! Input:  num = "0123"
//! Output: []                        ("01" has a leading zero; [0, 1, 2, 3] is not Fibonacci-like)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num.len() <= 200`
//! - `num` contains only digits.
//!
//! ## Hints
//!
//! 1. The first two numbers determine the whole sequence. Try every length for them (each has at
//!    most 10 digits, since it must fit in an `i32`).
//! 2. Write a helper `fn extend(num, a, b, pos) -> Option<Vec<i32>>` that keeps computing the next
//!    number with `checked_add(...)?` and checks that `num[pos..]` starts with its decimal text.
//! 3. `find_map` over all `(i, j)` length pairs returns the first sequence that works.

pub struct Solution;

impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks every rule of the statement for a returned sequence.
    fn is_valid(num: &str, seq: &[i32]) -> bool {
        seq.len() >= 3
            && seq
                .windows(3)
                .all(|w| i64::from(w[0]) + i64::from(w[1]) == i64::from(w[2]))
            && seq.iter().map(|x| x.to_string()).collect::<String>() == num
            && seq.iter().all(|&x| x >= 0)
    }

    #[test]
    fn example_with_zero() {
        let seq = Solution::split_into_fibonacci("1101111".to_string());
        assert!(is_valid("1101111", &seq), "{seq:?}");
    }

    #[test]
    fn example_impossible() {
        assert_eq!(
            Solution::split_into_fibonacci("112358130".to_string()),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn example_leading_zero() {
        assert_eq!(
            Solution::split_into_fibonacci("0123".to_string()),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn classic_and_zero_sequences() {
        let seq = Solution::split_into_fibonacci("11235813".to_string());
        assert_eq!(seq, vec![1, 1, 2, 3, 5, 8, 13]);
        assert_eq!(
            Solution::split_into_fibonacci("000".to_string()),
            vec![0, 0, 0]
        );
        assert_eq!(
            Solution::split_into_fibonacci("12".to_string()),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn overflow_is_rejected() {
        // 1, 2147483646, 2147483647 is valid; the next number would overflow, but none is needed.
        let ok = "121474836462147483647";
        let seq = Solution::split_into_fibonacci(ok.to_string());
        assert!(is_valid(ok, &seq), "{seq:?}");
        // Here the third number (2147483648) does not fit in an i32.
        assert_eq!(
            Solution::split_into_fibonacci("214748364712147483648".to_string()),
            Vec::<i32>::new()
        );
    }
}
