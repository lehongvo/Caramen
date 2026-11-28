//! # Problem 0981: Text Justification
//!
//! - **Difficulty:** Hard
//! - **Topics:** greedy line packing, distributing spaces with `/` and `%`, `String::with_capacity`, `" ".repeat(n)`
//! - **Source:** [LeetCode 68. Text Justification](https://leetcode.com/problems/text-justification/)
//!
//! ## Problem
//!
//! Format `words` into lines of exactly `max_width` characters:
//!
//! - Pack words greedily: put as many words on a line as fit with at least one space between them.
//! - Every line except the last is **fully justified**: the extra spaces are distributed between the
//!   words as evenly as possible, and when they do not divide evenly, the **leftmost** gaps get one
//!   more space. A line with a single word is left-justified (padded with spaces on the right).
//! - The **last** line is left-justified: words separated by single spaces, padded on the right.
//!
//! Implement `Solution::full_justify(words: Vec<String>, max_width: i32) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  words = ["This", "is", "an", "example", "of", "text", "justification."], max_width = 16
//! Output: ["This    is    an",
//!          "example  of text",
//!          "justification.  "]
//!
//! Input:  words = ["What", "must", "be", "acknowledgment", "shall", "be"], max_width = 16
//! Output: ["What   must   be",
//!          "acknowledgment  ",
//!          "shall be        "]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= words.len() <= 300`, `1 <= words[i].len() <= 20`, `words[i].len() <= max_width <= 100`
//! - Words consist of non-space ASCII characters.
//!
//! ## Hints
//!
//! 1. Find the words of the next line: keep adding while `letters + (count of words) + word.len()`
//!    fits.
//! 2. With `gaps = count - 1` and `spaces = max_width - letters`, gap `g` gets
//!    `spaces / gaps + (1 if g < spaces % gaps)`.
//! 3. Handle "last line" and "single word" with the same left-justify helper.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** walk through the words, cutting a line as soon as the next word would not fit. For a
//! middle line with several words, spread `max_width - letters` spaces over the gaps, giving the
//! remainder to the leftmost gaps. Last lines and one-word lines are joined with single spaces and
//! padded on the right.
//!
//! **Complexity:** `O(total characters)` time and space.
//!
//! **Rust concepts:**
//! - Slicing `&words[start..end]` passes one line's words to the helpers without copying.
//! - `format!("{:<width$}", text, width = w)` pads on the right; here an explicit `repeat` keeps the
//!   arithmetic visible.
// @solution-end

pub struct Solution;

// @solution-start remove
fn left_justified(words: &[String], width: usize) -> String {
    let mut line = words.join(" ");
    let padding = width - line.len();
    line.push_str(&" ".repeat(padding));
    line
}

fn fully_justified(words: &[String], width: usize) -> String {
    let letters: usize = words.iter().map(String::len).sum();
    let gaps = words.len() - 1;
    let spaces = width - letters;
    let mut line = String::with_capacity(width);
    for (g, word) in words.iter().enumerate() {
        line.push_str(word);
        if g < gaps {
            let extra = usize::from(g < spaces % gaps);
            line.push_str(&" ".repeat(spaces / gaps + extra));
        }
    }
    line
}
// @solution-end

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        // @solution-start
        let width = max_width as usize;
        let mut lines = Vec::new();
        let mut start = 0;
        while start < words.len() {
            let mut end = start + 1;
            let mut letters = words[start].len();
            while end < words.len() && letters + (end - start) + words[end].len() <= width {
                letters += words[end].len();
                end += 1;
            }
            let line_words = &words[start..end];
            let is_last = end == words.len();
            lines.push(if is_last || line_words.len() == 1 {
                left_justified(line_words, width)
            } else {
                fully_justified(line_words, width)
            });
            start = end;
        }
        lines
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(words: &[&str], width: i32) -> Vec<String> {
        Solution::full_justify(words.iter().map(|w| w.to_string()).collect(), width)
    }

    #[test]
    fn examples() {
        assert_eq!(
            run(
                &[
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ],
                16
            ),
            ["This    is    an", "example  of text", "justification.  "]
        );
        assert_eq!(
            run(&["What", "must", "be", "acknowledgment", "shall", "be"], 16),
            ["What   must   be", "acknowledgment  ", "shall be        "]
        );
    }

    #[test]
    fn uneven_gaps_favour_the_left() {
        let words = [
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do",
        ];
        assert_eq!(
            run(&words, 20),
            [
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ]
        );
    }

    #[test]
    fn single_words_and_exact_fits() {
        assert_eq!(run(&["a"], 1), ["a"]);
        assert_eq!(run(&["a", "b"], 3), ["a b"]);
        assert_eq!(run(&["abc", "d"], 3), ["abc", "d  "]);
    }

    #[test]
    fn every_line_has_the_width() {
        let words: Vec<String> = (0..300).map(|i| "w".repeat(i % 20 + 1)).collect();
        let refs: Vec<&str> = words.iter().map(String::as_str).collect();
        let lines = run(&refs, 37);
        assert!(lines.iter().all(|l| l.len() == 37));
        let rebuilt: Vec<&str> = lines.iter().flat_map(|l| l.split_whitespace()).collect();
        assert_eq!(rebuilt, refs);
    }
}
