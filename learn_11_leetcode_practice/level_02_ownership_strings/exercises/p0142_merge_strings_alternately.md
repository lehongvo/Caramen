# Problem 0142: Merge Strings Alternately

- **Difficulty:** Easy
- **Topics:** two `Chars` iterators, `String::with_capacity`, `String: Extend<char>`, `Option` as an iterator
- **Source:** [LeetCode 1768. Merge Strings Alternately](https://leetcode.com/problems/merge-strings-alternately/)

## Problem

Merge two words by taking characters alternately: first character of `word1`, first character of
`word2`, second character of `word1`, second character of `word2`, and so on. When one word runs
out of characters, append the rest of the other word unchanged.

Implement `Solution::merge_alternately(word1: String, word2: String) -> String`.

## Examples

```text
Input:  word1 = "abc", word2 = "pqr"
Output: "apbqcr"

Input:  word1 = "ab", word2 = "pqrs"
Output: "apbqrs"

Input:  word1 = "abcd", word2 = "pq"
Output: "apbqcd"
```

## Constraints

- `1 <= word1.len(), word2.len() <= 100`
- Both words consist of lowercase English letters.

## Hints

1. Create two iterators, `word1.chars()` and `word2.chars()`, and call `next()` on each in turn.
2. Stop when **both** return `None`.
3. `String` implements `Extend<char>`, and an `Option<char>` is an iterator of zero or one `char`,
   so `out.extend(a.next())` pushes the character only if there is one.

## Starter code

Write your code in [`src/practice/p0142_merge_strings_alternately.rs`](../src/practice/p0142_merge_strings_alternately.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0142
```

```rust
pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
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

    fn merge(a: &str, b: &str) -> String {
        Solution::merge_alternately(a.to_string(), b.to_string())
    }

    #[test]
    fn example_equal_lengths() {
        assert_eq!(merge("abc", "pqr"), "apbqcr");
    }

    #[test]
    fn example_second_longer() {
        assert_eq!(merge("ab", "pqrs"), "apbqrs");
    }

    #[test]
    fn example_first_longer() {
        assert_eq!(merge("abcd", "pq"), "apbqcd");
    }

    #[test]
    fn single_letters() {
        assert_eq!(merge("a", "z"), "az");
        assert_eq!(merge("a", "xyz"), "axyz");
    }

    #[test]
    fn longest_inputs() {
        let result = merge(&"a".repeat(100), &"b".repeat(50));
        assert_eq!(result.len(), 150);
        assert_eq!(&result[..4], "abab");
        assert_eq!(&result[100..], "a".repeat(50));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** advance both iterators in lock step. Each round pushes whatever each iterator produced
(maybe nothing). The loop ends when both are exhausted, which automatically appends the tail of
the longer word.

**Complexity:** `O(n + m)` time and space.

**Rust concepts:**
- `chars()` borrows the string and decodes it lazily; two iterators over two different strings
  can be advanced independently.
- Matching on a tuple `(a.next(), b.next())` handles the four combinations in one place.
- `Option<T>` implements `IntoIterator`, so `String::extend(option)` appends 0 or 1 `char`.

```rust
pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut out = String::with_capacity(word1.len() + word2.len());
        let (mut a, mut b) = (word1.chars(), word2.chars());
        loop {
            match (a.next(), b.next()) {
                (None, None) => break,
                (x, y) => {
                    out.extend(x);
                    out.extend(y);
                }
            }
        }
        out
    }
}
```

Full tested solution: [`src/solutions/p0142_merge_strings_alternately.rs`](../src/solutions/p0142_merge_strings_alternately.rs)

</details>
