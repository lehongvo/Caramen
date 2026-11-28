# Problem 0656: Remove Zero Sum Consecutive Nodes from Linked List

- **Difficulty:** Medium
- **Topics:** prefix sums, `HashMap` of last positions, rebuilding a list from kept values
- **Source:** [LeetCode 1171. Remove Zero Sum Consecutive Nodes from Linked List](https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/)

## Problem

Repeatedly delete any run of **consecutive** nodes whose values sum to `0`, until no such run
is left. Return the head of the final list. Several final lists may be possible; any of them is
accepted.

Implement
`Solution::remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.

## Examples

```text
Input:  head = [1, 2, -3, 3, 1]
Output: [3, 1]          // [1, 2, 1] is also accepted

Input:  head = [1, 2, 3, -3, 4]
Output: [1, 2, 4]

Input:  head = [1, 2, 3, -3, -2]
Output: [1]
```

## Constraints

- `1 <= n <= 1000`, `-1000 <= val <= 1000`

## Hints

1. Let `prefix[i]` be the sum of the first `i` values (`prefix[0] = 0`). A run from position
   `i` to `j - 1` sums to zero exactly when `prefix[i] == prefix[j]`.
2. Record, for every prefix sum, the **last** position where it occurs.
3. Walk from position `0`: jump to the last position with the same prefix sum (skipping a
   zero-sum run), keep the value there, and continue after it.

## Starter code

Write your code in [`src/practice/p0656_remove_zero_sum_consecutive_nodes_from_linked_list.rs`](../src/practice/p0656_remove_zero_sum_consecutive_nodes_from_linked_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0656
```

```rust
use crate::common::ListNode;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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

    /// Checks that `output` is a valid final list for `input`.
    fn assert_valid(input: &[i32], output: &[i32]) {
        // Nothing left to remove: no consecutive run sums to zero.
        for start in 0..output.len() {
            let mut sum = 0;
            for &value in &output[start..] {
                sum += value;
                assert_ne!(sum, 0, "{output:?} still has a zero-sum run");
            }
        }
        // Only zero-sum runs were removed: the total is unchanged, and the order is kept.
        assert_eq!(input.iter().sum::<i32>(), output.iter().sum::<i32>());
        let mut remaining = input.iter();
        assert!(
            output.iter().all(|v| remaining.any(|x| x == v)),
            "{output:?} is not a subsequence of {input:?}"
        );
    }

    fn run(input: &[i32]) -> Vec<i32> {
        let output = list_to_vec(&Solution::remove_zero_sum_sublists(list(input)));
        assert_valid(input, &output);
        output
    }

    #[test]
    fn example_one() {
        let output = run(&[1, 2, -3, 3, 1]);
        assert!(
            output == vec![3, 1] || output == vec![1, 2, 1],
            "{output:?}"
        );
    }

    #[test]
    fn example_two_and_three() {
        assert_eq!(run(&[1, 2, 3, -3, 4]), vec![1, 2, 4]);
        assert_eq!(run(&[1, 2, 3, -3, -2]), vec![1]);
    }

    #[test]
    fn everything_cancels() {
        assert_eq!(run(&[0]), Vec::<i32>::new());
        assert_eq!(run(&[5, -5, 3, -3]), Vec::<i32>::new());
    }

    #[test]
    fn long_mixed_input() {
        let input: Vec<i32> = (0..1_000).map(|i| (i * 37) % 21 - 10).collect();
        run(&input);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** copy the values, compute prefix sums and a map `prefix sum -> last index`. Starting
at index `0`, jump to `last[prefix[i]]`; everything skipped sums to zero. If the landing index
is a real value, keep it and continue with the next index. Finally build a fresh list.

**Complexity:** `O(n)` expected time, `O(n)` extra space.

**Rust concepts:**
- `HashMap::insert` overwrites, so inserting every prefix sum in order leaves the last index for
  each sum.
- Building the result from the kept values with a tail cursor (problem 0627) is simpler than
  unlinking runs inside the original list.

```rust
use crate::common::ListNode;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let values: Vec<i32> = std::iter::successors(head.as_deref(), |node| node.next.as_deref())
            .map(|node| node.val)
            .collect();
        let mut prefix = vec![0; values.len() + 1];
        for (i, value) in values.iter().enumerate() {
            prefix[i + 1] = prefix[i] + value;
        }
        let last: HashMap<i32, usize> = prefix.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        let mut result = None;
        let mut tail = &mut result;
        let mut i = 0;
        while i <= values.len() {
            i = last[&prefix[i]];
            if let Some(&value) = values.get(i) {
                tail = &mut tail.insert(Box::new(ListNode::new(value))).next;
            }
            i += 1;
        }
        result
    }
}
```

Full tested solution: [`src/solutions/p0656_remove_zero_sum_consecutive_nodes_from_linked_list.rs`](../src/solutions/p0656_remove_zero_sum_consecutive_nodes_from_linked_list.rs)

</details>
