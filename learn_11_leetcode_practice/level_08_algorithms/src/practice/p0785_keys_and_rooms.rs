// ==================================================================
// PRACTICE 0785: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0785_keys_and_rooms.md
// Run the tests:          cargo test practice::p0785
// ==================================================================

//! # Problem 0785: Keys and Rooms
//!
//! - **Difficulty:** Medium
//! - **Topics:** graph DFS with an adjacency list, `visited: Vec<bool>`, `extend` from an iterator
//! - **Source:** [LeetCode 841. Keys and Rooms](https://leetcode.com/problems/keys-and-rooms/)
//!
//! ## Problem
//!
//! There are `n` rooms numbered `0` to `n - 1`. Every room except room `0` starts locked. Room
//! `i` contains the keys `rooms[i]`: each key `k` unlocks room `k`. You start in room `0`, and you
//! may enter any room whose key you hold, collecting the keys inside.
//!
//! Return `true` if you can enter every room, `false` otherwise.
//!
//! Implement `Solution::can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  rooms = [[1], [2], [3], []]
//! Output: true          (0 gives key 1, 1 gives key 2, 2 gives key 3)
//!
//! Input:  rooms = [[1, 3], [3, 0, 1], [2], [0]]
//! Output: false         (the only key to room 2 is inside room 2)
//! ```
//!
//! ## Constraints
//!
//! - `2 <= rooms.len() <= 1000`
//! - `0 <= rooms[i].len() <= 1000`, and the total number of keys is at most `3000`
//! - `0 <= rooms[i][j] < rooms.len()`, the keys inside one room are distinct
//!
//! ## Hints
//!
//! 1. Rooms are nodes and keys are directed edges: the question is whether every node is reachable
//!    from node `0`.
//! 2. Keep a stack of rooms you can open but have not explored, and a `visited` list.

pub struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_chain() {
        assert!(Solution::can_visit_all_rooms(vec![
            vec![1],
            vec![2],
            vec![3],
            vec![]
        ]));
    }

    #[test]
    fn example_locked_room() {
        assert!(!Solution::can_visit_all_rooms(vec![
            vec![1, 3],
            vec![3, 0, 1],
            vec![2],
            vec![0]
        ]));
    }

    #[test]
    fn key_only_in_an_unreachable_room() {
        assert!(!Solution::can_visit_all_rooms(vec![vec![], vec![0]]));
    }

    #[test]
    fn all_keys_in_the_first_room() {
        assert!(Solution::can_visit_all_rooms(vec![
            vec![1, 2, 3],
            vec![],
            vec![],
            vec![]
        ]));
    }

    #[test]
    fn long_chain_forwards_and_backwards() {
        let forward: Vec<Vec<i32>> = (0..1000)
            .map(|i| if i < 999 { vec![i + 1] } else { vec![] })
            .collect();
        assert!(Solution::can_visit_all_rooms(forward));
        let backward: Vec<Vec<i32>> = (0..1000)
            .map(|i| if i > 0 { vec![i - 1] } else { vec![] })
            .collect();
        assert!(!Solution::can_visit_all_rooms(backward));
    }
}
