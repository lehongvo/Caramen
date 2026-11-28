# Problem 0260: Valid Parentheses

- **Difficulty:** Easy
- **Topics:** `match` on a tuple `(Option<char>, char)`, `|` alternatives across tuple patterns, a `Vec` as a stack
- **Source:** [LeetCode 20. Valid Parentheses](https://leetcode.com/problems/valid-parentheses/)

## Problem

The string `s` contains only the bracket characters `(`, `)`, `[`, `]`, `{` and `}`. It is
**valid** when:

1. every closing bracket closes the most recent still-open bracket, and it is of the same type;
2. every opening bracket is closed eventually.

Return `true` if `s` is valid.

Implement `Solution::is_valid(s: String) -> bool`.

## Examples

```text
Input:  s = "()[]{}"
Output: true

Input:  s = "([)]"
Output: false     (")" arrives while "[" is the most recent open bracket)

Input:  s = "{[()]}"
Output: true

Input:  s = "(("
Output: false     (never closed)
```

## Constraints

- `1 <= s.len() <= 10^4`
- `s` contains only `()[]{}`.

## Hints

1. Push opening brackets on a stack.
2. On a closing bracket, pop and check the pair with one `match (stack.pop(), c)`:
   `(Some('('), ')') | (Some('['), ']') | (Some('{'), '}') => continue`, anything else is invalid.

## Starter code

Write your code in [`src/practice/p0260_valid_parentheses.rs`](../src/practice/p0260_valid_parentheses.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0260
```

```rust
pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
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

    fn valid(s: &str) -> bool {
        Solution::is_valid(s.to_string())
    }

    #[test]
    fn example_sequence() {
        assert!(valid("()[]{}"));
    }

    #[test]
    fn example_crossed() {
        assert!(!valid("([)]"));
    }

    #[test]
    fn example_nested() {
        assert!(valid("{[()]}"));
    }

    #[test]
    fn unclosed_and_unopened() {
        assert!(!valid("(("));
        assert!(!valid(")"));
        assert!(!valid("())"));
        assert!(!valid("(]"));
    }

    #[test]
    fn deep_nesting() {
        let s = "(".repeat(5_000) + &")".repeat(5_000);
        assert!(valid(&s));
        let t = "[".repeat(5_000) + &")".repeat(5_000);
        assert!(!valid(&t));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a stack of open brackets. Each closing bracket must match the top of the stack; at
the end the stack must be empty.

**Complexity:** `O(n)` time, `O(n)` space.

**Rust concepts:**
- `stack.pop()` returns `Option<char>`, and matching the tuple `(stack.pop(), c)` checks
  "is there an open bracket?" and "is it the right one?" in a single pattern.
- Alternatives with `|` work on whole tuple patterns: `(Some('('), ')') | (Some('['), ']')`.
- `None` (an empty stack) falls into the final `_` arm automatically, so a closing bracket with
  nothing open is rejected.

```rust
pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                _ => match (stack.pop(), c) {
                    (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') => {}
                    _ => return false,
                },
            }
        }
        stack.is_empty()
    }
}
```

Full tested solution: [`src/solutions/p0260_valid_parentheses.rs`](../src/solutions/p0260_valid_parentheses.rs)

</details>
