# Problem 0177: Lexicographically Smallest Palindrome

- **Difficulty:** Easy
- **Topics:** mirrored indices `i` and `n - 1 - i`, writing two positions of one `Vec<u8>`, `min`
- **Source:** [LeetCode 2697. Lexicographically Smallest Palindrome](https://leetcode.com/problems/lexicographically-smallest-palindrome/)

## Problem

You may replace any character of the lowercase string `s` with another lowercase letter; each
replacement counts as one operation. Turn `s` into a palindrome using the **minimum** number of
operations. If several palindromes need that minimum, return the lexicographically smallest one
(the one that comes first in dictionary order).

Implement `Solution::make_smallest_palindrome(s: String) -> String`.

## Examples

```text
Input:  s = "egcfe"
Output: "efcfe"          (one operation: 'g' -> 'f')

Input:  s = "abcd"
Output: "abba"           (two operations)

Input:  s = "seven"
Output: "neven"
```

## Constraints

- `1 <= s.len() <= 1000`
- `s` consists of lowercase English letters.

## Hints

1. Each pair of mirrored positions `(i, n - 1 - i)` must end up equal; if they differ, one
   operation is needed and enough.
2. To make the result as small as possible, set both to the **smaller** of the two letters.

## Starter code

Write your code in [`src/practice/p0177_lexicographically_smallest_palindrome.rs`](../src/practice/p0177_lexicographically_smallest_palindrome.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0177
```

```rust
pub struct Solution;

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
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

    fn run(s: &str) -> String {
        Solution::make_smallest_palindrome(s.to_string())
    }

    #[test]
    fn example_one_change() {
        assert_eq!(run("egcfe"), "efcfe");
    }

    #[test]
    fn example_two_changes() {
        assert_eq!(run("abcd"), "abba");
    }

    #[test]
    fn example_seven() {
        assert_eq!(run("seven"), "neven");
    }

    #[test]
    fn already_palindrome_and_single_letter() {
        assert_eq!(run("racecar"), "racecar");
        assert_eq!(run("q"), "q");
    }

    #[test]
    fn longest_input() {
        let s = format!("{}{}", "z".repeat(500), "a".repeat(500));
        assert_eq!(run(&s), "a".repeat(1000));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** for every mirrored pair, write the smaller letter to both positions. Pairs that already
match are unchanged, mismatched pairs cost exactly one operation, and choosing the minimum letter
makes the result lexicographically smallest.

**Complexity:** `O(n)` time, `O(1)` extra space (the input buffer is reused).

**Rust concepts:**
- `into_bytes()` gives a mutable byte buffer; reading `bytes[i]` copies a `u8`, so no borrow is
  held when writing the two positions afterwards.
- `Ord::min` works on `u8`; lowercase ASCII letters compare in alphabetical order.
- Only ASCII bytes are written, so `String::from_utf8` cannot fail.

```rust
pub struct Solution;

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut bytes = s.into_bytes();
        let n = bytes.len();
        for i in 0..n / 2 {
            let smaller = bytes[i].min(bytes[n - 1 - i]);
            bytes[i] = smaller;
            bytes[n - 1 - i] = smaller;
        }
        String::from_utf8(bytes).expect("only ASCII letters were written")
    }
}
```

Full tested solution: [`src/solutions/p0177_lexicographically_smallest_palindrome.rs`](../src/solutions/p0177_lexicographically_smallest_palindrome.rs)

</details>
