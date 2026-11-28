# Problem 0194: Reverse Words in a String

- **Difficulty:** Medium
- **Topics:** in-place word reversal on the string's own bytes, compacting spaces, `split_whitespace().rev()`
- **Source:** [LeetCode 151. Reverse Words in a String](https://leetcode.com/problems/reverse-words-in-a-string/)

## Problem

Given a string `s`, return its words in reverse order, separated by exactly one space. A word is a
maximal run of non-space characters. `s` may have leading or trailing spaces and several spaces
between words; the result must not.

Implement `Solution::reverse_words(s: String) -> String`.

Follow-up (the interesting part in Rust): do it in place on the string's own buffer, with `O(1)`
extra space.

## Examples

```text
Input:  s = "the sky is blue"
Output: "blue is sky the"

Input:  s = "  hello world  "
Output: "world hello"

Input:  s = "a good   example"
Output: "example good a"
```

## Constraints

- `1 <= s.len() <= 10^4`
- `s` contains English letters, digits and spaces `' '`, and at least one word.

## Hints

1. One-liner: `s.split_whitespace().rev().collect::<Vec<_>>().join(" ")`.
2. In place: reverse the whole byte buffer, so the words are in the right order but each word is
   backwards; then reverse every word.
3. While doing that, copy the bytes towards the front with a `write` index so that only single
   spaces remain, and `truncate` at the end.

## Starter code

Write your code in [`src/practice/p0194_reverse_words_in_a_string.rs`](../src/practice/p0194_reverse_words_in_a_string.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0194
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
    fn example_sky() {
        assert_eq!(run("the sky is blue"), "blue is sky the");
    }

    #[test]
    fn example_outer_spaces() {
        assert_eq!(run("  hello world  "), "world hello");
    }

    #[test]
    fn example_inner_spaces() {
        assert_eq!(run("a good   example"), "example good a");
    }

    #[test]
    fn single_word() {
        assert_eq!(run("   rust   "), "rust");
        assert_eq!(run("x"), "x");
    }

    #[test]
    fn matches_split_whitespace_version() {
        let s = "  12 apples  and 7   Oranges ";
        let expected = s.split_whitespace().rev().collect::<Vec<_>>().join(" ");
        assert_eq!(run(s), expected);
    }

    #[test]
    fn large_input() {
        let words: Vec<String> = (0..2_000).map(|i| i.to_string()).collect();
        let s = format!("  {}  ", words.join("   "));
        let expected: Vec<&str> = words.iter().rev().map(|w| w.as_str()).collect();
        assert_eq!(run(&s), expected.join(" "));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** take the bytes out of the `String` and reverse them all. Then scan with a `read` index:
skip spaces; for each word, write one separating space (except before the first word), copy the
word forward to `write`, and reverse the copied word in place. `write` never passes `read`, so no
unread byte is overwritten. Truncate to `write` and turn the bytes back into a `String`.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `into_bytes()` / `String::from_utf8()` move the same buffer between `String` and `Vec<u8>`; the
  input is ASCII, so the bytes stay valid UTF-8.
- `bytes[start..write].reverse()` works on a mutable sub-slice of the buffer.
- The library version `split_whitespace().rev()` is shorter but allocates a `Vec<&str>` and a new
  `String`.

```rust
pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut bytes = s.into_bytes();
        bytes.reverse();
        let n = bytes.len();
        let (mut read, mut write) = (0, 0);
        while read < n {
            if bytes[read] == b' ' {
                read += 1;
                continue;
            }
            if write > 0 {
                bytes[write] = b' ';
                write += 1;
            }
            let start = write;
            while read < n && bytes[read] != b' ' {
                bytes[write] = bytes[read];
                write += 1;
                read += 1;
            }
            bytes[start..write].reverse();
        }
        bytes.truncate(write);
        String::from_utf8(bytes).expect("rearranging ASCII bytes keeps the text valid UTF-8")
    }
}
```

Full tested solution: [`src/solutions/p0194_reverse_words_in_a_string.rs`](../src/solutions/p0194_reverse_words_in_a_string.rs)

</details>
