# Problem 0090: Number of Common Factors

- **Difficulty:** Easy
- **Topics:** `for` over `1..=min(a, b)`, `%` with `&&`, counting
- **Source:** [LeetCode 2427. Number of Common Factors](https://leetcode.com/problems/number-of-common-factors/)

## Problem

Given two positive integers `a` and `b`, return the number of their **common factors**: the
integers `x` that divide both `a` and `b` exactly.

Implement `Solution::common_factors(a: i32, b: i32) -> i32`.

## Examples

```text
Input:  a = 12, b = 6
Output: 4
Explanation: 1, 2, 3 and 6.

Input:  a = 25, b = 30
Output: 2
Explanation: 1 and 5.
```

## Constraints

- `1 <= a, b <= 1000`

## Hints

1. A common factor cannot be larger than the smaller of the two numbers.
2. The common factors of `a` and `b` are exactly the factors of `gcd(a, b)`.

## Starter code

Write your code in [`src/practice/p0090_number_of_common_factors.rs`](../src/practice/p0090_number_of_common_factors.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0090
```

```rust
pub struct Solution;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
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
    fn example_twelve_six() {
        assert_eq!(Solution::common_factors(12, 6), 4);
    }

    #[test]
    fn example_twenty_five_thirty() {
        assert_eq!(Solution::common_factors(25, 30), 2);
    }

    #[test]
    fn coprime_and_equal() {
        assert_eq!(Solution::common_factors(1, 1000), 1);
        assert_eq!(Solution::common_factors(997, 991), 1);
        assert_eq!(Solution::common_factors(1000, 1000), 16);
    }

    #[test]
    fn highly_divisible() {
        // gcd(840, 720) = 120, which has 16 divisors.
        assert_eq!(Solution::common_factors(840, 720), 16);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** try every `x` from 1 to `min(a, b)` and count those with `a % x == 0 && b % x == 0`.

**Complexity:** `O(min(a, b))` time, `O(1)` space.

**Rust concepts:**
- `a.min(b)` as the upper end of an inclusive range.
- The iterator form is `(1..=a.min(b)).filter(|x| a % x == 0 && b % x == 0).count() as i32`;
  `count` returns a `usize`, which is why the cast is needed.

```rust
pub struct Solution;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut count = 0;
        for x in 1..=a.min(b) {
            if a % x == 0 && b % x == 0 {
                count += 1;
            }
        }
        count
    }
}
```

Full tested solution: [`src/solutions/p0090_number_of_common_factors.rs`](../src/solutions/p0090_number_of_common_factors.rs)

</details>
