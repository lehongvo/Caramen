# Problem 0100: Count Primes

- **Difficulty:** Medium
- **Topics:** sieve of Eratosthenes, `vec![true; n]`, `step_by`, `usize` indices, `as` casts
- **Source:** [LeetCode 204. Count Primes](https://leetcode.com/problems/count-primes/)

## Problem

Given an integer `n`, return the number of prime numbers that are **strictly less** than `n`.

Implement `Solution::count_primes(n: i32) -> i32`.

## Examples

```text
Input:  n = 10
Output: 4
Explanation: 2, 3, 5 and 7.

Input:  n = 0
Output: 0

Input:  n = 1
Output: 0
```

## Constraints

- `0 <= n <= 5 * 10^6`

## Hints

1. Testing each number by trial division (problem 0091) is too slow for five million numbers.
2. Sieve of Eratosthenes: start with every number marked "prime"; for each prime `p`, cross out
   its multiples.
3. Crossing out can start at `p * p`: smaller multiples were already crossed out by smaller primes.
   And you only need primes with `p * p < n`.

## Starter code

Write your code in [`src/practice/p0100_count_primes.rs`](../src/practice/p0100_count_primes.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0100
```

```rust
pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
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
    fn example_ten() {
        assert_eq!(Solution::count_primes(10), 4);
    }

    #[test]
    fn example_zero_and_one() {
        assert_eq!(Solution::count_primes(0), 0);
        assert_eq!(Solution::count_primes(1), 0);
    }

    #[test]
    fn strictly_less_than_n() {
        assert_eq!(Solution::count_primes(2), 0);
        assert_eq!(Solution::count_primes(3), 1);
        assert_eq!(Solution::count_primes(11), 4);
        assert_eq!(Solution::count_primes(12), 5);
    }

    #[test]
    fn known_counts() {
        assert_eq!(Solution::count_primes(100), 25);
        assert_eq!(Solution::count_primes(1_000), 168);
        assert_eq!(Solution::count_primes(1_000_000), 78_498);
    }

    #[test]
    fn largest_input() {
        assert_eq!(Solution::count_primes(5_000_000), 348_513);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** keep `is_prime: Vec<bool>` of length `n`, all `true` except indices 0 and 1. For each
`p` with `p * p < n` that is still marked, mark `p * p, p * p + p, ...` as `false`. Finally count
the `true` entries.

**Complexity:** `O(n log log n)` time, `O(n)` space.

**Rust concepts:**
- `vec![true; n]` allocates a vector of `n` copies of `true`.
- `(p * p..n).step_by(p)` visits exactly the multiples to cross out; this is `for` over a range
  with a step, as in problem 0055.
- The input is an `i32`, but indices are `usize`; convert once with `n as usize` after handling
  the small cases, and convert the count back with `as i32`.
- `.iter().filter(|&&b| b).count()` counts the `true` values; the double `&` appears because
  `filter` passes a reference to each item, and the items are themselves references.

```rust
pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }
        let n = n as usize;
        let mut is_prime = vec![true; n];
        is_prime[0] = false;
        is_prime[1] = false;
        let mut p = 2;
        while p * p < n {
            if is_prime[p] {
                for multiple in (p * p..n).step_by(p) {
                    is_prime[multiple] = false;
                }
            }
            p += 1;
        }
        is_prime.iter().filter(|&&b| b).count() as i32
    }
}
```

Full tested solution: [`src/solutions/p0100_count_primes.rs`](../src/solutions/p0100_count_primes.rs)

</details>
