# Problem 0172: Valid Palindrome

- **Difficulty:** Easy
- **Topics:** two indices over `as_bytes()`, `is_ascii_alphanumeric`, `eq_ignore_ascii_case`
- **Source:** [LeetCode 125. Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)

## Problem

A phrase is a palindrome if, after converting all uppercase letters to lowercase and removing
every character that is not a letter or a digit, it reads the same forwards and backwards.

Implement `Solution::is_palindrome(s: String) -> bool`. An empty phrase (or one with no letters
or digits) counts as a palindrome.

Try to do it without building a cleaned-up copy of the string.

## Examples

```text
Input:  s = "A man, a plan, a canal: Panama"
Output: true            ("amanaplanacanalpanama")

Input:  s = "race a car"
Output: false           ("raceacar")

Input:  s = " "
Output: true
```

## Constraints

- `1 <= s.len() <= 2 * 10^5`
- `s` consists only of printable ASCII characters.

## Hints

1. Use an index `i` from the left and an index `j` from the right; skip characters that are not
   alphanumeric on either side.
2. Compare the two bytes with `u8::eq_ignore_ascii_case`.
3. Keeping `j` exclusive (`b[j - 1]` is the right character) avoids `usize` underflow.

## Starter code

Write your code in [`src/practice/p0172_valid_palindrome.rs`](../src/practice/p0172_valid_palindrome.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0172
```

```rust
pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
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

    fn check(s: &str) -> bool {
        Solution::is_palindrome(s.to_string())
    }

    #[test]
    fn example_panama() {
        assert!(check("A man, a plan, a canal: Panama"));
    }

    #[test]
    fn example_race_a_car() {
        assert!(!check("race a car"));
    }

    #[test]
    fn example_only_space() {
        assert!(check(" "));
    }

    #[test]
    fn digits_matter() {
        assert!(!check("0P"));
        assert!(check("1a2, 2A1"));
    }

    #[test]
    fn only_punctuation() {
        assert!(check(".,!?"));
    }

    #[test]
    fn large_input() {
        let half = "Ab1, ".repeat(19_999);
        let mirrored: String = half.chars().rev().collect();
        assert!(check(&format!("{half}{mirrored}")));
        assert!(!check(&format!("{half}x{mirrored}y")));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** move two indices towards each other. If the left byte is not alphanumeric, skip it; same
on the right; otherwise compare them ignoring case and stop at the first mismatch. Meeting in the
middle means every pair matched.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `as_bytes()` borrows the bytes without copying; for ASCII input each byte is one character.
- `u8::is_ascii_alphanumeric` and `u8::eq_ignore_ascii_case` work directly on bytes.
- A cleaned copy (`s.bytes().filter(..).map(..).collect::<Vec<u8>>()`) is simpler but costs
  `O(n)` extra memory.

```rust
pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let b = s.as_bytes();
        let (mut i, mut j) = (0, b.len());
        while i < j {
            if !b[i].is_ascii_alphanumeric() {
                i += 1;
            } else if !b[j - 1].is_ascii_alphanumeric() {
                j -= 1;
            } else if !b[i].eq_ignore_ascii_case(&b[j - 1]) {
                return false;
            } else {
                i += 1;
                j -= 1;
            }
        }
        true
    }
}
```

Full tested solution: [`src/solutions/p0172_valid_palindrome.rs`](../src/solutions/p0172_valid_palindrome.rs)

</details>
