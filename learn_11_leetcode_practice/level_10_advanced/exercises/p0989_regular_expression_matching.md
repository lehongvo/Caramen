# Problem 0989: Regular Expression Matching

- **Difficulty:** Hard
- **Topics:** 2D dynamic programming over suffixes, `'.'` and `'*'` semantics, byte slices, `vec![vec![false; m + 1]; n + 1]`
- **Source:** [LeetCode 10. Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/)

## Problem

Implement full-string matching of `s` against a pattern `p` that supports:

- `'.'`: matches any single character;
- `'*'`: matches **zero or more** of the element right before it (`"a*"` matches `""`, `"a"`,
  `"aa"`, ...; `".*"` matches anything).

The whole string must match the whole pattern. Implement `Solution::is_match(s: String, p: String) -> bool`.

## Examples

```text
s = "aa",  p = "a"      => false
s = "aa",  p = "a*"     => true
s = "ab",  p = ".*"     => true
s = "aab", p = "c*a*b"  => true      ("c*" matches nothing)
```

## Constraints

- `1 <= s.len() <= 20`, `1 <= p.len() <= 20`
- `s` has lowercase letters; `p` has lowercase letters, `'.'` and `'*'`.
- Every `'*'` is preceded by a valid character.

## Hints

1. Let `dp[i][j]` mean "`s[i..]` matches `p[j..]`". The answer is `dp[0][0]`; `dp[n][m]` is true.
2. If `p[j + 1] == '*'`: either skip `p[j..j + 2]` (zero copies), or, if `s[i]` matches `p[j]`,
   consume one character of `s` and stay on the same pattern: `dp[i + 1][j]`.
3. Otherwise the first characters must match and `dp[i][j] = dp[i + 1][j + 1]`.

## Starter code

Write your code in [`src/practice/p0989_regular_expression_matching.rs`](../src/practice/p0989_regular_expression_matching.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0989
```

```rust
pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
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

    fn m(s: &str, p: &str) -> bool {
        Solution::is_match(s.to_string(), p.to_string())
    }

    /// A direct recursive matcher, exponential but obviously correct.
    fn naive(s: &[u8], p: &[u8]) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        let first = !s.is_empty() && (p[0] == b'.' || p[0] == s[0]);
        if p.len() >= 2 && p[1] == b'*' {
            naive(s, &p[2..]) || (first && naive(&s[1..], p))
        } else {
            first && naive(&s[1..], &p[1..])
        }
    }

    #[test]
    fn examples() {
        assert!(!m("aa", "a"));
        assert!(m("aa", "a*"));
        assert!(m("ab", ".*"));
        assert!(m("aab", "c*a*b"));
    }

    #[test]
    fn tricky_cases() {
        assert!(!m("mississippi", "mis*is*p*."));
        assert!(m("mississippi", "mis*is*ip*."));
        assert!(!m("ab", ".*c"));
        assert!(m("aaa", "a*a"));
        assert!(m("a", "ab*"));
        assert!(!m("a", ".*..a*"));
    }

    #[test]
    fn matches_naive_on_small_inputs() {
        let strings = ["", "a", "b", "ab", "aab", "aba", "bbb", "abab"];
        let patterns = [
            "a*", ".*", "a.b", "a*b*", ".a*", "ab*a", "b*a*b", ".*b.*", "a*.b", "aba",
        ];
        for s in strings {
            for p in patterns {
                assert_eq!(
                    m(s, p),
                    naive(s.as_bytes(), p.as_bytes()),
                    "s = {s:?}, p = {p:?}"
                );
            }
        }
    }

    #[test]
    fn pathological_backtracking_is_fast() {
        // A naive matcher would explore an exponential number of ways to split the a's.
        assert!(!m("aaaaaaaaaaaaaaaaaaab", "a*a*a*a*a*a*a*a*a*c"));
        assert!(m("aaaaaaaaaaaaaaaaaaaa", "a*a*a*a*a*a*a*a*a*a*"));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** fill the table from the end (`i` from `n` down to `0`, `j` from `m - 1` down to `0`).
`first` says whether `s[i]` exists and matches `p[j]`. With a star after `p[j]`, the state is true
if skipping the starred element works or if `first` holds and the rest of the string still
matches the same starred element.

**Complexity:** `O(n * m)` time and space.

**Rust concepts:**
- Working on `as_bytes()` gives `O(1)` indexing.
- `s.get(i)` returns `None` at the end of the string, which folds the bounds check into the match.

```rust
pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (n, m) = (s.len(), p.len());
        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[n][m] = true;
        for i in (0..=n).rev() {
            for j in (0..m).rev() {
                let first = s.get(i).is_some_and(|&c| p[j] == b'.' || p[j] == c);
                dp[i][j] = if p.get(j + 1) == Some(&b'*') {
                    dp[i][j + 2] || (first && dp[i + 1][j])
                } else {
                    first && dp[i + 1][j + 1]
                };
            }
        }
        dp[0][0]
    }
}
```

Full tested solution: [`src/solutions/p0989_regular_expression_matching.rs`](../src/solutions/p0989_regular_expression_matching.rs)

</details>
