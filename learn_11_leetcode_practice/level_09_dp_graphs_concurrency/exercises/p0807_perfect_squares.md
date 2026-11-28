# Problem 0807: Perfect Squares

- **Difficulty:** Medium
- **Topics:** unbounded "minimum count" DP, `take_while` on an infinite range, `map`/`min`
- **Source:** [LeetCode 279. Perfect Squares](https://leetcode.com/problems/perfect-squares/)

## Problem

A perfect square is the square of a positive integer: `1, 4, 9, 16, ...`. Given `n`, return the
smallest number of perfect squares (repetition allowed) whose sum is exactly `n`.

Implement `Solution::num_squares(n: i32) -> i32`.

## Examples

```text
Input:  n = 12
Output: 3
Explanation: 12 = 4 + 4 + 4.

Input:  n = 13
Output: 2
Explanation: 13 = 4 + 9.

Input:  n = 7
Output: 4
Explanation: 7 = 4 + 1 + 1 + 1; no shorter sum exists.
```

## Constraints

- `1 <= n <= 10^4`

## Hints

1. Let `dp[i]` be the answer for `i`. The last square in an optimal sum is some `s * s <= i`.
2. `dp[i] = 1 + min(dp[i - s * s])` over all such `s`, with `dp[0] = 0`.

## Starter code

Write your code in [`src/practice/p0807_perfect_squares.rs`](../src/practice/p0807_perfect_squares.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0807
```

```rust
pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
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
    fn example_twelve() {
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn example_thirteen() {
        assert_eq!(Solution::num_squares(13), 2);
    }

    #[test]
    fn seven_needs_four() {
        assert_eq!(Solution::num_squares(7), 4);
    }

    #[test]
    fn perfect_squares_need_one() {
        assert_eq!(Solution::num_squares(1), 1);
        assert_eq!(Solution::num_squares(10_000), 1);
    }

    #[test]
    fn largest_four_square_case() {
        // 9999 = 8 * 1249 + 7, and numbers of the form 8k + 7 always need four squares.
        assert_eq!(Solution::num_squares(9_999), 4);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** compute `dp[0..=n]` from small to large. For each `i`, try every square `s * s <= i`
as the last term and keep the best `dp[i - s * s] + 1`. Since `1` is a square, every `i` has an
answer, so the minimum always exists.

**Complexity:** `O(n * sqrt(n))` time, `O(n)` space.

**Rust concepts:**
- `(1..).map(|s| s * s).take_while(|&sq| sq <= i)` generates the squares lazily and stops at
  the first one that is too large.
- `Iterator::min` returns an `Option`; the range is never empty for `i >= 1` (it contains `1`),
  which justifies `unwrap_or` with a harmless fallback instead of `unwrap`.

```rust
pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0_i32; n + 1];
        for i in 1..=n {
            dp[i] = (1..)
                .map(|s| s * s)
                .take_while(|&sq| sq <= i)
                .map(|sq| dp[i - sq] + 1)
                .min()
                .unwrap_or(i as i32); // never used: 1 is always a square
        }
        dp[n]
    }
}
```

Full tested solution: [`src/solutions/p0807_perfect_squares.rs`](../src/solutions/p0807_perfect_squares.rs)

</details>
