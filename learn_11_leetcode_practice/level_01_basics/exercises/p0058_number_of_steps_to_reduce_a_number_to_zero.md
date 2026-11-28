# Problem 0058: Number of Steps to Reduce a Number to Zero

- **Difficulty:** Easy
- **Topics:** `while` loops, `if` / `else`, compound assignment `/=` and `-=`
- **Source:** [LeetCode 1342. Number of Steps to Reduce a Number to Zero](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/)

## Problem

Given a non-negative integer `num`, repeat the following step until it becomes `0`:

- if the current number is even, divide it by 2;
- otherwise, subtract 1.

Return how many steps were needed.

Implement `Solution::number_of_steps(num: i32) -> i32`.

## Examples

```text
Input:  num = 14
Output: 6
Explanation: 14 -> 7 -> 6 -> 3 -> 2 -> 1 -> 0.

Input:  num = 8
Output: 4
Explanation: 8 -> 4 -> 2 -> 1 -> 0.

Input:  num = 123
Output: 12
```

## Constraints

- `0 <= num <= 10^6`

## Hints

1. A `while` loop runs as long as its condition is `true`: `while n != 0 { ... }`.
2. Parameters are immutable; copy `num` into a `let mut` variable first.

## Starter code

Write your code in [`src/practice/p0058_number_of_steps_to_reduce_a_number_to_zero.rs`](../src/practice/p0058_number_of_steps_to_reduce_a_number_to_zero.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0058
```

```rust
pub struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
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
    fn example_fourteen() {
        assert_eq!(Solution::number_of_steps(14), 6);
    }

    #[test]
    fn example_eight() {
        assert_eq!(Solution::number_of_steps(8), 4);
    }

    #[test]
    fn example_123() {
        assert_eq!(Solution::number_of_steps(123), 12);
    }

    #[test]
    fn zero_and_one() {
        assert_eq!(Solution::number_of_steps(0), 0);
        assert_eq!(Solution::number_of_steps(1), 1);
    }

    #[test]
    fn matches_bit_formula() {
        for num in 1..=100_000i32 {
            let bits = 32 - (num as u32).leading_zeros() as i32;
            let ones = num.count_ones() as i32;
            assert_eq!(
                Solution::number_of_steps(num),
                bits + ones - 1,
                "num = {num}"
            );
        }
        assert_eq!(Solution::number_of_steps(1_000_000), 26);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** simulate the process with a `while` loop and count the iterations. Each halving
removes one binary digit, so the loop runs only about `2 * log2(num)` times.

**Complexity:** `O(log num)` time, `O(1)` space.

**Rust concepts:**
- `while cond { body }` checks the condition before each iteration, so `num = 0` returns `0`
  without entering the loop. See [The Rust Book 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html#conditional-loops-with-while).
- `n /= 2` and `n -= 1` are compound assignments, shorthand for `n = n / 2` and `n = n - 1`.
- Bit trick: the answer is the number of binary digits plus the number of `1` bits, minus one
  (for `num > 0`). The tests check it with `count_ones` and `leading_zeros`.

```rust
pub struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut n = num;
        let mut steps = 0;
        while n != 0 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n -= 1;
            }
            steps += 1;
        }
        steps
    }
}
```

Full tested solution: [`src/solutions/p0058_number_of_steps_to_reduce_a_number_to_zero.rs`](../src/solutions/p0058_number_of_steps_to_reduce_a_number_to_zero.rs)

</details>
