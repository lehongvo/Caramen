# Problem 0741: Final Prices With a Special Discount in a Shop

- **Difficulty:** Easy
- **Topics:** monotonic stack of indices, `Vec::pop_if`, `while let`
- **Source:** [LeetCode 1475. Final Prices With a Special Discount in a Shop](https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/)

## Problem

`prices[i]` is the price of the `i`-th item in a shop. When you buy item `i`, you get a
discount equal to `prices[j]`, where `j` is the **smallest** index with `j > i` and
`prices[j] <= prices[i]`. If there is no such `j`, there is no discount.

Return a vector with the final price of every item.

Implement `Solution::final_prices(prices: Vec<i32>) -> Vec<i32>`.

## Examples

```text
Input:  prices = [8, 4, 6, 2, 3]
Output: [4, 2, 4, 2, 3]
        (8 gets 4 off, 4 gets 2 off, 6 gets 2 off, 2 and 3 get nothing)

Input:  prices = [1, 2, 3, 4, 5]
Output: [1, 2, 3, 4, 5]

Input:  prices = [10, 1, 1, 6]
Output: [9, 0, 1, 6]
```

## Constraints

- `1 <= prices.len() <= 500`
- `1 <= prices[i] <= 1000`

## Hints

1. Two nested loops work for 500 items, but there is an `O(n)` method worth learning.
2. Walk left to right and keep the items that are still **waiting** for their discount. When a
   cheap item arrives, which waiting items does it serve?

## Starter code

Write your code in [`src/practice/p0741_final_prices_with_a_special_discount_in_a_shop.rs`](../src/practice/p0741_final_prices_with_a_special_discount_in_a_shop.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0741
```

```rust
pub struct Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
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
    fn example_mixed() {
        assert_eq!(
            Solution::final_prices(vec![8, 4, 6, 2, 3]),
            vec![4, 2, 4, 2, 3]
        );
    }

    #[test]
    fn example_increasing() {
        assert_eq!(
            Solution::final_prices(vec![1, 2, 3, 4, 5]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn example_equal_prices_count() {
        assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
    }

    #[test]
    fn single_item() {
        assert_eq!(Solution::final_prices(vec![5]), vec![5]);
    }

    #[test]
    fn decreasing_prices() {
        let prices: Vec<i32> = (1..=500).rev().collect();
        // Every item gets the next price (one cheaper) as a discount; the last one pays 1.
        assert_eq!(Solution::final_prices(prices), vec![1; 500]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** monotonic stack: keep a stack of indices whose discount is not known yet. When item
`j` arrives, it is the first later item that is cheap enough for every waiting item with a
price `>= prices[j]`; those items sit on top of the stack (the stack prices are increasing from
bottom to top, because any item that was not cheaper than the one after it has already been
served). Pop them, apply the discount, then push `j`, which now waits too.

This "monotonic stack" pattern answers "what is the next element to the right that is
smaller / larger than me?" for every position in `O(n)` total time: each index is pushed once
and popped at most once.

**Complexity:** `O(n)` time, `O(n)` space.

**Rust concepts:**
- [`Vec::pop_if`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop_if) (stable
  since 1.86) pops the last element only when the predicate is true; inside `while let` it
  pops "while the top satisfies a condition" in one line.
- The predicate receives `&mut usize`, so it reads `prices[*i]`.

```rust
pub struct Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut answer = prices.clone();
        let mut waiting: Vec<usize> = Vec::new(); // indices still waiting for a discount
        for (j, &price) in prices.iter().enumerate() {
            while let Some(i) = waiting.pop_if(|i| prices[*i] >= price) {
                answer[i] -= price;
            }
            waiting.push(j);
        }
        answer
    }
}
```

Full tested solution: [`src/solutions/p0741_final_prices_with_a_special_discount_in_a_shop.rs`](../src/solutions/p0741_final_prices_with_a_special_discount_in_a_shop.rs)

</details>
