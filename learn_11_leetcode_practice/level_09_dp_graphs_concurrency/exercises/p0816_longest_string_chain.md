# Problem 0816: Longest String Chain

- **Difficulty:** Medium
- **Topics:** DP over words sorted by length, `HashMap<&str, i32>`, `sort_unstable_by_key`
- **Source:** [LeetCode 1048. Longest String Chain](https://leetcode.com/problems/longest-string-chain/)

## Problem

Word `a` is a **predecessor** of word `b` if you can insert exactly one letter anywhere in `a`
(without reordering the other letters) to obtain `b`. For example `"abc"` is a predecessor of
`"abac"`, but `"cba"` is not a predecessor of `"bcad"`.

A **word chain** is a sequence `w1, w2, ..., wk` (with `k >= 1`) in which each word is a
predecessor of the next one. A single word is a chain of length 1. Given a list of words,
return the length of the longest chain that uses only words from the list.

Implement `Solution::longest_str_chain(words: Vec<String>) -> i32`.

## Examples

```text
Input:  words = ["a", "b", "ba", "bca", "bda", "bdca"]
Output: 4
Explanation: "a" -> "ba" -> "bda" -> "bdca".

Input:  words = ["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]
Output: 5
Explanation: "xb" -> "xbc" -> "cxbc" -> "pcxbc" -> "pcxbcf".

Input:  words = ["abcd", "dbqca"]
Output: 1
```

## Constraints

- `1 <= words.len() <= 1000`
- `1 <= words[i].len() <= 16`
- Words contain only lowercase English letters.

## Hints

1. A predecessor is always exactly one letter shorter, so process words from shortest to
   longest.
2. For a word of length `L`, there are only `L` candidate predecessors: delete each letter once.
3. Remember the best chain ending at each word in a map keyed by the word.

## Starter code

Write your code in [`src/practice/p0816_longest_string_chain.rs`](../src/practice/p0816_longest_string_chain.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0816
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
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

    fn run(words: &[&str]) -> i32 {
        Solution::longest_str_chain(words.iter().map(|w| w.to_string()).collect())
    }

    #[test]
    fn example_four() {
        assert_eq!(run(&["a", "b", "ba", "bca", "bda", "bdca"]), 4);
    }

    #[test]
    fn example_five() {
        assert_eq!(run(&["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]), 5);
    }

    #[test]
    fn example_no_chain() {
        assert_eq!(run(&["abcd", "dbqca"]), 1);
    }

    #[test]
    fn order_of_letters_matters() {
        // "cba" is not a predecessor of "bcad".
        assert_eq!(run(&["cba", "bcad"]), 1);
    }

    #[test]
    fn full_ladder_of_sixteen() {
        let words: Vec<String> = (1..=16).map(|k| "a".repeat(k)).collect();
        assert_eq!(Solution::longest_str_chain(words), 16);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** sort the words by length. For each word, build every string obtained by deleting one
letter and look it up: `best[word] = 1 + max(best[predecessor])` (or `1` if none is known).
Shorter words were processed first, so their values are final. The answer is the largest value.

**Complexity:** `O(n log n + n * L^2)` time (each of the `L` deletions builds a string of length
`L - 1`), `O(n * L)` space.

**Rust concepts:**
- The map `HashMap<&str, i32>` borrows keys from the sorted `Vec<String>`; lookups with a
  temporary `String` work through `best.get(candidate.as_str())`.
- Deleting one letter: `format!("{}{}", &w[..i], &w[i + 1..])` (fine because the input is ASCII).
- `sort_unstable_by_key(String::len)` sorts by a derived key without a custom comparator.

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words;
        words.sort_unstable_by_key(String::len);
        let mut best: HashMap<&str, i32> = HashMap::with_capacity(words.len());
        let mut answer = 0;
        for w in &words {
            let chain = (0..w.len())
                .map(|i| format!("{}{}", &w[..i], &w[i + 1..]))
                .filter_map(|pred| best.get(pred.as_str()).copied())
                .max()
                .map_or(1, |len| len + 1);
            best.insert(w.as_str(), chain);
            answer = answer.max(chain);
        }
        answer
    }
}
```

Full tested solution: [`src/solutions/p0816_longest_string_chain.rs`](../src/solutions/p0816_longest_string_chain.rs)

</details>
