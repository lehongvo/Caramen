# Problem 0174: Reverse Vowels of a String

- **Difficulty:** Easy
- **Topics:** `into_bytes`, two indices with `swap`, `matches!`, `String::from_utf8`
- **Source:** [LeetCode 345. Reverse Vowels of a String](https://leetcode.com/problems/reverse-vowels-of-a-string/)

## Problem

Reverse only the vowels of a string: the first vowel trades places with the last vowel, the
second with the second to last, and so on. All other characters stay where they are. The vowels
are `a`, `e`, `i`, `o`, `u` in lowercase and uppercase.

Implement `Solution::reverse_vowels(s: String) -> String`.

## Examples

```text
Input:  s = "IceCreAm"
Output: "AceCreIm"        (the vowels I, e, e, A become A, e, e, I)

Input:  s = "leetcode"
Output: "leotcede"

Input:  s = "xyz"
Output: "xyz"
```

## Constraints

- `1 <= s.len() <= 3 * 10^5`
- `s` consists of printable ASCII characters.

## Hints

1. Turn the owned `String` into its bytes with `into_bytes()` so you can swap positions.
2. Move `i` right until it is on a vowel and `j` left until it is on a vowel, swap, repeat.
3. `matches!(b, b'a' | b'e' | ...)` is a compact vowel test.

## Starter code

Write your code in [`src/practice/p0174_reverse_vowels_of_a_string.rs`](../src/practice/p0174_reverse_vowels_of_a_string.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0174
```

```rust
pub struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
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
        Solution::reverse_vowels(s.to_string())
    }

    #[test]
    fn example_ice_cream() {
        assert_eq!(run("IceCreAm"), "AceCreIm");
    }

    #[test]
    fn example_leetcode() {
        assert_eq!(run("leetcode"), "leotcede");
    }

    #[test]
    fn example_no_vowels() {
        assert_eq!(run("xyz"), "xyz");
    }

    #[test]
    fn single_vowel_and_punctuation() {
        assert_eq!(run("a"), "a");
        assert_eq!(run("a.b,O!"), "O.b,a!");
    }

    #[test]
    fn reversing_twice_restores() {
        let s = "Programming in Rust is fun";
        assert_eq!(run(&run(s)), s);
    }

    #[test]
    fn large_input() {
        let s = "aE".repeat(100_000);
        assert_eq!(run(&s), "Ea".repeat(100_000));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the classic two-pointer swap, restricted to vowels. Both indices skip consonants; when
both sit on vowels, swap them and move on. Working on the string's own byte buffer means no
allocation.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `into_bytes()` and `String::from_utf8()` move the buffer between `String` and `Vec<u8>`.
- Swapping two ASCII bytes keeps the text valid UTF-8, so the `expect` can never fire for the
  input the constraints allow.
- `matches!` expands to a `match` returning `bool`.

```rust
fn is_vowel(b: u8) -> bool {
    matches!(b.to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u')
}

pub struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut bytes = s.into_bytes();
        let (mut i, mut j) = (0, bytes.len());
        // `j` is exclusive: the right candidate is bytes[j - 1].
        while i < j {
            if !is_vowel(bytes[i]) {
                i += 1;
            } else if !is_vowel(bytes[j - 1]) {
                j -= 1;
            } else {
                bytes.swap(i, j - 1);
                i += 1;
                j -= 1;
            }
        }
        String::from_utf8(bytes).expect("swapping ASCII bytes keeps the text valid UTF-8")
    }
}
```

Full tested solution: [`src/solutions/p0174_reverse_vowels_of_a_string.rs`](../src/solutions/p0174_reverse_vowels_of_a_string.rs)

</details>
