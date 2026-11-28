//! # Problem 0747: Remove K Digits
//!
//! - **Difficulty:** Medium
//! - **Topics:** greedy monotonic stack, `truncate`, `skip_while` for leading zeros
//! - **Source:** [LeetCode 402. Remove K Digits](https://leetcode.com/problems/remove-k-digits/)
//!
//! ## Problem
//!
//! `num` is a non-negative integer written as a string of digits (no leading zeros, except for the
//! number `"0"` itself). Remove exactly `k` digits so that the remaining digits, read in their
//! original order, form the **smallest** possible number. Return it as a string without leading
//! zeros; return `"0"` if nothing (or only zeros) remains.
//!
//! Implement `Solution::remove_kdigits(num: String, k: i32) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = "1432219", k = 3
//! Output: "1219"        (remove 4, 3 and 2)
//!
//! Input:  num = "10200", k = 1
//! Output: "200"         (remove the 1; the leading zero disappears)
//!
//! Input:  num = "10", k = 2
//! Output: "0"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= num.len() <= 10^5`
//! - `num` contains only digits and has no leading zeros (except `"0"`).
//!
//! ## Hints
//!
//! 1. The leftmost digits matter most. If a digit is followed by a smaller one, removing the
//!    larger digit makes the number smaller.
//! 2. Keep the digits chosen so far on a stack; pop while the top is larger than the next digit and
//!    you still have removals left.
//! 3. If removals remain at the end, the kept digits are non-decreasing: drop them from the end.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** greedy monotonic stack: build the answer digit by digit. When the next digit `d` is
//! smaller than the last kept digit, removing that last digit makes the number smaller (a smaller
//! digit moves into a more significant position), so pop while removals remain and the top is
//! greater than `d`. Then push `d`. After the scan the kept digits are non-decreasing; if removals
//! remain, the largest digits are at the end, so `truncate` drops them. Finally, skip leading
//! zeros and turn an empty result into `"0"`.
//!
//! **Complexity:** `O(n)` time (each digit pushed and popped at most once), `O(n)` space.
//!
//! **Rust concepts:**
//! - `stack.last().is_some_and(|&top| top > d)` checks the top without popping it.
//! - [`Vec::truncate`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.truncate) keeps
//!   the first `len` elements.
//! - `skip_while(|&c| c == '0')` drops leading zeros lazily while collecting into a `String`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        // @solution-start
        let mut k = k as usize;
        let mut stack: Vec<u8> = Vec::with_capacity(num.len());
        for &d in num.as_bytes() {
            while k > 0 && stack.last().is_some_and(|&top| top > d) {
                stack.pop();
                k -= 1;
            }
            stack.push(d);
        }
        stack.truncate(stack.len() - k); // remaining removals come off the end
        let digits: String = stack
            .iter()
            .map(|&b| char::from(b))
            .skip_while(|&c| c == '0')
            .collect();
        if digits.is_empty() {
            "0".to_string()
        } else {
            digits
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(num: &str, k: i32) -> String {
        Solution::remove_kdigits(num.to_string(), k)
    }

    #[test]
    fn example_remove_three() {
        assert_eq!(run("1432219", 3), "1219");
    }

    #[test]
    fn example_leading_zero() {
        assert_eq!(run("10200", 1), "200");
    }

    #[test]
    fn example_remove_everything() {
        assert_eq!(run("10", 2), "0");
    }

    #[test]
    fn non_decreasing_digits_drop_from_the_end() {
        assert_eq!(run("123456", 3), "123");
        assert_eq!(run("112", 1), "11");
    }

    #[test]
    fn only_zeros_remain() {
        assert_eq!(run("100", 1), "0");
        assert_eq!(run("9", 1), "0");
    }

    #[test]
    fn long_input() {
        let num = format!("{}{}", "9".repeat(50_000), "1".repeat(50_000));
        assert_eq!(run(&num, 50_000), "1".repeat(50_000));
    }
}
