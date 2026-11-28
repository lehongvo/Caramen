# Problem 0383: Top K Frequent Words

- **Difficulty:** Medium
- **Topics:** `HashMap<&str, usize>` borrowing words, a two-level comparator (count descending, word ascending), `truncate`
- **Source:** [LeetCode 692. Top K Frequent Words](https://leetcode.com/problems/top-k-frequent-words/)

## Problem

Given a list of words and an integer `k`, return the `k` most frequent words, ordered by
frequency from highest to lowest. Words with the same frequency are ordered
**lexicographically** (alphabetically). Unlike "Top K Frequent Elements", the output order is
fully specified.

LeetCode's follow-up asks for `O(n log k)` time.

Implement `Solution::top_k_frequent(words: Vec<String>, k: i32) -> Vec<String>`.

## Examples

```text
Input:  words = ["i", "love", "leetcode", "i", "love", "coding"], k = 2
Output: ["i", "love"]      (both occur twice; "i" < "love")

Input:  words = ["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], k = 4
Output: ["the", "is", "sunny", "day"]     (4, 3, 2 and 1 occurrences)
```

## Constraints

- `1 <= words.len() <= 500`, `1 <= words[i].len() <= 10`, lowercase letters
- `1 <= k <=` number of distinct words

## Hints

1. Count with a `HashMap<&str, usize>` that borrows the words instead of cloning them.
2. Sort the `(word, count)` pairs with `b.count.cmp(&a.count).then_with(|| a.word.cmp(b.word))`.
3. For the follow-up, keep a heap of size `k` whose top is the **worst** of the current best
   `k`.

## Starter code

Write your code in [`src/practice/p0383_top_k_frequent_words.rs`](../src/practice/p0383_top_k_frequent_words.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0383
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
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

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_one() {
        let words = strings(&["i", "love", "leetcode", "i", "love", "coding"]);
        assert_eq!(Solution::top_k_frequent(words, 2), strings(&["i", "love"]));
    }

    #[test]
    fn example_two() {
        let words = strings(&[
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ]);
        assert_eq!(
            Solution::top_k_frequent(words, 4),
            strings(&["the", "is", "sunny", "day"])
        );
    }

    #[test]
    fn ties_are_alphabetical() {
        let words = strings(&["b", "a", "c", "b", "a", "c"]);
        assert_eq!(
            Solution::top_k_frequent(words.clone(), 3),
            strings(&["a", "b", "c"])
        );
        assert_eq!(Solution::top_k_frequent(words, 1), strings(&["a"]));
    }

    #[test]
    fn five_hundred_words() {
        // "w00" .. "w24": word i occurs i + 1 times, plus filler words that occur once.
        let mut words: Vec<String> = (0..25)
            .flat_map(|i| std::iter::repeat_n(format!("w{i:02}"), i + 1))
            .collect();
        words.extend((0..175).map(|i| format!("z{i:03}")));
        assert_eq!(words.len(), 500);
        assert_eq!(
            Solution::top_k_frequent(words.clone(), 3),
            strings(&["w24", "w23", "w22"])
        );
        // Count 1: "w00" sorts before every "z..." filler.
        let all = Solution::top_k_frequent(words, 26);
        assert_eq!(all[24], "w00");
        assert_eq!(all[25], "z000");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** count the words, collect the distinct `(word, count)` pairs into a `Vec`, sort them by
count descending and word ascending, keep the first `k`, and turn them into `String`s.

**Complexity:** `O(n + d log d)` time for `d` distinct words, `O(d)` space. The heap variant is
`O(n + d log k)`: store `(Reverse(count), word)` in a max-heap so that its top is the pair that
should be evicted first (lowest count, then the alphabetically **largest** word).

**Rust concepts:**
- The map's keys are `&str` slices borrowed from `words`; they are only converted to `String`
  for the final `k` answers.
- `then_with` combines the two sort keys; reversing only the count is done by swapping `a` and
  `b` in the first comparison.
- `truncate(k)` drops everything after the first `k` elements.

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for w in &words {
            *counts.entry(w.as_str()).or_insert(0) += 1;
        }
        let mut pairs: Vec<(&str, usize)> = counts.into_iter().collect();
        pairs.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
        pairs.truncate(k as usize);
        pairs.into_iter().map(|(w, _)| w.to_string()).collect()
    }
}
```

Full tested solution: [`src/solutions/p0383_top_k_frequent_words.rs`](../src/solutions/p0383_top_k_frequent_words.rs)

</details>
