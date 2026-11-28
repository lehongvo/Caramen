# Problem 0564: Count Common Words With One Occurrence

- **Difficulty:** Easy
- **Topics:** `HashMap<&str, (u32, u32)>` with keys borrowed from two vectors, `String::as_str`, counting with tuples
- **Source:** [LeetCode 2085. Count Common Words With One Occurrence](https://leetcode.com/problems/count-common-words-with-one-occurrence/)

## Problem

Given two arrays of words, `words1` and `words2`, count the words that appear **exactly once in
each** of the two arrays.

Implement `Solution::count_words(words1: Vec<String>, words2: Vec<String>) -> i32`.

Use one `HashMap<&str, (u32, u32)>` whose keys borrow from the two vectors (the pair counts the
occurrences in `words1` and in `words2`).

## Examples

```text
Input:  words1 = ["leetcode", "is", "amazing", "as", "is"], words2 = ["amazing", "leetcode", "is"]
Output: 2      ("leetcode" and "amazing"; "is" appears twice in words1, "as" is missing in words2)

Input:  words1 = ["b", "bb", "bbb"], words2 = ["a", "aa", "aaa"]
Output: 0

Input:  words1 = ["a", "ab"], words2 = ["a", "a", "a", "ab"]
Output: 1
```

## Constraints

- `1 <= words1.len(), words2.len() <= 1000`
- `1 <= words1[i].len(), words2[j].len() <= 30`, lowercase English letters.

## Hints

1. `counts.entry(word.as_str()).or_default().0 += 1` for `words1`, `.1` for `words2`.
2. The answer counts the entries equal to `(1, 1)`.

## Starter code

Write your code in [`src/practice/p0564_count_common_words_with_one_occurrence.rs`](../src/practice/p0564_count_common_words_with_one_occurrence.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0564
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
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

    fn words(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_inputs() {
        assert_eq!(
            Solution::count_words(
                words(&["leetcode", "is", "amazing", "as", "is"]),
                words(&["amazing", "leetcode", "is"])
            ),
            2
        );
        assert_eq!(
            Solution::count_words(words(&["b", "bb", "bbb"]), words(&["a", "aa", "aaa"])),
            0
        );
        assert_eq!(
            Solution::count_words(words(&["a", "ab"]), words(&["a", "a", "a", "ab"])),
            1
        );
    }

    #[test]
    fn identical_arrays() {
        let w = words(&["x", "y", "z"]);
        assert_eq!(Solution::count_words(w.clone(), w), 3);
    }

    #[test]
    fn large_arrays() {
        let w1: Vec<String> = (0..1000).map(|i| format!("w{}", i % 600)).collect();
        let w2: Vec<String> = (0..1000).map(|i| format!("w{i}")).collect();
        // In w1, w0..w399 appear twice and w400..w599 once; in w2 each of w0..w999 appears once.
        assert_eq!(Solution::count_words(w1, w2), 200);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** one map, two counters per word. A word qualifies when its pair is exactly `(1, 1)`.

**Complexity:** `O(n + m)` time and space.

**Rust concepts:**
- `word.as_str()` borrows a `&str` from a `String` stored in the vector; the vectors outlive the
  map, so the keys stay valid.
- `(u32, u32)` implements `Default` (`(0, 0)`), so `or_default()` creates the pair on first use.
- Comparing a whole tuple (`== (1, 1)`) expresses both conditions at once.

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut counts: HashMap<&str, (u32, u32)> = HashMap::new();
        for w in &words1 {
            counts.entry(w.as_str()).or_default().0 += 1;
        }
        for w in &words2 {
            counts.entry(w.as_str()).or_default().1 += 1;
        }
        counts.values().filter(|&&pair| pair == (1, 1)).count() as i32
    }
}
```

Full tested solution: [`src/solutions/p0564_count_common_words_with_one_occurrence.rs`](../src/solutions/p0564_count_common_words_with_one_occurrence.rs)

</details>
