// ==================================================================
// PRACTICE 0035: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0035_alphabet_position.md
// Run the tests:          cargo test practice::p0035
// ==================================================================

//! # Problem 0035: Alphabet Position
//!
//! - **Difficulty:** Easy
//! - **Topics:** `char as u8`, `u8 as char`, byte literals `b'a'`, `Option`, `match` on ranges
//! - **Source:** Original · [`char` primitive](https://doc.rust-lang.org/std/primitive.char.html)
//!
//! ## Problem
//!
//! Number the letters of the English alphabet from 1 (`a`) to 26 (`z`). Write two functions:
//!
//! 1. `alphabet_position(c: char) -> Option<u8>` returns the position of an ASCII letter in either
//!    case (`'a'` and `'A'` are both `1`, `'z'` and `'Z'` are both `26`), or `None` for any other
//!    character.
//! 2. `letter_at(position: u8, uppercase: bool) -> Option<char>` is the inverse: it returns the
//!    letter at `position` (in upper or lower case as requested), or `None` if `position` is not in
//!    `1..=26`.
//!
//! ## Examples
//!
//! ```text
//! alphabet_position('a') -> Some(1)
//! alphabet_position('E') -> Some(5)
//! alphabet_position('z') -> Some(26)
//! alphabet_position('3') -> None
//!
//! letter_at(1, false)    -> Some('a')
//! letter_at(26, true)    -> Some('Z')
//! letter_at(0, true)     -> None
//! letter_at(27, false)   -> None
//! ```
//!
//! ## Constraints
//!
//! - Any `char` / any `u8`.
//!
//! ## Hints
//!
//! 1. `b'a'` is the byte value of `'a'` (97), with type `u8`. Letters are consecutive in ASCII.
//! 2. `'e' as u8 - b'a'` is `4`. Check that `c` is an ASCII letter **before** casting: `as u8` on a
//!    non-ASCII `char` silently keeps only the lowest 8 bits.
//! 3. `some_u8 as char` turns a byte back into a character.

pub fn alphabet_position(c: char) -> Option<u8> {
    todo!("Write your solution here")
}

pub fn letter_at(position: u8, uppercase: bool) -> Option<char> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_of_letters() {
        assert_eq!(alphabet_position('a'), Some(1));
        assert_eq!(alphabet_position('E'), Some(5));
        assert_eq!(alphabet_position('z'), Some(26));
        assert_eq!(alphabet_position('Z'), Some(26));
    }

    #[test]
    fn non_letters_have_no_position() {
        for c in ['3', ' ', '@', '[', '`', '{', 'Ω'] {
            assert_eq!(alphabet_position(c), None, "{c:?}");
        }
        // 'Ł' is U+0141; its lowest byte is 0x41, which is 'A'. A careless `as u8` would say 1.
        assert_eq!(alphabet_position('\u{141}'), None);
    }

    #[test]
    fn letters_at_positions() {
        assert_eq!(letter_at(1, false), Some('a'));
        assert_eq!(letter_at(26, true), Some('Z'));
        assert_eq!(letter_at(13, true), Some('M'));
    }

    #[test]
    fn invalid_positions() {
        assert_eq!(letter_at(0, true), None);
        assert_eq!(letter_at(27, false), None);
        assert_eq!(letter_at(u8::MAX, false), None);
    }

    #[test]
    fn round_trip_for_every_position() {
        for p in 1..=26 {
            for upper in [false, true] {
                let c = letter_at(p, upper).expect("valid position");
                assert_eq!(alphabet_position(c), Some(p));
                assert_eq!(c.is_ascii_uppercase(), upper);
            }
        }
    }
}
