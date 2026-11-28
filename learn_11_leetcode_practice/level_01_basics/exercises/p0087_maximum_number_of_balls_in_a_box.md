# Problem 0087: Maximum Number of Balls in a Box

- **Difficulty:** Easy
- **Topics:** fixed-size array as a counter `[i32; 46]`, digit sums, `as usize`, finding a maximum
- **Source:** [LeetCode 1742. Maximum Number of Balls in a Box](https://leetcode.com/problems/maximum-number-of-balls-in-a-box/)

## Problem

A factory has one ball for every number from `low_limit` to `high_limit` (inclusive). Each ball
is placed into the box whose number equals the **sum of the digits** of the ball's number: ball
`321` goes into box `3 + 2 + 1 = 6`, and ball `10` into box `1`.

Return the number of balls in the box that receives the most balls.

Implement `Solution::count_balls(low_limit: i32, high_limit: i32) -> i32`.

## Examples

```text
Input:  low_limit = 1, high_limit = 10
Output: 2
Explanation: box 1 gets balls 1 and 10; every other box gets at most one ball.

Input:  low_limit = 5, high_limit = 15
Output: 2
Explanation: boxes 5 and 6 get two balls each (5, 14 and 6, 15).

Input:  low_limit = 19, high_limit = 28
Output: 2
Explanation: box 10 gets balls 19 and 28.
```

## Constraints

- `1 <= low_limit <= high_limit <= 10^5`

## Hints

1. What is the largest possible digit sum for numbers up to `10^5`? (Think of `99999`.)
2. That makes the number of boxes small and fixed: an array works as a counter.

## Starter code

Write your code in [`src/practice/p0087_maximum_number_of_balls_in_a_box.rs`](../src/practice/p0087_maximum_number_of_balls_in_a_box.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0087
```

```rust
pub struct Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
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
    fn example_one_to_ten() {
        assert_eq!(Solution::count_balls(1, 10), 2);
    }

    #[test]
    fn example_five_to_fifteen() {
        assert_eq!(Solution::count_balls(5, 15), 2);
    }

    #[test]
    fn example_nineteen_to_twenty_eight() {
        assert_eq!(Solution::count_balls(19, 28), 2);
    }

    #[test]
    fn single_ball() {
        assert_eq!(Solution::count_balls(99_999, 99_999), 1);
        assert_eq!(Solution::count_balls(100_000, 100_000), 1);
    }

    #[test]
    fn full_range() {
        assert_eq!(Solution::count_balls(1, 100_000), 6000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the digit sum of any number up to `100000` is at most `9 * 5 = 45`, so an array of 46
counters (boxes `0..=45`) is enough. Add one ball per number, then return the largest counter.

**Complexity:** `O((high - low) * d)` time, `O(1)` space.

**Rust concepts:**
- `[0; 46]` creates the counter array; `boxes[sum as usize] += 1` updates it. The cast to `usize`
  is required because array indices are `usize`.
- A `const` like `MAX_BOX: usize = 45` documents where the size comes from; array lengths must be
  constants known at compile time.
- `for &count in &boxes` reads the array by reference to find the maximum.

```rust
pub struct Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        const MAX_BOX: usize = 45; // digit sum of 99_999
        let mut boxes = [0; MAX_BOX + 1];
        for ball in low_limit..=high_limit {
            let mut rest = ball;
            let mut sum = 0;
            while rest > 0 {
                sum += rest % 10;
                rest /= 10;
            }
            boxes[sum as usize] += 1;
        }
        let mut most = 0;
        for &count in &boxes {
            most = most.max(count);
        }
        most
    }
}
```

Full tested solution: [`src/solutions/p0087_maximum_number_of_balls_in_a_box.rs`](../src/solutions/p0087_maximum_number_of_balls_in_a_box.rs)

</details>
