# Problem 0409: Check If a Word Occurs As a Prefix of Any Word in a Sentence

- **Difficulty:** Easy
- **Topics:** `split`, `Iterator::position`, `str::starts_with`, `Option::map_or`
- **Source:** [LeetCode 1455. Check If a Word Occurs As a Prefix of Any Word in a Sentence](https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/)

## Problem

A `sentence` is a list of words separated by single spaces. Given a `search_word`, find the
first word of the sentence that **starts with** `search_word` and return its position counted
from **1** (the first word has position 1). If no word starts with `search_word`, return `-1`.

Implement `Solution::is_prefix_of_word(sentence: String, search_word: String) -> i32`.

## Examples

```text
Input:  sentence = "i love eating burger", search_word = "burg"
Output: 4
Explanation: "burger" is the 4th word and starts with "burg".

Input:  sentence = "this problem is an easy problem", search_word = "pro"
Output: 2
Explanation: words 2 and 6 both start with "pro"; the smallest position is returned.

Input:  sentence = "i am tired", search_word = "you"
Output: -1
```

## Constraints

- `1 <= sentence.len() <= 100`
- `1 <= search_word.len() <= 10`
- `sentence` contains lowercase English letters and single spaces, with no leading or trailing
  space; `search_word` contains lowercase English letters.

## Hints

1. `Iterator::position(predicate)` returns the 0-based index of the first matching item as an
   `Option<usize>`.
2. Convert the `Option<usize>` into the answer with `map_or(-1, |i| ...)`.

## Starter code

Write your code in [`src/practice/p0409_check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence.rs`](../src/practice/p0409_check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0409
```

```rust
pub struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
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

    fn run(sentence: &str, word: &str) -> i32 {
        Solution::is_prefix_of_word(sentence.to_string(), word.to_string())
    }

    #[test]
    fn example_burger() {
        assert_eq!(run("i love eating burger", "burg"), 4);
    }

    #[test]
    fn example_first_of_two_matches() {
        assert_eq!(run("this problem is an easy problem", "pro"), 2);
    }

    #[test]
    fn example_not_found() {
        assert_eq!(run("i am tired", "you"), -1);
    }

    #[test]
    fn prefix_must_be_at_the_start_of_a_word() {
        assert_eq!(run("hellohello hellohellohello", "ell"), -1);
        assert_eq!(run("a", "a"), 1);
        assert_eq!(run("ab abc", "abcd"), -1);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** split the sentence into words, find the index of the first word that starts with the
search word, and translate "found at index `i`" into `i + 1` and "not found" into `-1`.

**Complexity:** `O(n * m)` time in the worst case (`n` = sentence length, `m` = search word
length), `O(1)` extra space: the words are borrowed slices, nothing is copied.

**Rust concepts:**
- `position` is a short-circuiting search: it stops at the first match.
- `starts_with` accepts many pattern types; `&search_word` (a `&String`) works because it
  derefs to `&str`.
- `map_or(default, f)` converts an `Option` into a plain value in one call, replacing a
  `match` with two arms.

```rust
pub struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split(' ')
            .position(|word| word.starts_with(&search_word))
            .map_or(-1, |i| i as i32 + 1)
    }
}
```

Full tested solution: [`src/solutions/p0409_check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence.rs`](../src/solutions/p0409_check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence.rs)

</details>
