# Problem 0980: The Skyline Problem

- **Difficulty:** Hard
- **Topics:** sweep line over sorted events, a multiset as `BTreeMap<height, count>`, grouping events by x, `chunk_by`
- **Source:** [LeetCode 218. The Skyline Problem](https://leetcode.com/problems/the-skyline-problem/)

## Problem

Each building is a rectangle on flat ground: `buildings[i] = [left, right, height]` covers
`left <= x < right`. The **skyline** is the outer contour of all buildings seen from far away,
described by its **key points** `[x, y]`: each one is where the contour's height changes to `y`
at position `x`, sorted by `x`. The last key point always has `y = 0` (where the last building
ends). Consecutive key points never have the same height (merge them).

Implement `Solution::get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.

## Examples

```text
Input:  buildings = [[2, 9, 10], [3, 7, 15], [5, 12, 12], [15, 20, 10], [19, 24, 8]]
Output: [[2, 10], [3, 15], [7, 12], [12, 0], [15, 10], [20, 8], [24, 0]]

Input:  buildings = [[0, 2, 3], [2, 5, 3]]
Output: [[0, 3], [5, 0]]       (same height: one flat segment)
```

## Constraints

- `1 <= buildings.len() <= 10^4`
- `0 <= left < right <= 2^31 - 1`, `1 <= height <= 2^31 - 1`
- `buildings` is sorted by `left`.

## Hints

1. Turn each building into two events: its height starts at `left` and stops at `right`.
2. Sweep the events in order of `x`, keeping the multiset of heights currently "on" (plus a ground
   height of 0). The contour height is the multiset's maximum.
3. Apply **all** events with the same `x` before comparing the maximum with the previous height;
   emit `[x, max]` only when it changed.

## Starter code

Write your code in [`src/practice/p0980_the_skyline_problem.rs`](../src/practice/p0980_the_skyline_problem.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0980
```

```rust
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

    fn run(buildings: &[[i32; 3]]) -> Vec<[i32; 2]> {
        Solution::get_skyline(buildings.iter().map(|b| b.to_vec()).collect())
            .into_iter()
            .map(|p| [p[0], p[1]])
            .collect()
    }

    #[test]
    fn examples() {
        assert_eq!(
            run(&[
                [2, 9, 10],
                [3, 7, 15],
                [5, 12, 12],
                [15, 20, 10],
                [19, 24, 8]
            ]),
            vec![
                [2, 10],
                [3, 15],
                [7, 12],
                [12, 0],
                [15, 10],
                [20, 8],
                [24, 0]
            ]
        );
        assert_eq!(run(&[[0, 2, 3], [2, 5, 3]]), vec![[0, 3], [5, 0]]);
    }

    #[test]
    fn shared_edges() {
        assert_eq!(
            run(&[[1, 2, 1], [1, 2, 2], [1, 2, 3]]),
            vec![[1, 3], [2, 0]]
        );
        assert_eq!(
            run(&[[0, 5, 7], [5, 10, 7], [5, 10, 12], [10, 15, 7]]),
            vec![[0, 7], [5, 12], [10, 7], [15, 0]]
        );
        assert_eq!(run(&[[0, 3, 5], [3, 6, 2]]), vec![[0, 5], [3, 2], [6, 0]]);
    }

    #[test]
    fn nested_buildings() {
        assert_eq!(
            run(&[[0, 10, 3], [2, 4, 5], [6, 8, 1]]),
            vec![[0, 3], [2, 5], [4, 3], [10, 0]]
        );
        assert_eq!(
            run(&[[0, i32::MAX, i32::MAX]]),
            vec![[0, i32::MAX], [i32::MAX, 0]]
        );
    }

    #[test]
    fn matches_a_height_scan() {
        let buildings: Vec<[i32; 3]> = (0..200)
            .map(|i: i32| {
                let left = (i * 37) % 300;
                [left, left + (i * 13) % 40 + 1, (i * 7) % 11 + 1]
            })
            .collect();
        let mut sorted = buildings.clone();
        sorted.sort_unstable();
        let mut expected = Vec::new();
        let mut previous = 0;
        for x in 0..400 {
            let h = sorted
                .iter()
                .filter(|b| b[0] <= x && x < b[1])
                .map(|b| b[2])
                .max()
                .unwrap_or(0);
            if h != previous {
                expected.push([x, h]);
                previous = h;
            }
        }
        assert_eq!(run(&sorted), expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** build `(x, height, is_start)` events, sort them by `x`, and process them in groups of
equal `x`. A `BTreeMap<i32, usize>` counts active heights (a multiset); its last key is the
current maximum. After each group, a changed maximum produces a key point.

**Complexity:** `O(n log n)` time, `O(n)` space.

**Rust concepts:**
- `BTreeMap::last_key_value` (stable since 1.66) gives the maximum in `O(log n)`.
- Removing one occurrence from a multiset: decrement the count and `remove` the key at zero.
- `slice::chunk_by` (stable since 1.77) splits the sorted events into groups with equal `x`.

```rust
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut events: Vec<(i32, i32, bool)> = Vec::with_capacity(2 * buildings.len());
        for b in &buildings {
            events.push((b[0], b[2], true));
            events.push((b[1], b[2], false));
        }
        events.sort_unstable();
        let mut active: BTreeMap<i32, usize> = BTreeMap::from([(0, 1)]);
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut previous = 0;
        for group in events.chunk_by(|a, b| a.0 == b.0) {
            for &(_, height, is_start) in group {
                if is_start {
                    *active.entry(height).or_insert(0) += 1;
                } else if let Some(count) = active.get_mut(&height) {
                    *count -= 1;
                    if *count == 0 {
                        active.remove(&height);
                    }
                }
            }
            let current = active.last_key_value().map_or(0, |(&h, _)| h);
            if current != previous {
                result.push(vec![group[0].0, current]);
                previous = current;
            }
        }
        result
    }
}
```

Full tested solution: [`src/solutions/p0980_the_skyline_problem.rs`](../src/solutions/p0980_the_skyline_problem.rs)

</details>
