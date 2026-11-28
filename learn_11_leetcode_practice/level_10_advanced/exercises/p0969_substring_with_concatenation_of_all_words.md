# Problem 0969: Substring with Concatenation of All Words

- **Difficulty:** Hard
- **Topics:** sliding window measured in words, one pass per offset `0..word_len`, `HashMap<&str, usize>` with borrowed keys
- **Source:** [LeetCode 30. Substring with Concatenation of All Words](https://leetcode.com/problems/substring-with-concatenation-of-all-words/)

## Problem

You are given a string `s` and an array `words` of strings that all have the **same length**. A
*concatenated string* is any string formed by joining all of `words` in some order, each word
used exactly as many times as it appears in `words`. Return the starting indices of every
substring of `s` that is a concatenated string, in **any order**.

Implement `Solution::find_substring(s: String, words: Vec<String>) -> Vec<i32>`.

## Examples

```text
Input:  s = "barfoothefoobarman", words = ["foo", "bar"]
Output: [0, 9]        ("barfoo" and "foobar")

Input:  s = "wordgoodgoodgoodbestword", words = ["word", "good", "best", "word"]
Output: []

Input:  s = "barfoofoobarthefoobarman", words = ["bar", "foo", "the"]
Output: [6, 9, 12]
```

## Constraints

- `1 <= s.len() <= 10^4`, `1 <= words.len() <= 5000`, `1 <= words[i].len() <= 30`
- `s` and the words consist of lowercase English letters.

## Hints

1. With word length `w`, every candidate start belongs to one of `w` "lanes": `offset`,
   `offset + w`, `offset + 2w`, ...
2. In each lane, slide a window of whole words: add the next word on the right; if it is not a
   word at all, reset the window; if it appears too often, drop words from the left.
3. When the window holds exactly `words.len()` words, its start is an answer.

## Starter code

Write your code in [`src/practice/p0969_substring_with_concatenation_of_all_words.rs`](../src/practice/p0969_substring_with_concatenation_of_all_words.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0969
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
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

    fn run(s: &str, words: &[&str]) -> Vec<i32> {
        let mut result =
            Solution::find_substring(s.to_string(), words.iter().map(|w| w.to_string()).collect());
        result.sort_unstable();
        result
    }

    fn naive(s: &str, words: &[&str]) -> Vec<i32> {
        let total: usize = words.iter().map(|w| w.len()).sum();
        let w = words[0].len();
        let mut sorted_words: Vec<&str> = words.to_vec();
        sorted_words.sort_unstable();
        (0..=s.len().saturating_sub(total))
            .filter(|&i| i + total <= s.len())
            .filter(|&i| {
                let mut pieces: Vec<&str> = (0..words.len())
                    .map(|k| &s[i + k * w..i + (k + 1) * w])
                    .collect();
                pieces.sort_unstable();
                pieces == sorted_words
            })
            .map(|i| i as i32)
            .collect()
    }

    #[test]
    fn examples() {
        assert_eq!(run("barfoothefoobarman", &["foo", "bar"]), vec![0, 9]);
        assert!(
            run(
                "wordgoodgoodgoodbestword",
                &["word", "good", "best", "word"]
            )
            .is_empty()
        );
        assert_eq!(
            run("barfoofoobarthefoobarman", &["bar", "foo", "the"]),
            vec![6, 9, 12]
        );
    }

    #[test]
    fn repeated_words() {
        assert_eq!(run("aaaaaa", &["aa", "aa"]), vec![0, 1, 2]);
        assert_eq!(
            run(
                "wordgoodgoodgoodbestword",
                &["word", "good", "best", "good"]
            ),
            vec![8]
        );
        assert!(run("ab", &["abc"]).is_empty());
    }

    #[test]
    fn matches_naive() {
        let s: String = (0..300)
            .map(|i: u32| if (i * 7 + i / 3) % 3 == 0 { 'a' } else { 'b' })
            .collect();
        for words in [
            vec!["ab", "ba"],
            vec!["a", "b", "a"],
            vec!["abb", "bab", "bba"],
        ] {
            assert_eq!(run(&s, &words), naive(&s, &words), "{words:?}");
        }
    }

    #[test]
    fn long_input() {
        let s = "ab".repeat(5_000);
        let words = vec!["ab"; 100];
        let result = run(&s, &words);
        assert_eq!(result.len(), (10_000 - 200) / 2 + 1);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 2);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** count the required words once. For each offset, a window `[left, right)` advances one
word at a time while keeping per-word counts that never exceed the required ones (excess words are
removed from the left). A window of `m` words that respects all counts is a permutation of
`words`.

**Complexity:** `O(w * (n / w)) = O(n)` window steps, each costing an `O(w)` hash of a word, so
`O(n * w)` time and `O(m)` space.

**Rust concepts:**
- The count maps use `&str` keys borrowed from `words` and from `s`, so no word is copied.
- `s.get(i..i + w)` returns `None` past the end, which ends a lane cleanly.
- Tests sort the result because the order is unspecified.

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let w = words[0].len();
        let m = words.len();
        let mut required: HashMap<&str, usize> = HashMap::new();
        for word in &words {
            *required.entry(word.as_str()).or_insert(0) += 1;
        }
        let mut result = Vec::new();
        for offset in 0..w {
            let mut window: HashMap<&str, usize> = HashMap::new();
            let mut left = offset;
            let mut count = 0;
            let mut right = offset;
            while let Some(word) = s.get(right..right + w) {
                right += w;
                let Some(&limit) = required.get(word) else {
                    window.clear();
                    count = 0;
                    left = right;
                    continue;
                };
                *window.entry(word).or_insert(0) += 1;
                count += 1;
                while window[word] > limit {
                    let first = &s[left..left + w];
                    if let Some(c) = window.get_mut(first) {
                        *c -= 1;
                    }
                    count -= 1;
                    left += w;
                }
                if count == m {
                    result.push(left as i32);
                    let first = &s[left..left + w];
                    if let Some(c) = window.get_mut(first) {
                        *c -= 1;
                    }
                    count -= 1;
                    left += w;
                }
            }
        }
        result
    }
}
```

Full tested solution: [`src/solutions/p0969_substring_with_concatenation_of_all_words.rs`](../src/solutions/p0969_substring_with_concatenation_of_all_words.rs)

</details>
