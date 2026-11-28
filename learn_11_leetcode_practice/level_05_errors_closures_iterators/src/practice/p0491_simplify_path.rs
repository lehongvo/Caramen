// ==================================================================
// PRACTICE 0491: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0491_simplify_path.md
// Run the tests:          cargo test practice::p0491
// ==================================================================

//! # Problem 0491: Simplify Path
//!
//! - **Difficulty:** Medium
//! - **Topics:** `split('/')`, `fold` into a `Vec` used as a stack, `match` on `&str`, `join`
//! - **Source:** [LeetCode 71. Simplify Path](https://leetcode.com/problems/simplify-path/)
//!
//! ## Problem
//!
//! Given an absolute Unix-style path (it starts with `'/'`), return its *canonical* form:
//!
//! - `"."` means the current directory and is dropped;
//! - `".."` goes up one directory (at the root it stays at the root);
//! - several consecutive slashes count as one;
//! - every other name, including names like `"..."` or `"..a"`, is a normal directory name.
//!
//! The canonical path starts with a single `'/'`, separates names with single slashes, and has no
//! trailing slash (except the root itself, `"/"`).
//!
//! Implement `Solution::simplify_path(path: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  path = "/home/"                              Output: "/home"
//! Input:  path = "/home//foo/"                         Output: "/home/foo"
//! Input:  path = "/home/user/Documents/../Pictures"    Output: "/home/user/Pictures"
//! Input:  path = "/../"                                Output: "/"
//! Input:  path = "/.../a/../b/c/../d/./"               Output: "/.../b/d"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= path.len() <= 3000`
//! - `path` contains English letters, digits, `'.'`, `'/'` and `'_'`, and is a valid absolute path.
//!
//! ## Hints
//!
//! 1. `path.split('/')` yields the parts between slashes; empty parts come from repeated slashes.
//! 2. Keep the directories in a stack: push names, pop on `".."`, ignore `""` and `"."`.
//! 3. `fold(Vec::new(), |mut stack, part| { ...; stack })` builds the stack in one expression.

pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(path: &str) -> String {
        Solution::simplify_path(path.to_string())
    }

    #[test]
    fn example_trailing_slash() {
        assert_eq!(run("/home/"), "/home");
    }

    #[test]
    fn example_double_slash() {
        assert_eq!(run("/home//foo/"), "/home/foo");
    }

    #[test]
    fn example_parent_directory() {
        assert_eq!(
            run("/home/user/Documents/../Pictures"),
            "/home/user/Pictures"
        );
    }

    #[test]
    fn example_above_root() {
        assert_eq!(run("/../"), "/");
        assert_eq!(run("/"), "/");
    }

    #[test]
    fn example_three_dots_is_a_name() {
        assert_eq!(run("/.../a/../b/c/../d/./"), "/.../b/d");
        assert_eq!(run("/a/..b/.c/"), "/a/..b/.c");
    }
}
