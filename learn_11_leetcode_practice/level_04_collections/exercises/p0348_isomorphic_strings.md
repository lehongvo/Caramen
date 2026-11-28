# Problem 0348: Isomorphic Strings

- **Difficulty:** Easy
- **Topics:** two `HashMap`s for a bijection, `entry(k).or_insert(v)` as "insert or check"
- **Source:** [LeetCode 205. Isomorphic Strings](https://leetcode.com/problems/isomorphic-strings/)

## Problem

Two strings `s` and `t` of the same length are **isomorphic** if the characters of `s` can be
replaced to obtain `t`, where:

- every occurrence of a character is replaced by the same character;
- two different characters never map to the same character;
- a character may map to itself.

Return `true` if `s` and `t` are isomorphic.

Implement `Solution::is_isomorphic(s: String, t: String) -> bool`.

## Examples

```text
Input:  s = "egg", t = "add"
Output: true      (e -> a, g -> d)

Input:  s = "foo", t = "bar"
Output: false     ('o' would have to map to both 'a' and 'r')

Input:  s = "paper", t = "title"
Output: true

Input:  s = "badc", t = "baba"
Output: false     ('b' and 'd' would both map to 'b')
```

## Constraints

- `1 <= s.len() <= 5 * 10^4`, `t.len() == s.len()`
- `s` and `t` consist of valid ASCII characters.

## Hints

1. One map `s -> t` catches the "same character, different images" case of `"foo"`/`"bar"`.
2. A second map `t -> s` catches the "different characters, same image" case of `"badc"`/`"baba"`.
3. `*map.entry(a).or_insert(b) == b` inserts `a -> b` if `a` is new, and otherwise checks that
   the stored image is `b`.

## Starter code

Write your code in [`src/practice/p0348_isomorphic_strings.rs`](../src/practice/p0348_isomorphic_strings.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0348
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
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

    fn iso(s: &str, t: &str) -> bool {
        Solution::is_isomorphic(s.to_string(), t.to_string())
    }

    #[test]
    fn example_egg_add() {
        assert!(iso("egg", "add"));
    }

    #[test]
    fn example_foo_bar() {
        assert!(!iso("foo", "bar"));
    }

    #[test]
    fn example_paper_title() {
        assert!(iso("paper", "title"));
    }

    #[test]
    fn example_badc_baba_needs_backward_map() {
        assert!(!iso("badc", "baba"));
    }

    #[test]
    fn identity_and_symbols() {
        assert!(iso("a", "a"));
        assert!(iso("13", "42"));
        assert!(iso("!?!", " . "));
        assert!(!iso("ab", "aa"));
    }

    #[test]
    fn large_strings() {
        let s: String = (0..50_000)
            .map(|i| (b'a' + (i % 26) as u8) as char)
            .collect();
        let t: String = (0..50_000)
            .map(|i| (b'A' + (25 - i % 26) as u8) as char)
            .collect();
        assert!(Solution::is_isomorphic(s.clone(), t));
        let mut u = s.clone();
        u.pop();
        u.push('a');
        assert!(!Solution::is_isomorphic(s, u));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** walk both strings in parallel and keep a mapping in each direction. For each pair
`(a, b)`, the mapping `a -> b` must agree with what was recorded before (or be recorded now), and
likewise `b -> a`. Any disagreement means the strings are not isomorphic.

**Complexity:** `O(n)` average time, `O(1)` space (at most 128 ASCII keys per map).

**Rust concepts:**
- `entry(a).or_insert(b)` returns `&mut char` pointing at the stored image, whether it was just
  inserted or already there; comparing it with `b` is a one-line "insert or verify".
- `zip` walks two iterators in lock step; `all` stops at the first conflict.
- With ASCII input, two `[Option<u8>; 128]` arrays would avoid hashing altogether.

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut forward: HashMap<char, char> = HashMap::new();
        let mut backward: HashMap<char, char> = HashMap::new();
        s.chars().zip(t.chars()).all(|(a, b)| {
            *forward.entry(a).or_insert(b) == b && *backward.entry(b).or_insert(a) == a
        })
    }
}
```

Full tested solution: [`src/solutions/p0348_isomorphic_strings.rs`](../src/solutions/p0348_isomorphic_strings.rs)

</details>
