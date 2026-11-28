# Problem 0751: Insert Interval

- **Difficulty:** Medium
- **Topics:** three-phase scan of sorted intervals, `Peekable::next_if`, `extend`
- **Source:** [LeetCode 57. Insert Interval](https://leetcode.com/problems/insert-interval/)

## Problem

`intervals` is a list of closed, non-overlapping intervals `[start, end]`, sorted by `start`.
Insert `new_interval` into it and merge where necessary, so the result is again sorted by start
and non-overlapping. Intervals that touch (such as `[1, 2]` and `[2, 3]`) are merged.

Implement `Solution::insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>>`.

## Examples

```text
Input:  intervals = [[1, 3], [6, 9]], new_interval = [2, 5]
Output: [[1, 5], [6, 9]]

Input:  intervals = [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], new_interval = [4, 8]
Output: [[1, 2], [3, 10], [12, 16]]     ([4, 8] overlaps [3, 5], [6, 7] and [8, 10])

Input:  intervals = [], new_interval = [5, 7]
Output: [[5, 7]]
```

## Constraints

- `0 <= intervals.len() <= 10^4`
- `intervals[i].len() == 2`, `0 <= start <= end <= 10^5`, sorted by start, non-overlapping
- `new_interval.len() == 2` and `0 <= new_interval[0] <= new_interval[1] <= 10^5`

## Hints

1. The input is already sorted: no need to sort again. There are three groups of intervals:
   entirely before the new one, overlapping it, and entirely after it.
2. Overlapping intervals are absorbed into the new interval by taking the minimum start and the
   maximum end.

## Starter code

Write your code in [`src/practice/p0751_insert_interval.rs`](../src/practice/p0751_insert_interval.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0751
```

```rust
pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
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
    fn example_merge_with_one() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }

    #[test]
    fn example_merge_with_three() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        assert_eq!(
            Solution::insert(intervals, vec![4, 8]),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }

    #[test]
    fn example_empty_list() {
        assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
    }

    #[test]
    fn insert_at_the_front_and_the_end() {
        assert_eq!(
            Solution::insert(vec![vec![3, 4]], vec![1, 2]),
            vec![vec![1, 2], vec![3, 4]]
        );
        assert_eq!(
            Solution::insert(vec![vec![3, 4]], vec![6, 8]),
            vec![vec![3, 4], vec![6, 8]]
        );
    }

    #[test]
    fn touching_intervals_merge() {
        assert_eq!(
            Solution::insert(vec![vec![1, 2], vec![5, 6]], vec![2, 5]),
            vec![vec![1, 6]]
        );
    }

    #[test]
    fn new_interval_inside_an_existing_one() {
        assert_eq!(
            Solution::insert(vec![vec![1, 10]], vec![3, 4]),
            vec![vec![1, 10]]
        );
    }

    #[test]
    fn long_list() {
        let intervals: Vec<Vec<i32>> = (0..10_000).map(|i| vec![i * 10, i * 10 + 1]).collect();
        let result = Solution::insert(intervals, vec![15, 51]);
        // [10, 11] stays; [15, 51] swallows the 4 intervals [20, 21] .. [50, 51]; the rest stays.
        assert_eq!(result.len(), 10_000 - 4 + 1);
        assert_eq!(result[1], vec![10, 11]);
        assert_eq!(result[2], vec![15, 51]);
        assert_eq!(result[3], vec![60, 61]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** three-phase linear scan: (1) copy every interval that ends before the new interval
starts (`end < start`); (2) absorb every interval that starts at or before the new interval's
(growing) end, widening it to `min` start and `max` end; (3) push the merged interval and copy
the rest unchanged. Each interval is looked at once.

**Complexity:** `O(n)` time, `O(n)` space for the output.

**Rust concepts:**
- [`Peekable::next_if`](https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.next_if)
  returns the next item only if it satisfies the predicate, so `while let Some(iv) = iter.next_if(...)`
  consumes "as long as the condition holds" and leaves the first failing item in the iterator.
- `result.extend(iter)` moves all remaining intervals into the result without cloning.

```rust
pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut start, mut end) = (new_interval[0], new_interval[1]);
        let mut result = Vec::with_capacity(intervals.len() + 1);
        let mut iter = intervals.into_iter().peekable();
        // 1. Entirely before the new interval.
        while let Some(iv) = iter.next_if(|iv| iv[1] < start) {
            result.push(iv);
        }
        // 2. Overlapping (or touching) the new interval: absorb them.
        while let Some(iv) = iter.next_if(|iv| iv[0] <= end) {
            start = start.min(iv[0]);
            end = end.max(iv[1]);
        }
        result.push(vec![start, end]);
        // 3. Entirely after it.
        result.extend(iter);
        result
    }
}
```

Full tested solution: [`src/solutions/p0751_insert_interval.rs`](../src/solutions/p0751_insert_interval.rs)

</details>
