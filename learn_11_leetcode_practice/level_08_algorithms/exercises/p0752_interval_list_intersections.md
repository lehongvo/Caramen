# Problem 0752: Interval List Intersections

- **Difficulty:** Medium
- **Topics:** two pointers over two sorted lists, `max`/`min` of bounds
- **Source:** [LeetCode 986. Interval List Intersections](https://leetcode.com/problems/interval-list-intersections/)

## Problem

You are given two lists of closed intervals, `first_list` and `second_list`. Within each list
the intervals are pairwise disjoint and sorted. Return the intersection of the two lists: every
non-empty interval that is covered by both an interval of the first list and an interval of the
second list, in sorted order. The intersection of two closed intervals can be a single point,
such as `[5, 5]`.

Implement
`Solution::interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.

## Examples

```text
Input:  first_list  = [[0, 2], [5, 10], [13, 23], [24, 25]]
        second_list = [[1, 5], [8, 12], [15, 24], [25, 26]]
Output: [[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]

Input:  first_list = [[1, 3], [5, 9]], second_list = []
Output: []
```

## Constraints

- `0 <= first_list.len(), second_list.len() <= 1000`
- `0 <= start < end <= 10^9` for every interval
- Inside each list, every interval ends before the next one starts.

## Hints

1. Two intervals intersect in `[max(starts), min(ends)]` when that range is not empty.
2. Use one pointer per list. After comparing two intervals, the one that ends first cannot
   intersect anything else: advance its pointer.

## Starter code

Write your code in [`src/practice/p0752_interval_list_intersections.rs`](../src/practice/p0752_interval_list_intersections.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0752
```

```rust
pub struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
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

    #[test]
    fn example_many_intersections() {
        let first = vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]];
        let second = vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]];
        assert_eq!(
            Solution::interval_intersection(first, second),
            vec![
                vec![1, 2],
                vec![5, 5],
                vec![8, 10],
                vec![15, 23],
                vec![24, 24],
                vec![25, 25]
            ]
        );
    }

    #[test]
    fn example_empty_list() {
        assert!(Solution::interval_intersection(vec![vec![1, 3], vec![5, 9]], vec![]).is_empty());
        assert!(Solution::interval_intersection(vec![], vec![vec![4, 8]]).is_empty());
    }

    #[test]
    fn partial_overlap() {
        assert_eq!(
            Solution::interval_intersection(vec![vec![1, 7]], vec![vec![3, 10]]),
            vec![vec![3, 7]]
        );
    }

    #[test]
    fn one_interval_covers_many() {
        let first = vec![vec![0, 100]];
        let second = vec![vec![1, 2], vec![10, 20], vec![99, 200]];
        assert_eq!(
            Solution::interval_intersection(first, second),
            vec![vec![1, 2], vec![10, 20], vec![99, 100]]
        );
    }

    #[test]
    fn disjoint_lists() {
        let first = vec![vec![0, 1], vec![10, 11]];
        let second = vec![vec![2, 3], vec![12, 13]];
        assert!(Solution::interval_intersection(first, second).is_empty());
    }

    #[test]
    fn long_interleaved_lists() {
        let first: Vec<Vec<i32>> = (0..1000).map(|i| vec![i * 10, i * 10 + 5]).collect();
        let second: Vec<Vec<i32>> = (0..1000).map(|i| vec![i * 10 + 5, i * 10 + 8]).collect();
        let result = Solution::interval_intersection(first, second);
        assert_eq!(result.len(), 1000);
        assert!(
            result
                .iter()
                .enumerate()
                .all(|(i, iv)| *iv == vec![i as i32 * 10 + 5; 2])
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** two pointers: look at `a = first_list[i]` and `b = second_list[j]`. Their overlap
is `[max(a.start, b.start), min(a.end, b.end)]`, kept when `lo <= hi`. Then the interval that
ends first is finished: every later interval of the other list starts after the current one,
hence after this end too. Advance its pointer (either one on a tie). Each step advances a
pointer, so there are at most `m + n` steps, and the output is produced in sorted order.

**Complexity:** `O(m + n)` time, `O(1)` extra space besides the output.

**Rust concepts:**
- `while i < first.len() && j < second.len()` with plain indices is the clearest form when two
  sequences advance at different times.
- Borrowing `let (a, b) = (&first_list[i], &second_list[j]);` reads both intervals without
  moving them out of the vectors.

```rust
pub struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let (mut i, mut j) = (0, 0);
        let mut result = Vec::new();
        while i < first_list.len() && j < second_list.len() {
            let (a, b) = (&first_list[i], &second_list[j]);
            let (lo, hi) = (a[0].max(b[0]), a[1].min(b[1]));
            if lo <= hi {
                result.push(vec![lo, hi]);
            }
            if a[1] < b[1] {
                i += 1;
            } else {
                j += 1;
            }
        }
        result
    }
}
```

Full tested solution: [`src/solutions/p0752_interval_list_intersections.rs`](../src/solutions/p0752_interval_list_intersections.rs)

</details>
