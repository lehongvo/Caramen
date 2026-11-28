// ==================================================================
// PRACTICE 0137: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0137_defanging_an_ip_address.md
// Run the tests:          cargo test practice::p0137
// ==================================================================

//! # Problem 0137: Defanging an IP Address
//!
//! - **Difficulty:** Easy
//! - **Topics:** building a `String` with `with_capacity`, `push`, `push_str`, `str::replace`
//! - **Source:** [LeetCode 1108. Defanging an IP Address](https://leetcode.com/problems/defanging-an-ip-address/)
//!
//! ## Problem
//!
//! A *defanged* IP address is one where every period `"."` is replaced by `"[.]"`, so that it is no
//! longer recognized as a link. Given a valid IPv4 address (four decimal numbers separated by three
//! periods), return its defanged version.
//!
//! Implement `Solution::defang_i_paddr(address: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  address = "1.1.1.1"
//! Output: "1[.]1[.]1[.]1"
//!
//! Input:  address = "255.100.50.0"
//! Output: "255[.]100[.]50[.]0"
//! ```
//!
//! ## Constraints
//!
//! - `address` is a valid IPv4 address.
//!
//! ## Hints
//!
//! 1. The result is exactly 6 bytes longer than the input (3 periods, each gaining 2 bytes), so you
//!    can allocate it once with `String::with_capacity(address.len() + 6)`.
//! 2. Loop over `address.chars()`: push `"[.]"` with `push_str` for a period, otherwise `push(c)`.
//! 3. The one-line version is `address.replace('.', "[.]")`.

pub struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ones() {
        assert_eq!(
            Solution::defang_i_paddr("1.1.1.1".to_string()),
            "1[.]1[.]1[.]1"
        );
    }

    #[test]
    fn example_mixed_lengths() {
        assert_eq!(
            Solution::defang_i_paddr("255.100.50.0".to_string()),
            "255[.]100[.]50[.]0"
        );
    }

    #[test]
    fn zeros_address() {
        assert_eq!(
            Solution::defang_i_paddr("0.0.0.0".to_string()),
            "0[.]0[.]0[.]0"
        );
    }

    #[test]
    fn result_is_six_bytes_longer() {
        let address = "192.168.100.254".to_string();
        let len = address.len();
        assert_eq!(Solution::defang_i_paddr(address).len(), len + 6);
    }
}
