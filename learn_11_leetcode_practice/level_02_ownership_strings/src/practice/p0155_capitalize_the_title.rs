// ==================================================================
// PRACTICE 0155: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0155_capitalize_the_title.md
// Run the tests:          cargo test practice::p0155
// ==================================================================

//! # Problem 0155: Capitalize the Title
//!
//! - **Difficulty:** Easy
//! - **Topics:** case conversion on sub-slices (`out[a..b].make_ascii_lowercase()`), `split(' ')`, building one `String`
//! - **Source:** [LeetCode 2129. Capitalize the Title](https://leetcode.com/problems/capitalize-the-title/)
//!
//! ## Problem
//!
//! A title is made of words separated by single spaces. Capitalize it with these rules:
//!
//! - a word of length 1 or 2 is written entirely in lowercase;
//! - a longer word gets an uppercase first letter and lowercase remaining letters.
//!
//! Return the capitalized title, with the same words in the same order separated by single spaces.
//!
//! Implement `Solution::capitalize_title(title: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  title = "capiTalIze tHe titLe"
//! Output: "Capitalize The Title"
//!
//! Input:  title = "First leTTeR of EACH Word"
//! Output: "First Letter of Each Word"
//!
//! Input:  title = "i lOve leetcode"
//! Output: "i Love Leetcode"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= title.len() <= 100`
//! - `title` consists of English letters and single spaces, with no leading or trailing space.
//!
//! ## Hints
//!
//! 1. Copy each word into the output, then fix its case in place: `out[start..]` is a `&mut str`
//!    on which you can call `make_ascii_lowercase()`.
//! 2. For long words, upper-case just `out[start..start + 1]`.

pub struct Solution;

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(title: &str) -> String {
        Solution::capitalize_title(title.to_string())
    }

    #[test]
    fn example_three_long_words() {
        assert_eq!(run("capiTalIze tHe titLe"), "Capitalize The Title");
    }

    #[test]
    fn example_short_word_in_the_middle() {
        assert_eq!(
            run("First leTTeR of EACH Word"),
            "First Letter of Each Word"
        );
    }

    #[test]
    fn example_one_letter_word() {
        assert_eq!(run("i lOve leetcode"), "i Love Leetcode");
    }

    #[test]
    fn only_short_words() {
        assert_eq!(run("A Bc DE"), "a bc de");
    }

    #[test]
    fn three_letter_boundary() {
        assert_eq!(run("aBC xY"), "Abc xy");
    }

    #[test]
    fn longest_title() {
        let title = "wORD ".repeat(20);
        let result = run(title.trim_end());
        assert_eq!(result, vec!["Word"; 20].join(" "));
    }
}
