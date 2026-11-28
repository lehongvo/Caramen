# Problem 0711: Maximum Number of Vowels in a Substring of Given Length

- **Difficulty:** Medium
- **Topics:** fixed-size sliding window, `matches!`, `usize::from(bool)`, `zip`
- **Source:** [LeetCode 1456. Maximum Number of Vowels in a Substring of Given Length](https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/)

## Problem

You are given a string `s` of lowercase English letters and an integer `k`. Look at every
substring of `s` of length exactly `k` and return the largest number of vowels any of them
contains. The vowels are `'a'`, `'e'`, `'i'`, `'o'` and `'u'`.

Implement `Solution::max_vowels(s: String, k: i32) -> i32`.

## Examples

```text
Input:  s = "abciiidef", k = 3
Output: 3             ("iii")

Input:  s = "aeiou", k = 2
Output: 2

Input:  s = "leetcode", k = 3
Output: 2             ("lee", "eet" and "ode" each contain 2 vowels)
```

## Constraints

- `1 <= s.len() <= 10^5`
- `s` contains only lowercase English letters.
- `1 <= k <= s.len()`

## Hints

1. Count the vowels of the first window, then update the count as the window slides.
2. Each step only one character enters and one leaves.

## Starter code

Write your code in [`src/practice/p0711_maximum_number_of_vowels_in_a_substring_of_given_length.rs`](../src/practice/p0711_maximum_number_of_vowels_in_a_substring_of_given_length.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0711
```

```rust
pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
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
    fn example_three_vowels() {
        assert_eq!(Solution::max_vowels("abciiidef".to_string(), 3), 3);
    }

    #[test]
    fn example_only_vowels() {
        assert_eq!(Solution::max_vowels("aeiou".to_string(), 2), 2);
    }

    #[test]
    fn example_leetcode() {
        assert_eq!(Solution::max_vowels("leetcode".to_string(), 3), 2);
    }

    #[test]
    fn no_vowels() {
        assert_eq!(Solution::max_vowels("rhythms".to_string(), 4), 0);
    }

    #[test]
    fn best_window_at_the_end() {
        assert_eq!(Solution::max_vowels("tryhardou".to_string(), 3), 2);
    }

    #[test]
    fn largest_input() {
        let s = "a".repeat(100_000);
        assert_eq!(Solution::max_vowels(s, 100_000), 100_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** fixed-size sliding window: count the vowels in `s[..k]`. When the window moves one
step, the byte `s[i]` enters and `s[i - k]` leaves, so add one if the entering byte is a vowel
and subtract one if the leaving byte is. Track the best count. Adding before subtracting keeps
the unsigned counter from ever going below zero.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `matches!(b, b'a' | b'e' | ...)` tests a value against a pattern and returns a `bool`.
- `usize::from(true)` is `1` and `usize::from(false)` is `0`: a lossless, explicit conversion
  that avoids an `if`.
- `bytes[k..].iter().zip(bytes)` pairs each entering byte with the byte leaving the window.

```rust
pub struct Solution;

fn is_vowel(b: u8) -> bool {
    matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')
}

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let k = k as usize;
        let mut count = bytes[..k].iter().filter(|&&b| is_vowel(b)).count();
        let mut best = count;
        for (&entering, &leaving) in bytes[k..].iter().zip(bytes) {
            count += usize::from(is_vowel(entering));
            count -= usize::from(is_vowel(leaving));
            best = best.max(count);
        }
        best as i32
    }
}
```

Full tested solution: [`src/solutions/p0711_maximum_number_of_vowels_in_a_substring_of_given_length.rs`](../src/solutions/p0711_maximum_number_of_vowels_in_a_substring_of_given_length.rs)

</details>
