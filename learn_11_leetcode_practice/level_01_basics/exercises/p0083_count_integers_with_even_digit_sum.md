# Problem 0083: Count Integers With Even Digit Sum

- **Difficulty:** Easy
- **Topics:** `for` over `1..=num`, a nested helper function, digit sums
- **Source:** [LeetCode 2180. Count Integers With Even Digit Sum](https://leetcode.com/problems/count-integers-with-even-digit-sum/)

## Problem

Given a positive integer `num`, return how many positive integers `x <= num` have an **even**
digit sum (the sum of the decimal digits of `x`).

Implement `Solution::count_even(num: i32) -> i32`.

## Examples

```text
Input:  num = 4
Output: 2
Explanation: 2 and 4.

Input:  num = 30
Output: 14
Explanation: 2, 4, 6, 8, 11, 13, 15, 17, 19, 20, 22, 24, 26 and 28.
```

## Constraints

- `1 <= num <= 1000`

## Hints

1. Check every `x` in `1..=num` with a digit-sum helper.
2. Pattern spotting (optional): among every 10 consecutive numbers `10k ..= 10k + 9`, exactly 5 have an even digit sum.

## Starter code

Write your code in [`src/practice/p0083_count_integers_with_even_digit_sum.rs`](../src/practice/p0083_count_integers_with_even_digit_sum.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0083
```

```rust
pub struct Solution;

impl Solution {
    pub fn count_even(num: i32) -> i32 {
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
    fn example_four() {
        assert_eq!(Solution::count_even(4), 2);
    }

    #[test]
    fn example_thirty() {
        assert_eq!(Solution::count_even(30), 14);
    }

    #[test]
    fn smallest_inputs() {
        assert_eq!(Solution::count_even(1), 0);
        assert_eq!(Solution::count_even(2), 1);
    }

    #[test]
    fn largest_input() {
        // 1..=999 contains 499 such numbers (0 is excluded from 0..=999's 500); 1000 has digit sum 1.
        assert_eq!(Solution::count_even(999), 499);
        assert_eq!(Solution::count_even(1000), 499);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** brute force is tiny here: at most 1000 numbers with at most 4 digits each. Count the
`x` whose digit sum is even.

**Complexity:** `O(num * d)` time, `O(1)` space.

**Rust concepts:**
- A nested `fn digit_sum` declared inside the method is visible only there.
- The helper takes its parameter as `mut x: i32`: the binding is mutable inside the helper, and
  the caller's value is unaffected because integers are copied.

```rust
pub struct Solution;

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        fn digit_sum(mut x: i32) -> i32 {
            let mut sum = 0;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            sum
        }
        let mut count = 0;
        for x in 1..=num {
            if digit_sum(x) % 2 == 0 {
                count += 1;
            }
        }
        count
    }
}
```

Full tested solution: [`src/solutions/p0083_count_integers_with_even_digit_sum.rs`](../src/solutions/p0083_count_integers_with_even_digit_sum.rs)

</details>
