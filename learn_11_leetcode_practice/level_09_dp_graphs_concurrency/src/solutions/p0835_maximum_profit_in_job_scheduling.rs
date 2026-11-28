//! # Problem 0835: Maximum Profit in Job Scheduling
//!
//! - **Difficulty:** Hard
//! - **Topics:** weighted interval scheduling, sort + binary search (`partition_point`) + DP,
//!   zipping three vectors into tuples
//! - **Source:** [LeetCode 1235. Maximum Profit in Job Scheduling](https://leetcode.com/problems/maximum-profit-in-job-scheduling/)
//!
//! ## Problem
//!
//! There are `n` jobs. Job `i` runs from `start_time[i]` to `end_time[i]` and pays `profit[i]`.
//! Choose a set of jobs with no two overlapping in time and return the maximum total profit. A job
//! that ends at time `x` does **not** overlap a job that starts at time `x`.
//!
//! Implement `Solution::job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  start_time = [1, 2, 3, 3], end_time = [3, 4, 5, 6], profit = [50, 10, 40, 70]
//! Output: 120
//! Explanation: jobs 0 (1-3) and 3 (3-6): 50 + 70.
//!
//! Input:  start_time = [1, 2, 3, 4, 6], end_time = [3, 5, 10, 6, 9],
//!         profit = [20, 20, 100, 70, 60]
//! Output: 150
//! Explanation: jobs 0, 3 and 4: 20 + 70 + 60.
//!
//! Input:  start_time = [1, 1, 1], end_time = [2, 3, 4], profit = [5, 6, 4]
//! Output: 6
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 5 * 10^4` (all three vectors have length `n`)
//! - `1 <= start_time[i] < end_time[i] <= 10^9`
//! - `1 <= profit[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. Sort the jobs by end time. Let `best[i]` be the best profit using only the first `i` jobs in
//!    that order.
//! 2. For job `i`, either skip it (`best[i]`) or take it: its profit plus `best[j]`, where `j` is
//!    the number of jobs that end no later than job `i` starts.
//! 3. The end times are sorted, so `j` can be found by binary search.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build `(end, start, profit)` tuples and sort them. `best[0] = 0`; for the `i`-th job,
//! `j = ends[..i].partition_point(|&e| e <= start)` counts the compatible earlier jobs, and
//! `best[i + 1] = max(best[i], best[j] + profit)`.
//!
//! **Complexity:** `O(n log n)` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - Zipping three vectors: `a.iter().zip(&b).zip(&c)` yields nested tuples `((x, y), z)`, which
//!   the closure pattern `|((&e, &s), &p)|` takes apart.
//! - Tuples compare lexicographically, so `sort_unstable()` on `(end, start, profit)` sorts by end.
//! - `partition_point` works on any prefix slice `&ends[..i]`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        // @solution-start
        let mut jobs: Vec<(i32, i32, i32)> = end_time
            .iter()
            .zip(&start_time)
            .zip(&profit)
            .map(|((&e, &s), &p)| (e, s, p))
            .collect();
        jobs.sort_unstable();
        let ends: Vec<i32> = jobs.iter().map(|&(e, _, _)| e).collect();
        let mut best = vec![0_i32; jobs.len() + 1];
        for (i, &(_, start, p)) in jobs.iter().enumerate() {
            let j = ends[..i].partition_point(|&e| e <= start);
            best[i + 1] = best[i].max(best[j] + p);
        }
        best[jobs.len()]
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_jobs() {
        let r = Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]);
        assert_eq!(r, 120);
    }

    #[test]
    fn example_three_jobs() {
        let r = Solution::job_scheduling(
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 20, 100, 70, 60],
        );
        assert_eq!(r, 150);
    }

    #[test]
    fn example_all_overlap() {
        let r = Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]);
        assert_eq!(r, 6);
    }

    #[test]
    fn touching_jobs_do_not_overlap() {
        let r = Solution::job_scheduling(vec![1, 5, 9], vec![5, 9, 13], vec![1, 2, 3]);
        assert_eq!(r, 6);
    }

    #[test]
    fn many_jobs() {
        // Jobs [i, i + 2) for i in 0..50000 with profit 1: take every other job.
        let n = 50_000;
        let start: Vec<i32> = (1..=n).collect();
        let end: Vec<i32> = (1..=n).map(|s| s + 2).collect();
        let r = Solution::job_scheduling(start, end, vec![1; n as usize]);
        assert_eq!(r, n / 2);
    }
}
