# Problem 0798: Combination Sum III

- **Difficulty:** Medium
- **Topics:** backtracking with two constraints (size and sum), pruning on both, `i32` ranges
- **Source:** [LeetCode 216. Combination Sum III](https://leetcode.com/problems/combination-sum-iii/)

## Problem

Find every combination of exactly `k` **distinct** numbers from `1` to `9` whose sum is `n`.
Each number is used at most once. Return the combinations in any order (the values inside each
combination may be in any order too); each combination must appear once.

Implement `Solution::combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>>`.

## Examples

```text
Input:  k = 3, n = 7
Output: [[1, 2, 4]]

Input:  k = 3, n = 9
Output: [[1, 2, 6], [1, 3, 5], [2, 3, 4]]

Input:  k = 4, n = 1
Output: []            (four distinct positive numbers sum to at least 10)
```

## Constraints

- `2 <= k <= 9`
- `1 <= n <= 60`

## Hints

1. Pick the numbers in increasing order, as in Combinations.
2. A branch is finished when it has `k` numbers; it is a solution when the remaining sum is
   then exactly `0`.

## Starter code

Write your code in [`src/practice/p0798_combination_sum_iii.rs`](../src/practice/p0798_combination_sum_iii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0798
```

```rust
pub struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
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

    fn normalized(mut combos: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for c in &mut combos {
            c.sort_unstable();
        }
        combos.sort();
        combos
    }

    #[test]
    fn example_single_answer() {
        assert_eq!(
            normalized(Solution::combination_sum3(3, 7)),
            vec![vec![1, 2, 4]]
        );
    }

    #[test]
    fn example_three_answers() {
        assert_eq!(
            normalized(Solution::combination_sum3(3, 9)),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
    }

    #[test]
    fn example_impossible_small_sum() {
        assert!(Solution::combination_sum3(4, 1).is_empty());
    }

    #[test]
    fn impossible_large_sum() {
        assert!(Solution::combination_sum3(2, 18).is_empty()); // 8 + 9 = 17 at most
        assert!(Solution::combination_sum3(9, 60).is_empty());
    }

    #[test]
    fn all_nine_numbers() {
        assert_eq!(
            Solution::combination_sum3(9, 45),
            vec![(1..=9).collect::<Vec<i32>>()]
        );
    }

    #[test]
    fn pairs_summing_to_ten() {
        assert_eq!(
            normalized(Solution::combination_sum3(2, 10)),
            vec![vec![1, 9], vec![2, 8], vec![3, 7], vec![4, 6]]
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** backtracking with two constraints: `backtrack(start, remaining)` appends a number
`x` in `start..=9` and recurses with `x + 1` and `remaining - x`. A branch stops when it holds
`k` numbers (a solution if `remaining == 0`). Since the candidates are increasing, the loop can
`break` once `x > remaining`: every later number is larger. The search space is tiny (at most
`2^9` subsets), so these prunings are enough.

**Complexity:** `O(C(9, k) * k)` time, `O(k)` recursion depth.

**Rust concepts:**
- The numbers are `i32` like the signature, so `start..=9` and `remaining - x` need no casts.
- `current.len() == k` compares with `k` converted once to `usize`.

```rust
pub struct Solution;

fn backtrack(
    start: i32,
    remaining: i32,
    k: usize,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if current.len() == k {
        if remaining == 0 {
            result.push(current.clone());
        }
        return;
    }
    for x in start..=9 {
        if x > remaining {
            break; // every later number is larger still
        }
        current.push(x);
        backtrack(x + 1, remaining - x, k, current, result);
        current.pop();
    }
}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        backtrack(1, n, k as usize, &mut Vec::new(), &mut result);
        result
    }
}
```

Full tested solution: [`src/solutions/p0798_combination_sum_iii.rs`](../src/solutions/p0798_combination_sum_iii.rs)

</details>
