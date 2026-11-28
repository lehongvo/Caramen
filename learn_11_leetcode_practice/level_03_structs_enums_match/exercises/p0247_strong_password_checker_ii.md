# Problem 0247: Strong Password Checker II

- **Difficulty:** Easy
- **Topics:** the `matches!` macro, `char` range patterns, `any`, `windows`, a small rules struct
- **Source:** [LeetCode 2299. Strong Password Checker II](https://leetcode.com/problems/strong-password-checker-ii/)

## Problem

A password is **strong** when it satisfies all of these rules:

1. it has at least 8 characters;
2. it contains at least one lowercase letter, one uppercase letter and one digit;
3. it contains at least one special character from the set `!@#$%^&*()-+`;
4. no two **adjacent** characters are equal (`"aab"` breaks this rule, `"aba"` does not).

Given `password`, return `true` if it is strong.

Implement `Solution::strong_password_checker_ii(password: String) -> bool`.

## Examples

```text
Input:  password = "IloveLe3tcode!"
Output: true

Input:  password = "Me+You--IsMyDream"
Output: false     (no digit, and "--" repeats a character)

Input:  password = "1aB!"
Output: false     (too short)
```

## Constraints

- `1 <= password.len() <= 100`
- `password` contains letters, digits and characters from `!@#$%^&*()-+`.

## Hints

1. `matches!(b, b'!' | b'@')` is a `bool` expression: "does `b` match this pattern?".
   For standard classes, `u8::is_ascii_lowercase` and friends already exist.
2. `bytes.windows(2).all(|w| w[0] != w[1])` checks every adjacent pair.

## Starter code

Write your code in [`src/practice/p0247_strong_password_checker_ii.rs`](../src/practice/p0247_strong_password_checker_ii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0247
```

```rust
pub struct Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
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

    fn strong(p: &str) -> bool {
        Solution::strong_password_checker_ii(p.to_string())
    }

    #[test]
    fn example_strong() {
        assert!(strong("IloveLe3tcode!"));
    }

    #[test]
    fn example_missing_digit_and_repeat() {
        assert!(!strong("Me+You--IsMyDream"));
    }

    #[test]
    fn example_too_short() {
        assert!(!strong("1aB!"));
    }

    #[test]
    fn each_rule_alone_breaks_it() {
        assert!(strong("aB3!aB3!"));
        assert!(!strong("aB3!aB3")); // 7 characters
        assert!(!strong("AB3!AB3!")); // no lowercase
        assert!(!strong("ab3!ab3!")); // no uppercase
        assert!(!strong("aBc!aBc!")); // no digit
        assert!(!strong("aB3xaB3y")); // no special character
        assert!(!strong("aB3!!aB3")); // "!!" repeats
    }

    #[test]
    fn long_password() {
        let p = "aB3+".repeat(25);
        assert_eq!(p.len(), 100);
        assert!(strong(&p));
        // Same length, but every "3+a" becomes "3++": adjacent repeats.
        assert!(!strong(&p.replace("3+a", "3++")));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** check the length, then use `any` for each required kind of character (the standard
`is_ascii_*` methods for letters and digits, a `matches!` pattern for the special characters),
and `windows(2)` for the "no repeated neighbours" rule.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `matches!(value, PATTERN)` expands to `match value { PATTERN => true, _ => false }`; it accepts
  ranges, `|` alternatives and even guards (`matches!(x, n if n > 3)`).
- Working on `as_bytes()` is fine here because the password is ASCII; byte literals like
  `b'!'` are patterns too. A hand-written range such as `matches!(b, b'a'..=b'z')` also works,
  but clippy (`manual_is_ascii_check`) prefers the named method `b.is_ascii_lowercase()`.
- `u8::is_ascii_lowercase` takes `&self`, so it can be passed directly to `any` over `&u8` items.
- `slice::windows(2)` yields overlapping sub-slices `[b0, b1]`, `[b1, b2]`, ...

```rust
pub struct Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        let bytes = password.as_bytes();
        bytes.len() >= 8
            && bytes.iter().any(u8::is_ascii_lowercase)
            && bytes.iter().any(u8::is_ascii_uppercase)
            && bytes.iter().any(u8::is_ascii_digit)
            && bytes.iter().any(|b| {
                matches!(
                    b,
                    b'!' | b'@'
                        | b'#'
                        | b'$'
                        | b'%'
                        | b'^'
                        | b'&'
                        | b'*'
                        | b'('
                        | b')'
                        | b'-'
                        | b'+'
                )
            })
            && bytes.windows(2).all(|w| w[0] != w[1])
    }
}
```

Full tested solution: [`src/solutions/p0247_strong_password_checker_ii.rs`](../src/solutions/p0247_strong_password_checker_ii.rs)

</details>
