# Problem 0964: Longest Valid Parentheses

- **Difficulty:** Hard
- **Topics:** stack of indices with a sentinel base, byte iteration, `Vec<isize>` to allow `-1`
- **Source:** [LeetCode 32. Longest Valid Parentheses](https://leetcode.com/problems/longest-valid-parentheses/)

## Problem

Given a string containing only `'('` and `')'`, return the length of the longest **contiguous**
substring that is a well-formed (balanced and properly nested) parentheses string.

Implement `Solution::longest_valid_parentheses(s: String) -> i32`.

## Examples

```text
Input:  s = "(()"
Output: 2          ("()")

Input:  s = ")()())"
Output: 4          ("()()")

Input:  s = ""
Output: 0
```

## Constraints

- `0 <= s.len() <= 3 * 10^4`
- `s[i]` is `'('` or `')'`.

## Hints

1. Keep a stack of indices of unmatched `'('`, with the index **just before** the current valid
   run at the bottom (start with `-1`).
2. On `')'`, pop. If the stack becomes empty, this `')'` is unmatched: push its index as the new
   base. Otherwise the run ending here starts after the index now on top.
3. The length of that run is `i - top`.

## Starter code

Write your code in [`src/practice/p0964_longest_valid_parentheses.rs`](../src/practice/p0964_longest_valid_parentheses.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0964
```

```rust
pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
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
        Solution::longest_valid_parentheses(s.to_string())
    }

    fn naive(s: &str) -> i32 {
        let b = s.as_bytes();
        let mut best = 0;
        for i in 0..b.len() {
            let mut depth = 0;
            for (j, &c) in b.iter().enumerate().skip(i) {
                depth += if c == b'(' { 1 } else { -1 };
                if depth < 0 {
                    break;
                }
                if depth == 0 {
                    best = best.max(j - i + 1);
                }
            }
        }
        best as i32
    }

    #[test]
    fn examples() {
        assert_eq!(run("(()"), 2);
        assert_eq!(run(")()())"), 4);
        assert_eq!(run(""), 0);
    }

    #[test]
    fn nested_and_adjacent() {
        assert_eq!(run("()(())"), 6);
        assert_eq!(run("()(()"), 2);
        assert_eq!(run("))(("), 0);
        assert_eq!(run("(()())"), 6);
    }

    #[test]
    fn matches_naive() {
        for mask in 0..(1_u32 << 12) {
            let s: String = (0..12)
                .map(|bit| if (mask >> bit) & 1 == 1 { '(' } else { ')' })
                .collect();
            assert_eq!(run(&s), naive(&s), "{s}");
        }
    }

    #[test]
    fn long_input() {
        let s = "()".repeat(15_000);
        assert_eq!(run(&s), 30_000);
        let s = "(".repeat(15_000) + &")".repeat(15_000);
        assert_eq!(run(&s), 30_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the stack's bottom element is the position of the last unmatched `')'` (or `-1`), and
the other elements are unmatched `'('` positions. After matching a `')'`, everything between the
new top and `i` is balanced, so `i - top` is a candidate answer.

**Complexity:** `O(n)` time, `O(n)` space.

**Rust concepts:**
- `isize` indices let the sentinel `-1` live in the same stack.
- `s.bytes().enumerate()` walks ASCII input without UTF-8 decoding.
- `last()` returns `Option<&isize>`; the stack is never empty when it is read here.

```rust
pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<isize> = vec![-1];
        let mut best = 0;
        for (i, b) in s.bytes().enumerate() {
            let i = i as isize;
            if b == b'(' {
                stack.push(i);
                continue;
            }
            stack.pop();
            match stack.last() {
                Some(&top) => best = best.max(i - top),
                None => stack.push(i),
            }
        }
        best as i32
    }
}
```

Full tested solution: [`src/solutions/p0964_longest_valid_parentheses.rs`](../src/solutions/p0964_longest_valid_parentheses.rs)

</details>
