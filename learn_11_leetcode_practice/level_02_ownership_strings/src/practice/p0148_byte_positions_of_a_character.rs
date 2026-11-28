// ==================================================================
// PRACTICE 0148: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0148_byte_positions_of_a_character.md
// Run the tests:          cargo test practice::p0148
// ==================================================================

//! # Problem 0148: Byte Positions of a Character
//!
//! - **Difficulty:** Easy
//! - **Topics:** `char_indices`, byte offsets vs character positions, `enumerate`, UTF-8
//! - **Source:** Original · [`str::char_indices`](https://doc.rust-lang.org/std/primitive.str.html#method.char_indices) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! In Rust, every string index is a **byte offset**. Once a string contains non-ASCII characters,
//! "the 5th character" and "byte 5" are different things.
//!
//! Implement `find_positions(text: &str, target: char) -> Vec<(usize, usize)>`. For every occurrence
//! of `target` in `text`, from left to right, the result contains the pair
//! `(char_position, byte_offset)`:
//!
//! - `char_position` is the index of the occurrence when counting characters (`chars()`), from 0;
//! - `byte_offset` is the index of its first byte in `text`, so `text[byte_offset..]` starts with
//!   `target`.
//!
//! Return an empty vector if `target` does not occur.
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "banana", target = 'a'
//! Output: [(1, 1), (3, 3), (5, 5)]        (ASCII: both indices agree)
//!
//! Input:  text = "über alles", target = 'l'
//! Output: [(6, 7), (7, 8)]                ('ü' takes 2 bytes, so bytes run one ahead)
//!
//! Input:  text = "日本の本", target = '本'
//! Output: [(1, 3), (3, 9)]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `text.char_indices()` yields `(byte_offset, char)` for every character.
//! 2. Add `.enumerate()` on top of it to also get the character position.

pub fn find_positions(text: &str, target: char) -> Vec<(usize, usize)> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ascii() {
        assert_eq!(find_positions("banana", 'a'), vec![(1, 1), (3, 3), (5, 5)]);
    }

    #[test]
    fn example_two_byte_char_before() {
        assert_eq!(find_positions("über alles", 'l'), vec![(6, 7), (7, 8)]);
    }

    #[test]
    fn example_three_byte_chars() {
        assert_eq!(find_positions("日本の本", '本'), vec![(1, 3), (3, 9)]);
    }

    #[test]
    fn not_found_and_empty() {
        assert!(find_positions("banana", 'z').is_empty());
        assert!(find_positions("", 'a').is_empty());
    }

    #[test]
    fn byte_offsets_are_valid_slice_starts() {
        let text = "a🦀b🦀c";
        let positions = find_positions(text, '🦀');
        assert_eq!(positions, vec![(1, 1), (3, 6)]);
        for (_, offset) in positions {
            assert!(text[offset..].starts_with('🦀'));
        }
    }

    #[test]
    fn large_input() {
        let text = "üa".repeat(30_000);
        let positions = find_positions(&text, 'a');
        assert_eq!(positions.len(), 30_000);
        assert_eq!(positions[29_999], (59_999, 89_999));
    }
}
