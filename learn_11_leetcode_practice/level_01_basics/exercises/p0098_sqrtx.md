# Problem 0098: Sqrt(x)

- **Difficulty:** Easy
- **Topics:** binary search with `while`, widening to `i64` to avoid overflow, loop invariants
- **Source:** [LeetCode 69. Sqrt(x)](https://leetcode.com/problems/sqrtx/)

## Problem

Given a non-negative integer `x`, return its square root **rounded down** to the nearest
integer: the largest integer `r` with `r * r <= x`. Do not use built-in power or square-root
functions (such as `f64::sqrt` or `i32::isqrt`).

Implement `Solution::my_sqrt(x: i32) -> i32`.

## Examples

```text
Input:  x = 4
Output: 2

Input:  x = 8
Output: 2
Explanation: the square root of 8 is 2.828..., which rounds down to 2.
```

## Constraints

- `0 <= x <= 2^31 - 1`

## Hints

1. Trying `r = 0, 1, 2, ...` takes up to 46 341 steps; binary search needs about 31.
2. Keep a range `[lo, hi]` that always contains the answer, and test the middle.
3. `mid * mid` overflows an `i32` when `mid` is about 46 341 or more: compute it in `i64`.

## Starter code

Write your code in [`src/practice/p0098_sqrtx.rs`](../src/practice/p0098_sqrtx.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0098
```

```rust
pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
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
    fn example_perfect_square() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn example_rounds_down() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }

    #[test]
    fn small_values() {
        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(2), 1);
        assert_eq!(Solution::my_sqrt(3), 1);
        assert_eq!(Solution::my_sqrt(9), 3);
    }

    #[test]
    fn largest_values() {
        assert_eq!(Solution::my_sqrt(i32::MAX), 46_340);
        assert_eq!(Solution::my_sqrt(46_340 * 46_340), 46_340);
        assert_eq!(Solution::my_sqrt(46_340 * 46_340 - 1), 46_339);
    }

    #[test]
    fn floor_property_holds() {
        for x in (0..=i32::MAX).step_by(9_973).chain(0..10_000) {
            let r = i64::from(Solution::my_sqrt(x));
            let x = i64::from(x);
            assert!(r * r <= x && x < (r + 1) * (r + 1), "x = {x}, r = {r}");
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** binary search for the last `r` with `r * r <= x`. Keep the invariant
`lo * lo <= x < (hi + 1) * (hi + 1)`, starting from `lo = 0`, `hi = x`. Pick the **upper** middle
`mid = lo + (hi - lo + 1) / 2`; if `mid * mid <= x` the answer is at least `mid` (`lo = mid`),
otherwise it is below `mid` (`hi = mid - 1`). When `lo == hi`, that is the answer.

**Complexity:** `O(log x)` time, `O(1)` space.

**Rust concepts:**
- Doing the arithmetic in `i64` makes `mid * mid` safe (at most about `2^62`), and the final
  `as i32` cannot truncate because the answer is at most 46 340.
- Rounding the middle **up** matters: with `lo = mid` as one branch, a middle rounded down could
  leave the range unchanged and loop forever.
- `lo + (hi - lo + 1) / 2` avoids computing `lo + hi`, the classic overflow in binary search
  (see problem 0029).

```rust
pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = i64::from(x);
        let (mut lo, mut hi) = (0_i64, x);
        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            if mid * mid <= x {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo as i32
    }
}
```

Full tested solution: [`src/solutions/p0098_sqrtx.rs`](../src/solutions/p0098_sqrtx.rs)

</details>
