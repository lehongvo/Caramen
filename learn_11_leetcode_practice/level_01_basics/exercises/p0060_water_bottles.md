# Problem 0060: Water Bottles

- **Difficulty:** Easy
- **Topics:** `while` simulation, `/` and `%` together, several `mut` state variables
- **Source:** [LeetCode 1518. Water Bottles](https://leetcode.com/problems/water-bottles/)

## Problem

You start with `num_bottles` full bottles of water. Drinking a full bottle leaves an empty one.
A shop exchanges `num_exchange` empty bottles for one new full bottle, and you may exchange as
many times as you like. Return the maximum number of bottles you can drink.

Implement `Solution::num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32`.

## Examples

```text
Input:  num_bottles = 9, num_exchange = 3
Output: 13
Explanation: drink 9 (9 empty) -> exchange for 3, drink them (3 empty) -> exchange for 1, drink it.
             9 + 3 + 1 = 13.

Input:  num_bottles = 15, num_exchange = 4
Output: 19
Explanation: 15 + 3 + 1 = 19 (after the first exchange 3 empties are left over and reused).
```

## Constraints

- `1 <= num_bottles <= 100`
- `2 <= num_exchange <= 100`

## Hints

1. Keep track of the number of **empty** bottles you hold.
2. `empty / num_exchange` new bottles can be obtained, and `empty % num_exchange` empties are left over.

## Starter code

Write your code in [`src/practice/p0060_water_bottles.rs`](../src/practice/p0060_water_bottles.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0060
```

```rust
pub struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
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
    fn example_nine_three() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
    }

    #[test]
    fn example_fifteen_four() {
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }

    #[test]
    fn not_enough_to_exchange() {
        assert_eq!(Solution::num_water_bottles(1, 2), 1);
        assert_eq!(Solution::num_water_bottles(5, 100), 5);
    }

    #[test]
    fn exchange_two_for_one() {
        // With rate 2, each exchange nets one bottle: answer is 2n - 1.
        assert_eq!(Solution::num_water_bottles(2, 2), 3);
        assert_eq!(Solution::num_water_bottles(100, 2), 199);
    }

    #[test]
    fn exact_formula_for_all_inputs() {
        // Each exchange turns (k - 1) empties into one drink, as long as one bottle stays in hand.
        for n in 1..=100 {
            for k in 2..=100 {
                let expected = n + (n - 1) / (k - 1);
                assert_eq!(Solution::num_water_bottles(n, k), expected, "n={n}, k={k}");
            }
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** drink everything first. Then, while you have at least `num_exchange` empties,
exchange as many as possible, drink the new bottles, and add their empties to the leftovers.

**Complexity:** `O(log num_bottles)` exchange rounds (the empties shrink by a factor of at
least 2 each round), `O(1)` space.

**Rust concepts:**
- Integer `/` and `%` used together split a quantity into "full groups" and "remainder".
- The loop keeps two pieces of state (`drunk`, `empty`); writing the update in two lines keeps
  the order explicit: compute `new_full` from the old `empty` first.

```rust
pub struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut drunk = num_bottles;
        let mut empty = num_bottles;
        while empty >= num_exchange {
            let new_full = empty / num_exchange;
            drunk += new_full;
            empty = empty % num_exchange + new_full;
        }
        drunk
    }
}
```

Full tested solution: [`src/solutions/p0060_water_bottles.rs`](../src/solutions/p0060_water_bottles.rs)

</details>
