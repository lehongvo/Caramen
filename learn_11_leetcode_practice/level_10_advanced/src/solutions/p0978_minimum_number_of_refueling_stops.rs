//! # Problem 0978: Minimum Number of Refueling Stops
//!
//! - **Difficulty:** Hard
//! - **Topics:** lazy greedy ("refuel in hindsight"), a max-heap of passed stations, `i64` fuel, early exit
//! - **Source:** [LeetCode 871. Minimum Number of Refueling Stops](https://leetcode.com/problems/minimum-number-of-refueling-stops/)
//!
//! ## Problem
//!
//! A car drives east from position 0 to `target`, using one liter of fuel per mile. It starts with
//! `start_fuel` liters and an unlimited tank. `stations[i] = [position, fuel]` is a gas station
//! (sorted by position) where the car may stop and take **all** of its `fuel`. Reaching a place
//! with exactly 0 fuel left still counts as reaching it.
//!
//! Return the minimum number of refueling stops needed to reach `target`, or `-1` if it is impossible.
//!
//! Implement `Solution::min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  target = 1, start_fuel = 1, stations = []                    Output: 0
//! Input:  target = 100, start_fuel = 1, stations = [[10, 100]]         Output: -1
//! Input:  target = 100, start_fuel = 10,
//!         stations = [[10, 60], [20, 30], [30, 30], [60, 40]]          Output: 2
//!         (stop at 10 for 60 liters, then at 60 for 40 liters)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= target, start_fuel <= 10^9`
//! - `0 <= stations.len() <= 500`, `1 <= position < target`, `1 <= fuel <= 10^9`
//!
//! ## Hints
//!
//! 1. Drive as far as the fuel allows; remember every station passed on the way.
//! 2. When you cannot reach the next point, pretend you had stopped at the **largest** station passed
//!    so far: a max-heap gives it.
//! 3. If the heap is empty at that moment, the target is unreachable.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** treat the target as a final station with no fuel. Before each point, while the current
//! reach is not enough, "retroactively" refuel at the best station already passed and count a stop.
//! Taking the largest fuel first is optimal: any solution with the same number of stops cannot reach
//! further.
//!
//! **Complexity:** `O(n log n)` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - The reach can exceed `i32` (`10^9 + 500 * 10^9`), so it is kept in `i64`.
//! - `chain(std::iter::once((target, 0)))` appends the target as a pseudo-station.
// @solution-end

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        let mut reach = i64::from(start_fuel);
        let mut passed: BinaryHeap<i64> = BinaryHeap::new();
        let mut stops = 0;
        let points = stations
            .iter()
            .map(|s| (i64::from(s[0]), i64::from(s[1])))
            .chain(std::iter::once((i64::from(target), 0)));
        for (position, fuel) in points {
            while reach < position {
                match passed.pop() {
                    Some(best) => {
                        reach += best;
                        stops += 1;
                    }
                    None => return -1,
                }
            }
            passed.push(fuel);
        }
        stops
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(target: i32, start_fuel: i32, stations: &[[i32; 2]]) -> i32 {
        Solution::min_refuel_stops(
            target,
            start_fuel,
            stations.iter().map(|s| s.to_vec()).collect(),
        )
    }

    #[test]
    fn examples() {
        assert_eq!(run(1, 1, &[]), 0);
        assert_eq!(run(100, 1, &[[10, 100]]), -1);
        assert_eq!(run(100, 10, &[[10, 60], [20, 30], [30, 30], [60, 40]]), 2);
    }

    #[test]
    fn arriving_with_an_empty_tank_counts() {
        assert_eq!(run(100, 50, &[[50, 50]]), 1);
        assert_eq!(run(100, 49, &[[50, 50]]), -1);
    }

    #[test]
    fn prefers_the_biggest_station() {
        assert_eq!(run(1_000, 100, &[[10, 5], [50, 900], [90, 1]]), 1);
        assert_eq!(run(1_000, 100, &[[10, 400], [50, 500], [90, 100]]), 2);
        assert_eq!(run(1_000, 100, &[[10, 300], [50, 300], [90, 300]]), 3);
    }

    #[test]
    fn huge_numbers() {
        let stations: Vec<[i32; 2]> = (1..=500).map(|i| [i * 1_000_000, 1_000_000]).collect();
        assert_eq!(run(999_999_999, 1_000_000, &stations), -1);
        assert_eq!(run(500_000_001, 1_000_000, &stations), 500);
        assert_eq!(run(999_999_999, 1_000_000_000, &stations), 0);
    }
}
