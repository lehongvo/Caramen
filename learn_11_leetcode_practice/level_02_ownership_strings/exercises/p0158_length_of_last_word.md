# Problem 0158: Length of Last Word

- **Difficulty:** Easy
- **Topics:** `split_whitespace().next_back()`, `Option::map_or`, `trim_end`, `rsplit`
- **Source:** [LeetCode 58. Length of Last Word](https://leetcode.com/problems/length-of-last-word/)

## Problem

A string `s` consists of words and spaces. A word is a maximal run of non-space characters.
Return the length of the **last** word of `s`. There may be spaces at the beginning, between the
words (several in a row) and at the end.

Implement `Solution::length_of_last_word(s: String) -> i32`.

## Examples

```text
Input:  s = "Hello World"
Output: 5

Input:  s = "   fly me   to   the moon  "
Output: 4

Input:  s = "luffy is still joyboy"
Output: 6
```

## Constraints

- `1 <= s.len() <= 10^4`
- `s` consists of English letters and spaces `' '`.
- There is at least one word in `s`.

## Hints

1. `split_whitespace()` is a double-ended iterator: `next_back()` returns the last word directly.
2. Another way: `s.trim_end()` removes the trailing spaces, then `rsplit(' ').next()` is the last
   word.

## Starter code

Write your code in [`src/practice/p0158_length_of_last_word.rs`](../src/practice/p0158_length_of_last_word.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0158
```

```rust
pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
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
        Solution::length_of_last_word(s.to_string())
    }

    #[test]
    fn example_two_words() {
        assert_eq!(run("Hello World"), 5);
    }

    #[test]
    fn example_extra_spaces() {
        assert_eq!(run("   fly me   to   the moon  "), 4);
    }

    #[test]
    fn example_four_words() {
        assert_eq!(run("luffy is still joyboy"), 6);
    }

    #[test]
    fn single_word_and_single_letter() {
        assert_eq!(run("a"), 1);
        assert_eq!(run(" word "), 4);
    }

    #[test]
    fn long_input() {
        let s = format!(
            "{}{}{}",
            "x ".repeat(4_000),
            "y".repeat(1_000),
            " ".repeat(500)
        );
        assert_eq!(run(&s), 1_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** take the last item of the word iterator with `next_back()`. It walks from the end of
the string, so it only looks at the trailing spaces and the last word.

**Complexity:** `O(k)` time where `k` is the number of characters at the end that are inspected
(at most `n`), `O(1)` extra space.

**Rust concepts:**
- `SplitWhitespace` implements `DoubleEndedIterator`, so `next_back()` (or `.rev().next()`) is
  cheap; `.last()` would scan the whole string from the front.
- `Option::map_or(default, f)` turns `Option<&str>` into an `i32` in one call.
- The input is ASCII, so `len()` (bytes) is also the number of letters.

```rust
pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace()
            .next_back()
            .map_or(0, |word| word.len() as i32)
    }
}
```

Full tested solution: [`src/solutions/p0158_length_of_last_word.rs`](../src/solutions/p0158_length_of_last_word.rs)

</details>
