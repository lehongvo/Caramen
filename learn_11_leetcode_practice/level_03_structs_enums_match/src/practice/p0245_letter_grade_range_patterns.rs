// ==================================================================
// PRACTICE 0245: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0245_letter_grade_range_patterns.md
// Run the tests:          cargo test practice::p0245
// ==================================================================

//! # Problem 0245: Letter Grades with Range Patterns
//!
//! - **Difficulty:** Easy
//! - **Topics:** inclusive range patterns `a..=b`, half-open range patterns `a..`, exclusive range patterns `a..b`, enums as results
//! - **Source:** Original · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html) · [The Rust Reference: Range patterns](https://doc.rust-lang.org/reference/patterns.html#range-patterns)
//!
//! ## Problem
//!
//! Integer ranges can be used directly as `match` patterns. Implement three classifiers, each with
//! a single `match` on the number:
//!
//! - `letter_grade(score: u8) -> Option<char>`: `90..=100` is `'A'`, `80..=89` `'B'`, `70..=79`
//!   `'C'`, `60..=69` `'D'`, `0..=59` `'F'`. A score above 100 is invalid: `None`.
//! - `age_group(age: u32) -> AgeGroup`: `0..=1` `Baby`, `2..=12` `Child`, `13..=19` `Teen`,
//!   `20..=64` `Adult`, `65` and above `Senior`.
//! - `tax_rate(income: u32) -> u8`: the percentage rate for a yearly income, with brackets
//!   `[0, 10_000)` 0 %, `[10_000, 40_000)` 10 %, `[40_000, 100_000)` 20 %, `100_000` and above 35 %.
//!   (Square bracket includes the bound, round bracket excludes it.)
//!
//! Then write `grade_histogram(scores: &[u8]) -> [usize; 5]`: how many valid scores got `A`, `B`,
//! `C`, `D` and `F`, in that order (invalid scores are ignored).
//!
//! ## Examples
//!
//! ```text
//! letter_grade(95)   => Some('A')
//! letter_grade(60)   => Some('D')
//! letter_grade(59)   => Some('F')
//! letter_grade(101)  => None
//! age_group(13)      => Teen
//! age_group(90)      => Senior
//! tax_rate(9_999)    => 0
//! tax_rate(10_000)   => 10
//! tax_rate(250_000)  => 35
//! grade_histogram(&[95, 85, 85, 40, 200]) => [1, 2, 0, 0, 1]
//! ```
//!
//! ## Constraints
//!
//! - `scores.len() <= 10^5`.
//!
//! ## Hints
//!
//! 1. `90..=100 => Some('A')` uses an inclusive range pattern.
//! 2. `65.. => AgeGroup::Senior` (no upper bound) covers everything from 65 up.
//! 3. Exclusive range patterns such as `10_000..40_000` (Rust 1.80+) mirror the `[a, b)` brackets.

/// A coarse age bracket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgeGroup {
    Baby,
    Child,
    Teen,
    Adult,
    Senior,
}

/// The letter grade for a score, or `None` above 100.
pub fn letter_grade(score: u8) -> Option<char> {
    todo!("Write your solution here")
}

/// The age bracket.
pub fn age_group(age: u32) -> AgeGroup {
    todo!("Write your solution here")
}

/// The tax rate in percent.
pub fn tax_rate(income: u32) -> u8 {
    todo!("Write your solution here")
}

/// Counts of A, B, C, D and F among the valid scores.
pub fn grade_histogram(scores: &[u8]) -> [usize; 5] {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grade_boundaries() {
        assert_eq!(letter_grade(100), Some('A'));
        assert_eq!(letter_grade(95), Some('A'));
        assert_eq!(letter_grade(90), Some('A'));
        assert_eq!(letter_grade(89), Some('B'));
        assert_eq!(letter_grade(70), Some('C'));
        assert_eq!(letter_grade(60), Some('D'));
        assert_eq!(letter_grade(59), Some('F'));
        assert_eq!(letter_grade(0), Some('F'));
        assert_eq!(letter_grade(101), None);
        assert_eq!(letter_grade(255), None);
    }

    #[test]
    fn age_groups() {
        assert_eq!(age_group(0), AgeGroup::Baby);
        assert_eq!(age_group(1), AgeGroup::Baby);
        assert_eq!(age_group(2), AgeGroup::Child);
        assert_eq!(age_group(13), AgeGroup::Teen);
        assert_eq!(age_group(19), AgeGroup::Teen);
        assert_eq!(age_group(64), AgeGroup::Adult);
        assert_eq!(age_group(90), AgeGroup::Senior);
        assert_eq!(age_group(u32::MAX), AgeGroup::Senior);
    }

    #[test]
    fn tax_brackets() {
        assert_eq!(tax_rate(0), 0);
        assert_eq!(tax_rate(9_999), 0);
        assert_eq!(tax_rate(10_000), 10);
        assert_eq!(tax_rate(39_999), 10);
        assert_eq!(tax_rate(40_000), 20);
        assert_eq!(tax_rate(100_000), 35);
        assert_eq!(tax_rate(250_000), 35);
    }

    #[test]
    fn histogram() {
        assert_eq!(grade_histogram(&[95, 85, 85, 40, 200]), [1, 2, 0, 0, 1]);
        assert_eq!(grade_histogram(&[]), [0; 5]);
        assert_eq!(grade_histogram(&[101, 150]), [0; 5]);
    }

    #[test]
    fn histogram_of_every_byte() {
        let scores: Vec<u8> = (0..=255).collect();
        // A: 90..=100 (11), B/C/D: 10 each, F: 0..=59 (60); 101..=255 ignored.
        assert_eq!(grade_histogram(&scores), [11, 10, 10, 10, 60]);
        let many = vec![75u8; 100_000];
        assert_eq!(grade_histogram(&many), [0, 0, 100_000, 0, 0]);
    }
}
