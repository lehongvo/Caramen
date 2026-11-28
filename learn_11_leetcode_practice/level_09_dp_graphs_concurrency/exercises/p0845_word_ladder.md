# Problem 0845: Word Ladder

- **Difficulty:** Hard
- **Topics:** BFS where neighbours are generated on the fly, `HashSet<Vec<u8>>`, removing visited
  words from the set
- **Source:** [LeetCode 127. Word Ladder](https://leetcode.com/problems/word-ladder/)

## Problem

A **transformation sequence** from `begin_word` to `end_word` is a list of words
`begin_word = w1, w2, ..., wk = end_word` in which consecutive words differ in exactly one
letter, and every word after `w1` belongs to `word_list`. (`begin_word` itself does not need to
be in the list.) Return the number of words `k` in the **shortest** such sequence, or `0` if
there is none.

Implement `Solution::ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32`.

## Examples

```text
Input:  begin_word = "hit", end_word = "cog",
        word_list = ["hot", "dot", "dog", "lot", "log", "cog"]
Output: 5
Explanation: hit -> hot -> dot -> dog -> cog.

Input:  begin_word = "hit", end_word = "cog",
        word_list = ["hot", "dot", "dog", "lot", "log"]
Output: 0
Explanation: "cog" is not in the list, so it can never be reached.
```

## Constraints

- `1 <= begin_word.len() <= 10`, and all words have the same length
- `1 <= word_list.len() <= 5000`; the words in the list are distinct
- `begin_word != end_word`; all words contain only lowercase English letters

## Hints

1. Words are nodes; two words are adjacent if they differ in one letter. The answer is a BFS
   distance (plus one, because it counts words, not moves).
2. Comparing every pair of words costs `O(n^2 * L)`. Instead, generate the `25 * L` possible
   one-letter changes of the current word and look each one up in a set.
3. Remove a word from the set when you enqueue it; that doubles as the visited mark.

## Starter code

Write your code in [`src/practice/p0845_word_ladder.rs`](../src/practice/p0845_word_ladder.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0845
```

```rust
use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
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

    fn run(begin: &str, end: &str, list: &[&str]) -> i32 {
        Solution::ladder_length(
            begin.to_string(),
            end.to_string(),
            list.iter().map(|w| w.to_string()).collect(),
        )
    }

    #[test]
    fn example_five_words() {
        assert_eq!(
            run("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"]),
            5
        );
    }

    #[test]
    fn example_end_word_missing() {
        assert_eq!(run("hit", "cog", &["hot", "dot", "dog", "lot", "log"]), 0);
    }

    #[test]
    fn one_letter_words() {
        assert_eq!(run("a", "c", &["a", "b", "c"]), 2);
    }

    #[test]
    fn no_connection() {
        assert_eq!(run("hot", "dog", &["hot", "dog"]), 0);
    }

    #[test]
    fn long_ladder() {
        // "aaaaa" -> "baaaa" -> "bbaaa" -> ... -> "bbbbb": 6 words.
        let list = [
            "baaaa", "bbaaa", "bbbaa", "bbbba", "bbbbb", "abbbb", "aabbb",
        ];
        assert_eq!(run("aaaaa", "bbbbb", &list), 6);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** put the list into a `HashSet<Vec<u8>>`. BFS from `begin_word` with `(word, length)`
pairs. For each popped word, try replacing each position with every other letter; a candidate
still in the set is removed from it and enqueued with `length + 1`. Return the length when the
end word is generated.

**Complexity:** `O(n * L * 26 * L)` time (each of the `n` words is expanded once, each candidate
costs `O(L)` to hash), `O(n * L)` space.

**Rust concepts:**
- Words are handled as `Vec<u8>` so one letter can be changed in place (`word[i] = c`) and
  restored afterwards; `String` does not allow indexing by position.
- `HashSet::remove` returns `true` if the value was present: test and mark visited in one call.

```rust
use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut unused: HashSet<Vec<u8>> = word_list.into_iter().map(String::into_bytes).collect();
        let end = end_word.into_bytes();
        if !unused.contains(&end) {
            return 0;
        }
        let mut queue = VecDeque::from([(begin_word.into_bytes(), 1)]);
        while let Some((mut word, len)) = queue.pop_front() {
            for i in 0..word.len() {
                let original = word[i];
                for c in b'a'..=b'z' {
                    if c == original {
                        continue;
                    }
                    word[i] = c;
                    if unused.remove(&word) {
                        if word == end {
                            return len + 1;
                        }
                        queue.push_back((word.clone(), len + 1));
                    }
                }
                word[i] = original;
            }
        }
        0
    }
}
```

Full tested solution: [`src/solutions/p0845_word_ladder.rs`](../src/solutions/p0845_word_ladder.rs)

</details>
