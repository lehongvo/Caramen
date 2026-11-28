# Problem 0084: Minimum Sum of Four Digit Number After Splitting Digits

- **Difficulty:** Easy
- **Topics:** fixed-size array `[i32; 4]`, sorting an array, place values, greedy reasoning
- **Source:** [LeetCode 2160. Minimum Sum of Four Digit Number After Splitting Digits](https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/)

## Problem

You are given a four-digit positive integer `num`. Use **all four** of its digits (each exactly
once) to form two new integers `new1` and `new2`. Leading zeros are allowed in them, and either
may have any number of the digits. Return the minimum possible value of `new1 + new2`.

Implement `Solution::minimum_sum(num: i32) -> i32`.

## Examples

```text
Input:  num = 2932
Output: 52
Explanation: digits 2, 9, 3, 2. Form 29 and 23: 29 + 23 = 52.

Input:  num = 4009
Output: 13
Explanation: form 04 and 09: 4 + 9 = 13.
```

## Constraints

- `1000 <= num <= 9999`

## Hints

1. Using two 2-digit numbers is always best: a 3-digit number would put some digit in the hundreds place.
2. The two smallest digits should be the tens digits, and the two largest the units digits.
3. Put the digits into an array `[i32; 4]` and call `.sort()` on it.

## Starter code

Write your code in [`src/practice/p0084_minimum_sum_of_four_digit_number_after_splitting_digits.rs`](../src/practice/p0084_minimum_sum_of_four_digit_number_after_splitting_digits.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0084
```

```rust
pub struct Solution;

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
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
    fn example_2932() {
        assert_eq!(Solution::minimum_sum(2932), 52);
    }

    #[test]
    fn example_4009() {
        assert_eq!(Solution::minimum_sum(4009), 13);
    }

    #[test]
    fn boundaries() {
        assert_eq!(Solution::minimum_sum(1000), 1);
        assert_eq!(Solution::minimum_sum(9999), 198);
    }

    #[test]
    fn all_different_digits() {
        // digits 1, 2, 3, 4: 13 + 24 = 37
        assert_eq!(Solution::minimum_sum(4321), 37);
        assert_eq!(Solution::minimum_sum(1234), 37);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** extract the four digits into an array, sort it, and build `10 * d[0] + d[2]` and
`10 * d[1] + d[3]`. The sum is `10 * (d[0] + d[1]) + d[2] + d[3]`.

**Complexity:** `O(1)` time and space (always four digits).

**Rust concepts:**
- `let mut digits = [0; 4];` is a stack array whose size is fixed at compile time; arrays have
  the slice methods, including `sort`.
- Writing `digits[i]` with `i` from `0..4` is bounds-checked, but the compiler can often remove
  the checks because it knows the array length.
- `for slot in digits.iter_mut()` gives a `&mut i32` to each element; `*slot = ...` writes through it.

```rust
pub struct Solution;

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits = [0; 4];
        let mut rest = num;
        for slot in digits.iter_mut() {
            *slot = rest % 10;
            rest /= 10;
        }
        digits.sort();
        10 * (digits[0] + digits[1]) + digits[2] + digits[3]
    }
}
```

Full tested solution: [`src/solutions/p0084_minimum_sum_of_four_digit_number_after_splitting_digits.rs`](../src/solutions/p0084_minimum_sum_of_four_digit_number_after_splitting_digits.rs)

</details>
