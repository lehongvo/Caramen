# Problem 0654: Insertion Sort List

- **Difficulty:** Medium
- **Topics:** dummy head of a sorted list, searching with a `&mut Box<ListNode>` cursor, splicing a node in
- **Source:** [LeetCode 147. Insertion Sort List](https://leetcode.com/problems/insertion-sort-list/)

## Problem

Sort a linked list in ascending order using **insertion sort**: take the nodes from the input
one at a time and insert each into its correct place in a growing sorted list. Return the head
of the sorted list. Equal values keep their original relative order.

Implement `Solution::insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.

## Examples

```text
Input:  head = [4, 2, 1, 3]
Output: [1, 2, 3, 4]

Input:  head = [-1, 5, 3, 4, 0]
Output: [-1, 0, 3, 4, 5]
```

## Constraints

- `1 <= n <= 5000`, `-5000 <= val <= 5000`

## Hints

1. Keep the sorted nodes after a dummy node: `let mut sorted = Box::new(ListNode::new(0));`.
2. For each detached node, start a cursor at the dummy and move it while the **next** sorted
   node is smaller than or equal to the new value (`<=` keeps equal values stable).
3. Splice: `node.next = cursor.next.take(); cursor.next = Some(node);`.

## Starter code

Write your code in [`src/practice/p0654_insertion_sort_list.rs`](../src/practice/p0654_insertion_sort_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0654
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
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
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the dummy node means there is always a node *before* the insertion point, so
inserting at the front is not a special case. The cursor stops on the last sorted node whose
value is `<=` the new one, and the new node goes right after it.

**Complexity:** `O(n^2)` time in the worst case, `O(1)` extra space.

**Rust concepts:**
- `cursor = cursor.next.as_mut()?;` advances a `&mut Box<ListNode>`. The loop condition has just
  seen a next node, so the `?` never returns early.
- Writing `cursor.next` after the search loop is fine because `cursor` itself is the most recent
  reborrow; a `match` with a `break` arm instead would hit a borrow-checker limitation.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sorted = Box::new(ListNode::new(0));
        let mut rest = head;
        while let Some(mut node) = rest {
            rest = node.next.take();
            let mut cursor = &mut sorted;
            while cursor
                .next
                .as_ref()
                .is_some_and(|next| next.val <= node.val)
            {
                cursor = cursor.next.as_mut()?;
            }
            node.next = cursor.next.take();
            cursor.next = Some(node);
        }
        sorted.next
    }
}
```

Full tested solution: [`src/solutions/p0654_insertion_sort_list.rs`](../src/solutions/p0654_insertion_sort_list.rs)

</details>
