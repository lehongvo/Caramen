# Problem 0990: Wildcard Matching

- **Difficulty:** Hard
- **Topics:** DP with a rolling 1D row, `'?'` and `'*'` semantics, `std::mem::swap` of rows, large inputs
- **Source:** [LeetCode 44. Wildcard Matching](https://leetcode.com/problems/wildcard-matching/)

## Problem

Match the whole string `s` against a wildcard pattern `p` where:

- `'?'` matches any single character;
- `'*'` matches any sequence of characters, including the empty one.

Unlike problem 0989, `'*'` is a stand-alone wildcard here, not a modifier of the previous element.
Implement `Solution::is_match(s: String, p: String) -> bool`.

## Examples

```text
s = "aa",  p = "a"     => false
s = "aa",  p = "*"     => true
s = "cb",  p = "?a"    => false
s = "adceb", p = "*a*b" => true
```

## Constraints

- `0 <= s.len(), p.len() <= 2000`
- `s` has lowercase letters; `p` has lowercase letters, `'?'` and `'*'`.

## Hints

1. Let `dp[i][j]` mean "`s[..i]` matches `p[..j]`"; `dp[0][0]` is true, and `dp[0][j]` is true only
   while `p[..j]` is all stars.
2. A `'*'` at `p[j - 1]` gives `dp[i][j] = dp[i][j - 1]` (matches nothing) `|| dp[i - 1][j]`
   (matches one more character). A letter or `'?'` needs `dp[i - 1][j - 1]` and a matching char.
3. Row `i` only depends on row `i - 1`: keep two rows of length `m + 1`.

## Starter code

Write your code in [`src/practice/p0990_wildcard_matching.rs`](../src/practice/p0990_wildcard_matching.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0990
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

    fn naive(s: &[u8], p: &[u8]) -> bool {
        match p.first() {
            None => s.is_empty(),
            Some(b'*') => naive(s, &p[1..]) || (!s.is_empty() && naive(&s[1..], p)),
            Some(&c) => !s.is_empty() && (c == b'?' || c == s[0]) && naive(&s[1..], &p[1..]),
        }
    }

    #[test]
    fn examples() {
        assert!(!m("aa", "a"));
        assert!(m("aa", "*"));
        assert!(!m("cb", "?a"));
        assert!(m("adceb", "*a*b"));
        assert!(!m("acdcb", "a*c?b"));
    }

    #[test]
    fn empty_strings_and_stars() {
        assert!(m("", ""));
        assert!(m("", "***"));
        assert!(!m("", "?"));
        assert!(!m("a", ""));
        assert!(m("abc", "a**?"));
    }

    #[test]
    fn matches_naive() {
        let strings = ["", "a", "ab", "abc", "aab", "bba", "abab"];
        let patterns = [
            "*", "?", "a*", "*b", "?*?", "a?b", "*a*b*", "a*a", "??*", "b*",
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
    fn large_inputs() {
        let s = "a".repeat(2_000);
        let p = "*".repeat(1_000) + "b";
        assert!(!m(&s, &p));
        let p = "a*".repeat(999) + "a";
        assert!(m(&s, &p));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the classic prefix DP, stored as two rolling rows. For each character of `s`, the new
row is computed left to right: a star copies from the left (empty match) or from above (absorb one
more character); other pattern characters copy diagonally when they match.

**Complexity:** `O(n * m)` time, `O(m)` space.

**Rust concepts:**
- `std::mem::swap(&mut previous, &mut current)` reuses the two buffers instead of allocating a new
  row every time.
- `iter().take_while(|&&b| b == b'*')` fills the first row.

```rust
pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let m = p.len();
        let mut previous = vec![false; m + 1];
        previous[0] = true;
        for (j, _) in p.iter().enumerate().take_while(|&(_, &b)| b == b'*') {
            previous[j + 1] = true;
        }
        let mut current = vec![false; m + 1];
        for &c in s {
            current[0] = false;
            for j in 1..=m {
                current[j] = match p[j - 1] {
                    b'*' => current[j - 1] || previous[j],
                    b'?' => previous[j - 1],
                    letter => previous[j - 1] && letter == c,
                };
            }
            std::mem::swap(&mut previous, &mut current);
        }
        previous[m]
    }
}
```

Full tested solution: [`src/solutions/p0990_wildcard_matching.rs`](../src/solutions/p0990_wildcard_matching.rs)

</details>
