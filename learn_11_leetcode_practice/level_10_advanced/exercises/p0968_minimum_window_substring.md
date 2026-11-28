# Problem 0968: Minimum Window Substring

- **Difficulty:** Hard
- **Topics:** variable-size sliding window, byte-indexed count arrays `[i32; 128]`, a "missing" counter, slicing the answer
- **Source:** [LeetCode 76. Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/)

## Problem

Given strings `s` and `t`, return the **shortest substring** of `s` that contains every character
of `t` **with multiplicity** (if `t` has two `'a'`s, the window needs at least two). Return `""`
if there is no such substring. When several shortest windows exist, return the leftmost one (the
tests are built so that this choice is the one LeetCode expects).

Implement `Solution::min_window(s: String, t: String) -> String`.

## Examples

```text
Input:  s = "ADOBECODEBANC", t = "ABC"
Output: "BANC"

Input:  s = "a", t = "aa"
Output: ""          (only one 'a' is available)
```

## Constraints

- `1 <= s.len(), t.len() <= 10^5`
- `s` and `t` consist of English letters (ASCII).

## Hints

1. `need[c]` = how many more copies of `c` the window still needs (can go negative for surplus);
   `missing` = the total number of required characters not yet covered.
2. Extend the right end; when `missing == 0`, shrink from the left as long as the window stays
   valid, recording the best window.
3. A character's arrival decreases `missing` only if `need[c]` was positive before.

## Starter code

Write your code in [`src/practice/p0968_minimum_window_substring.rs`](../src/practice/p0968_minimum_window_substring.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0968
```

```rust
pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
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

    fn run(s: &str, t: &str) -> String {
        Solution::min_window(s.to_string(), t.to_string())
    }

    #[test]
    fn examples() {
        assert_eq!(run("ADOBECODEBANC", "ABC"), "BANC");
        assert_eq!(run("a", "a"), "a");
        assert_eq!(run("a", "aa"), "");
    }

    #[test]
    fn multiplicity_and_case() {
        assert_eq!(run("aaflslflsldkalskaaa", "aaa"), "aaa");
        assert_eq!(run("abcAbc", "Ab"), "Ab");
        assert_eq!(run("ab", "b"), "b");
        assert_eq!(run("bba", "ab"), "ba");
        assert_eq!(run("xyz", "q"), "");
    }

    #[test]
    fn window_is_the_whole_string() {
        assert_eq!(run("cabwefgewcwaefgcf", "cae"), "cwae");
        assert_eq!(run("abc", "cba"), "abc");
    }

    #[test]
    fn long_input() {
        let s = "x".repeat(50_000) + "abc" + &"y".repeat(50_000);
        assert_eq!(run(&s, "cba"), "abc");
        let t = "z".repeat(10);
        assert_eq!(run(&s, &t), "");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** classic two-pointer window. Adding `s[r]` decrements `need[s[r]]` (and `missing` if that
character was still needed). Once everything is covered, move `l` right: removing `s[l]`
increments its need, and when that need becomes positive the window stops being valid. Each
valid window seen just before that point is a candidate.

**Complexity:** `O(|s| + |t|)` time, `O(1)` space (a fixed 128-entry table).

**Rust concepts:**
- Indexing `[i32; 128]` by `usize::from(byte)` is faster than a `HashMap` for ASCII.
- Keeping `(start, len)` of the best window and slicing `&s[start..start + len]` once at the end
  avoids building intermediate strings.

```rust
pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let bytes = s.as_bytes();
        let mut need = [0_i32; 128];
        for b in t.bytes() {
            need[usize::from(b)] += 1;
        }
        let mut missing = t.len();
        let mut best: Option<(usize, usize)> = None;
        let mut left = 0;
        for (right, &b) in bytes.iter().enumerate() {
            let slot = &mut need[usize::from(b)];
            if *slot > 0 {
                missing -= 1;
            }
            *slot -= 1;
            while missing == 0 {
                let len = right + 1 - left;
                if best.is_none_or(|(_, best_len)| len < best_len) {
                    best = Some((left, len));
                }
                let slot = &mut need[usize::from(bytes[left])];
                *slot += 1;
                if *slot > 0 {
                    missing += 1;
                }
                left += 1;
            }
        }
        best.map_or(String::new(), |(start, len)| {
            s[start..start + len].to_string()
        })
    }
}
```

Full tested solution: [`src/solutions/p0968_minimum_window_substring.rs`](../src/solutions/p0968_minimum_window_substring.rs)

</details>
