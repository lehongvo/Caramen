// ==================================================================
// PRACTICE 0569: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0569_sentence_similarity_iii.md
// Run the tests:          cargo test practice::p0569
// ==================================================================

//! # Problem 0569: Sentence Similarity III
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Vec<&str>` word slices, comparing slices with `==`, common prefix/suffix with `zip` and `take_while`
//! - **Source:** [LeetCode 1813. Sentence Similarity III](https://leetcode.com/problems/sentence-similarity-iii/)
//!
//! ## Problem
//!
//! Two sentences (words separated by single spaces) are **similar** if you can insert an arbitrary
//! sentence, possibly empty, at **one** position of one of them (between two words, at the start or at
//! the end) so that the two become equal. Insertions must consist of whole words: `"Hello"` and
//! `"Helloworld"` are not similar.
//!
//! Return whether `sentence1` and `sentence2` are similar.
//!
//! Implement `Solution::are_sentences_similar(sentence1: String, sentence2: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  "My name is Haley", "My Haley"        Output: true   (insert "name is" into the second)
//! Input:  "of", "A lot of words"                Output: false  (would need two insertions)
//! Input:  "Eating right now", "Eating"          Output: true   (insert "right now" at the end)
//! Input:  "Luky", "Lucccky"                     Output: false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= sentence1.len(), sentence2.len() <= 100`
//! - Sentences contain English letters and single spaces, with no leading or trailing spaces.
//!
//! ## Hints
//!
//! 1. Let `short` be the sentence with fewer words. The insertion goes into the other one, so `short`
//!    must equal a prefix of `long` followed by a suffix of `long`.
//! 2. Count the common words at the start (`p`) and at the end (`s`). The answer is `p + s >=
//!    short.len()`; make sure not to count the same word of `short` twice.

pub struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn similar(a: &str, b: &str) -> bool {
        Solution::are_sentences_similar(a.to_string(), b.to_string())
    }

    #[test]
    fn example_inputs() {
        assert!(similar("My name is Haley", "My Haley"));
        assert!(!similar("of", "A lot of words"));
        assert!(similar("Eating right now", "Eating"));
        assert!(!similar("Luky", "Lucccky"));
    }

    #[test]
    fn insertion_at_the_start_or_end() {
        assert!(similar("now", "right now"));
        assert!(similar("right now", "now"));
        assert!(similar("a", "a"));
        assert!(!similar("Hello", "Helloworld"));
    }

    #[test]
    fn repeated_words_are_not_double_counted() {
        assert!(similar("A A", "A A A"));
        assert!(!similar("A B", "A C B D"));
        assert!(similar("x y z", "x y q z"));
        assert!(similar("c h p Ny", "c BDQ r h p Ny")); // insert "BDQ r" after "c"
        assert!(!similar("A B C", "A X B Y C"));
    }

    #[test]
    fn longer_sentences() {
        let base: Vec<String> = (0..20).map(|i| format!("w{i}")).collect();
        let mut with_gap = base[..10].to_vec();
        with_gap.extend(["extra".to_string(), "words".to_string()]);
        with_gap.extend_from_slice(&base[10..]);
        assert!(similar(&base.join(" "), &with_gap.join(" ")));
        let mut two_gaps = with_gap.clone();
        two_gaps.insert(3, "oops".to_string());
        assert!(!similar(&base.join(" "), &two_gaps.join(" ")));
    }
}
