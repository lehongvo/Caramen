# Problem 0474: License Key Formatting

- **Difficulty:** Easy
- **Topics:** `filter` + `map` on `chars`, `slice::rchunks`, `rev`, `join`
- **Source:** [LeetCode 482. License Key Formatting](https://leetcode.com/problems/license-key-formatting/)

## Problem

A license key `s` contains letters, digits and dashes `'-'` in arbitrary places. Reformat it:

- remove all dashes and convert letters to upper case;
- split the remaining characters into groups of exactly `k` characters, **counting from the
  end**, so only the first group may be shorter (but it is never empty);
- join the groups with single dashes.

Implement `Solution::license_key_formatting(s: String, k: i32) -> String`.

## Examples

```text
Input:  s = "5F3Z-2e-9-w", k = 4
Output: "5F3Z-2E9W"

Input:  s = "2-5g-3-J", k = 2
Output: "2-5G-3J"
Explanation: 5 characters "25G3J" grouped by 2 from the end: "2", "5G", "3J".

Input:  s = "---", k = 3
Output: ""
```

## Constraints

- `1 <= s.len() <= 10^5`
- `s` contains English letters, digits and dashes.
- `1 <= k <= 10^4`

## Hints

1. `s.chars().filter(|&c| c != '-').map(|c| c.to_ascii_uppercase())` cleans the key.
2. `slice::rchunks(k)` splits a slice into chunks of `k` starting from the end (the last chunk it
   yields is the short one at the front). Reverse the chunks to restore the order.

## Starter code

Write your code in [`src/practice/p0474_license_key_formatting.rs`](../src/practice/p0474_license_key_formatting.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0474
```

```rust
pub struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
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

    fn run(s: &str, k: i32) -> String {
        Solution::license_key_formatting(s.to_string(), k)
    }

    #[test]
    fn example_groups_of_four() {
        assert_eq!(run("5F3Z-2e-9-w", 4), "5F3Z-2E9W");
    }

    #[test]
    fn example_short_first_group() {
        assert_eq!(run("2-5g-3-J", 2), "2-5G-3J");
    }

    #[test]
    fn example_only_dashes() {
        assert_eq!(run("---", 3), "");
    }

    #[test]
    fn other_cases() {
        assert_eq!(run("a", 1), "A");
        assert_eq!(run("abcd", 10), "ABCD");
        assert_eq!(run("-a-b-c-d-", 1), "A-B-C-D");
        let long = "x".repeat(100_000);
        let out = run(&long, 3);
        assert_eq!(out.len(), 100_000 + 33_333);
        assert!(out.starts_with("X-XXX"));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** clean the characters into a `Vec<char>`, cut it into groups from the end with
`rchunks`, put the groups back in reading order with `rev`, and join them.

**Complexity:** `O(n)` time and space.

**Rust concepts:**
- `rchunks(k)` is the mirror of `chunks(k)`: full chunks are taken from the end, so the leftover
  short chunk is the first group of the key, exactly as required.
- `Rchunks` is a double-ended iterator, so `.rev()` is free.
- `chunk.iter().collect::<String>()` turns a `&[char]` into a `String`; `join("-")` on a
  `Vec<String>` inserts the separators.

```rust
pub struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let chars: Vec<char> = s
            .chars()
            .filter(|&c| c != '-')
            .map(|c| c.to_ascii_uppercase())
            .collect();
        let groups: Vec<String> = chars
            .rchunks(k as usize)
            .rev()
            .map(|chunk| chunk.iter().collect())
            .collect();
        groups.join("-")
    }
}
```

Full tested solution: [`src/solutions/p0474_license_key_formatting.rs`](../src/solutions/p0474_license_key_formatting.rs)

</details>
