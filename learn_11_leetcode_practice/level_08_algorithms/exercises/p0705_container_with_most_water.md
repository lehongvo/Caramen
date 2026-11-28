# Problem 0705: Container With Most Water

- **Difficulty:** Medium
- **Topics:** two pointers, greedy elimination argument, `max`
- **Source:** [LeetCode 11. Container With Most Water](https://leetcode.com/problems/container-with-most-water/)

## Problem

`height[i]` is the height of a vertical line drawn at position `i` on the x-axis. Choose two of
the lines; together with the x-axis they form a container. The water it holds is
`(j - i) * min(height[i], height[j])` for lines `i < j` (the container cannot be tilted).

Return the largest amount of water any pair of lines can hold.

Implement `Solution::max_area(height: Vec<i32>) -> i32`.

## Examples

```text
Input:  height = [1, 8, 6, 2, 5, 4, 8, 3, 7]
Output: 49            (lines 1 and 8: width 7, height min(8, 7) = 7)

Input:  height = [1, 1]
Output: 1
```

## Constraints

- `2 <= height.len() <= 10^5`
- `0 <= height[i] <= 10^4`

## Hints

1. Trying every pair is `O(n^2)`, too slow for `10^5` lines.
2. Start with the widest container (the two outer lines). Which line can you safely discard?

## Starter code

Write your code in [`src/practice/p0705_container_with_most_water.rs`](../src/practice/p0705_container_with_most_water.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0705
```

```rust
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
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
    fn example_classic() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn example_two_lines() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn outer_lines_win() {
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn zero_height_lines() {
        assert_eq!(Solution::max_area(vec![0, 0, 0]), 0);
    }

    #[test]
    fn best_pair_is_inside() {
        // Lines at 1 and 3 (height 10, width 2) beat the outer pair (height 1, width 4).
        assert_eq!(Solution::max_area(vec![1, 10, 1, 10, 1]), 20);
    }

    #[test]
    fn largest_input() {
        assert_eq!(Solution::max_area(vec![10_000; 100_000]), 10_000 * 99_999);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** two pointers: start with `lo = 0` and `hi = n - 1`, the widest container. Suppose
`height[lo] <= height[hi]`. Every other container that uses line `lo` is narrower, and its
height is still capped by `height[lo]`, so none of them can beat the current one. Line `lo` is
therefore finished and `lo` moves right. Symmetrically, move `hi` left when it is the shorter
line. Every step discards one line, and the best container is never discarded.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `i32::max` / `i32::min` are methods on the integer types: `best.max(area)`.
- The widths are `usize` indices; `(hi - lo) as i32` is safe because `n <= 10^5`, and the
  largest area `10^4 * 10^5 = 10^9` fits in `i32`.

```rust
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, height.len() - 1); // len >= 2 by the constraints
        let mut best = 0;
        while lo < hi {
            let area = (hi - lo) as i32 * height[lo].min(height[hi]);
            best = best.max(area);
            // The shorter line cannot be part of a better container: discard it.
            if height[lo] <= height[hi] {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
        best
    }
}
```

Full tested solution: [`src/solutions/p0705_container_with_most_water.rs`](../src/solutions/p0705_container_with_most_water.rs)

</details>
