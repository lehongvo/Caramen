# Problem 0017: Account Balance After Rounded Purchase

- **Difficulty:** Easy
- **Topics:** rounding with integer arithmetic, `/` and `*`
- **Source:** [LeetCode 2806. Account Balance After Rounded Purchase](https://leetcode.com/problems/account-balance-after-rounded-purchase/)

## Problem

Your bank account starts with a balance of `100` dollars. You buy something that costs
`purchase_amount` dollars, but the shop only accepts multiples of 10, so the price is first
rounded to the **nearest** multiple of 10. When the price is exactly halfway between two
multiples (it ends in 5), it is rounded **up**. Zero counts as a multiple of 10.

Return the balance after paying the rounded price.

Implement `Solution::account_balance_after_purchase(purchase_amount: i32) -> i32`.

## Examples

```text
Input:  purchase_amount = 9
Output: 90
Explanation: 9 rounds to 10, and 100 - 10 = 90.

Input:  purchase_amount = 15
Output: 80
Explanation: 15 is halfway between 10 and 20, so it rounds up to 20.

Input:  purchase_amount = 4
Output: 100
Explanation: 4 rounds down to 0.
```

## Constraints

- `0 <= purchase_amount <= 100`

## Hints

1. `x / 10 * 10` rounds `x` **down** to a multiple of 10.
2. What can you add to `x` before rounding down so that the result is "round to nearest, ties up"?

## Starter code

Write your code in [`src/practice/p0017_account_balance_after_rounded_purchase.rs`](../src/practice/p0017_account_balance_after_rounded_purchase.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0017
```

```rust
pub struct Solution;

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
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
    fn example_rounds_up() {
        assert_eq!(Solution::account_balance_after_purchase(9), 90);
    }

    #[test]
    fn example_tie_rounds_up() {
        assert_eq!(Solution::account_balance_after_purchase(15), 80);
    }

    #[test]
    fn example_rounds_down_to_zero() {
        assert_eq!(Solution::account_balance_after_purchase(4), 100);
        assert_eq!(Solution::account_balance_after_purchase(0), 100);
    }

    #[test]
    fn exact_multiples_and_extremes() {
        assert_eq!(Solution::account_balance_after_purchase(10), 90);
        assert_eq!(Solution::account_balance_after_purchase(95), 0);
        assert_eq!(Solution::account_balance_after_purchase(100), 0);
        assert_eq!(Solution::account_balance_after_purchase(94), 10);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** adding 5 before rounding down turns "round down" into "round half up":
`(x + 5) / 10 * 10`. For `x = 14` this is `19 / 10 * 10 = 10`; for `x = 15` it is `20`.

**Complexity:** `O(1)` time, `O(1)` space.

**Rust concepts:**
- Integer division truncates, so `/ 10 * 10` removes the last digit. `*` and `/` have the same
  precedence and are evaluated left to right.
- Avoiding floating point (`(x as f64 / 10.0).round()`) keeps the computation exact and simple.

```rust
pub struct Solution;

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        let rounded = (purchase_amount + 5) / 10 * 10;
        100 - rounded
    }
}
```

Full tested solution: [`src/solutions/p0017_account_balance_after_rounded_purchase.rs`](../src/solutions/p0017_account_balance_after_rounded_purchase.rs)

</details>
