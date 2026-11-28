# Problem 0746: Minimum Remove to Make Valid Parentheses

- **Difficulty:** Medium
- **Topics:** stack of indices, marking with a `Vec<bool>`, `zip` + `filter`, building a `String`
- **Source:** [LeetCode 1249. Minimum Remove to Make Valid Parentheses](https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/)

## Problem

You are given a string `s` of `'('`, `')'` and lowercase English letters. Remove the
**minimum** number of parentheses (at any positions) so that the remaining string is valid, and
return it. A string is valid when every `')'` closes an earlier unmatched `'('` and every
`'('` is eventually closed. Letters are always kept. If several answers exist, return any one.

Implement `Solution::min_remove_to_make_valid(s: String) -> String`.

## Examples

```text
Input:  s = "lee(t(c)o)de)"
Output: "lee(t(c)o)de"     ("lee(t(co)de)" and "lee(t(c)ode)" are also accepted)

Input:  s = "a)b(c)d"
Output: "ab(c)d"

Input:  s = "))(("
Output: ""
```

## Constraints

- `1 <= s.len() <= 10^5`
- `s` contains only `'('`, `')'` and lowercase English letters.

## Hints

1. A `')'` with no unmatched `'('` before it must be removed.
2. After the scan, every `'('` that is still unmatched must be removed as well. Remember their
   **positions** on a stack.

## Starter code

Write your code in [`src/practice/p0746_minimum_remove_to_make_valid_parentheses.rs`](../src/practice/p0746_minimum_remove_to_make_valid_parentheses.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0746
```

```rust
pub struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
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

    fn is_valid(s: &str) -> bool {
        let mut depth = 0i32;
        for c in s.chars() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth < 0 {
                        return false;
                    }
                }
                _ => {}
            }
        }
        depth == 0
    }

    fn is_subsequence(small: &str, big: &str) -> bool {
        let mut rest = big.chars();
        small.chars().all(|c| rest.any(|d| d == c))
    }

    /// Accepts any valid answer of the expected (minimal) length.
    fn check(s: &str, expected_len: usize) {
        let result = Solution::min_remove_to_make_valid(s.to_string());
        assert!(is_valid(&result), "{result:?} is not valid");
        assert_eq!(result.len(), expected_len, "{result:?}");
        assert!(
            is_subsequence(&result, s),
            "{result:?} is not obtained by removing characters"
        );
    }

    #[test]
    fn example_one_extra_closing() {
        check("lee(t(c)o)de)", 12);
    }

    #[test]
    fn example_closing_before_opening() {
        assert_eq!(
            Solution::min_remove_to_make_valid("a)b(c)d".to_string()),
            "ab(c)d"
        );
    }

    #[test]
    fn example_nothing_survives() {
        assert_eq!(Solution::min_remove_to_make_valid("))((".to_string()), "");
    }

    #[test]
    fn unclosed_opening() {
        check("(a(b(c)d)", 8);
    }

    #[test]
    fn letters_only() {
        assert_eq!(Solution::min_remove_to_make_valid("abc".to_string()), "abc");
    }

    #[test]
    fn long_input_with_one_extra() {
        let s = format!("{}{}", "(".repeat(50_000), ")".repeat(50_001));
        check(&s, 100_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** stack of indices: scan left to right with a stack holding the positions of the
unmatched `'('`. A `')'` matches the most recent unmatched `'('` (pop); if the stack is empty,
that `')'` can never be matched, so mark it for removal. After the scan, the positions left on
the stack are `'('` that never closed; mark them too. Every marked character must go in any
valid answer, and removing exactly those leaves a valid string, so the removal is minimal.

**Complexity:** `O(n)` time, `O(n)` space.

**Rust concepts:**
- `keep: Vec<bool>` records decisions first, and the string is rebuilt once at the end; removing
  characters from a `String` one by one would be `O(n)` each.
- `bytes.iter().zip(&keep).filter(...).map(...).collect::<String>()` builds the result; ASCII
  bytes become `char`s with `char::from`.
- The tests accept any valid answer by checking validity, length and order instead of one string.

```rust
pub struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let bytes = s.as_bytes();
        let mut keep = vec![true; bytes.len()];
        let mut open: Vec<usize> = Vec::new(); // positions of unmatched '('
        for (i, &b) in bytes.iter().enumerate() {
            match b {
                b'(' => open.push(i),
                b')' => {
                    if open.pop().is_none() {
                        keep[i] = false; // nothing to close
                    }
                }
                _ => {}
            }
        }
        for i in open {
            keep[i] = false; // never closed
        }
        bytes
            .iter()
            .zip(&keep)
            .filter(|&(_, &kept)| kept)
            .map(|(&b, _)| char::from(b))
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0746_minimum_remove_to_make_valid_parentheses.rs`](../src/solutions/p0746_minimum_remove_to_make_valid_parentheses.rs)

</details>
