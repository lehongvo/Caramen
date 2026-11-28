// ==================================================================
// PRACTICE 0492: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0492_longest_absolute_file_path.md
// Run the tests:          cargo test practice::p0492
// ==================================================================

//! # Problem 0492: Longest Absolute File Path
//!
//! - **Difficulty:** Medium
//! - **Topics:** `split('\n')`, `trim_start_matches` to measure depth, `Vec::truncate` as a depth stack, `last().copied()`
//! - **Source:** [LeetCode 388. Longest Absolute File Path](https://leetcode.com/problems/longest-absolute-file-path/)
//!
//! ## Problem
//!
//! A file system is described by a string where each line is a file or directory name, and the
//! number of leading tab characters `'\t'` gives its depth (a child is one tab deeper than its
//! parent, and appears after it):
//!
//! ```text
//! dir
//! \tsubdir1
//! \tsubdir2
//! \t\tfile.ext
//! ```
//!
//! (written as the single string `"dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext"`). A name containing a
//! `'.'` is a **file**; any other name is a directory. The absolute path of `file.ext` above is
//! `"dir/subdir2/file.ext"`, of length 20.
//!
//! Return the length of the longest absolute path to a **file**, or `0` if there is no file.
//!
//! Implement `Solution::length_longest_path(input: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext"
//! Output: 20
//!
//! Input:  "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
//! Output: 32          ("dir/subdir2/subsubdir2/file2.ext")
//!
//! Input:  "a"
//! Output: 0           (a directory, no file)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= input.len() <= 10^4`
//! - `input` contains letters, digits, spaces, `'.'`, `'\n'` and `'\t'`; it is a valid description,
//!   and every name is non-empty.
//!
//! ## Hints
//!
//! 1. For a line, `name = line.trim_start_matches('\t')` and `depth = line.len() - name.len()`.
//! 2. Keep `lengths[d]` = length of the path of the current directory at depth `d`, **including** a
//!    trailing `'/'`. Before handling a line at depth `d`, drop everything deeper with
//!    `truncate(d)`.

pub struct Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &str) -> i32 {
        Solution::length_longest_path(input.to_string())
    }

    #[test]
    fn example_one_file() {
        assert_eq!(run("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext"), 20);
    }

    #[test]
    fn example_deeper_file_wins() {
        assert_eq!(
            run(
                "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
            ),
            32
        );
    }

    #[test]
    fn example_no_file() {
        assert_eq!(run("a"), 0);
        assert_eq!(run("a\n\tb\n\t\tc"), 0);
    }

    #[test]
    fn top_level_files_and_spaces() {
        assert_eq!(run("file.txt"), 8);
        assert_eq!(run("a\n\tb.txt\nlonger name.md"), 14);
        assert_eq!(run("dir\n\tx.y\nd2\n\tlong_file.txt"), 16);
    }
}
