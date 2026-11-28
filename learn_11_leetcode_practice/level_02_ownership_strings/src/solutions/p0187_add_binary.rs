//! # Problem 0187: Add Binary
//!
//! - **Difficulty:** Easy
//! - **Topics:** walking two byte slices from the back, carry, `Vec<u8>` + `reverse`, `String::from_utf8`
//! - **Source:** [LeetCode 67. Add Binary](https://leetcode.com/problems/add-binary/)
//!
//! ## Problem
//!
//! Given two binary numbers `a` and `b` as strings of `'0'` and `'1'`, return their sum as a binary
//! string. The numbers can be far too long for any integer type, so add them digit by digit.
//!
//! Implement `Solution::add_binary(a: String, b: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  a = "11", b = "1"
//! Output: "100"
//!
//! Input:  a = "1010", b = "1011"
//! Output: "10101"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= a.len(), b.len() <= 10^4`
//! - `a` and `b` contain only `'0'` and `'1'`, and have no leading zeros except the number `"0"`.
//!
//! ## Hints
//!
//! 1. Add from the least significant digit (the end of each string), keeping a carry.
//! 2. Use indices that count the remaining digits (`i = a.len()`, read `a[i - 1]`), so they never
//!    go below zero.
//! 3. Collect the result digits in reverse order, then `reverse()` once at the end.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** schoolbook addition in base 2. While either number has digits left or a carry remains,
//! add the available digits and the carry, emit `sum % 2`, keep `sum / 2` as the new carry. The
//! digits come out least significant first, so the buffer is reversed at the end.
//!
//! **Complexity:** `O(max(n, m))` time and space.
//!
//! **Rust concepts:**
//! - `let (a, b) = (a.as_bytes(), b.as_bytes());` shadows the `String`s with byte-slice views; the
//!   `String`s are still alive (shadowing does not drop them), so the borrows are valid.
//! - `byte - b'0'` turns an ASCII digit into its value; `b'0' + d` turns it back.
//! - Building a `Vec<u8>` of ASCII digits and converting it with `String::from_utf8` avoids
//!   inserting at the front of a `String` (which would be `O(n)` per insert).
// @solution-end

pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        // @solution-start
        let (a, b) = (a.as_bytes(), b.as_bytes());
        let mut digits = Vec::with_capacity(a.len().max(b.len()) + 1);
        let (mut i, mut j, mut carry) = (a.len(), b.len(), 0u8);
        while i > 0 || j > 0 || carry > 0 {
            let mut sum = carry;
            if i > 0 {
                i -= 1;
                sum += a[i] - b'0';
            }
            if j > 0 {
                j -= 1;
                sum += b[j] - b'0';
            }
            digits.push(b'0' + sum % 2);
            carry = sum / 2;
        }
        digits.reverse();
        String::from_utf8(digits).expect("only ASCII digits were pushed")
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add(a: &str, b: &str) -> String {
        Solution::add_binary(a.to_string(), b.to_string())
    }

    #[test]
    fn example_carry_out() {
        assert_eq!(add("11", "1"), "100");
    }

    #[test]
    fn example_equal_lengths() {
        assert_eq!(add("1010", "1011"), "10101");
    }

    #[test]
    fn zeros() {
        assert_eq!(add("0", "0"), "0");
        assert_eq!(add("0", "101"), "101");
    }

    #[test]
    fn matches_integer_addition() {
        for (x, y) in [(5u64, 9u64), (255, 1), (1023, 1023), (123_456, 654_321)] {
            assert_eq!(
                add(&format!("{x:b}"), &format!("{y:b}")),
                format!("{:b}", x + y)
            );
        }
    }

    #[test]
    fn longest_inputs() {
        let a = "1".repeat(10_000);
        let result = add(&a, "1");
        assert_eq!(result.len(), 10_001);
        assert!(result.starts_with('1'));
        assert!(result[1..].bytes().all(|b| b == b'0'));
    }
}
