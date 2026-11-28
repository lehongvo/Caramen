# Problem 0163: Reverse Words in a String III

- **Difficulty:** Easy
- **Topics:** `into_bytes`, `split_mut` on `&mut [u8]`, `slice::reverse`, `String::from_utf8`
- **Source:** [LeetCode 557. Reverse Words in a String III](https://leetcode.com/problems/reverse-words-in-a-string-iii/)

## Problem

Given a sentence `s`, reverse the characters of **each word** while keeping the words in their
original order and keeping the spaces where they are.

Implement `Solution::reverse_words(s: String) -> String`.

## Examples

```text
Input:  s = "Let's take LeetCode contest"
Output: "s'teL ekat edoCteeL tsetnoc"

Input:  s = "Mr Ding"
Output: "rM gniD"
```

## Constraints

- `1 <= s.len() <= 5 * 10^4`
- `s` contains printable ASCII characters.
- `s` has no leading or trailing spaces, at least one word, and words separated by single spaces.

## Hints

1. You own `s`: `s.into_bytes()` turns it into a `Vec<u8>` without copying.
2. `bytes.split_mut(|&b| b == b' ')` yields each word as a `&mut [u8]`; call `reverse()` on it.
3. Convert back with `String::from_utf8(bytes)`; for ASCII input it cannot fail.

## Starter code

Write your code in [`src/practice/p0163_reverse_words_in_a_string_iii.rs`](../src/practice/p0163_reverse_words_in_a_string_iii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0163
```

```rust
pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
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

    fn run(s: &str) -> String {
        Solution::reverse_words(s.to_string())
    }

    #[test]
    fn example_contest() {
        assert_eq!(run("Let's take LeetCode contest"), "s'teL ekat edoCteeL tsetnoc");
    }

    #[test]
    fn example_two_words() {
        assert_eq!(run("Mr Ding"), "rM gniD");
    }

    #[test]
    fn single_word_and_single_letters() {
        assert_eq!(run("abc"), "cba");
        assert_eq!(run("a b c"), "a b c");
    }

    #[test]
    fn reversing_twice_restores_input() {
        let s = "The quick brown fox";
        assert_eq!(run(&run(s)), s);
    }

    #[test]
    fn large_input() {
        let s = vec!["abcd"; 10_000].join(" ");
        assert_eq!(run(&s), vec!["dcba"; 10_000].join(" "));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** work in place on the string's own buffer. Taking the bytes out of the `String`,
reversing every space-separated sub-slice and putting the bytes back into a `String` needs no
extra allocation at all.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `String::into_bytes(self) -> Vec<u8>` and `String::from_utf8(Vec<u8>)` move the same buffer
  between the two types. `from_utf8` re-validates and returns a `Result`.
- Reversing bytes is only valid for ASCII text (multi-byte characters would be scrambled); the
  constraints guarantee ASCII, which is why the `expect` cannot fire. For general text use
  `word.chars().rev()` instead.
- `split_mut` hands out disjoint `&mut [u8]` pieces one after another.

```rust
pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut bytes = s.into_bytes();
        for word in bytes.split_mut(|&b| b == b' ') {
            word.reverse();
        }
        String::from_utf8(bytes).expect("reversing ASCII words keeps the text valid UTF-8")
    }
}
```

Full tested solution: [`src/solutions/p0163_reverse_words_in_a_string_iii.rs`](../src/solutions/p0163_reverse_words_in_a_string_iii.rs)

</details>
