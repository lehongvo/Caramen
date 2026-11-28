# Problem 0721: Minimum Penalty for a Shop

- **Difficulty:** Medium
- **Topics:** prefix + suffix counts, running update, earliest minimum
- **Source:** [LeetCode 2483. Minimum Penalty for a Shop](https://leetcode.com/problems/minimum-penalty-for-a-shop/)

## Problem

A shop's log is a string `customers` of `'Y'` and `'N'` characters: `customers[i] == 'Y'` means
customers came during hour `i`, `'N'` means nobody came. The shop will close at some hour `j`
with `0 <= j <= n` (`n = customers.len()`): it is open during hours `0..j` and closed during
hours `j..n`. Closing at hour `n` means it never closes during the log.

The **penalty** of closing at hour `j` is:

- `+1` for every open hour with no customers (an `'N'` in `0..j`), plus
- `+1` for every closed hour in which customers came (a `'Y'` in `j..n`).

Return the **earliest** hour `j` with the smallest penalty.

Implement `Solution::best_closing_time(customers: String) -> i32`.

## Examples

```text
Input:  customers = "YYNY"
Output: 2             (penalties for j = 0..=4: 3, 2, 1, 2, 1; the earliest minimum is at 2)

Input:  customers = "NNNNN"
Output: 0

Input:  customers = "YYYY"
Output: 4
```

## Constraints

- `1 <= customers.len() <= 10^5`
- `customers` contains only `'Y'` and `'N'`.

## Hints

1. The penalty is "`'N'`s in the prefix" plus "`'Y'`s in the suffix". Both can be counted in one
   pass, as prefix and suffix counts.
2. Moving the closing hour from `j` to `j + 1` changes the penalty by exactly one. In which
   direction?

## Starter code

Write your code in [`src/practice/p0721_minimum_penalty_for_a_shop.rs`](../src/practice/p0721_minimum_penalty_for_a_shop.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0721
```

```rust
pub struct Solution;

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
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

    fn run(s: &str) -> i32 {
        Solution::best_closing_time(s.to_string())
    }

    #[test]
    fn example_mixed() {
        assert_eq!(run("YYNY"), 2);
    }

    #[test]
    fn example_never_open() {
        assert_eq!(run("NNNNN"), 0);
    }

    #[test]
    fn example_never_close() {
        assert_eq!(run("YYYY"), 4);
    }

    #[test]
    fn single_hour() {
        assert_eq!(run("Y"), 1);
        assert_eq!(run("N"), 0);
    }

    #[test]
    fn late_minimum() {
        // Penalties: 3, 2, 3, 2, 1.
        assert_eq!(run("YNYY"), 4);
    }

    #[test]
    fn ties_keep_the_earliest_hour() {
        // "YN" repeated: penalties alternate 50_000, 49_999, 50_000, 49_999, ...
        assert_eq!(run(&"YN".repeat(50_000)), 1);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** prefix + suffix counts: closing at hour `0` costs one point per `'Y'` in the whole
log. Moving the closing hour from `j` to `j + 1` moves hour `j` from the "closed" suffix to the
"open" prefix: a `'Y'` there stops costing a point (penalty `- 1`), an `'N'` starts costing one
(penalty `+ 1`). One left-to-right pass therefore visits every penalty. Update the best hour
only on a **strictly** smaller penalty, so ties keep the earliest hour.

This "split point" pattern (one value for the prefix, one for the suffix, updated as the split
moves) is the same idea as computing products or sums of everything except one position.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `bytes.iter().filter(|&&b| b == b'Y').count()` counts matching bytes; the `&&b` pattern
  removes the two layers of references (`iter` yields `&u8` and `filter` passes `&&u8`).
- The penalty can go below its starting value, so it is kept as a signed `i32`.

```rust
pub struct Solution;

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let bytes = customers.as_bytes();
        // Closing at hour 0: every 'Y' hour is a closed hour with customers.
        let mut penalty = bytes.iter().filter(|&&b| b == b'Y').count() as i32;
        let (mut best_penalty, mut best_hour) = (penalty, 0);
        for (hour, &b) in bytes.iter().enumerate() {
            // Hour `hour` moves from the closed part to the open part.
            penalty += if b == b'Y' { -1 } else { 1 };
            if penalty < best_penalty {
                best_penalty = penalty;
                best_hour = hour + 1;
            }
        }
        best_hour as i32
    }
}
```

Full tested solution: [`src/solutions/p0721_minimum_penalty_for_a_shop.rs`](../src/solutions/p0721_minimum_penalty_for_a_shop.rs)

</details>
