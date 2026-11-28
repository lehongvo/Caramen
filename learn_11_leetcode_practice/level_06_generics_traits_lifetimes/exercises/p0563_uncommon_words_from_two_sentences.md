# Problem 0563: Uncommon Words from Two Sentences

- **Difficulty:** Easy
- **Topics:** `HashMap<&str, usize>` whose keys borrow from local `String`s, `chain` of two `split_whitespace` iterators
- **Source:** [LeetCode 884. Uncommon Words from Two Sentences](https://leetcode.com/problems/uncommon-words-from-two-sentences/)

## Problem

A sentence is a string of lowercase words separated by single spaces. A word is **uncommon** if it
appears exactly once in one of the two sentences and does not appear in the other one, i.e. it
appears exactly once in total.

Return all uncommon words of `s1` and `s2`, in any order.

Implement `Solution::uncommon_from_sentences(s1: String, s2: String) -> Vec<String>`.

Count the words in a `HashMap<&str, usize>` whose keys are slices of `s1` and `s2`: no word should
be copied into a `String` until it is part of the answer.

## Examples

```text
Input:  s1 = "this apple is sweet", s2 = "this apple is sour"
Output: ["sweet", "sour"]   (any order)

Input:  s1 = "apple apple", s2 = "banana"
Output: ["banana"]
```

## Constraints

- `1 <= s1.len(), s2.len() <= 200`
- Both sentences contain lowercase letters and single spaces, without leading or trailing spaces.

## Hints

1. "Exactly once in one and absent from the other" equals "exactly once in the concatenation".
2. `s1.split_whitespace().chain(s2.split_whitespace())` iterates over all words of both.

## Starter code

Write your code in [`src/practice/p0563_uncommon_words_from_two_sentences.rs`](../src/practice/p0563_uncommon_words_from_two_sentences.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0563
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
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

    fn sorted(s1: &str, s2: &str) -> Vec<String> {
        let mut v = Solution::uncommon_from_sentences(s1.to_string(), s2.to_string());
        v.sort();
        v
    }

    #[test]
    fn example_sweet_and_sour() {
        assert_eq!(
            sorted("this apple is sweet", "this apple is sour"),
            vec!["sour", "sweet"]
        );
    }

    #[test]
    fn example_repeated_word() {
        assert_eq!(sorted("apple apple", "banana"), vec!["banana"]);
    }

    #[test]
    fn nothing_uncommon() {
        assert!(sorted("a b", "b a").is_empty());
        assert!(sorted("x x", "x").is_empty());
    }

    #[test]
    fn everything_uncommon() {
        assert_eq!(sorted("a b", "c"), vec!["a", "b", "c"]);
        let left: Vec<String> = (0..30).map(|i| format!("w{i}")).collect();
        let right: Vec<String> = (15..45).map(|i| format!("w{i}")).collect();
        let result = sorted(&left.join(" "), &right.join(" "));
        assert_eq!(result.len(), 30); // w0..w14 and w30..w44
        assert!(result.contains(&"w0".to_string()) && result.contains(&"w44".to_string()));
        assert!(!result.contains(&"w20".to_string()));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** count every word of both sentences together, then keep the words with count `1`.

**Complexity:** `O(n)` time and space, where `n` is the total length.

**Rust concepts:**
- The map's keys are `&str` slices into `s1` and `s2`. This is fine because `s1` and `s2` live until
  the end of the function, longer than the map: the borrow checker verifies it.
- Only the final answer allocates (`to_string()` on the selected words).
- Tests sort the output because the statement allows any order and `HashMap` iteration order is
  unspecified.

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for word in s1.split_whitespace().chain(s2.split_whitespace()) {
            *counts.entry(word).or_insert(0) += 1;
        }
        counts
            .into_iter()
            .filter(|&(_, c)| c == 1)
            .map(|(w, _)| w.to_string())
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0563_uncommon_words_from_two_sentences.rs`](../src/solutions/p0563_uncommon_words_from_two_sentences.rs)

</details>
