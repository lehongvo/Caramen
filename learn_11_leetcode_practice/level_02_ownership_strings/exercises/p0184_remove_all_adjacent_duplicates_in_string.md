# Problem 0184: Remove All Adjacent Duplicates In String

- **Difficulty:** Easy
- **Topics:** a `String` used as a stack (`push`, `pop`, `ends_with`)
- **Source:** [LeetCode 1047. Remove All Adjacent Duplicates In String](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/)

## Problem

A *duplicate removal* deletes two adjacent, equal letters. Starting from the lowercase string `s`,
keep applying duplicate removals until none is possible, and return the final string. The answer
does not depend on the order of the removals.

Implement `Solution::remove_duplicates(s: String) -> String`.

## Examples

```text
Input:  s = "abbaca"
Output: "ca"
Explanation: remove "bb" to get "aaca", then remove "aa" to get "ca".

Input:  s = "azxxzy"
Output: "ay"
```

## Constraints

- `1 <= s.len() <= 10^5`
- `s` consists of lowercase English letters.

## Hints

1. Process the letters from left to right and keep the result built so far.
2. If the next letter equals the last letter of the result, the two cancel out: `pop()` it.
   Otherwise `push()` the letter.

## Starter code

Write your code in [`src/practice/p0184_remove_all_adjacent_duplicates_in_string.rs`](../src/practice/p0184_remove_all_adjacent_duplicates_in_string.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0184
```

```rust
pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
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
        Solution::remove_duplicates(s.to_string())
    }

    #[test]
    fn example_abbaca() {
        assert_eq!(run("abbaca"), "ca");
    }

    #[test]
    fn example_azxxzy() {
        assert_eq!(run("azxxzy"), "ay");
    }

    #[test]
    fn everything_cancels() {
        assert_eq!(run("aa"), "");
        assert_eq!(run("abccba"), "");
    }

    #[test]
    fn odd_run_leaves_one() {
        assert_eq!(run("aaa"), "a");
        assert_eq!(run("x"), "x");
    }

    #[test]
    fn large_input() {
        let s = format!("{}{}", "ab".repeat(25_000), "ba".repeat(25_000));
        assert_eq!(run(&s), "");
        assert_eq!(run(&"abc".repeat(30_000)).len(), 90_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the output `String` behaves like a stack. Every new letter either cancels the letter on
top (they were adjacent once everything between them disappeared) or is pushed. One pass gives
the fully reduced string.

**Complexity:** `O(n)` time and space.

**Rust concepts:**
- `String::push` and `String::pop` work at the end in amortized `O(1)`, like `Vec::push` / `pop`.
- `ends_with(c)` checks the top of the stack without removing it.
- `pop()` returns `Option<char>`; the value is not needed here, only the removal.

```rust
pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = String::with_capacity(s.len());
        for c in s.chars() {
            if stack.ends_with(c) {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack
    }
}
```

Full tested solution: [`src/solutions/p0184_remove_all_adjacent_duplicates_in_string.rs`](../src/solutions/p0184_remove_all_adjacent_duplicates_in_string.rs)

</details>
