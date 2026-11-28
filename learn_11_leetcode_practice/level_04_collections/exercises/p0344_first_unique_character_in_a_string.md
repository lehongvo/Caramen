# Problem 0344: First Unique Character in a String

- **Difficulty:** Easy
- **Topics:** two passes: count with a `HashMap`, then scan in the original order; `position`
- **Source:** [LeetCode 387. First Unique Character in a String](https://leetcode.com/problems/first-unique-character-in-a-string/)

## Problem

Given a string `s`, find the first character that occurs **exactly once** in `s` and return its
index. If every character repeats, return `-1`.

Implement `Solution::first_uniq_char(s: String) -> i32`.

## Examples

```text
Input:  s = "leetcode"
Output: 0       ('l' occurs once)

Input:  s = "loveleetcode"
Output: 2       ('l' and 'o' repeat; 'v' at index 2 is the first unique one)

Input:  s = "aabb"
Output: -1
```

## Constraints

- `1 <= s.len() <= 10^5`
- `s` contains only lowercase English letters.

## Hints

1. You cannot know whether a character is unique before you have seen the whole string, so
   count first.
2. A `HashMap` has no order. Get the "first" by scanning the **string** again, not the map.

## Starter code

Write your code in [`src/practice/p0344_first_unique_character_in_a_string.rs`](../src/practice/p0344_first_unique_character_in_a_string.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0344
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
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
    fn example_one() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }

    #[test]
    fn example_three_none() {
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
    }

    #[test]
    fn single_and_last() {
        assert_eq!(Solution::first_uniq_char("z".to_string()), 0);
        assert_eq!(Solution::first_uniq_char("aabbc".to_string()), 4);
    }

    #[test]
    fn large_string_unique_at_the_end() {
        let s = format!("{}q", "abcdefghijklmnoprstuvwxyz".repeat(3_999));
        assert_eq!(Solution::first_uniq_char(s), 99_975);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** first pass: count every byte. Second pass over the string, in order: the first byte
whose count is `1` is the answer. Iterating the map instead would give an arbitrary order.

**Complexity:** `O(n)` average time, `O(1)` space (at most 26 keys).

**Rust concepts:**
- The input is ASCII, so bytes and characters coincide and `bytes().position(...)` returns the
  index LeetCode expects. (For general UTF-8, use `char_indices` to get byte offsets.)
- `counts[&b]` indexes a `HashMap` and **panics** if the key is missing; here every byte of `s`
  was counted in the first pass, so the key always exists.
- `map_or(-1, |i| i as i32)` converts the `Option<usize>` into LeetCode's `-1` convention.

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counts: HashMap<u8, usize> = HashMap::new();
        for b in s.bytes() {
            *counts.entry(b).or_insert(0) += 1;
        }
        // Every byte of `s` is a key of `counts`, so indexing cannot panic.
        s.bytes()
            .position(|b| counts[&b] == 1)
            .map_or(-1, |i| i as i32)
    }
}
```

Full tested solution: [`src/solutions/p0344_first_unique_character_in_a_string.rs`](../src/solutions/p0344_first_unique_character_in_a_string.rs)

</details>
