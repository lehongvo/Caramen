# Problem 0799: Generate Parentheses

- **Difficulty:** Medium
- **Topics:** backtracking with counters, `String::push` / `pop` as the path, Catalan numbers
- **Source:** [LeetCode 22. Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)

## Problem

Given `n`, the number of pairs of parentheses, return every **well-formed** string made of `n`
`'('` and `n` `')'`, in any order. A string is well-formed when every `')'` closes an earlier
unmatched `'('`.

Implement `Solution::generate_parenthesis(n: i32) -> Vec<String>`.

## Examples

```text
Input:  n = 3
Output: ["((()))", "(()())", "(())()", "()(())", "()()()"]

Input:  n = 1
Output: ["()"]
```

## Constraints

- `1 <= n <= 8`

## Hints

1. Generating all `2^(2n)` strings and filtering the valid ones is wasteful.
2. Build the string left to right, counting the `'('` and `')'` used so far. When may you add
   a `'('`? When may you add a `')'`?

## Starter code

Write your code in [`src/practice/p0799_generate_parentheses.rs`](../src/practice/p0799_generate_parentheses.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0799
```

```rust
pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
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

    fn is_well_formed(s: &str) -> bool {
        let mut depth = 0i32;
        for c in s.chars() {
            depth += if c == '(' { 1 } else { -1 };
            if depth < 0 {
                return false;
            }
        }
        depth == 0
    }

    fn run(n: i32) -> Vec<String> {
        let mut result = Solution::generate_parenthesis(n);
        result.sort(); // any order is accepted
        result
    }

    #[test]
    fn example_three_pairs() {
        assert_eq!(
            run(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn example_one_pair() {
        assert_eq!(run(1), vec!["()"]);
    }

    #[test]
    fn two_pairs() {
        assert_eq!(run(2), vec!["(())", "()()"]);
    }

    #[test]
    fn eight_pairs_give_the_catalan_number() {
        let result = run(8);
        assert_eq!(result.len(), 1430);
        assert!(result.iter().all(|s| s.len() == 16 && is_well_formed(s)));
        let mut deduped = result.clone();
        deduped.dedup();
        assert_eq!(deduped.len(), 1430);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** backtracking with counters: while building the string, a `'('` may be added if fewer
than `n` have been used, and a `')'` only if it would close something (`close < open`). Every
prefix built this way can be completed, and every well-formed string is built exactly once
(each one corresponds to one sequence of choices). When the string reaches length `2n`, record
it. The number of results is the Catalan number `C(n)`: 5 for `n = 3`, 1430 for `n = 8`.

**Complexity:** `O(n * C(n))` time, about `O(4^n / sqrt(n))`; `O(n)` recursion depth.

**Rust concepts:**
- A single `String` is the path: `push('(')`, recurse, `pop()` is the choose / un-choose step.
- `current.clone()` stores a finished string; the buffer keeps being reused.

```rust
pub struct Solution;

fn backtrack(n: usize, open: usize, close: usize, current: &mut String, result: &mut Vec<String>) {
    if current.len() == 2 * n {
        result.push(current.clone());
        return;
    }
    if open < n {
        current.push('(');
        backtrack(n, open + 1, close, current, result);
        current.pop();
    }
    if close < open {
        current.push(')');
        backtrack(n, open, close + 1, current, result);
        current.pop();
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut result = Vec::new();
        backtrack(n, 0, 0, &mut String::with_capacity(2 * n), &mut result);
        result
    }
}
```

Full tested solution: [`src/solutions/p0799_generate_parentheses.rs`](../src/solutions/p0799_generate_parentheses.rs)

</details>
