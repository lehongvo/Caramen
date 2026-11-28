# Problem 0824: Longest Common Subsequence

- **Difficulty:** Medium
- **Topics:** 2-D DP over two strings, `as_bytes`, two rolling rows with `std::mem::swap`
- **Source:** [LeetCode 1143. Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence/)

## Problem

Given two strings `text1` and `text2`, return the length of their longest **common
subsequence**: the longest string that is a subsequence of both (a subsequence deletes some
characters, possibly none, without changing the order of the rest). Return `0` if they share no
character.

Implement `Solution::longest_common_subsequence(text1: String, text2: String) -> i32`.

## Examples

```text
Input:  text1 = "abcde", text2 = "ace"
Output: 3
Explanation: "ace".

Input:  text1 = "abc", text2 = "abc"
Output: 3

Input:  text1 = "abc", text2 = "def"
Output: 0
```

## Constraints

- `1 <= text1.len(), text2.len() <= 1000`
- Both strings contain only lowercase English letters.

## Hints

1. Let `L(i, j)` be the answer for the prefixes `text1[..i]` and `text2[..j]`.
2. If the last characters match, `L(i, j) = L(i - 1, j - 1) + 1`; otherwise drop one of them:
   `max(L(i - 1, j), L(i, j - 1))`.
3. Row `i` only needs row `i - 1`.

## Starter code

Write your code in [`src/practice/p0824_longest_common_subsequence.rs`](../src/practice/p0824_longest_common_subsequence.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0824
```

```rust
pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
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

    fn lcs(a: &str, b: &str) -> i32 {
        Solution::longest_common_subsequence(a.to_string(), b.to_string())
    }

    #[test]
    fn example_ace() {
        assert_eq!(lcs("abcde", "ace"), 3);
    }

    #[test]
    fn example_equal_strings() {
        assert_eq!(lcs("abc", "abc"), 3);
    }

    #[test]
    fn example_nothing_in_common() {
        assert_eq!(lcs("abc", "def"), 0);
    }

    #[test]
    fn order_matters() {
        assert_eq!(lcs("abc", "cba"), 1);
        assert_eq!(lcs("bsbininm", "jmjkbkjkv"), 1);
    }

    #[test]
    fn long_inputs() {
        let a = "abcdefghij".repeat(100);
        let b = "jihgfedcba".repeat(100);
        assert_eq!(lcs(&a, &b), 199);
        assert_eq!(lcs(&a, &a), 1000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** fill the table row by row using two vectors `prev` (row `i - 1`) and `cur` (row `i`),
both of length `len2 + 1` with a zero in column 0. After each row, swap them.

**Complexity:** `O(len1 * len2)` time, `O(len2)` space.

**Rust concepts:**
- `as_bytes()` turns both strings into `&[u8]` so characters compare in `O(1)` by index.
- `std::mem::swap(&mut prev, &mut cur)` exchanges the two buffers without allocating.

```rust
pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (a, b) = (text1.as_bytes(), text2.as_bytes());
        let mut prev = vec![0_i32; b.len() + 1];
        let mut cur = vec![0_i32; b.len() + 1];
        for &x in a {
            for (j, &y) in b.iter().enumerate() {
                cur[j + 1] = if x == y {
                    prev[j] + 1
                } else {
                    prev[j + 1].max(cur[j])
                };
            }
            std::mem::swap(&mut prev, &mut cur);
        }
        prev[b.len()]
    }
}
```

Full tested solution: [`src/solutions/p0824_longest_common_subsequence.rs`](../src/solutions/p0824_longest_common_subsequence.rs)

</details>
