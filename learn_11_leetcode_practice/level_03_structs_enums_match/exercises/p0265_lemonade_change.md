# Problem 0265: Lemonade Change

- **Difficulty:** Easy
- **Topics:** a `Bill` enum parsed from `i32`, a `Till` struct with a `&mut self` method, greedy choice in `match` arms
- **Source:** [LeetCode 860. Lemonade Change](https://leetcode.com/problems/lemonade-change/)

## Problem

A lemonade costs `$5`. Customers arrive in the order given by `bills`, each buys exactly one
lemonade and pays with a single `$5`, `$10` or `$20` bill. You start with **no** money and must
give every customer the exact change (`$0`, `$5` or `$15`) using only the bills received so far.

Return `true` if you can give every customer correct change.

Implement `Solution::lemonade_change(bills: Vec<i32>) -> bool`.

For this level, convert each number into an `enum Bill { Five, Ten, Twenty }` and keep the cash
in a `struct Till { fives, tens }` with a method `accept(&mut self, bill: Bill) -> bool`.

## Examples

```text
Input:  bills = [5, 5, 5, 10, 20]
Output: true     (the last customer gets a $10 and a $5)

Input:  bills = [5, 5, 10, 10, 20]
Output: false    (after two $10 customers, only $10 bills are left: no way to give $15)
```

## Constraints

- `1 <= bills.len() <= 10^5`
- `bills[i]` is `5`, `10` or `20`.

## Hints

1. Twenty-dollar bills are never useful as change, so the till only needs to count fives and tens.
2. For a `$20`, prefer giving `$10 + $5` over `$5 + $5 + $5`: fives are more flexible.

## Starter code

Write your code in [`src/practice/p0265_lemonade_change.rs`](../src/practice/p0265_lemonade_change.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0265
```

```rust
pub struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
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
    fn example_can_serve_everyone() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    }

    #[test]
    fn example_runs_out_of_fives() {
        assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
    }

    #[test]
    fn first_customer_needs_change() {
        assert!(!Solution::lemonade_change(vec![10]));
        assert!(!Solution::lemonade_change(vec![20]));
        assert!(Solution::lemonade_change(vec![5]));
    }

    #[test]
    fn prefers_ten_plus_five() {
        // Giving 5+5+5 for the first 20 would leave no five for the final 10.
        assert!(Solution::lemonade_change(vec![5, 5, 5, 5, 10, 20, 10]));
    }

    #[test]
    fn three_fives_when_no_ten() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 20]));
        assert!(!Solution::lemonade_change(vec![5, 5, 20]));
    }

    #[test]
    fn long_queue() {
        // Every ten uses up one five, so the final twenty finds no five left.
        let mut bills = vec![5; 49_999];
        bills.extend(vec![10; 49_999]);
        assert!(Solution::lemonade_change(bills.clone()));
        bills.push(20);
        assert!(!Solution::lemonade_change(bills));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** greedy. A `$5` needs no change; a `$10` needs one `$5`; a `$20` needs `$15`, taken
as `$10 + $5` when possible (keeping fives for later), otherwise as three fives.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `Bill::from_dollars(i32) -> Option<Bill>` keeps the "only three bill values exist" rule at the
  boundary; the rest of the code works with the enum.
- Guards order the two ways of changing a `$20`: `Bill::Twenty if self.tens >= 1 && self.fives >= 1`
  is tried before `Bill::Twenty if self.fives >= 3`.
- `bills.iter().all(...)` stops at the first customer who cannot be served; the closure mutates
  the till, which `all` allows because it takes an `FnMut`.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Bill {
    Five,
    Ten,
    Twenty,
}

impl Bill {
    fn from_dollars(d: i32) -> Option<Bill> {
        match d {
            5 => Some(Bill::Five),
            10 => Some(Bill::Ten),
            20 => Some(Bill::Twenty),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
struct Till {
    fives: u32,
    tens: u32,
}

impl Till {
    /// Takes the bill and gives change; `false` if the change cannot be made.
    fn accept(&mut self, bill: Bill) -> bool {
        match bill {
            Bill::Five => self.fives += 1,
            Bill::Ten if self.fives >= 1 => {
                self.fives -= 1;
                self.tens += 1;
            }
            Bill::Twenty if self.tens >= 1 && self.fives >= 1 => {
                self.tens -= 1;
                self.fives -= 1;
            }
            Bill::Twenty if self.fives >= 3 => self.fives -= 3,
            Bill::Ten | Bill::Twenty => return false,
        }
        true
    }
}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut till = Till::default();
        bills
            .iter()
            .all(|&d| Bill::from_dollars(d).is_some_and(|bill| till.accept(bill)))
    }
}
```

Full tested solution: [`src/solutions/p0265_lemonade_change.rs`](../src/solutions/p0265_lemonade_change.rs)

</details>
