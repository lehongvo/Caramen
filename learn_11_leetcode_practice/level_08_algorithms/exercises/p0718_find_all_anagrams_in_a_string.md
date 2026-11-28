# Problem 0718: Find All Anagrams in a String

- **Difficulty:** Medium
- **Topics:** fixed-size sliding window, letter counts, collecting window start indices
- **Source:** [LeetCode 438. Find All Anagrams in a String](https://leetcode.com/problems/find-all-anagrams-in-a-string/)

## Problem

Given two strings `s` and `p` of lowercase English letters, return the start indices of every
substring of `s` that is an **anagram** of `p` (uses the same letters with the same counts, in
any order). The indices may be returned in any order. Return an empty vector if there are none.

Implement `Solution::find_anagrams(s: String, p: String) -> Vec<i32>`.

## Examples

```text
Input:  s = "cbaebabacd", p = "abc"
Output: [0, 6]        ("cba" at 0 and "bac" at 6)

Input:  s = "abab", p = "ab"
Output: [0, 1, 2]     ("ab", "ba", "ab")
```

## Constraints

- `1 <= s.len(), p.len() <= 3 * 10^4`
- Both strings contain only lowercase English letters.

## Hints

1. This is the previous problem, but instead of stopping at the first match you record every one.
2. When the window ends at index `i`, it starts at `i + 1 - p.len()`.

## Starter code

Write your code in [`src/practice/p0718_find_all_anagrams_in_a_string.rs`](../src/practice/p0718_find_all_anagrams_in_a_string.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0718
```

```rust
pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
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

    fn run(s: &str, p: &str) -> Vec<i32> {
        let mut result = Solution::find_anagrams(s.to_string(), p.to_string());
        result.sort_unstable(); // any order is accepted
        result
    }

    #[test]
    fn example_two_matches() {
        assert_eq!(run("cbaebabacd", "abc"), vec![0, 6]);
    }

    #[test]
    fn example_overlapping_matches() {
        assert_eq!(run("abab", "ab"), vec![0, 1, 2]);
    }

    #[test]
    fn pattern_longer_than_text() {
        assert!(run("a", "ab").is_empty());
    }

    #[test]
    fn single_letter_pattern() {
        assert_eq!(run("aaaa", "a"), vec![0, 1, 2, 3]);
    }

    #[test]
    fn no_match() {
        assert!(run("abcdefg", "hij").is_empty());
    }

    #[test]
    fn long_text() {
        let s = "ab".repeat(15_000);
        let result = run(&s, "ba");
        assert_eq!(result.len(), 29_999);
        assert_eq!(result[29_998], 29_998);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** fixed-size sliding window: keep the letter counts of the last `p.len()` letters of
`s`. After adding `s[i]` (and removing `s[i - m]` once the window is full), the window covers
`i + 1 - m ..= i`; if its counts equal the counts of `p`, record its start.

**Complexity:** `O(26 * n)` = `O(n)` time, `O(1)` extra space besides the output.

**Rust concepts:**
- `i + 1 >= m` is the overflow-free way to write "the window is full" with unsigned indices
  (`i - m + 1 >= 0` would underflow).
- Arrays compare with `==` element by element.

```rust
pub struct Solution;

fn letter(b: u8) -> usize {
    usize::from(b - b'a')
}

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (text, pattern) = (s.as_bytes(), p.as_bytes());
        let m = pattern.len();
        let mut need = [0u32; 26];
        for &b in pattern {
            need[letter(b)] += 1;
        }
        let mut window = [0u32; 26];
        let mut starts = Vec::new();
        for (i, &b) in text.iter().enumerate() {
            window[letter(b)] += 1;
            if i >= m {
                window[letter(text[i - m])] -= 1;
            }
            if i + 1 >= m && window == need {
                starts.push((i + 1 - m) as i32);
            }
        }
        starts
    }
}
```

Full tested solution: [`src/solutions/p0718_find_all_anagrams_in_a_string.rs`](../src/solutions/p0718_find_all_anagrams_in_a_string.rs)

</details>
