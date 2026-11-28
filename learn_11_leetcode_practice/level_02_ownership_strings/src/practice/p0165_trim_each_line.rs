// ==================================================================
// PRACTICE 0165: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0165_trim_each_line.md
// Run the tests:          cargo test practice::p0165
// ==================================================================

//! # Problem 0165: Trim Each Line
//!
//! - **Difficulty:** Easy
//! - **Topics:** `lines()`, `trim`, returning `Vec<&str>` that borrows from the input
//! - **Source:** Original · [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines) · [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim)
//!
//! ## Problem
//!
//! Implement `trim_lines(text: &str) -> Vec<&str>`, which splits `text` into lines, removes the
//! leading and trailing whitespace of every line, and drops the lines that become empty. The
//! remaining lines are returned in their original order.
//!
//! Lines are separated by `"\n"` or `"\r\n"`. Every returned `&str` must be a slice of `text`: the
//! function copies no text at all.
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "  first line  \n\n\tsecond\n   \nthird"
//! Output: ["first line", "second", "third"]
//!
//! Input:  text = "a\r\nb\r\n"
//! Output: ["a", "b"]
//!
//! Input:  text = ""
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `text.lines()` yields every line without its line terminator (`"\n"` or `"\r\n"`).
//! 2. `trim()` returns a sub-slice; `filter(|line| !line.is_empty())` drops blank lines.

pub fn trim_lines(text: &str) -> Vec<&str> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed_lines() {
        assert_eq!(
            trim_lines("  first line  \n\n\tsecond\n   \nthird"),
            vec!["first line", "second", "third"]
        );
    }

    #[test]
    fn example_windows_line_endings() {
        assert_eq!(trim_lines("a\r\nb\r\n"), vec!["a", "b"]);
    }

    #[test]
    fn example_empty() {
        assert!(trim_lines("").is_empty());
    }

    #[test]
    fn only_blank_lines() {
        assert!(trim_lines("\n \n\t\n").is_empty());
    }

    #[test]
    fn results_point_into_the_text() {
        let text = String::from("  alpha\nbeta  ");
        let lines = trim_lines(&text);
        assert_eq!(lines, vec!["alpha", "beta"]);
        assert_eq!(lines[0].as_ptr(), text[2..].as_ptr());
        assert_eq!(lines[1].as_ptr(), text[8..].as_ptr());
    }

    #[test]
    fn large_input() {
        let text = "  item  \n\n".repeat(10_000);
        let lines = trim_lines(&text);
        assert_eq!(lines.len(), 10_000);
        assert!(lines.iter().all(|&l| l == "item"));
    }
}
