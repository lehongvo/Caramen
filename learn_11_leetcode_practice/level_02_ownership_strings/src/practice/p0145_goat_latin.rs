// ==================================================================
// PRACTICE 0145: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0145_goat_latin.md
// Run the tests:          cargo test practice::p0145
// ==================================================================

//! # Problem 0145: Goat Latin
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split(' ')`, `enumerate`, `split_at`, `push_str`, `push`, building one output `String`
//! - **Source:** [LeetCode 824. Goat Latin](https://leetcode.com/problems/goat-latin/)
//!
//! ## Problem
//!
//! A sentence is made of words separated by single spaces; every word consists of English letters
//! only. Convert it to "Goat Latin" by transforming each word as follows:
//!
//! 1. If the word starts with a vowel (`a`, `e`, `i`, `o`, `u`, in either case), append `"ma"`.
//! 2. Otherwise, move the first letter to the end of the word, then append `"ma"`.
//! 3. Finally, append as many letters `'a'` as the word's position in the sentence, counting from 1:
//!    the first word gets `"a"`, the second `"aa"`, and so on.
//!
//! Return the transformed words joined by single spaces.
//!
//! Implement `Solution::to_goat_latin(sentence: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  sentence = "I speak Goat Latin"
//! Output: "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
//!
//! Input:  sentence = "The quick brown fox"
//! Output: "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= sentence.len() <= 150`
//! - `sentence` has only English letters and single spaces, with no leading or trailing space.
//!
//! ## Hints
//!
//! 1. `sentence.split(' ').enumerate()` gives `(index, word)`; the number of extra `'a'`s is
//!    `index + 1`.
//! 2. All characters are ASCII, so `word.split_at(1)` safely separates the first letter from the rest.
//! 3. Build a single output `String` and push a space before every word except the first.

pub struct Solution;

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn goat(sentence: &str) -> String {
        Solution::to_goat_latin(sentence.to_string())
    }

    #[test]
    fn example_speak_goat_latin() {
        assert_eq!(
            goat("I speak Goat Latin"),
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
        );
    }

    #[test]
    fn example_quick_brown_fox() {
        assert_eq!(
            goat("The quick brown fox"),
            "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa"
        );
    }

    #[test]
    fn full_leetcode_sentence() {
        assert_eq!(
            goat("The quick brown fox jumped over the lazy dog"),
            "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa \
             hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
        );
    }

    #[test]
    fn single_letter_words() {
        assert_eq!(goat("a b"), "amaa bmaaa");
        assert_eq!(goat("U"), "Umaa");
    }

    #[test]
    fn many_words() {
        let sentence = vec!["x"; 75].join(" ");
        let result = goat(&sentence);
        let last = result.split(' ').next_back().expect("75 words");
        assert_eq!(last, format!("xma{}", "a".repeat(75)));
    }
}
