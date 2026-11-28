# Problem 0173: Find First Palindromic String in the Array

- **Difficulty:** Easy
- **Topics:** `into_iter().find` to move a `String` out of a `Vec`, `bytes().eq(bytes().rev())`, `unwrap_or_default`
- **Source:** [LeetCode 2108. Find First Palindromic String in the Array](https://leetcode.com/problems/find-first-palindromic-string-in-the-array/)

## Problem

Given a vector of words, return the **first** word that is a palindrome (reads the same forwards
and backwards). If there is none, return an empty string.

Implement `Solution::first_palindrome(words: Vec<String>) -> String`.

The function owns `words`, so it can hand the matching `String` itself to the caller instead of
cloning it.

## Examples

```text
Input:  words = ["abc", "car", "ada", "racecar", "cool"]
Output: "ada"

Input:  words = ["notapalindrome", "racecar"]
Output: "racecar"

Input:  words = ["def", "ghi"]
Output: ""
```

## Constraints

- `1 <= words.len() <= 100`
- `1 <= words[i].len() <= 100`
- Words consist of lowercase English letters.

## Hints

1. `w.bytes().eq(w.bytes().rev())` compares a word with its reverse without allocating.
2. `words.into_iter().find(...)` returns `Option<String>`: the owned word, moved out of the vector.

## Starter code

Write your code in [`src/practice/p0173_find_first_palindromic_string_in_the_array.rs`](../src/practice/p0173_find_first_palindromic_string_in_the_array.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0173
```

```rust
pub struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
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

    fn run(words: &[&str]) -> String {
        Solution::first_palindrome(words.iter().map(|w| w.to_string()).collect())
    }

    #[test]
    fn example_ada_first() {
        assert_eq!(run(&["abc", "car", "ada", "racecar", "cool"]), "ada");
    }

    #[test]
    fn example_last_word() {
        assert_eq!(run(&["notapalindrome", "racecar"]), "racecar");
    }

    #[test]
    fn example_none() {
        assert_eq!(run(&["def", "ghi"]), "");
    }

    #[test]
    fn single_letters_are_palindromes() {
        assert_eq!(run(&["ab", "z", "aa"]), "z");
    }

    #[test]
    fn returned_string_is_moved_not_copied() {
        let words = vec!["xy".to_string(), "abba".to_string()];
        let ptr = words[1].as_ptr();
        let result = Solution::first_palindrome(words);
        assert_eq!(result, "abba");
        assert_eq!(result.as_ptr(), ptr);
    }

    #[test]
    fn largest_input() {
        let mut words: Vec<String> = (0..100).map(|_| format!("a{}b", "c".repeat(98))).collect();
        words[99] = "a".repeat(100);
        assert_eq!(Solution::first_palindrome(words), "a".repeat(100));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** consume the vector and stop at the first palindrome. Because `into_iter` yields owned
`String`s, `find` returns the word itself; `unwrap_or_default()` turns "not found" into `""`.

**Complexity:** `O(total length)` time, `O(1)` extra space.

**Rust concepts:**
- `iter().find(..)` would give `Option<&String>` and force a clone; `into_iter()` gives ownership,
  so the answer is moved out for free.
- The closure passed to `find` receives `&String` (a reference to each item), so checking a word
  does not consume it.
- `Option::unwrap_or_default` uses `String::default()`, the empty string.

```rust
pub struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|w| w.bytes().eq(w.bytes().rev()))
            .unwrap_or_default()
    }
}
```

Full tested solution: [`src/solutions/p0173_find_first_palindromic_string_in_the_array.rs`](../src/solutions/p0173_find_first_palindromic_string_in_the_array.rs)

</details>
