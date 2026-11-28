# Problem 0096: Fibonacci Number

- **Difficulty:** Easy
- **Topics:** iteration instead of recursion, tuple assignment `(a, b) = (b, a + b)`, `for _ in`
- **Source:** [LeetCode 509. Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)

## Problem

The Fibonacci numbers are defined by `F(0) = 0`, `F(1) = 1`, and `F(n) = F(n - 1) + F(n - 2)`
for `n > 1`. Given `n`, return `F(n)`.

Implement `Solution::fib(n: i32) -> i32`.

## Examples

```text
Input:  n = 2
Output: 1
Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.

Input:  n = 3
Output: 2

Input:  n = 4
Output: 3
```

## Constraints

- `0 <= n <= 30`

## Hints

1. A direct recursive function works but recomputes the same values exponentially many times.
2. Keep only the last two values and move forward `n` times.

## Starter code

Write your code in [`src/practice/p0096_fibonacci_number.rs`](../src/practice/p0096_fibonacci_number.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0096
```

```rust
pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
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
    fn examples() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }

    #[test]
    fn base_cases() {
        assert_eq!(Solution::fib(0), 0);
        assert_eq!(Solution::fib(1), 1);
    }

    #[test]
    fn first_ten() {
        let values: Vec<i32> = (0..10).map(Solution::fib).collect();
        assert_eq!(values, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn largest_input() {
        assert_eq!(Solution::fib(30), 832_040);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** start with `(a, b) = (F(0), F(1)) = (0, 1)`. One step turns `(F(k), F(k + 1))` into
`(F(k + 1), F(k + 2))`. After `n` steps, `a` is `F(n)`.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `(a, b) = (b, a + b);` updates both variables "at the same time": the right side is evaluated
  with the old values before either is assigned.
- `for _ in 0..n` repeats a body `n` times; `_` says the counter is not needed.
- `F(30) = 832_040` fits easily in an `i32`; `F(46)` is the largest Fibonacci number that does.

```rust
pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let (mut a, mut b) = (0, 1);
        for _ in 0..n {
            (a, b) = (b, a + b);
        }
        a
    }
}
```

Full tested solution: [`src/solutions/p0096_fibonacci_number.rs`](../src/solutions/p0096_fibonacci_number.rs)

</details>
