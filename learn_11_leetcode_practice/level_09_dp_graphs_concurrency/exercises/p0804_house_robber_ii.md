# Problem 0804: House Robber II

- **Difficulty:** Medium
- **Topics:** 1-D dynamic programming, sub-slices `&nums[1..]`, private helper on `&[i32]`
- **Source:** [LeetCode 213. House Robber II](https://leetcode.com/problems/house-robber-ii/)

## Problem

Same rules as House Robber (problem 0803): `nums[i]` is the money in house `i`, and you may not
take from two adjacent houses. This time the houses stand in a **circle**, so the first house
and the last house are also neighbours. Return the largest total you can collect.

Implement `Solution::rob(nums: Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [2, 3, 2]
Output: 3
Explanation: houses 0 and 2 are neighbours in the circle, so take house 1 only.

Input:  nums = [1, 2, 3, 1]
Output: 4
Explanation: take houses 0 and 2.

Input:  nums = [1, 2, 3]
Output: 3
```

## Constraints

- `1 <= nums.len() <= 100`
- `0 <= nums[i] <= 1000`

## Hints

1. In any valid choice, the first house or the last house (or both) is skipped.
2. So the answer is the better of two straight-line problems: houses `1..n` and houses `0..n-1`.
3. A single house is a special case: it is not its own neighbour.

## Starter code

Write your code in [`src/practice/p0804_house_robber_ii.rs`](../src/practice/p0804_house_robber_ii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0804
```

```rust
pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
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
    fn example_three_houses() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    }

    #[test]
    fn example_four_houses() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn example_increasing() {
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
    }

    #[test]
    fn single_and_two_houses() {
        assert_eq!(Solution::rob(vec![5]), 5);
        assert_eq!(Solution::rob(vec![4, 9]), 9);
    }

    #[test]
    fn big_value_next_to_the_first_house() {
        // Taking 100 (last) forbids house 0, which is fine: 3 + 100 = 103.
        assert_eq!(Solution::rob(vec![1, 3, 1, 3, 100]), 103);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** break the circle. Solve the linear House Robber twice, once without the first house
(`&nums[1..]`) and once without the last (`&nums[..n - 1]`), and return the larger result.
With one house, both slices would be empty, so return `nums[0]` directly.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- A helper that takes `&[i32]` works on any part of the vector without copying it: range
  indexing `&nums[1..]` produces a borrowed sub-slice.
- Associated functions (`fn rob_line(...)` without `self`) are called as `Self::rob_line(...)`.

```rust
pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        Self::rob_line(&nums[1..]).max(Self::rob_line(&nums[..n - 1]))
    }

    /// Classic House Robber on a straight line of houses.
    fn rob_line(houses: &[i32]) -> i32 {
        houses
            .iter()
            .fold((0, 0), |(prev, cur), &money| (cur, cur.max(prev + money)))
            .1
    }
}
```

Full tested solution: [`src/solutions/p0804_house_robber_ii.rs`](../src/solutions/p0804_house_robber_ii.rs)

</details>
