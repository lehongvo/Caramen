# Problem 0701: Valid Palindrome II

- **Difficulty:** Easy
- **Topics:** two pointers, byte slices `&[u8]`, half-open ranges, `Iterator::eq`
- **Source:** [LeetCode 680. Valid Palindrome II](https://leetcode.com/problems/valid-palindrome-ii/)

## Problem

You are given a string `s` made of lowercase English letters. Decide whether `s` can be turned
into a palindrome (a string that reads the same from left to right and from right to left) by
deleting **at most one** character. Deleting nothing is allowed, so a string that is already a
palindrome is a valid answer.

Return `true` if it is possible and `false` otherwise.

Implement `Solution::valid_palindrome(s: String) -> bool`.

## Examples

```text
Input:  s = "aba"
Output: true          (already a palindrome)

Input:  s = "abca"
Output: true          (delete 'c' to get "aba", or 'b' to get "aca")

Input:  s = "abc"
Output: false         (no single deletion gives a palindrome)
```

## Constraints

- `1 <= s.len() <= 10^5`
- `s` contains only lowercase English letters.

## Hints

1. Compare the two ends and walk inwards while they match (two pointers).
2. At the first mismatch you must delete either the left or the right character. You do not
   know which one, so try both.
3. After that one deletion, the remaining middle part must be a palindrome with no more deletions.

## Starter code

Write your code in [`src/practice/p0701_valid_palindrome_ii.rs`](../src/practice/p0701_valid_palindrome_ii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0701
```

```rust
pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
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

    #[test]
    fn example_already_palindrome() {
        assert!(Solution::valid_palindrome("aba".to_string()));
    }

    #[test]
    fn example_one_deletion() {
        assert!(Solution::valid_palindrome("abca".to_string()));
    }

    #[test]
    fn example_impossible() {
        assert!(!Solution::valid_palindrome("abc".to_string()));
    }

    #[test]
    fn single_character() {
        assert!(Solution::valid_palindrome("a".to_string()));
    }

    #[test]
    fn only_the_right_deletion_works() {
        assert!(Solution::valid_palindrome("cbbcc".to_string()));
    }

    #[test]
    fn only_the_left_deletion_works() {
        assert!(Solution::valid_palindrome("ccbbc".to_string()));
    }

    #[test]
    fn two_deletions_needed() {
        assert!(!Solution::valid_palindrome("abcda".to_string()));
    }

    #[test]
    fn long_string_with_one_extra_letter() {
        let s = format!("{}b{}", "a".repeat(50_000), "a".repeat(49_999));
        assert!(Solution::valid_palindrome(s));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** two pointers: keep a half-open window `lo..hi` over the bytes. While the outer
characters match, shrink the window from both sides; they cannot be part of the problem. At the
first mismatch, one of the two characters `s[lo]` or `s[hi - 1]` has to go, so the answer is
`true` exactly when `s[lo + 1..hi]` or `s[lo..hi - 1]` is a palindrome. Checking only one side is
a classic bug: for `"cbbcc"` deleting the left `'b'` fails but deleting the right `'c'` works.

**Complexity:** `O(n)` time (each side check is `O(n)` and happens at most once), `O(1)` space.

**Rust concepts:**
- `s.as_bytes()` gives a `&[u8]` view: indexing bytes is `O(1)`, unlike indexing a `String`,
  which Rust does not allow because UTF-8 characters can span several bytes.
- A half-open window `lo..hi` avoids the `len() - 1` underflow problem of inclusive bounds.
- `a.iter().eq(a.iter().rev())` compares two iterators element by element, which is a neat way
  to test a palindrome without allocating a reversed copy.

```rust
pub struct Solution;

/// Returns `true` if the bytes read the same in both directions.
fn is_palindrome(bytes: &[u8]) -> bool {
    bytes.iter().eq(bytes.iter().rev())
}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let (mut lo, mut hi) = (0, bytes.len());
        while lo + 1 < hi {
            if bytes[lo] != bytes[hi - 1] {
                // Delete the left character or the right one; the rest must then be a palindrome.
                return is_palindrome(&bytes[lo + 1..hi]) || is_palindrome(&bytes[lo..hi - 1]);
            }
            lo += 1;
            hi -= 1;
        }
        true
    }
}
```

Full tested solution: [`src/solutions/p0701_valid_palindrome_ii.rs`](../src/solutions/p0701_valid_palindrome_ii.rs)

</details>
