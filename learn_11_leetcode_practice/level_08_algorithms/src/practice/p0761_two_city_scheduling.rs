// ==================================================================
// PRACTICE 0761: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0761_two_city_scheduling.md
// Run the tests:          cargo test practice::p0761
// ==================================================================

//! # Problem 0761: Two City Scheduling
//!
//! - **Difficulty:** Medium
//! - **Topics:** greedy by cost difference, `sort_unstable_by_key`, `enumerate` + `map` + `sum`
//! - **Source:** [LeetCode 1029. Two City Scheduling](https://leetcode.com/problems/two-city-scheduling/)
//!
//! ## Problem
//!
//! A company is flying `2n` people to interviews in two cities. `costs[i] = [a_cost, b_cost]` is
//! the price of flying person `i` to city A or to city B. Exactly `n` people must go to each city.
//! Return the minimum total cost.
//!
//! Implement `Solution::two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  costs = [[10, 20], [30, 200], [400, 50], [30, 20]]
//! Output: 110           (people 0 and 1 to A: 10 + 30; people 2 and 3 to B: 50 + 20)
//!
//! Input:  costs = [[259, 770], [448, 54], [926, 667], [184, 139], [840, 118], [577, 469]]
//! Output: 1859
//! ```
//!
//! ## Constraints
//!
//! - `2 <= costs.len() <= 100` and `costs.len()` is even
//! - `1 <= a_cost, b_cost <= 1000`
//!
//! ## Hints
//!
//! 1. Imagine first sending everybody to B. Moving person `i` to A then changes the total by
//!    `a_cost - b_cost`.
//! 2. You must move exactly `n` people to A. Which `n` should they be?

pub struct Solution;

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(costs: &[[i32; 2]]) -> i32 {
        Solution::two_city_sched_cost(costs.iter().map(|c| c.to_vec()).collect())
    }

    #[test]
    fn example_four_people() {
        assert_eq!(run(&[[10, 20], [30, 200], [400, 50], [30, 20]]), 110);
    }

    #[test]
    fn example_six_people() {
        assert_eq!(
            run(&[
                [259, 770],
                [448, 54],
                [926, 667],
                [184, 139],
                [840, 118],
                [577, 469]
            ]),
            1859
        );
    }

    #[test]
    fn eight_people() {
        let costs = [
            [515, 563],
            [451, 713],
            [537, 709],
            [343, 819],
            [855, 779],
            [457, 60],
            [650, 359],
            [631, 42],
        ];
        assert_eq!(run(&costs), 3086);
    }

    #[test]
    fn two_people_with_equal_differences() {
        assert_eq!(run(&[[1, 2], [3, 4]]), 5);
    }

    #[test]
    fn everyone_prefers_b() {
        // Half of them must still go to A.
        assert_eq!(run(&[[1000, 1]; 100]), 50 * 1000 + 50);
    }
}
