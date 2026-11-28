// ==================================================================
// PRACTICE 0038: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0038_digits_in_any_base.md
// Run the tests:          cargo test practice::p0038
// ==================================================================

//! # Problem 0038: Digits in Any Base
//!
//! - **Difficulty:** Easy
//! - **Topics:** `char::from_digit`, `char::to_digit`, `while` loops, `checked_mul`/`checked_add`, `?` on `Option`
//! - **Source:** Original · [`char::from_digit`](https://doc.rust-lang.org/std/primitive.char.html#method.from_digit)
//!
//! ## Problem
//!
//! Numbers can be written in any base from 2 to 36, using the digits `0-9` followed by the letters
//! `a-z` (so `a` is 10 and `z` is 35). Write two functions:
//!
//! 1. `to_base(n: u32, base: u32) -> Vec<char>` returns the digits of `n` in `base`, most
//!    significant first, using lowercase letters. Zero is written as `['0']`.
//! 2. `from_base(digits: &[char], base: u32) -> Option<u32>` parses digits back into a number. It
//!    accepts lowercase and uppercase letters. It returns `None` if the slice is empty, if a
//!    character is not a valid digit in `base`, or if the value does not fit in a `u32`.
//!
//! You may assume `2 <= base <= 36` in both functions.
//!
//! ## Examples
//!
//! ```text
//! to_base(10, 2)       -> ['1', '0', '1', '0']
//! to_base(255, 16)     -> ['f', 'f']
//! to_base(0, 7)        -> ['0']
//! to_base(35, 36)      -> ['z']
//!
//! from_base(&['f', 'F'], 16)       -> Some(255)
//! from_base(&['1', '2'], 2)        -> None          '2' is not a binary digit
//! from_base(&['1', '0', '0', '0', '0', '0', '0', '0', '0'], 16) -> None   16^8 > u32::MAX
//! ```
//!
//! ## Constraints
//!
//! - `2 <= base <= 36`
//! - `0 <= digits.len() <= 100`
//!
//! ## Hints
//!
//! 1. `n % base` is the last digit and `n / base` removes it. Collect digits, then reverse the vector.
//! 2. `char::from_digit(d, base)` returns `Some(char)` for `d < base`; `c.to_digit(base)` goes the
//!    other way and returns `None` for invalid characters.
//! 3. When parsing, `value = value * base + digit` can overflow: use `checked_mul` and `checked_add`.

pub fn to_base(n: u32, base: u32) -> Vec<char> {
    todo!("Write your solution here")
}

pub fn from_base(digits: &[char], base: u32) -> Option<u32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_base_examples() {
        assert_eq!(to_base(10, 2), vec!['1', '0', '1', '0']);
        assert_eq!(to_base(255, 16), vec!['f', 'f']);
        assert_eq!(to_base(0, 7), vec!['0']);
        assert_eq!(to_base(35, 36), vec!['z']);
    }

    #[test]
    fn to_base_maximum() {
        assert_eq!(to_base(u32::MAX, 16), vec!['f'; 8]);
        assert_eq!(to_base(u32::MAX, 2), vec!['1'; 32]);
    }

    #[test]
    fn from_base_examples() {
        assert_eq!(from_base(&['f', 'F'], 16), Some(255));
        assert_eq!(from_base(&['1', '0', '1', '0'], 2), Some(10));
        assert_eq!(from_base(&['z', 'z'], 36), Some(1295));
        assert_eq!(from_base(&['0', '0', '7'], 10), Some(7));
    }

    #[test]
    fn from_base_rejects_invalid_input() {
        assert_eq!(from_base(&[], 10), None);
        assert_eq!(from_base(&['1', '2'], 2), None);
        assert_eq!(from_base(&['g'], 16), None);
        assert_eq!(from_base(&['-', '1'], 10), None);
    }

    #[test]
    fn from_base_detects_overflow() {
        assert_eq!(from_base(&['f'; 8], 16), Some(u32::MAX));
        assert_eq!(
            from_base(&['1', '0', '0', '0', '0', '0', '0', '0', '0'], 16),
            None
        );
        let max_plus_one: Vec<char> = "4294967296".chars().collect();
        assert_eq!(from_base(&max_plus_one, 10), None);
    }

    #[test]
    fn round_trip_in_every_base() {
        for base in 2..=36 {
            for n in [0, 1, 2, 35, 36, 1000, 65_535, 123_456_789, u32::MAX] {
                assert_eq!(
                    from_base(&to_base(n, base), base),
                    Some(n),
                    "n={n}, base={base}"
                );
            }
        }
    }
}
