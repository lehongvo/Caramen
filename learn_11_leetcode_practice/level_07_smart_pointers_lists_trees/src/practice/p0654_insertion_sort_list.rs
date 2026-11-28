// ==================================================================
// PRACTICE 0654: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0654_insertion_sort_list.md
// Run the tests:          cargo test practice::p0654
// ==================================================================

//! # Problem 0654: Insertion Sort List
//!
//! - **Difficulty:** Medium
//! - **Topics:** dummy head of a sorted list, searching with a `&mut Box<ListNode>` cursor, splicing a node in
//! - **Source:** [LeetCode 147. Insertion Sort List](https://leetcode.com/problems/insertion-sort-list/)
//!
//! ## Problem
//!
//! Sort a linked list in ascending order using **insertion sort**: take the nodes from the input
//! one at a time and insert each into its correct place in a growing sorted list. Return the head
//! of the sorted list. Equal values keep their original relative order.
//!
//! Implement `Solution::insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [4, 2, 1, 3]
//! Output: [1, 2, 3, 4]
//!
//! Input:  head = [-1, 5, 3, 4, 0]
//! Output: [-1, 0, 3, 4, 5]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 5000`, `-5000 <= val <= 5000`
//!
//! ## Hints
//!
//! 1. Keep the sorted nodes after a dummy node: `let mut sorted = Box::new(ListNode::new(0));`.
//! 2. For each detached node, start a cursor at the dummy and move it while the **next** sorted
//!    node is smaller than or equal to the new value (`<=` keeps equal values stable).
//! 3. Splice: `node.next = cursor.next.take(); cursor.next = Some(node);`.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let result = Solution::insertion_sort_list(list(&[4, 2, 1, 3]));
        assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn example_two() {
        let result = Solution::insertion_sort_list(list(&[-1, 5, 3, 4, 0]));
        assert_eq!(list_to_vec(&result), vec![-1, 0, 3, 4, 5]);
    }

    #[test]
    fn single_and_sorted_inputs() {
        assert_eq!(Solution::insertion_sort_list(list(&[1])), list(&[1]));
        assert_eq!(
            Solution::insertion_sort_list(list(&[1, 1, 2])),
            list(&[1, 1, 2])
        );
        assert_eq!(
            Solution::insertion_sort_list(list(&[3, 2, 1])),
            list(&[1, 2, 3])
        );
    }

    #[test]
    fn pseudo_random_input() {
        let values: Vec<i32> = (0..2_000).map(|i| (i * 7_919) % 10_001 - 5_000).collect();
        let mut expected = values.clone();
        expected.sort();
        let result = Solution::insertion_sort_list(list(&values));
        assert_eq!(list_to_vec(&result), expected);
    }
}
