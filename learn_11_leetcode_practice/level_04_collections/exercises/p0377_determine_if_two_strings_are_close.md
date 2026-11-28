# Problem 0377: Determine if Two Strings Are Close

- **Difficulty:** Medium
- **Topics:** count arrays `[usize; 26]`, comparing key sets and sorted counts, reasoning about invariants
- **Source:** [LeetCode 1657. Determine if Two Strings Are Close](https://leetcode.com/problems/determine-if-two-strings-are-close/)

## Problem

Two strings are **close** if one can be turned into the other using these operations any
number of times:

- **Operation 1:** swap any two characters (for example `"abcde" -> "aecdb"`);
- **Operation 2:** pick two letters that both occur, and exchange **all** their occurrences
  (for example `"aacabb" -> "bbcbaa"`: every `a` becomes `b` and every `b` becomes `a`).

Given `word1` and `word2`, return `true` if they are close.

Implement `Solution::close_strings(word1: String, word2: String) -> bool`.

## Examples

```text
Input:  word1 = "abc", word2 = "bca"
Output: true      (operation 1 twice)

Input:  word1 = "a", word2 = "aa"
Output: false     (the lengths can never change)

Input:  word1 = "cabbba", word2 = "abbccc"
Output: true      (counts {a:2, b:3, c:1} vs {a:1, b:2, c:3})
```

## Constraints

- `1 <= word1.len(), word2.len() <= 10^5`
- Both contain only lowercase English letters.

## Hints

1. Operation 1 means the order of the characters does not matter, only their counts.
2. Operation 2 permutes the counts among letters that are present, but it can never introduce a
   letter that does not occur.
3. So: the two words must use the same **set** of letters, and the same **multiset** of counts.

## Starter code

Write your code in [`src/practice/p0377_determine_if_two_strings_are_close.rs`](../src/practice/p0377_determine_if_two_strings_are_close.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0377
```

```rust
pub struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
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

    fn close(a: &str, b: &str) -> bool {
        Solution::close_strings(a.to_string(), b.to_string())
    }

    #[test]
    fn example_one() {
        assert!(close("abc", "bca"));
    }

    #[test]
    fn example_two_lengths_differ() {
        assert!(!close("a", "aa"));
    }

    #[test]
    fn example_three() {
        assert!(close("cabbba", "abbccc"));
    }

    #[test]
    fn same_counts_but_different_letters() {
        // Counts {a:1, b:2} vs {a:1, c:2}: 'c' cannot appear from nothing.
        assert!(!close("abb", "acc"));
        assert!(!close("uau", "ssx"));
    }

    #[test]
    fn large_words() {
        let w1 = format!("{}{}", "a".repeat(60_000), "b".repeat(40_000));
        let w2 = format!("{}{}", "b".repeat(60_000), "a".repeat(40_000));
        assert!(Solution::close_strings(w1.clone(), w2));
        let w3 = format!("{}{}", "a".repeat(50_000), "b".repeat(50_000));
        assert!(!Solution::close_strings(w1, w3));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** count the letters of both words. They are close exactly when (a) the same letters
occur in both (a count is zero in one word iff it is zero in the other) and (b) the sorted lists
of counts are equal.

**Complexity:** `O(n + m)` time, `O(1)` space (arrays of 26 counts).

**Rust concepts:**
- With a small, fixed alphabet, `[usize; 26]` arrays are the fastest "maps"; arrays of the same
  length compare with `==` and can be sorted in place.
- `a.iter().zip(&b).all(|(x, y)| (*x == 0) == (*y == 0))` compares the key sets.
- Arrays are `Copy` when their elements are, so `let mut sorted_a = a;` copies the counts before
  sorting them.

```rust
pub struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        fn counts(word: &str) -> [usize; 26] {
            let mut c = [0; 26];
            for b in word.bytes() {
                c[(b - b'a') as usize] += 1;
            }
            c
        }
        let (a, b) = (counts(&word1), counts(&word2));
        let same_letters = a.iter().zip(&b).all(|(x, y)| (*x == 0) == (*y == 0));
        let (mut sorted_a, mut sorted_b) = (a, b);
        sorted_a.sort_unstable();
        sorted_b.sort_unstable();
        same_letters && sorted_a == sorted_b
    }
}
```

Full tested solution: [`src/solutions/p0377_determine_if_two_strings_are_close.rs`](../src/solutions/p0377_determine_if_two_strings_are_close.rs)

</details>
