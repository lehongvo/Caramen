//! # Problem 0078: Find Numbers with Even Number of Digits
//!
//! - **Difficulty:** Easy
//! - **Topics:** counting digits, nested functions, iterating a `Vec` by reference
//! - **Source:** [LeetCode 1295. Find Numbers with Even Number of Digits](https://leetcode.com/problems/find-numbers-with-even-number-of-digits/)
//!
//! ## Problem
//!
//! Given a vector `nums` of positive integers, return how many of them have an **even** number of
//! decimal digits.
//!
//! Implement `Solution::find_numbers(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [12, 345, 2, 6, 7896]
//! Output: 2
//! Explanation: 12 has 2 digits and 7896 has 4 digits; the others have an odd count.
//!
//! Input:  nums = [555, 901, 482, 1771]
//! Output: 1
//! Explanation: only 1771 has an even number of digits.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 500`
//! - `1 <= nums[i] <= 10^5`
//!
//! ## Hints
//!
//! 1. Count digits by dividing by 10 until the number becomes 0.
//! 2. Alternative: compare with powers of ten (`10..=99`, `1000..=9999`, `100_000`).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a helper counts the digits of each number; count the numbers whose digit count is even.
//!
//! **Complexity:** `O(n * d)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - The helper `digits` is declared inside the method, so it does not need `Self::`.
//! - With the given limits, a `match` on ranges also works:
//!   `matches!(x, 10..=99 | 1000..=9999 | 100_000)`. The digit loop is more general.
//! - `i32::ilog10` (Rust 1.67+) returns `digits - 1` directly for positive numbers.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        // @solution-start
        fn digits(mut x: i32) -> u32 {
            let mut count = 0;
            while x > 0 {
                count += 1;
                x /= 10;
            }
            count
        }
        let mut result = 0;
        for &x in &nums {
            if digits(x) % 2 == 0 {
                result += 1;
            }
        }
        result
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed() {
        assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    }

    #[test]
    fn example_one_even() {
        assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
    }

    #[test]
    fn digit_count_boundaries() {
        assert_eq!(Solution::find_numbers(vec![9, 10, 99, 100, 999, 1000]), 3);
        assert_eq!(
            Solution::find_numbers(vec![9999, 10_000, 99_999, 100_000]),
            2
        );
    }

    #[test]
    fn single_elements() {
        assert_eq!(Solution::find_numbers(vec![1]), 0);
        assert_eq!(Solution::find_numbers(vec![42]), 1);
    }

    #[test]
    fn largest_input() {
        let nums: Vec<i32> = (1..=500).map(|i| i * 200).collect();
        // 200..=800 step 200 -> 3 digits (4 numbers: 200, 400, 600, 800)
        // 1000..=9800 -> 4 digits (45 numbers), 10000..=99800 -> 5 digits (450), 100000 -> 6 digits (1).
        assert_eq!(Solution::find_numbers(nums), 46);
    }
}
