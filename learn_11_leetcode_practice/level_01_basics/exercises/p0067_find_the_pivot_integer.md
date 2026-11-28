# Problem 0067: Find the Pivot Integer

- **Difficulty:** Easy
- **Topics:** `for` with an early `return`, running sums, sum formula `n * (n + 1) / 2`
- **Source:** [LeetCode 2485. Find the Pivot Integer](https://leetcode.com/problems/find-the-pivot-integer/)

## Problem

Given a positive integer `n`, find the **pivot integer** `x`: the number with
`1 + 2 + ... + x == x + (x + 1) + ... + n` (both sums include `x`). Return `x`, or `-1` if no
such integer exists. There is at most one pivot for a given `n`.

Implement `Solution::pivot_integer(n: i32) -> i32`.

## Examples

```text
Input:  n = 8
Output: 6
Explanation: 1 + 2 + 3 + 4 + 5 + 6 = 21 and 6 + 7 + 8 = 21.

Input:  n = 1
Output: 1

Input:  n = 4
Output: -1
```

## Constraints

- `1 <= n <= 1000`

## Hints

1. The total `1 + ... + n` is `n * (n + 1) / 2`.
2. Walk `x` upward, keeping the left sum `1 + ... + x`. The right sum is `total - left + x`.

## Starter code

Write your code in [`src/practice/p0067_find_the_pivot_integer.rs`](../src/practice/p0067_find_the_pivot_integer.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0067
```

```rust
pub struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
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
    fn example_eight() {
        assert_eq!(Solution::pivot_integer(8), 6);
    }

    #[test]
    fn example_one() {
        assert_eq!(Solution::pivot_integer(1), 1);
    }

    #[test]
    fn example_no_pivot() {
        assert_eq!(Solution::pivot_integer(4), -1);
        assert_eq!(Solution::pivot_integer(2), -1);
    }

    #[test]
    fn all_pivots_up_to_1000() {
        let found: Vec<(i32, i32)> = (1..=1000)
            .map(|n| (n, Solution::pivot_integer(n)))
            .filter(|&(_, x)| x != -1)
            .collect();
        assert_eq!(found, vec![(1, 1), (8, 6), (49, 35), (288, 204)]);
    }

    #[test]
    fn matches_square_root_rule() {
        for n in 1..=1000 {
            let total = n * (n + 1) / 2;
            let x = Solution::pivot_integer(n);
            if x == -1 {
                let r = (total as f64).sqrt() as i32;
                assert_ne!(r * r, total, "n = {n}");
            } else {
                assert_eq!(x * x, total, "n = {n}");
            }
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** compute `total` once, then for `x` in `1..=n` add `x` to `left`. Because both sums
include `x`, the right sum is `total - left + x`; return `x` when they are equal. If the loop
ends, return `-1`.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `return x;` inside a `for` loop exits the whole function, not just the loop.
- After the loop, `-1` is the final expression, the "not found" result.
- Algebra gives a closed form: the equation simplifies to `x * x == total`, so the answer is the
  integer square root of `total` when it is a perfect square. The tests check that.

```rust
pub struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let total = n * (n + 1) / 2;
        let mut left = 0;
        for x in 1..=n {
            left += x;
            let right = total - left + x;
            if left == right {
                return x;
            }
        }
        -1
    }
}
```

Full tested solution: [`src/solutions/p0067_find_the_pivot_integer.rs`](../src/solutions/p0067_find_the_pivot_integer.rs)

</details>
