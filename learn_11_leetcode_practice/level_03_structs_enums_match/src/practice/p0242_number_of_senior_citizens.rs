// ==================================================================
// PRACTICE 0242: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0242_number_of_senior_citizens.md
// Run the tests:          cargo test practice::p0242
// ==================================================================

//! # Problem 0242: Number of Senior Citizens
//!
//! - **Difficulty:** Easy
//! - **Topics:** parsing fixed-width records into a struct, an `enum` field, string slicing with `get`, `filter_map`
//! - **Source:** [LeetCode 2678. Number of Senior Citizens](https://leetcode.com/problems/number-of-senior-citizens/)
//!
//! ## Problem
//!
//! Each passenger is described by a string of exactly 15 characters:
//!
//! - characters `0..10`: the phone number (10 digits);
//! - character `10`: the gender, `'M'`, `'F'` or `'O'`;
//! - characters `11..13`: the age (2 digits);
//! - characters `13..15`: the seat.
//!
//! Return how many passengers are **strictly older than 60**.
//!
//! Implement `Solution::count_seniors(details: Vec<String>) -> i32`.
//!
//! For this level, parse each string into a `struct Passenger { phone, gender, age, seat }` where
//! `gender` is an `enum Gender { Male, Female, Other }`, then count with the struct.
//!
//! ## Examples
//!
//! ```text
//! Input:  details = ["7868190130M7522", "5303914400F9211", "9273338290F4010"]
//! Output: 2        (ages 75, 92 and 40)
//!
//! Input:  details = ["1313579440F2036", "2921522980M5644"]
//! Output: 0        (ages 20 and 56)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= details.len() <= 100`
//! - Every string has length 15, digits where digits are expected, and a gender of `'M'`, `'F'` or `'O'`.
//!
//! ## Hints
//!
//! 1. `s.get(11..13)` returns `Option<&str>`; parse it with `.parse::<u8>().ok()`.
//! 2. `Passenger::parse(s: &str) -> Option<Passenger>` can use `?` on every step.

pub struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_two_seniors() {
        assert_eq!(
            Solution::count_seniors(list(&[
                "7868190130M7522",
                "5303914400F9211",
                "9273338290F4010"
            ])),
            2
        );
    }

    #[test]
    fn example_no_seniors() {
        assert_eq!(
            Solution::count_seniors(list(&["1313579440F2036", "2921522980M5644"])),
            0
        );
    }

    #[test]
    fn sixty_is_not_senior() {
        assert_eq!(Solution::count_seniors(list(&["1111111111O6001"])), 0);
        assert_eq!(Solution::count_seniors(list(&["1111111111O6101"])), 1);
    }

    #[test]
    fn hundred_passengers() {
        let all: Vec<String> = (0..100).map(|i| format!("9999999999F{:02}AB", i)).collect();
        // Ages 61..=99 are seniors.
        assert_eq!(Solution::count_seniors(all), 39);
    }
}
