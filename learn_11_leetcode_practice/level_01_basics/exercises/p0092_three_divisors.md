# Problem 0092: Three Divisors

- **Difficulty:** Easy
- **Topics:** counting divisors in pairs up to `sqrt(n)`, `while`, early exit
- **Source:** [LeetCode 1952. Three Divisors](https://leetcode.com/problems/three-divisors/)

## Problem

Given an integer `n`, return `true` if `n` has **exactly three** positive divisors, and `false`
otherwise.

Implement `Solution::is_three(n: i32) -> bool`.

## Examples

```text
Input:  n = 2
Output: false
Explanation: 2 has only two divisors: 1 and 2.

Input:  n = 4
Output: true
Explanation: 4 has the divisors 1, 2 and 4.

Input:  n = 12
Output: false
```

## Constraints

- `1 <= n <= 10^4`

## Hints

1. Divisors come in pairs `(d, n / d)`. Only a perfect square has an unpaired divisor (its root).
2. So an odd number of divisors means `n` is a perfect square; exactly three means `n = p^2` for a prime `p`.

## Starter code

Write your code in [`src/practice/p0092_three_divisors.rs`](../src/practice/p0092_three_divisors.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0092
```

```rust
pub struct Solution;

impl Solution {
    pub fn is_three(n: i32) -> bool {
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
        assert!(!Solution::is_three(2));
    }

    #[test]
    fn example_four() {
        assert!(Solution::is_three(4));
    }

    #[test]
    fn example_twelve() {
        assert!(!Solution::is_three(12));
    }

    #[test]
    fn squares_of_primes_only() {
        assert!(!Solution::is_three(1));
        assert!(Solution::is_three(9));
        assert!(Solution::is_three(49));
        assert!(!Solution::is_three(16)); // 1, 2, 4, 8, 16
        assert!(!Solution::is_three(36));
        assert!(Solution::is_three(9409)); // 97^2
    }

    #[test]
    fn count_up_to_the_limit() {
        // The primes below 100 are 25, so 25 numbers up to 10^4 are squares of primes.
        let count = (1..=10_000).filter(|&n| Solution::is_three(n)).count();
        assert_eq!(count, 25);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** count divisors by checking `d` up to `sqrt(n)`: each `d` that divides `n` adds 2 (for
`d` and `n / d`), or 1 when `d * d == n`. Stop early once the count exceeds 3.

**Complexity:** `O(sqrt(n))` time, `O(1)` space.

**Rust concepts:**
- `if d * d == n { 1 } else { 2 }` used as a value inside `+=`.
- `while d * d <= n` is safe here: `d` stays around 100, far from overflow.
- Leaving the loop early with `return false` saves work for numbers with many divisors.

```rust
pub struct Solution;

impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut count = 0;
        let mut d = 1;
        while d * d <= n {
            if n % d == 0 {
                count += if d * d == n { 1 } else { 2 };
                if count > 3 {
                    return false;
                }
            }
            d += 1;
        }
        count == 3
    }
}
```

Full tested solution: [`src/solutions/p0092_three_divisors.rs`](../src/solutions/p0092_three_divisors.rs)

</details>
