//! # Problem 0655: Sort List
//!
//! - **Difficulty:** Medium
//! - **Topics:** merge sort on owned lists, splitting by length, recursive helpers inside `impl Solution`
//! - **Source:** [LeetCode 148. Sort List](https://leetcode.com/problems/sort-list/)
//!
//! ## Problem
//!
//! Sort a linked list in ascending order in `O(n log n)` time and return its head. Do it by
//! re-linking the nodes (do not copy the values into a `Vec` and sort that).
//!
//! Implement `Solution::sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [4, 2, 1, 3]
//! Output: [1, 2, 3, 4]
//!
//! Input:  head = [-1, 5, 3, 4, 0]
//! Output: [-1, 0, 3, 4, 5]
//!
//! Input:  head = []
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 5 * 10^4`, `-10^5 <= val <= 10^5`
//!
//! ## Hints
//!
//! 1. Merge sort fits linked lists well: merging two sorted lists needs no extra memory (problem
//!    0631), and splitting needs no random access.
//! 2. Write `sort(head, len)`: if `len <= 1` the list is sorted; otherwise cut it after `len / 2`
//!    nodes, sort both halves, merge them.
//! 3. Passing the length down avoids counting each half again.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** top-down merge sort. Count once, then recursively split at the middle by walking
//! `len / 2 - 1` steps and taking the next link, sort the halves and merge them with the iterative
//! merge from problem 0631.
//!
//! **Complexity:** `O(n log n)` time; `O(log n)` stack for the recursion (the merge is iterative).
//!
//! **Rust concepts:**
//! - Ownership makes the split explicit: after `take()`, the two halves are separate lists that
//!   the recursive calls own.
//! - Private associated functions (`Self::merge`, `Self::sort`) keep helpers inside the pasteable
//!   `impl Solution` block.
// @solution-end

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    // @solution-start remove
    fn merge(mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        loop {
            match (a, b) {
                (Some(mut x), Some(mut y)) => {
                    if x.val <= y.val {
                        a = x.next.take();
                        b = Some(y);
                        tail = &mut tail.insert(x).next;
                    } else {
                        b = y.next.take();
                        a = Some(x);
                        tail = &mut tail.insert(y).next;
                    }
                }
                (rest_a, rest_b) => {
                    *tail = rest_a.or(rest_b);
                    return head;
                }
            }
        }
    }

    fn sort(head: Option<Box<ListNode>>, len: usize) -> Option<Box<ListNode>> {
        if len <= 1 {
            return head;
        }
        let half = len / 2;
        let mut head = head;
        let mut cursor = head.as_mut();
        for _ in 0..half - 1 {
            cursor = cursor.and_then(|node| node.next.as_mut());
        }
        let second = cursor.and_then(|node| node.next.take());
        Self::merge(Self::sort(head, half), Self::sort(second, len - half))
    }
    // @solution-end

    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // @solution-start
        let len = std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        Self::sort(head, len)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let result = Solution::sort_list(list(&[4, 2, 1, 3]));
        assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn example_two() {
        let result = Solution::sort_list(list(&[-1, 5, 3, 4, 0]));
        assert_eq!(list_to_vec(&result), vec![-1, 0, 3, 4, 5]);
    }

    #[test]
    fn empty_single_and_duplicates() {
        assert_eq!(Solution::sort_list(None), None);
        assert_eq!(Solution::sort_list(list(&[9])), list(&[9]));
        assert_eq!(
            Solution::sort_list(list(&[2, 1, 2, 1])),
            list(&[1, 1, 2, 2])
        );
    }

    #[test]
    fn large_pseudo_random_input() {
        let values: Vec<i32> = (0..5_000)
            .map(|i| (i * 48_271) % 200_001 - 100_000)
            .collect();
        let mut expected = values.clone();
        expected.sort();
        let result = Solution::sort_list(list(&values));
        assert_eq!(list_to_vec(&result), expected);
    }
}
