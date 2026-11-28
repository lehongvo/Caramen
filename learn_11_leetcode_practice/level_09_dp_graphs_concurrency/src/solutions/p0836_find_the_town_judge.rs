//! # Problem 0836: Find the Town Judge
//!
//! - **Difficulty:** Easy
//! - **Topics:** directed graphs from an edge list, in-degree and out-degree, `position`
//! - **Source:** [LeetCode 997. Find the Town Judge](https://leetcode.com/problems/find-the-town-judge/)
//!
//! ## Problem
//!
//! A town has `n` people labelled `1` to `n`. `trust[i] = [a, b]` means person `a` trusts person
//! `b` (a directed edge `a -> b`). The **town judge**, if one exists, is the person who:
//!
//! 1. trusts nobody (out-degree `0`), and
//! 2. is trusted by everybody else (in-degree `n - 1`).
//!
//! At most one person can satisfy both rules. Return the judge's label, or `-1` if there is none.
//!
//! Implement `Solution::find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 2, trust = [[1, 2]]
//! Output: 2
//!
//! Input:  n = 3, trust = [[1, 3], [2, 3]]
//! Output: 3
//!
//! Input:  n = 3, trust = [[1, 3], [2, 3], [3, 1]]
//! Output: -1
//! Explanation: person 3 trusts person 1, so 3 cannot be the judge.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 1000`
//! - `0 <= trust.len() <= 10^4`, `trust[i].len() == 2`
//! - `1 <= a, b <= n`, `a != b`, and all pairs are distinct.
//!
//! ## Hints
//!
//! 1. This is the first graph problem: the people are nodes and each trust pair is a directed edge.
//! 2. You do not need adjacency lists, only degrees. Give each person a score
//!    `in-degree - out-degree`.
//! 3. The judge is exactly the person whose score is `n - 1`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** for every edge `a -> b`, decrement `score[a]` and increment `score[b]`. A score of
//! `n - 1` is only possible with in-degree `n - 1` and out-degree `0` (in-degree can never exceed
//! `n - 1` because pairs are distinct), so the first person with that score is the judge.
//!
//! **Complexity:** `O(n + trust.len())` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - Labels start at `1`; a vector of length `n + 1` lets you index with the label directly and
//!   ignore slot `0`.
//! - `iter().skip(1).position(...)` finds the first match; `map_or(-1, ...)` converts the
//!   `Option<usize>` into LeetCode's `-1` convention.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        let n = n as usize;
        let mut score = vec![0_i32; n + 1];
        for pair in &trust {
            score[pair[0] as usize] -= 1;
            score[pair[1] as usize] += 1;
        }
        score
            .iter()
            .skip(1)
            .position(|&s| s == n as i32 - 1)
            .map_or(-1, |idx| idx as i32 + 1)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_people() {
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
    }

    #[test]
    fn example_three_people() {
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    }

    #[test]
    fn example_judge_trusts_someone() {
        let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
        assert_eq!(Solution::find_judge(3, trust), -1);
    }

    #[test]
    fn single_person_is_the_judge() {
        assert_eq!(Solution::find_judge(1, vec![]), 1);
    }

    #[test]
    fn nobody_trusts_anybody() {
        assert_eq!(Solution::find_judge(3, vec![]), -1);
    }

    #[test]
    fn bigger_town() {
        let trust = vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]];
        assert_eq!(Solution::find_judge(4, trust), 3);
        let all: Vec<Vec<i32>> = (1..1000).map(|a| vec![a, 1000]).collect();
        assert_eq!(Solution::find_judge(1000, all), 1000);
    }
}
