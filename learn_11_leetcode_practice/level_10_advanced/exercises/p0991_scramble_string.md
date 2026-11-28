# Problem 0991: Scramble String

- **Difficulty:** Hard
- **Topics:** interval DP over (start1, start2, length), memoisation with a flat `Vec<Option<bool>>`, letter-count pruning
- **Source:** [LeetCode 87. Scramble String](https://leetcode.com/problems/scramble-string/)

## Problem

A string can be **scrambled** recursively: a string of length 1 stays as it is; a longer string is
split at any position into two non-empty parts `x + y`, the parts are optionally swapped (giving
`x + y` or `y + x`), and then each part is scrambled recursively. Given two strings of the same
length, decide whether `s2` is a scrambled version of `s1`.

Implement `Solution::is_scramble(s1: String, s2: String) -> bool`.

## Examples

```text
s1 = "great", s2 = "rgeat"   => true    ("gr|eat" -> swap "g|r" -> "rg" + "eat")
s1 = "abcde", s2 = "caebd"   => false
s1 = "a",     s2 = "a"       => true
```

## Constraints

- `1 <= s1.len() == s2.len() <= 30`
- Lowercase English letters only.

## Hints

1. `scramble(i, j, len)`: is `s2[j..j + len]` a scramble of `s1[i..i + len]`? Try every split `k`:
   without swap, `(i, j, k)` and `(i + k, j + k, len - k)`; with swap, `(i, j + len - k, k)` and
   `(i + k, j, len - k)`.
2. Memoise the `n^3` states.
3. Prune: two substrings with different letter counts can never match.

## Starter code

Write your code in [`src/practice/p0991_scramble_string.rs`](../src/practice/p0991_scramble_string.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0991
```

```rust
pub struct Solution;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
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

    fn run(a: &str, b: &str) -> bool {
        Solution::is_scramble(a.to_string(), b.to_string())
    }

    #[test]
    fn examples() {
        assert!(run("great", "rgeat"));
        assert!(!run("abcde", "caebd"));
        assert!(run("a", "a"));
    }

    #[test]
    fn swaps_at_several_levels() {
        assert!(run("abcd", "badc"));
        assert!(run("abcd", "dcba"));
        assert!(run("abc", "bca"));
        assert!(!run("abcd", "bdac"));
        assert!(!run("ab", "cd"));
    }

    #[test]
    fn every_scramble_of_a_short_string_is_found() {
        // Generate all scrambles of "abcde" and check them; also check a non-scramble.
        fn scrambles(s: &str) -> Vec<String> {
            if s.len() == 1 {
                return vec![s.to_string()];
            }
            let mut out = Vec::new();
            for k in 1..s.len() {
                for x in scrambles(&s[..k]) {
                    for y in scrambles(&s[k..]) {
                        out.push(format!("{x}{y}"));
                        out.push(format!("{y}{x}"));
                    }
                }
            }
            out.sort();
            out.dedup();
            out
        }
        let all = scrambles("abcde");
        assert!(all.iter().all(|t| run("abcde", t)));
        assert!(!all.contains(&"caebd".to_string()));
    }

    #[test]
    fn longest_inputs() {
        let a = "abcdefghijklmnopqrstuvwxyzabcd";
        let b: String = a.chars().rev().collect();
        assert!(run(a, &b));
        assert!(!run(
            "eebaacbcbcadaaedceaaacadccd",
            "eadcaacabaddaceacbceaabeccd"
        ));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** recursive search over the three-parameter state with a memo table. Equal substrings are
an immediate yes, different letter multisets an immediate no; otherwise each split point is tried
with and without the swap.

**Complexity:** `O(n^4)` time (`n^3` states, `O(n)` splits each), `O(n^3)` space.

**Rust concepts:**
- A flat memo indexed by `(i * n + j) * (n + 1) + len` avoids a triple-nested `Vec`.
- A private struct carries the byte slices and the memo, so the recursive method needs only the
  three integers.

```rust
pub struct Solution;

struct Scramble<'a> {
    a: &'a [u8],
    b: &'a [u8],
    n: usize,
    memo: Vec<Option<bool>>,
}

impl Scramble<'_> {
    fn check(&mut self, i: usize, j: usize, len: usize) -> bool {
        let key = (i * self.n + j) * (self.n + 1) + len;
        if let Some(known) = self.memo[key] {
            return known;
        }
        let (x, y) = (&self.a[i..i + len], &self.b[j..j + len]);
        let result = if x == y {
            true
        } else {
            let mut counts = [0_i32; 26];
            for (&p, &q) in x.iter().zip(y) {
                counts[usize::from(p - b'a')] += 1;
                counts[usize::from(q - b'a')] -= 1;
            }
            counts.iter().all(|&c| c == 0)
                && (1..len).any(|k| {
                    (self.check(i, j, k) && self.check(i + k, j + k, len - k))
                        || (self.check(i, j + len - k, k) && self.check(i + k, j, len - k))
                })
        };
        self.memo[key] = Some(result);
        result
    }
}

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        let mut search = Scramble {
            a: s1.as_bytes(),
            b: s2.as_bytes(),
            n,
            memo: vec![None; n * n * (n + 1)],
        };
        search.check(0, 0, n)
    }
}
```

Full tested solution: [`src/solutions/p0991_scramble_string.rs`](../src/solutions/p0991_scramble_string.rs)

</details>
