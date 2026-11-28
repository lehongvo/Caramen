// ==================================================================
// PRACTICE 0008: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0008_split_seconds_into_units.md
// Run the tests:          cargo test practice::p0008
// ==================================================================

//! # Problem 0008: Split Seconds into Units
//!
//! - **Difficulty:** Easy
//! - **Topics:** `const`, integer division `/` and remainder `%`, tuples
//! - **Source:** Original · [The Rust Book 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants)
//!
//! ## Problem
//!
//! A stopwatch reports a duration as a total number of seconds. Convert it into days, hours,
//! minutes and seconds, where hours are `0..=23`, minutes are `0..=59` and seconds are `0..=59`
//! (days can be any size).
//!
//! Define the conversion factors as constants (`SECONDS_PER_MINUTE`, `SECONDS_PER_HOUR`,
//! `SECONDS_PER_DAY`) instead of repeating "magic numbers" in your code.
//!
//! Implement `split_seconds(total: u64) -> (u64, u64, u64, u64)` returning
//! `(days, hours, minutes, seconds)`.
//!
//! ## Examples
//!
//! ```text
//! split_seconds(59)      -> (0, 0, 0, 59)
//! split_seconds(3661)    -> (0, 1, 1, 1)      3661 = 1 h + 1 min + 1 s
//! split_seconds(90061)   -> (1, 1, 1, 1)      90061 = 1 day + 1 h + 1 min + 1 s
//! split_seconds(86400)   -> (1, 0, 0, 0)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= total <= u64::MAX`
//!
//! ## Hints
//!
//! 1. Integer division drops the fractional part: `3661 / 3600 == 1`.
//! 2. The remainder `3661 % 3600 == 61` is what is left over for the smaller units.
//! 3. Constants are declared with `const NAME: Type = value;` and may use other constants in their value.

pub fn split_seconds(total: u64) -> (u64, u64, u64, u64) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn under_a_minute() {
        assert_eq!(split_seconds(59), (0, 0, 0, 59));
        assert_eq!(split_seconds(0), (0, 0, 0, 0));
    }

    #[test]
    fn example_one_of_each_below_a_day() {
        assert_eq!(split_seconds(3661), (0, 1, 1, 1));
    }

    #[test]
    fn example_one_of_each_unit() {
        assert_eq!(split_seconds(90_061), (1, 1, 1, 1));
    }

    #[test]
    fn exact_day() {
        assert_eq!(split_seconds(86_400), (1, 0, 0, 0));
        assert_eq!(split_seconds(86_399), (0, 23, 59, 59));
    }

    #[test]
    fn maximum_value() {
        // u64::MAX = 18446744073709551615 seconds.
        assert_eq!(split_seconds(u64::MAX), (213_503_982_334_601, 7, 0, 15));
    }
}
