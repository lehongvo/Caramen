# Problem 0802: Min Cost Climbing Stairs

- **Difficulty:** Easy
- **Topics:** 1-D dynamic programming, `Iterator::fold` with a tuple state
- **Source:** [LeetCode 746. Min Cost Climbing Stairs](https://leetcode.com/problems/min-cost-climbing-stairs/)

## Problem

A staircase has `cost.len()` steps, numbered from `0`. Stepping **on** step `i` costs
`cost[i]`. After you pay for a step you may climb either one or two steps. You may start on
step `0` or on step `1` (starting is free; you pay for the step you stand on). The "top" is the
position just past the last step, index `cost.len()`, and reaching it costs nothing.

Return the minimum total cost to reach the top.

Implement `Solution::min_cost_climbing_stairs(cost: Vec<i32>) -> i32`.

## Examples

```text
Input:  cost = [10, 15, 20]
Output: 15
Explanation: start on step 1, pay 15, climb two steps to the top.

Input:  cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
Output: 6
Explanation: step on indices 0, 2, 4, 6, 7, 9 (six steps that cost 1 each).
```

## Constraints

- `2 <= cost.len() <= 1000`
- `0 <= cost[i] <= 999`

## Hints

1. Let `f(i)` be the cheapest way to stand on step `i` *including* its cost. You arrive at `i`
   from `i - 1` or `i - 2`.
2. The top can be reached from either of the last two steps.
3. `f(i)` only depends on `f(i - 1)` and `f(i - 2)`: two variables are enough.

## Starter code

Write your code in [`src/practice/p0802_min_cost_climbing_stairs.rs`](../src/practice/p0802_min_cost_climbing_stairs.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0802
```

```rust
pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
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
    fn example_three_steps() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn example_ten_steps() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
    }

    #[test]
    fn two_steps_pick_the_cheaper() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![7, 3]), 3);
        assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 0]), 0);
    }

    #[test]
    fn long_constant_staircase() {
        // With equal costs, the best plan jumps two steps every time: 500 paid steps.
        assert_eq!(Solution::min_cost_climbing_stairs(vec![2; 1000]), 1000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `f(i) = cost[i] + min(f(i - 1), f(i - 2))` with `f(-1) = f(-2) = 0` (you may start on
step 0 or 1 for free). The answer is `min(f(n - 1), f(n - 2))`. A `fold` carries the pair
`(f(i - 2), f(i - 1))` through the slice.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `iter().fold(init, |acc, item| ...)` threads a state through an iterator; here the state is a
  tuple, destructured directly in the closure parameters `|(two_back, one_back), &c|`.
- The `&c` pattern copies the `i32` out of the `&i32` item the iterator yields.

```rust
pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (two_back, one_back) = cost.iter().fold((0, 0), |(two_back, one_back), &c| {
            (one_back, c + two_back.min(one_back))
        });
        two_back.min(one_back)
    }
}
```

Full tested solution: [`src/solutions/p0802_min_cost_climbing_stairs.rs`](../src/solutions/p0802_min_cost_climbing_stairs.rs)

</details>
