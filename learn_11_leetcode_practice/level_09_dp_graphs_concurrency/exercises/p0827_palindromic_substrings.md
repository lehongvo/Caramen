# Problem 0827: Palindromic Substrings

- **Difficulty:** Medium
- **Topics:** expand around centers (a space-free alternative to the `O(n^2)` DP table), `sum`
- **Source:** [LeetCode 647. Palindromic Substrings](https://leetcode.com/problems/palindromic-substrings/)

## Problem

Given a string `s`, return how many of its substrings are palindromes (read the same forwards
and backwards). Substrings at different positions count separately, even if they are equal
strings. Every single character is a palindrome.

Implement `Solution::count_substrings(s: String) -> i32`.

## Examples

```text
Input:  s = "abc"
Output: 3
Explanation: "a", "b", "c".

Input:  s = "aaa"
Output: 6
Explanation: "a" three times, "aa" twice, "aaa" once.
```

## Constraints

- `1 <= s.len() <= 1000`
- `s` contains only lowercase English letters.

## Hints

1. The DP view: `s[i..=j]` is a palindrome if `s[i] == s[j]` and `s[i + 1..j]` is one.
2. Equivalent and lighter: every palindrome has a center, either a character (odd length) or the
   gap between two characters (even length). There are `2n - 1` centers.
3. From each center, grow outwards while the two ends match; each step is one more palindrome.

## Starter code

Write your code in [`src/practice/p0827_palindromic_substrings.rs`](../src/practice/p0827_palindromic_substrings.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0827
```

```rust
pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
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

    fn count(s: &str) -> i32 {
        Solution::count_substrings(s.to_string())
    }

    #[test]
    fn example_distinct_letters() {
        assert_eq!(count("abc"), 3);
    }

    #[test]
    fn example_repeated_letter() {
        assert_eq!(count("aaa"), 6);
    }

    #[test]
    fn single_character() {
        assert_eq!(count("z"), 1);
    }

    #[test]
    fn even_length_palindromes() {
        // a, b, b, a, bb, abba
        assert_eq!(count("abba"), 6);
    }

    #[test]
    fn longest_input_all_equal() {
        // Every one of the 1000 * 1001 / 2 substrings is a palindrome.
        assert_eq!(count(&"a".repeat(1000)), 500_500);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** number the `2n - 1` centers `c = 0..2n - 1`; center `c` starts with `left = c / 2`
and `right = left + c % 2`. Expand while `left` and `right` are in range and the bytes match,
counting one palindrome per successful step.

**Complexity:** `O(n^2)` time in the worst case (`"aaaa..."`), `O(1)` space.

**Rust concepts:**
- Moving `left` downwards in `usize` would underflow at `0`; `checked_sub` returns `None`
  there and ends the loop cleanly.
- `(0..2 * n - 1).map(...).sum::<i32>()` adds up the per-center counts.

```rust
pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        (0..2 * n - 1)
            .map(|c| {
                let (mut left, mut right) = (c / 2, c / 2 + c % 2);
                let mut count = 0;
                while right < n && b[left] == b[right] {
                    count += 1;
                    match left.checked_sub(1) {
                        Some(l) => left = l,
                        None => break,
                    }
                    right += 1;
                }
                count
            })
            .sum()
    }
}
```

Full tested solution: [`src/solutions/p0827_palindromic_substrings.rs`](../src/solutions/p0827_palindromic_substrings.rs)

</details>
