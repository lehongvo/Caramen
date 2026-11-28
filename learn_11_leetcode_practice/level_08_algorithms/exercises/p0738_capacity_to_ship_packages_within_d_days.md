# Problem 0738: Capacity To Ship Packages Within D Days

- **Difficulty:** Medium
- **Topics:** binary search on the answer, greedy feasibility check, iterator `max` and `sum`
- **Source:** [LeetCode 1011. Capacity To Ship Packages Within D Days](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/)

## Problem

Packages wait on a conveyor belt in a fixed order; package `i` weighs `weights[i]`. Every day a
ship loads packages **in order** (it cannot skip or reorder them) as long as the total weight
loaded that day does not exceed the ship's capacity, then sails.

Return the smallest capacity that ships every package within `days` days.

Implement `Solution::ship_within_days(weights: Vec<i32>, days: i32) -> i32`.

## Examples

```text
Input:  weights = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], days = 5
Output: 15            (days: [1, 2, 3, 4, 5], [6, 7], [8], [9], [10])

Input:  weights = [3, 2, 2, 4, 1, 4], days = 3
Output: 6             (days: [3, 2], [2, 4], [1, 4])

Input:  weights = [1, 2, 3, 1, 1], days = 4
Output: 3             (days: [1], [2], [3], [1, 1])
```

## Constraints

- `1 <= days <= weights.len() <= 5 * 10^4`
- `1 <= weights[i] <= 500`

## Hints

1. For a fixed capacity, the best plan is greedy: keep loading until the next package does not
   fit, then start a new day.
2. A larger capacity never needs more days. What are the smallest and largest capacities worth
   trying?

## Starter code

Write your code in [`src/practice/p0738_capacity_to_ship_packages_within_d_days.rs`](../src/practice/p0738_capacity_to_ship_packages_within_d_days.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0738
```

```rust
pub struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
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
    fn example_five_days() {
        assert_eq!(Solution::ship_within_days((1..=10).collect(), 5), 15);
    }

    #[test]
    fn example_three_days() {
        assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
    }

    #[test]
    fn example_four_days() {
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    }

    #[test]
    fn one_package_per_day() {
        assert_eq!(Solution::ship_within_days(vec![7, 2, 9, 4], 4), 9);
    }

    #[test]
    fn single_day() {
        assert_eq!(Solution::ship_within_days(vec![7, 2, 9, 4], 1), 22);
    }

    #[test]
    fn long_input() {
        // 50_000 packages of 500 in 10 days: 5_000 packages per day.
        assert_eq!(Solution::ship_within_days(vec![500; 50_000], 10), 2_500_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** binary search on the answer + greedy check: for a capacity `c`, count the days
greedily (start a new day whenever the next package would overflow the current one). Loading as
much as possible each day is optimal, because delaying a package never helps the later days.
The day count never increases with `c`, so search `max(weights)..=sum(weights)` for the
smallest capacity whose day count is at most `days`. The lower bound is needed because the
heaviest package must fit; the upper bound ships everything in one day.

**Complexity:** `O(n log S)` time where `S = sum(weights)`, `O(1)` space.

**Rust concepts:**
- A private helper function `days_needed(&[i32], i32) -> i32` keeps the feasibility test
  separate from the search, which makes both easier to check.
- `iter().max()` and `iter().sum()` compute the search bounds in one line each.

```rust
pub struct Solution;

/// Number of days needed with the given capacity (every package must fit: capacity >= max weight).
fn days_needed(weights: &[i32], capacity: i32) -> i32 {
    let (mut days, mut load) = (1, 0);
    for &w in weights {
        if load + w > capacity {
            days += 1;
            load = 0;
        }
        load += w;
    }
    days
}

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut lo = weights.iter().copied().max().unwrap_or(0); // the heaviest package must fit
        let mut hi: i32 = weights.iter().sum(); // everything in one day
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if days_needed(&weights, mid) <= days {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}
```

Full tested solution: [`src/solutions/p0738_capacity_to_ship_packages_within_d_days.rs`](../src/solutions/p0738_capacity_to_ship_packages_within_d_days.rs)

</details>
