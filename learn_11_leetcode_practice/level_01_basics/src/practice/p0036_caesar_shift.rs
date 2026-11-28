// ==================================================================
// PRACTICE 0036: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0036_caesar_shift.md
// Run the tests:          cargo test practice::p0036
// ==================================================================

//! # Problem 0036: Caesar Shift
//!
//! - **Difficulty:** Easy
//! - **Topics:** `char` arithmetic through `u8`, `rem_euclid`, avoiding overflow, `&[char]` to `Vec<char>`
//! - **Source:** Original · [`i32::rem_euclid`](https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid)
//!
//! ## Problem
//!
//! The Caesar cipher replaces every letter with the letter `shift` positions later in the alphabet,
//! wrapping around after `z`. Write two functions:
//!
//! 1. `caesar_shift(c: char, shift: i32) -> char` shifts one character:
//!    - ASCII lowercase letters stay lowercase and ASCII uppercase letters stay uppercase;
//!    - after `'z'` comes `'a'` again (and after `'Z'` comes `'A'`);
//!    - a negative `shift` moves backwards (`'a'` shifted by `-1` is `'z'`);
//!    - every other character is returned unchanged.
//! 2. `caesar_encode(text: &[char], shift: i32) -> Vec<char>` shifts every character of `text`.
//!
//! `shift` can be **any** `i32`, including `i32::MIN` and `i32::MAX`.
//!
//! ## Examples
//!
//! ```text
//! caesar_shift('a', 3)    -> 'd'
//! caesar_shift('y', 3)    -> 'b'      y -> z -> a -> b
//! caesar_shift('B', -3)   -> 'Y'
//! caesar_shift('m', 26)   -> 'm'      a full turn
//! caesar_shift('!', 5)    -> '!'
//!
//! caesar_encode(&['H', 'i', '!'], 1) -> ['I', 'j', '!']
//! ```
//!
//! ## Constraints
//!
//! - `i32::MIN <= shift <= i32::MAX`
//! - `0 <= text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. Work with the offset from `'a'` (`0..26`), shift it, and convert back.
//! 2. `shift.rem_euclid(26)` reduces any shift (even a negative one) to `0..26`.
//! 3. Reduce the shift **before** adding it to the offset, or `offset + i32::MAX` overflows.

pub fn caesar_shift(c: char, shift: i32) -> char {
    todo!("Write your solution here")
}

pub fn caesar_encode(text: &[char], shift: i32) -> Vec<char> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_shifts() {
        assert_eq!(caesar_shift('a', 3), 'd');
        assert_eq!(caesar_shift('y', 3), 'b');
        assert_eq!(caesar_shift('B', -3), 'Y');
        assert_eq!(caesar_shift('m', 26), 'm');
        assert_eq!(caesar_shift('a', -1), 'z');
    }

    #[test]
    fn non_letters_unchanged() {
        for c in ['!', ' ', '5', 'Ω', '[', '`'] {
            assert_eq!(caesar_shift(c, 5), c);
        }
    }

    #[test]
    fn extreme_shifts_do_not_overflow() {
        // i32::MAX = 2147483647 = 26 * 82595524 + 23, and i32::MIN leaves remainder 2.
        assert_eq!(caesar_shift('a', i32::MAX), 'x');
        assert_eq!(caesar_shift('z', i32::MAX), 'w');
        assert_eq!(caesar_shift('a', i32::MIN), 'c');
        assert_eq!(caesar_shift('Z', i32::MIN), 'B');
    }

    #[test]
    fn encode_text() {
        assert_eq!(caesar_encode(&['H', 'i', '!'], 1), vec!['I', 'j', '!']);
        assert_eq!(caesar_encode(&[], 7), Vec::<char>::new());
    }

    #[test]
    fn shifting_back_decodes() {
        let text: Vec<char> = "Hello, World! xyz ABC".chars().collect();
        for shift in [-100, -27, -1, 0, 1, 13, 25, 1000] {
            let encoded = caesar_encode(&text, shift);
            assert_eq!(caesar_encode(&encoded, -shift), text, "shift {shift}");
        }
    }
}
