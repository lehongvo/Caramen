# Problem 0809: Coin Change

- **Difficulty:** Medium
- **Topics:** unbounded knapsack (minimum), `Option<u32>` as "unreachable", `filter_map`
- **Source:** [LeetCode 322. Coin Change](https://leetcode.com/problems/coin-change/)

## Problem

You have an unlimited supply of coins of each value in `coins`. Return the fewest coins whose
values add up to exactly `amount`, or `-1` if that amount cannot be made. An `amount` of `0`
needs `0` coins.

Implement `Solution::coin_change(coins: Vec<i32>, amount: i32) -> i32`.

## Examples

```text
Input:  coins = [1, 2, 5], amount = 11
Output: 3
Explanation: 11 = 5 + 5 + 1.

Input:  coins = [2], amount = 3
Output: -1

Input:  coins = [1], amount = 0
Output: 0
```

## Constraints

- `1 <= coins.len() <= 12`
- `1 <= coins[i] <= 2^31 - 1`
- `0 <= amount <= 10^4`

## Hints

1. Greedy (always take the largest coin) fails: with coins `[1, 3, 4]` and amount `6`, greedy
   uses `4 + 1 + 1` but `3 + 3` is better.
2. Let `dp[a]` be the fewest coins for amount `a`. The last coin used is some `c <= a`.
3. Some amounts are unreachable; represent that explicitly instead of with a magic number.

## Starter code

Write your code in [`src/practice/p0809_coin_change.rs`](../src/practice/p0809_coin_change.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0809
```

```rust
pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
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
    fn example_eleven() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn example_unreachable() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn example_zero_amount() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }

    #[test]
    fn greedy_would_fail() {
        assert_eq!(Solution::coin_change(vec![1, 3, 4], 6), 2);
    }

    #[test]
    fn huge_coin_is_ignored() {
        assert_eq!(Solution::coin_change(vec![i32::MAX, 7], 14), 2);
    }

    #[test]
    fn larger_amount() {
        assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `dp[0] = Some(0)`; for `a` in `1..=amount`,
`dp[a] = min over coins c <= a of dp[a - c] + 1` (ignoring unreachable `dp[a - c]`). The answer
is `dp[amount]`, or `-1` when it is `None`.

**Complexity:** `O(amount * coins.len())` time, `O(amount)` space.

**Rust concepts:**
- `Option<u32>` models "no way to make this amount" without sentinel values like `i32::MAX`
  (which overflow when you add 1 to them).
- `filter_map` keeps only the reachable predecessors, and `min()` on an empty iterator returns
  `None`, which is exactly "unreachable".
- Coins larger than `amount` are skipped before converting to `usize`, so huge coin values are
  harmless.

```rust
pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let coins: Vec<usize> = coins
            .into_iter()
            .map(|c| c as usize)
            .filter(|&c| c <= amount)
            .collect();
        let mut dp: Vec<Option<u32>> = vec![None; amount + 1];
        dp[0] = Some(0);
        for a in 1..=amount {
            dp[a] = coins
                .iter()
                .filter(|&&c| c <= a)
                .filter_map(|&c| dp[a - c])
                .min()
                .map(|best| best + 1);
        }
        dp[amount].map_or(-1, |count| count as i32)
    }
}
```

Full tested solution: [`src/solutions/p0809_coin_change.rs`](../src/solutions/p0809_coin_change.rs)

</details>
