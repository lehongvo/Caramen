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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** graph reachability with DFS: rooms are vertices and the keys in room `i` are edges
//! `i -> k`. Start from room `0`, marking rooms as visited when their key is first found and
//! pushing them on a stack. Pop a room, push every key that opens a room not yet visited. When the
//! stack is empty, every reachable room has been visited; compare that count with `n`.
//!
//! The input already is an **adjacency list** (`rooms[i]` lists the neighbours of `i`), the usual
//! way to store a sparse graph.
//!
//! **Complexity:** `O(n + k)` time for `n` rooms and `k` keys, `O(n)` space.
//!
//! **Rust concepts:**
//! - Marking a room as visited when it is **pushed** (not when popped) keeps every room on the
//!   stack at most once.
//! - `visited.iter().all(|&v| v)` checks the final state; a counter would work as well.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        // @solution-start
        let mut visited = vec![false; rooms.len()];
        visited[0] = true;
        let mut stack = vec![0usize];
        while let Some(room) = stack.pop() {
            for &key in &rooms[room] {
                let next = key as usize;
                if !visited[next] {
                    visited[next] = true;
                    stack.push(next);
                }
            }
        }
        visited.iter().all(|&v| v)
        // @solution-end
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
