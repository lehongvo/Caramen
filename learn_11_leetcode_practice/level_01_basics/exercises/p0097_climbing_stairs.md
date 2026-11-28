# Problem 0097: Climbing Stairs

- **Difficulty:** Easy
- **Topics:** recognizing a recurrence, iterative Fibonacci, tuple assignment
- **Source:** [LeetCode 70. Climbing Stairs](https://leetcode.com/problems/climbing-stairs/)

## Problem

A staircase has `n` steps. Each move climbs either **1** or **2** steps. In how many distinct
ways can you reach the top? Two ways are different if the sequence of moves differs.

Implement `Solution::climb_stairs(n: i32) -> i32`.

## Examples

```text
Input:  n = 2
Output: 2
Explanation: 1 + 1, or 2.

Input:  n = 3
Output: 3
Explanation: 1 + 1 + 1, 1 + 2, or 2 + 1.
```

## Constraints

- `1 <= n <= 45`

## Hints

1. Look at the **last** move: it is either a 1-step from step `n - 1` or a 2-step from step `n - 2`.
2. So `ways(n) = ways(n - 1) + ways(n - 2)`. Which sequence is that?

## Starter code

Write your code in [`src/practice/p0097_climbing_stairs.rs`](../src/practice/p0097_climbing_stairs.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0097
```

```rust
pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
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
    fn example_two() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn small_staircases() {
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
    }

    #[test]
    fn largest_input_fits_in_i32() {
        assert_eq!(Solution::climb_stairs(45), 1_836_311_903);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** with `ways(0) = 1` (one way to stay at the bottom) and `ways(1) = 1`, the recurrence
`ways(n) = ways(n - 1) + ways(n - 2)` gives the Fibonacci numbers shifted by one:
`ways(n) = F(n + 1)`. Iterate with two variables as in problem 0096.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- The same `(a, b) = (b, a + b)` loop as problem 0096, with different starting values.
- `ways(45) = 1_836_311_903` is just below `i32::MAX = 2_147_483_647`, which is why the
  constraint stops at 45. `ways(46)` would overflow, so the loop runs `n - 1` times (`1..n`) and
  returns `b = ways(n)`; a loop that computed one value "too far" would panic for `n = 45` in a
  debug build.

```rust
pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // (a, b) = (ways(k - 1), ways(k)), starting at k = 1.
        let (mut a, mut b) = (1, 1);
        for _ in 1..n {
            (a, b) = (b, a + b);
        }
        b
    }
}
```

Full tested solution: [`src/solutions/p0097_climbing_stairs.rs`](../src/solutions/p0097_climbing_stairs.rs)

</details>
