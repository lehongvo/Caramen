# Problem 0831: Ones and Zeroes

- **Difficulty:** Medium
- **Topics:** 0/1 knapsack with two capacities, 2-D DP table updated backwards, `bytes().filter`
- **Source:** [LeetCode 474. Ones and Zeroes](https://leetcode.com/problems/ones-and-zeroes/)

## Problem

You are given a list of binary strings `strs` and two budgets `m` and `n`. Choose as many
strings as possible such that, all together, the chosen strings contain **at most** `m`
characters `'0'` and **at most** `n` characters `'1'`. Each string can be chosen at most once.
Return the size of the largest such subset.

Implement `Solution::find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32`.

## Examples

```text
Input:  strs = ["10", "0001", "111001", "1", "0"], m = 5, n = 3
Output: 4
Explanation: {"10", "0001", "1", "0"} uses five 0s and three 1s.

Input:  strs = ["10", "0", "1"], m = 1, n = 1
Output: 2
Explanation: {"0", "1"}.
```

## Constraints

- `1 <= strs.len() <= 600`
- `1 <= strs[i].len() <= 100`, made of `'0'` and `'1'` only
- `1 <= m, n <= 100`

## Hints

1. This is knapsack with two weights per item (zeros and ones) and value `1` per item.
2. `best[z][o]` = largest subset using at most `z` zeros and `o` ones, over the strings seen so
   far.
3. As in problem 0829, update both capacities from high to low so each string is used once.

## Starter code

Write your code in [`src/practice/p0831_ones_and_zeroes.rs`](../src/practice/p0831_ones_and_zeroes.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0831
```

```rust
pub struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
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

    fn run(strs: &[&str], m: i32, n: i32) -> i32 {
        Solution::find_max_form(strs.iter().map(|s| s.to_string()).collect(), m, n)
    }

    #[test]
    fn example_five_strings() {
        assert_eq!(run(&["10", "0001", "111001", "1", "0"], 5, 3), 4);
    }

    #[test]
    fn example_small_budget() {
        assert_eq!(run(&["10", "0", "1"], 1, 1), 2);
    }

    #[test]
    fn ones_budget_is_the_limit() {
        assert_eq!(run(&["111", "1000", "1000", "1000"], 9, 3), 3);
    }

    #[test]
    fn nothing_fits() {
        assert_eq!(run(&["00", "11"], 1, 1), 0);
    }

    #[test]
    fn many_strings() {
        let strs: Vec<String> = (0..600)
            .map(|i| if i % 2 == 0 { "0".into() } else { "1".into() })
            .collect();
        assert_eq!(Solution::find_max_form(strs, 100, 100), 200);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** for each string with `zeros` and `ones`, for `z` from `m` down to `zeros` and `o`
from `n` down to `ones`: `best[z][o] = max(best[z][o], best[z - zeros][o - ones] + 1)`.
The answer is `best[m][n]`.

**Complexity:** `O(len * m * n + total characters)` time, `O(m * n)` space.

**Rust concepts:**
- `s.bytes().filter(|&b| b == b'0').count()` counts characters without allocating.
- A `Vec<Vec<i32>>` table indexed `best[z][o]`; both loops go backwards with `.rev()`.

```rust
pub struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut best = vec![vec![0_i32; n + 1]; m + 1];
        for s in &strs {
            let zeros = s.bytes().filter(|&b| b == b'0').count();
            let ones = s.len() - zeros;
            for z in (zeros..=m).rev() {
                for o in (ones..=n).rev() {
                    best[z][o] = best[z][o].max(best[z - zeros][o - ones] + 1);
                }
            }
        }
        best[m][n]
    }
}
```

Full tested solution: [`src/solutions/p0831_ones_and_zeroes.rs`](../src/solutions/p0831_ones_and_zeroes.rs)

</details>
