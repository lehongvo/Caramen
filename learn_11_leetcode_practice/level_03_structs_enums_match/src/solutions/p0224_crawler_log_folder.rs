//! # Problem 0224: Crawler Log Folder
//!
//! - **Difficulty:** Easy
//! - **Topics:** parsing strings into an `enum`, `match` on `&str`, `saturating_sub`, `fold`
//! - **Source:** [LeetCode 1598. Crawler Log Folder](https://leetcode.com/problems/crawler-log-folder/)
//!
//! ## Problem
//!
//! A file-system browser starts in the **main** (root) folder. It records every navigation step in
//! `logs`, one string per step:
//!
//! - `"../"` goes up to the parent folder; if you are already in the main folder you stay there;
//! - `"./"` stays in the current folder;
//! - `"name/"` (any other string) enters the child folder `name`, which always exists.
//!
//! After all the steps, return the **minimum number of steps** needed to get back to the main
//! folder (each `"../"` is one step).
//!
//! Implement `Solution::min_operations(logs: Vec<String>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  logs = ["d1/", "d2/", "../", "d21/", "./"]
//! Output: 2      (main -> d1 -> d2 -> d1 -> d21 -> d21: two levels deep)
//!
//! Input:  logs = ["d1/", "d2/", "./", "d3/", "../", "d31/"]
//! Output: 3
//!
//! Input:  logs = ["d1/", "../", "../", "../"]
//! Output: 0      (going up from the main folder does nothing)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= logs.len() <= 10^3`
//! - `2 <= logs[i].len() <= 10`; folder names are lowercase letters and digits, followed by `/`.
//!
//! ## Hints
//!
//! 1. Only the **depth** matters, not the folder names.
//! 2. Model each log as `enum Step { Up, Stay, Down }` and fold the steps into the depth.
//! 3. `depth.saturating_sub(1)` stops at zero for an unsigned depth.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** parse each log into a `Step`; going `Down` adds one to the depth, `Up` removes one but
//! never goes below zero, and `Stay` changes nothing. The final depth is the answer.
//!
//! **Complexity:** `O(n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `Step::parse` matches the two special literals and sends every other string to `Down`
//!   with the `_` arm: here every input is valid, so the parser returns `Step`, not `Option<Step>`.
//! - `usize::saturating_sub` returns `0` instead of underflowing, which is exactly "stay in main".
//! - `i32::try_from(depth)` converts back to LeetCode's return type; depth is at most `10^3`.
// @solution-end

pub struct Solution;

// @solution-start remove
/// One navigation step.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Step {
    Up,
    Stay,
    Down,
}

impl Step {
    fn parse(log: &str) -> Step {
        match log {
            "../" => Step::Up,
            "./" => Step::Stay,
            _ => Step::Down,
        }
    }
}
// @solution-end

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        // @solution-start
        let depth =
            logs.iter()
                .map(|log| Step::parse(log))
                .fold(0usize, |depth, step| match step {
                    Step::Up => depth.saturating_sub(1),
                    Step::Stay => depth,
                    Step::Down => depth + 1,
                });
        // At most 10^3 logs, so the depth always fits in an `i32`.
        i32::try_from(depth).unwrap_or(i32::MAX)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logs(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::min_operations(logs(&["d1/", "d2/", "../", "d21/", "./"])),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::min_operations(logs(&["d1/", "d2/", "./", "d3/", "../", "d31/"])),
            3
        );
    }

    #[test]
    fn example_three_up_from_main() {
        assert_eq!(
            Solution::min_operations(logs(&["d1/", "../", "../", "../"])),
            0
        );
    }

    #[test]
    fn up_at_main_then_down() {
        assert_eq!(
            Solution::min_operations(logs(&["../", "./", "a/", "b/"])),
            2
        );
    }

    #[test]
    fn thousand_levels_deep() {
        let deep = vec!["x9/".to_string(); 1_000];
        assert_eq!(Solution::min_operations(deep), 1_000);
    }
}
