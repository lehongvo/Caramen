//! # Problem 0014: Exactly One True
//!
//! - **Difficulty:** Easy
//! - **Topics:** `bool`, `&&`, `||`, `!`, `^`, `bool as u8`
//! - **Source:** Original · [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type)
//!
//! ## Problem
//!
//! Three sensors each report `true` (alarm) or `false` (quiet). Write two functions:
//!
//! 1. `exactly_one(a: bool, b: bool, c: bool) -> bool`: `true` when **exactly one** of the three
//!    sensors reports an alarm.
//! 2. `majority(a: bool, b: bool, c: bool) -> bool`: `true` when **at least two** sensors report
//!    an alarm.
//!
//! ## Examples
//!
//! ```text
//! exactly_one(true,  false, false) -> true
//! exactly_one(true,  true,  false) -> false
//! exactly_one(true,  true,  true)  -> false    note: a ^ b ^ c would wrongly say true here
//! exactly_one(false, false, false) -> false
//!
//! majority(true,  true,  false) -> true
//! majority(true,  false, false) -> false
//! ```
//!
//! ## Constraints
//!
//! - No special constraints. There are only 8 possible inputs, and the tests check all of them.
//!
//! ## Hints
//!
//! 1. `true as u8` is `1` and `false as u8` is `0`, so you can count the `true` values.
//! 2. Or describe it with logic: `(a && !b && !c) || ...`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** convert each `bool` to `0` or `1` with `as u8`, add them up and compare the count:
//! exactly one is `count == 1`, majority is `count >= 2`. A purely logical version of `majority` is
//! `(a && b) || (a && c) || (b && c)`.
//!
//! **Rust concepts:**
//! - A `bool` is its own type; Rust never treats integers as booleans or booleans as integers
//!   implicitly. `as u8` is the explicit conversion (`u8::from(a)` works too).
//! - `^` on two `bool` values is "exclusive or". It is tempting for "exactly one", but `true ^ true ^ true`
//!   is `true`, so XOR only answers "an odd number of them are true".
//! - A small private helper (`count`) keeps both public functions one line long.
// @solution-end

// @solution-start remove
fn count(a: bool, b: bool, c: bool) -> u8 {
    a as u8 + b as u8 + c as u8
}
// @solution-end

pub fn exactly_one(a: bool, b: bool, c: bool) -> bool {
    // @solution-start
    count(a, b, c) == 1
    // @solution-end
}

pub fn majority(a: bool, b: bool, c: bool) -> bool {
    // @solution-start
    count(a, b, c) >= 2
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [bool; 2] = [false, true];

    #[test]
    fn exactly_one_examples() {
        assert!(exactly_one(true, false, false));
        assert!(exactly_one(false, false, true));
        assert!(!exactly_one(true, true, false));
        assert!(!exactly_one(true, true, true));
        assert!(!exactly_one(false, false, false));
    }

    #[test]
    fn majority_examples() {
        assert!(majority(true, true, false));
        assert!(majority(true, true, true));
        assert!(!majority(true, false, false));
        assert!(!majority(false, false, false));
    }

    #[test]
    fn full_truth_table() {
        for a in ALL {
            for b in ALL {
                for c in ALL {
                    let ones = [a, b, c].iter().filter(|&&x| x).count();
                    assert_eq!(exactly_one(a, b, c), ones == 1, "{a} {b} {c}");
                    assert_eq!(majority(a, b, c), ones >= 2, "{a} {b} {c}");
                }
            }
        }
    }
}
