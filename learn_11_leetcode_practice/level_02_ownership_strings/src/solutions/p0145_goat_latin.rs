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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** transform the words one by one, appending directly to a single output `String` instead
//! of creating a `String` per word. A vowel check on the first letter decides between keeping the
//! word as is and rotating its first letter to the end.
//!
//! **Complexity:** `O(n + w^2)` time and space for `n` characters and `w` words (the `'a'` suffixes
//! alone have total length `w (w + 1) / 2`).
//!
//! **Rust concepts:**
//! - `split_at(1)` returns two `&str` slices of the word; it would panic on a non-boundary index,
//!   which cannot happen with ASCII letters.
//! - `str::contains(char)` on `"aeiouAEIOU"` is a compact membership test (a `matches!` on the
//!   first byte works too).
//! - `push_str` appends slices, `push` appends one `char`; the output owns everything it contains.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        // @solution-start
        let mut out = String::with_capacity(sentence.len() * 2);
        for (i, word) in sentence.split(' ').enumerate() {
            if i > 0 {
                out.push(' ');
            }
            // Words are non-empty ASCII, so splitting after the first byte is safe.
            let (first, rest) = word.split_at(1);
            if "aeiouAEIOU".contains(first) {
                out.push_str(word);
            } else {
                out.push_str(rest);
                out.push_str(first);
            }
            out.push_str("ma");
            for _ in 0..=i {
                out.push('a');
            }
        }
        out
        // @solution-end
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
