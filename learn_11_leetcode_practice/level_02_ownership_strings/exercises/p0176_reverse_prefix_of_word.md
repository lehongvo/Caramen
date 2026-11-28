# Problem 0176: Reverse Prefix of Word

- **Difficulty:** Easy
- **Topics:** `str::find`, `split_at`, `chars().rev().chain(..)`, returning the input unchanged
- **Source:** [LeetCode 2000. Reverse Prefix of Word](https://leetcode.com/problems/reverse-prefix-of-word/)

## Problem

Given a word and a character `ch`, reverse the part of the word that goes from its start up to
and **including** the first occurrence of `ch`. If `ch` does not occur in the word, return the
word unchanged.

Implement `Solution::reverse_prefix(word: String, ch: char) -> String`.

## Examples

```text
Input:  word = "abcdefd", ch = 'd'
Output: "dcbaefd"        (the first 'd' is at index 3: reverse "abcd")

Input:  word = "xyxzxe", ch = 'z'
Output: "zxyxxe"

Input:  word = "abcd", ch = 'z'
Output: "abcd"
```

## Constraints

- `1 <= word.len() <= 250`
- `word` consists of lowercase English letters; `ch` is a lowercase English letter.

## Hints

1. `word.find(ch)` returns the byte index of the first `ch`, or `None`.
2. `word.split_at(i + ch.len_utf8())` separates the prefix (including `ch`) from the rest.
3. Build the answer from `prefix.chars().rev()` followed by `rest.chars()`.

## Starter code

Write your code in [`src/practice/p0176_reverse_prefix_of_word.rs`](../src/practice/p0176_reverse_prefix_of_word.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0176
```

```rust
pub struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
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

    fn run(word: &str, ch: char) -> String {
        Solution::reverse_prefix(word.to_string(), ch)
    }

    #[test]
    fn example_first_d() {
        assert_eq!(run("abcdefd", 'd'), "dcbaefd");
    }

    #[test]
    fn example_z_in_the_middle() {
        assert_eq!(run("xyxzxe", 'z'), "zxyxxe");
    }

    #[test]
    fn example_not_found() {
        assert_eq!(run("abcd", 'z'), "abcd");
    }

    #[test]
    fn first_or_last_character() {
        assert_eq!(run("abc", 'a'), "abc");
        assert_eq!(run("abc", 'c'), "cba");
    }

    #[test]
    fn longest_word() {
        let word = format!("{}z{}", "a".repeat(200), "b".repeat(49));
        let expected = format!("z{}{}", "a".repeat(200), "b".repeat(49));
        assert_eq!(run(&word, 'z'), expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** locate `ch`. If it is missing, give the owned `word` straight back. Otherwise split the
word right after `ch` and collect the reversed prefix chained with the untouched rest.

**Complexity:** `O(n)` time and space.

**Rust concepts:**
- `str::find` accepts a `char` pattern and returns a byte index, which `split_at` needs.
- Adding `ch.len_utf8()` instead of `1` makes the split correct for any character, not just ASCII.
- `Iterator::chain` glues two iterators together; `collect()` builds the `String` in one go.
- In the `None` arm, returning `word` moves the original `String` out without any copy.

```rust
pub struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        match word.find(ch) {
            Some(i) => {
                let (prefix, rest) = word.split_at(i + ch.len_utf8());
                prefix.chars().rev().chain(rest.chars()).collect()
            }
            None => word,
        }
    }
}
```

Full tested solution: [`src/solutions/p0176_reverse_prefix_of_word.rs`](../src/solutions/p0176_reverse_prefix_of_word.rs)

</details>
