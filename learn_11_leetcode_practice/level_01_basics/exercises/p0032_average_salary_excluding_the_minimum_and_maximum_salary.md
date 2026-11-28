# Problem 0032: Average Salary Excluding the Minimum and Maximum Salary

- **Difficulty:** Easy
- **Topics:** `as f64` casts, `min`/`max` tracking, iterating a `Vec` by reference
- **Source:** [LeetCode 1491. Average Salary Excluding the Minimum and Maximum Salary](https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/)

## Problem

You are given a vector `salary` of **distinct** integers, where `salary[i]` is the salary of
employee `i`. Return the average salary of the employees after leaving out the single lowest
and the single highest salary. Answers within `10^-5` of the exact value are accepted.

Implement `Solution::average(salary: Vec<i32>) -> f64`.

## Examples

```text
Input:  salary = [4000, 3000, 1000, 2000]
Output: 2500.00000
Explanation: without 1000 (minimum) and 4000 (maximum): (2000 + 3000) / 2 = 2500.

Input:  salary = [1000, 2000, 3000]
Output: 2000.00000
Explanation: only 2000 remains.
```

## Constraints

- `3 <= salary.len() <= 100`
- `1000 <= salary[i] <= 10^6`
- All values are distinct.

## Hints

1. One pass can compute the sum, the minimum and the maximum at the same time.
2. Integer division would drop the fraction: convert to `f64` **before** dividing.

## Starter code

Write your code in [`src/practice/p0032_average_salary_excluding_the_minimum_and_maximum_salary.rs`](../src/practice/p0032_average_salary_excluding_the_minimum_and_maximum_salary.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0032
```

```rust
pub struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
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

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-5,
            "got {actual}, expected {expected}"
        );
    }

    #[test]
    fn example_four_salaries() {
        assert_close(Solution::average(vec![4000, 3000, 1000, 2000]), 2500.0);
    }

    #[test]
    fn example_three_salaries() {
        assert_close(Solution::average(vec![1000, 2000, 3000]), 2000.0);
    }

    #[test]
    fn fractional_average() {
        // Without 1000 and 9000: (2000 + 3000 + 5000) / 3 = 3333.33...
        assert_close(
            Solution::average(vec![9000, 2000, 1000, 3000, 5000]),
            10_000.0 / 3.0,
        );
    }

    #[test]
    fn largest_input() {
        let salary: Vec<i32> = (0..100).map(|i| 1_000_000 - i * 9_000).collect();
        // Remove 1_000_000 and 109_000; the 98 middle values average to 554_500.
        assert_close(Solution::average(salary), 554_500.0);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** sum every salary while tracking the smallest and largest. The answer is
`(sum - min - max) / (n - 2)`, computed in floating point. The sum is at most `10^8`, so an
`i32` is enough for it.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `i32::MAX` and `i32::MIN` are convenient starting values for a running minimum and maximum;
  `x.min(y)` and `x.max(y)` are methods on integers.
- `as f64` converts an integer to a float. Integers up to `2^53` convert exactly.
- `for &s in &salary` borrows the vector and copies each `i32` out, so `salary` stays usable.

```rust
pub struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut sum = 0;
        let mut lowest = i32::MAX;
        let mut highest = i32::MIN;
        for &s in &salary {
            sum += s;
            lowest = lowest.min(s);
            highest = highest.max(s);
        }
        let kept = salary.len() - 2;
        (sum - lowest - highest) as f64 / kept as f64
    }
}
```

Full tested solution: [`src/solutions/p0032_average_salary_excluding_the_minimum_and_maximum_salary.rs`](../src/solutions/p0032_average_salary_excluding_the_minimum_and_maximum_salary.rs)

</details>
