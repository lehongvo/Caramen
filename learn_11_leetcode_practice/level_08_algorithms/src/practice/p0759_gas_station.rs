// ==================================================================
// PRACTICE 0759: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0759_gas_station.md
// Run the tests:          cargo test practice::p0759
// ==================================================================

//! # Problem 0759: Gas Station
//!
//! - **Difficulty:** Medium
//! - **Topics:** greedy restart, running sums, `zip` + `enumerate`
//! - **Source:** [LeetCode 134. Gas Station](https://leetcode.com/problems/gas-station/)
//!
//! ## Problem
//!
//! There are `n` gas stations on a circular road. Station `i` provides `gas[i]` units of fuel, and
//! driving from station `i` to station `i + 1` (or from the last station to station `0`) costs
//! `cost[i]` units. Your car has an unlimited tank and starts empty at a station of your choice.
//!
//! Return the index of the starting station from which you can drive around the whole circle
//! once, clockwise, without the tank ever going negative. Return `-1` if it is impossible. If a
//! solution exists, it is unique.
//!
//! Implement `Solution::can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  gas = [1, 2, 3, 4, 5], cost = [3, 4, 5, 1, 2]
//! Output: 3
//!         (start at 3 with 4 fuel: 4-1+5 = 8, 8-2+1 = 7, 7-3+2 = 6, 6-4+3 = 5, 5-5 = 0 back at 3)
//!
//! Input:  gas = [2, 3, 4], cost = [3, 4, 3]
//! Output: -1            (total gas 9 < total cost 10)
//! ```
//!
//! ## Constraints
//!
//! - `n == gas.len() == cost.len()` and `1 <= n <= 10^5`
//! - `0 <= gas[i], cost[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. If the total gas is smaller than the total cost, no start works. Otherwise some start does.
//! 2. If you start at `a` and run dry before reaching `b`, then no station between `a` and `b`
//!    can be the start either. Where should you try next?

pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_start_at_three() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn example_impossible() {
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }

    #[test]
    fn single_station() {
        assert_eq!(Solution::can_complete_circuit(vec![5], vec![4]), 0);
        assert_eq!(Solution::can_complete_circuit(vec![4], vec![5]), -1);
        assert_eq!(Solution::can_complete_circuit(vec![0], vec![0]), 0);
    }

    #[test]
    fn start_at_zero() {
        assert_eq!(
            Solution::can_complete_circuit(vec![3, 1, 1], vec![1, 2, 2]),
            0
        );
    }

    #[test]
    fn start_at_the_last_station() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 1, 5], vec![2, 2, 1]),
            2
        );
    }

    #[test]
    fn long_circle() {
        // Every station loses 1 unit except the last, which gains 10_000: only the last one works.
        let n = 10_001;
        let mut gas = vec![0; n];
        gas[n - 1] = 10_000;
        let mut cost = vec![1; n];
        cost[n - 1] = 0;
        assert_eq!(Solution::can_complete_circuit(gas, cost), (n - 1) as i32);
    }
}
