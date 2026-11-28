# Problem 0834: Burst Balloons

- **Difficulty:** Hard
- **Topics:** interval DP ("which one goes last?"), padding with sentinels, filling by length
- **Source:** [LeetCode 312. Burst Balloons](https://leetcode.com/problems/burst-balloons/)

## Problem

There are `n` balloons in a row; balloon `i` shows the number `nums[i]`. You burst them one at a
time, in any order you like. Bursting balloon `i` earns `left * nums[i] * right` coins, where
`left` and `right` are the numbers on the balloons currently adjacent to `i` (balloons already
burst are gone, so neighbours change). A missing neighbour (past either end) counts as `1`.
Return the maximum number of coins you can collect by bursting all balloons.

Implement `Solution::max_coins(nums: Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [3, 1, 5, 8]
Output: 167
Explanation: [3,1,5,8] -> [3,5,8] -> [3,8] -> [8] -> []
             coins = 3*1*5 + 3*5*8 + 1*3*8 + 1*8*1 = 15 + 120 + 24 + 8 = 167

Input:  nums = [1, 5]
Output: 10
```

## Constraints

- `1 <= nums.len() <= 300`
- `0 <= nums[i] <= 100`

## Hints

1. Choosing the **first** balloon to burst makes the two sides depend on each other. Choose the
   **last** one instead: when balloon `k` is the last one burst between two fixed borders, its
   neighbours at that moment are exactly those borders.
2. Pad the row with a `1` on each side. Let `best(l, r)` be the most coins from bursting every
   balloon strictly between positions `l` and `r`.
3. `best(l, r) = max over l < k < r of best(l, k) + best(k, r) + v[l] * v[k] * v[r]`.

## Starter code

Write your code in [`src/practice/p0834_burst_balloons.rs`](../src/practice/p0834_burst_balloons.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0834
```

```rust
pub struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
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
    fn example_four_balloons() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    }

    #[test]
    fn example_two_balloons() {
        assert_eq!(Solution::max_coins(vec![1, 5]), 10);
    }

    #[test]
    fn single_balloon_and_zeros() {
        assert_eq!(Solution::max_coins(vec![7]), 7);
        assert_eq!(Solution::max_coins(vec![0, 0]), 0);
    }

    #[test]
    fn thirty_balloons() {
        let nums = vec![
            41, 19, 50, 83, 6, 9, 68, 12, 46, 74, 7, 64, 27, 4, 11, 55, 53, 8, 30, 11, 70, 54, 7,
            72, 15, 28, 80, 80, 74, 7,
        ];
        assert_eq!(Solution::max_coins(nums), 5_199_072);
    }

    #[test]
    fn largest_input_runs() {
        // 300 balloons of 1: every burst earns exactly 1.
        assert_eq!(Solution::max_coins(vec![1; 300]), 300);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `v = [1] + nums + [1]`. Fill `best[l][r]` for increasing gap `r - l` (from 2 up to the
whole padded row) so the two sub-intervals are always ready. Intervals with no balloon inside
(`r = l + 1`) are worth `0`. The answer is `best[0][n + 1]`.

**Complexity:** `O(n^3)` time, `O(n^2)` space.

**Rust concepts:**
- Building the padded vector with `std::iter::once(1).chain(nums).chain(std::iter::once(1))`.
- `(l + 1..r).map(...).max()` picks the best last balloon; the range is non-empty because the
  gap is at least 2, so `unwrap_or(0)` never actually triggers.

```rust
pub struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let v: Vec<i32> = std::iter::once(1)
            .chain(nums)
            .chain(std::iter::once(1))
            .collect();
        let len = v.len();
        let mut best = vec![vec![0_i32; len]; len];
        for gap in 2..len {
            for l in 0..len - gap {
                let r = l + gap;
                best[l][r] = (l + 1..r)
                    .map(|k| best[l][k] + best[k][r] + v[l] * v[k] * v[r])
                    .max()
                    .unwrap_or(0);
            }
        }
        best[0][len - 1]
    }
}
```

Full tested solution: [`src/solutions/p0834_burst_balloons.rs`](../src/solutions/p0834_burst_balloons.rs)

</details>
