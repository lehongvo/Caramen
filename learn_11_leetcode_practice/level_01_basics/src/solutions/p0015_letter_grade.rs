//! # Problem 0015: Letter Grade
//!
//! - **Difficulty:** Easy
//! - **Topics:** `match` on integer ranges, `char`, `Option`
//! - **Source:** Original · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#matching-ranges-of-values-with-)
//!
//! ## Problem
//!
//! A teacher converts exam scores (whole numbers from 0 to 100) into letter grades:
//!
//! | Score      | Grade |
//! | ---------- | ----- |
//! | 90 to 100  | `'A'` |
//! | 80 to 89   | `'B'` |
//! | 70 to 79   | `'C'` |
//! | 60 to 69   | `'D'` |
//! | 0 to 59    | `'F'` |
//!
//! A score above 100 is a typing mistake and has no grade. Write
//! `letter_grade(score: u8) -> Option<char>` that returns `Some(grade)` for valid scores and `None`
//! for scores above 100.
//!
//! ## Examples
//!
//! ```text
//! letter_grade(95)  -> Some('A')
//! letter_grade(80)  -> Some('B')
//! letter_grade(59)  -> Some('F')
//! letter_grade(101) -> None
//! ```
//!
//! ## Constraints
//!
//! - `0 <= score <= 255` (any `u8`)
//!
//! ## Hints
//!
//! 1. A `match` arm can match a whole range: `90..=100 => ...`.
//! 2. The compiler checks that the arms cover **every** `u8` value. What arm handles 101 to 255?
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** one `match` with an inclusive range pattern per grade, plus a catch-all arm `_` for
//! the invalid scores.
//!
//! **Rust concepts:**
//! - Inclusive range patterns `a..=b` work for integers and `char`. Because `score` is a `u8`, the
//!   compiler knows the full set of values and requires the `match` to be exhaustive.
//! - Character literals use single quotes (`'A'`); string literals use double quotes (`"A"`).
//! - `Option<char>` expresses "maybe a grade": `Some('A')` or `None`. It is Rust's alternative to
//!   returning a special sentinel character.
// @solution-end

pub fn letter_grade(score: u8) -> Option<char> {
    // @solution-start
    match score {
        90..=100 => Some('A'),
        80..=89 => Some('B'),
        70..=79 => Some('C'),
        60..=69 => Some('D'),
        0..=59 => Some('F'),
        _ => None,
    }
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(letter_grade(95), Some('A'));
        assert_eq!(letter_grade(80), Some('B'));
        assert_eq!(letter_grade(59), Some('F'));
        assert_eq!(letter_grade(101), None);
    }

    #[test]
    fn boundaries() {
        assert_eq!(letter_grade(100), Some('A'));
        assert_eq!(letter_grade(90), Some('A'));
        assert_eq!(letter_grade(89), Some('B'));
        assert_eq!(letter_grade(79), Some('C'));
        assert_eq!(letter_grade(70), Some('C'));
        assert_eq!(letter_grade(69), Some('D'));
        assert_eq!(letter_grade(60), Some('D'));
        assert_eq!(letter_grade(0), Some('F'));
    }

    #[test]
    fn every_invalid_score_is_none() {
        for score in 101..=u8::MAX {
            assert_eq!(letter_grade(score), None, "score {score}");
        }
    }

    #[test]
    fn grade_counts() {
        let a_count = (0..=100).filter(|&s| letter_grade(s) == Some('A')).count();
        let f_count = (0..=100).filter(|&s| letter_grade(s) == Some('F')).count();
        assert_eq!(a_count, 11);
        assert_eq!(f_count, 60);
    }
}
