# Problem 0075: Alternating Digit Sum

- **Difficulty:** Easy
- **Topics:** digit loop, alternating signs, fixing the sign at the end
- **Source:** [LeetCode 2544. Alternating Digit Sum](https://leetcode.com/problems/alternating-digit-sum/)

## Problem

Given a positive integer `n`, give its digits alternating signs, starting with a **positive**
sign on the most significant (leftmost) digit: `+ - + - ...`. Return the resulting sum.

Implement `Solution::alternate_digit_sum(n: i32) -> i32`.

## Examples

```text
Input:  n = 521
Output: 4
Explanation: (+5) + (-2) + (+1) = 4.

Input:  n = 111
Output: 1

Input:  n = 886996
Output: 0
Explanation: 8 - 8 + 6 - 9 + 9 - 6 = 0.
```

## Constraints

- `1 <= n <= 10^9`

## Hints

1. The digit loop reads digits from the **right**, but the signs are defined from the **left**.
2. Compute the sum with signs starting from the right (`+` for the last digit). If the number of
   digits is even, the leftmost digit got a `-`, so the whole result must be negated.

## Starter code

Write your code in [`src/practice/p0075_alternating_digit_sum.rs`](../src/practice/p0075_alternating_digit_sum.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0075
```

```rust
pub struct Solution;

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
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
    fn example_521() {
        assert_eq!(Solution::alternate_digit_sum(521), 4);
    }

    #[test]
    fn example_111() {
        assert_eq!(Solution::alternate_digit_sum(111), 1);
    }

    #[test]
    fn example_886996() {
        assert_eq!(Solution::alternate_digit_sum(886_996), 0);
    }

    #[test]
    fn even_number_of_digits() {
        assert_eq!(Solution::alternate_digit_sum(10), 1);
        assert_eq!(Solution::alternate_digit_sum(19), -8);
        // 9081: +9 -0 +8 -1 = 16
        assert_eq!(Solution::alternate_digit_sum(9_081), 16);
    }

    #[test]
    fn extremes() {
        assert_eq!(Solution::alternate_digit_sum(1), 1);
        assert_eq!(Solution::alternate_digit_sum(1_000_000_000), 1);
        assert_eq!(Solution::alternate_digit_sum(909_090_909), 45);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** walk the digits from right to left with a sign that flips after every digit, starting
at `+1`. When the loop ends, `sign` tells which sign the **next** digit would have had: if it
is `+1`, the leftmost digit got `-1`, so negate the total.

**Complexity:** `O(d)` time, `O(1)` space.

**Rust concepts:**
- `sign = -sign` flips between `1` and `-1`; `sign * d` applies it.
- The final `if` expression chooses between `sum` and `-sum` and is the return value.

```rust
pub struct Solution;

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut x = n;
        let mut sum = 0;
        let mut sign = 1;
        while x > 0 {
            sum += sign * (x % 10);
            sign = -sign;
            x /= 10;
        }
        // After the loop, `sign` is the sign a further digit would get.
        // If that is +1, the leftmost digit received -1: flip everything.
        if sign == 1 { -sum } else { sum }
    }
}
```

Full tested solution: [`src/solutions/p0075_alternating_digit_sum.rs`](../src/solutions/p0075_alternating_digit_sum.rs)

</details>
