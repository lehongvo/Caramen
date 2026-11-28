# Problem 0200: Longest Palindromic Substring

- **Difficulty:** Medium
- **Topics:** expanding around centers with byte indices, a helper taking `&[u8]`, slicing the answer out of the input
- **Source:** [LeetCode 5. Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/)

## Problem

Return the longest substring of `s` that is a palindrome (reads the same forwards and backwards).
If several substrings have the maximum length, any of them is accepted.

Implement `Solution::longest_palindrome(s: String) -> String`.

## Examples

```text
Input:  s = "babad"
Output: "bab"            ("aba" is also accepted)

Input:  s = "cbbd"
Output: "bb"

Input:  s = "a"
Output: "a"
```

## Constraints

- `1 <= s.len() <= 1000`
- `s` consists of English letters and digits.

## Hints

1. Every palindrome has a center: a single character (odd length) or the gap between two
   characters (even length). There are `2n - 1` centers.
2. From each center, expand outwards while the two ends match.
3. Represent a palindrome as a half-open byte range `start..end`; expanding means `start -= 1` and
   `end += 1` while `start > 0`, `end < n` and `b[start - 1] == b[end]`.

## Starter code

Write your code in [`src/practice/p0200_longest_palindromic_substring.rs`](../src/practice/p0200_longest_palindromic_substring.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0200
```

```rust
pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
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
        Solution::longest_palindrome(s.to_string())
    }

    fn is_palindrome(s: &str) -> bool {
        s.bytes().eq(s.bytes().rev())
    }

    #[test]
    fn example_babad() {
        let result = run("babad");
        assert!(result == "bab" || result == "aba", "{result}");
    }

    #[test]
    fn example_even_length() {
        assert_eq!(run("cbbd"), "bb");
    }

    #[test]
    fn example_single_char() {
        assert_eq!(run("a"), "a");
    }

    #[test]
    fn whole_string_and_no_repeats() {
        assert_eq!(run("racecar"), "racecar");
        assert_eq!(run("abba"), "abba");
        let result = run("abc");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn palindrome_at_the_end() {
        assert_eq!(run("xyz12321"), "12321");
    }

    #[test]
    fn longest_input() {
        // Longest candidates: 199 chars inside "abab...", 500 x's, 299 chars inside "cdcd...".
        // The x's cannot grow because their neighbours ('b' and 'c') differ.
        let s = format!(
            "{}{}{}",
            "ab".repeat(100),
            "x".repeat(500),
            "cd".repeat(150)
        );
        let result = run(&s);
        assert!(is_palindrome(&result));
        assert_eq!(result, "x".repeat(500));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** for every index, expand both an odd-length candidate (starting as the single character)
and an even-length candidate (starting empty, between two characters). Keep the widest range seen.
The answer is that range sliced out of `s`.

**Complexity:** `O(n^2)` time, `O(1)` extra space. (Manacher's algorithm reaches `O(n)`.)

**Rust concepts:**
- The helper `expand(b: &[u8], start, end)` only reads bytes, so it takes a slice; the input is
  ASCII, so byte ranges are character ranges.
- Half-open ranges with the `start > 0` check keep every index a valid `usize` (no `-1`).
- `s[start..end].to_string()` copies just the answer. Since `s` is owned, `s.truncate(end)`
  followed by `s.replace_range(..start, "")` would reuse its buffer instead.

```rust
/// Grows the palindrome `b[start..end]` while the characters around it match.
fn expand(b: &[u8], mut start: usize, mut end: usize) -> (usize, usize) {
    while start > 0 && end < b.len() && b[start - 1] == b[end] {
        start -= 1;
        end += 1;
    }
    (start, end)
}

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let b = s.as_bytes();
        let mut best = (0, 0);
        for center in 0..b.len() {
            for (start, end) in [expand(b, center, center + 1), expand(b, center, center)] {
                if end - start > best.1 - best.0 {
                    best = (start, end);
                }
            }
        }
        s[best.0..best.1].to_string()
    }
}
```

Full tested solution: [`src/solutions/p0200_longest_palindromic_substring.rs`](../src/solutions/p0200_longest_palindromic_substring.rs)

</details>
