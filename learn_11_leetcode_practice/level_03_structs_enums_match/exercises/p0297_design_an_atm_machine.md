# Problem 0297: Design an ATM Machine

- **Difficulty:** Medium
- **Topics:** design struct with fixed-size arrays, greedy plan computed before mutating, `zip` over parallel arrays, `[-1]` sentinel
- **Source:** [LeetCode 2241. Design an ATM Machine](https://leetcode.com/problems/design-an-atm-machine/)

## Problem

An ATM holds banknotes of five values: `$20`, `$50`, `$100`, `$200` and `$500`, initially none.
Implement the `ATM` struct:

- `ATM::new() -> Self`: an empty machine;
- `deposit(&mut self, banknotes_count: Vec<i32>)`: add `banknotes_count[i]` notes of the `i`-th
  value (in the order 20, 50, 100, 200, 500);
- `withdraw(&mut self, amount: i32) -> Vec<i32>`: hand out `amount` dollars. The machine always
  takes as many notes of the **largest** value as possible (without exceeding the remaining
  amount and the notes available), then continues with the next value down. If this procedure
  does not reach exactly `amount`, the withdrawal is **rejected**: nothing is handed out, the
  stock does not change, and the result is `[-1]`. Otherwise the result is the number of notes
  of each value handed out, in the same order as `deposit`.

The greedy rule is part of the specification: for example with one `$50` and three `$20` notes,
a request for `$60` is rejected (the machine takes the `$50` first and cannot pay `$10`), even
though three `$20` notes would work.

This is a LeetCode *design* problem, so there is no `impl Solution`.

## Examples

```text
let mut atm = ATM::new();
atm.deposit(vec![0, 0, 1, 2, 1]);   // one $100, two $200, one $500
atm.withdraw(600)  => [0, 0, 1, 0, 1]   ($500 + $100)
atm.deposit(vec![0, 1, 0, 1, 1]);
atm.withdraw(600)  => [-1]              ($500, then $50 leaves $50 that cannot be paid)
atm.withdraw(550)  => [0, 1, 0, 0, 1]
```

## Constraints

- `banknotes_count.len() == 5`, `0 <= banknotes_count[i] <= 10^9`, `1 <= amount <= 10^9`.
- At most 5000 calls in total. Stocks can exceed `i32`: use `i64`.

## Hints

1. Compute the plan first in a local array, from the largest value to the smallest:
   `take = min(stock[i], remaining / value[i])`.
2. Only if `remaining == 0` at the end, subtract the plan from the stock.

## Starter code

Write your code in [`src/practice/p0297_design_an_atm_machine.rs`](../src/practice/p0297_design_an_atm_machine.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0297
```

```rust
/// An ATM with five kinds of banknotes.
// The name and the argument-less `new` come from LeetCode's template.
#[allow(clippy::upper_case_acronyms)]
pub struct ATM {}

impl ATM {
    /// An empty machine.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    /// Adds banknotes, in the order 20, 50, 100, 200, 500.
    pub fn deposit(&mut self, banknotes_count: Vec<i32>) {
        todo!("Write your solution here")
    }

    /// Withdraws `amount` greedily, or returns `[-1]` without changes.
    pub fn withdraw(&mut self, amount: i32) -> Vec<i32> {
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
    fn example_session() {
        let mut atm = ATM::new();
        atm.deposit(vec![0, 0, 1, 2, 1]);
        assert_eq!(atm.withdraw(600), vec![0, 0, 1, 0, 1]);
        atm.deposit(vec![0, 1, 0, 1, 1]);
        assert_eq!(atm.withdraw(600), vec![-1]);
        assert_eq!(atm.withdraw(550), vec![0, 1, 0, 0, 1]);
    }

    #[test]
    fn greedy_rejection_from_the_statement() {
        let mut atm = ATM::new();
        atm.deposit(vec![3, 1, 0, 0, 0]);
        assert_eq!(atm.withdraw(60), vec![-1]);
        // Nothing changed: 50 + 20 still works.
        assert_eq!(atm.withdraw(70), vec![1, 1, 0, 0, 0]);
        assert_eq!(atm.withdraw(40), vec![2, 0, 0, 0, 0]);
        assert_eq!(atm.withdraw(20), vec![-1]);
    }

    #[test]
    fn empty_machine() {
        let mut atm = ATM::new();
        assert_eq!(atm.withdraw(20), vec![-1]);
    }

    #[test]
    fn huge_stock() {
        let mut atm = ATM::new();
        atm.deposit(vec![1_000_000_000; 5]);
        atm.deposit(vec![1_000_000_000; 5]);
        assert_eq!(atm.withdraw(1_000_000_000), vec![0, 0, 0, 0, 2_000_000]);
        assert_eq!(atm.withdraw(170), vec![1, 1, 1, 0, 0]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** greedy as specified, but "plan, then commit": the withdrawal is computed without
touching the stock, and applied only when it pays the exact amount.

**Complexity:** `O(1)` per call (five denominations), `O(1)` space.

**Rust concepts:**
- `const VALUES: [i64; 5]` and `counts: [i64; 5]` are parallel fixed-size arrays; `iter().zip()`
  walks them together.
- `(0..5).rev()` visits the denominations from the largest.
- LeetCode's struct name `ATM` and argument-less `new` trigger the clippy lints
  `upper_case_acronyms` and `new_without_default`; they are allowed here to keep the exact
  LeetCode API.

```rust
/// Banknote values, in LeetCode's order.
const VALUES: [i64; 5] = [20, 50, 100, 200, 500];

/// An ATM with five kinds of banknotes.
// The name and the argument-less `new` come from LeetCode's template.
#[allow(clippy::upper_case_acronyms)]
pub struct ATM {
    counts: [i64; 5],
}

impl ATM {
    /// An empty machine.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { counts: [0; 5] }
    }

    /// Adds banknotes, in the order 20, 50, 100, 200, 500.
    pub fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for (stock, &added) in self.counts.iter_mut().zip(&banknotes_count) {
            *stock += i64::from(added);
        }
    }

    /// Withdraws `amount` greedily, or returns `[-1]` without changes.
    pub fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut remaining = i64::from(amount);
        let mut plan = [0i64; 5];
        for i in (0..5).rev() {
            let take = self.counts[i].min(remaining / VALUES[i]);
            plan[i] = take;
            remaining -= take * VALUES[i];
        }
        if remaining != 0 {
            return vec![-1];
        }
        for (stock, taken) in self.counts.iter_mut().zip(plan) {
            *stock -= taken;
        }
        // Each `take` is at most amount / 20 <= 5 * 10^7, so it fits in an i32.
        plan.iter().map(|&t| t as i32).collect()
    }
}
```

Full tested solution: [`src/solutions/p0297_design_an_atm_machine.rs`](../src/solutions/p0297_design_an_atm_machine.rs)

</details>
