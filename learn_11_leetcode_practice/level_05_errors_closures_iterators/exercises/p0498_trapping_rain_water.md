# Problem 0498: Trapping Rain Water

- **Difficulty:** Hard
- **Topics:** running maximum with `scan` from both ends, `rev`, three-way `zip`, `map` + `sum`
- **Source:** [LeetCode 42. Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/)

## Problem

`height[i]` is the height of a bar of width 1 in an elevation map. After it rains, water is
trapped between the bars. Compute how many units of water the map holds.

Implement `Solution::trap(height: Vec<i32>) -> i32`.

## Examples

```text
Input:  height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
Output: 6

Input:  height = [4, 2, 0, 3, 2, 5]
Output: 9
```

## Constraints

- `1 <= height.len() <= 2 * 10^4`
- `0 <= height[i] <= 10^5`

## Hints

1. Above position `i`, the water level is limited by the highest bar on its left **and** the
   highest bar on its right: `level = min(max_left[i], max_right[i])` (both including `i`). The
   water above `i` is `level - height[i]`.
2. `max_left` is a running maximum: `scan(0, |m, &h| { *m = (*m).max(h); Some(*m) })`.
   `max_right` is the same scan over `height.iter().rev()`.

## Starter code

Write your code in [`src/practice/p0498_trapping_rain_water.rs`](../src/practice/p0498_trapping_rain_water.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0498
```

```rust
pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
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
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn example_deep_valley() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn nothing_to_trap() {
        assert_eq!(Solution::trap(vec![5]), 0);
        assert_eq!(Solution::trap(vec![1, 2, 3, 4]), 0);
        assert_eq!(Solution::trap(vec![4, 3, 2, 1]), 0);
        assert_eq!(Solution::trap(vec![3, 0]), 0);
    }

    #[test]
    fn largest_possible_amount() {
        let mut height = vec![0; 20_000];
        height[0] = 100_000;
        height[19_999] = 100_000;
        assert_eq!(Solution::trap(height), 19_998 * 100_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** compute the running maximum from the left and from the right with two `scan`s, then
zip the three sequences (heights, left maxima, right maxima restored to normal order) and sum the
water column by column.

**Complexity:** `O(n)` time, `O(n)` extra space. (A two-pointer version needs only `O(1)` space;
it is covered in the algorithms level.)

**Rust concepts:**
- The same scan closure works for both directions because it only depends on the order in which
  items arrive.
- `zip` can be chained: `a.zip(b).zip(c)` yields `((x, y), z)`, destructured in the closure.
- `right.into_iter().rev()` reverses the right maxima back to index order without copying.
- Every column contributes at most `10^5`, and there are at most `2 * 10^4` columns, so the sum
  (at most `2 * 10^9`) fits in an `i32`.

```rust
pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let running_max = |m: &mut i32, &h: &i32| {
            *m = (*m).max(h);
            Some(*m)
        };
        let left: Vec<i32> = height.iter().scan(0, running_max).collect();
        let right: Vec<i32> = height.iter().rev().scan(0, running_max).collect();
        height
            .iter()
            .zip(left)
            .zip(right.into_iter().rev())
            .map(|((&h, l), r)| l.min(r) - h)
            .sum()
    }
}
```

Full tested solution: [`src/solutions/p0498_trapping_rain_water.rs`](../src/solutions/p0498_trapping_rain_water.rs)

</details>
