# Problem 0258: Roman to Integer

- **Difficulty:** Easy
- **Topics:** `match` on `char` returning `Option`, look-ahead with `peekable`, `while let`, subtractive notation
- **Source:** [LeetCode 13. Roman to Integer](https://leetcode.com/problems/roman-to-integer/)

## Problem

Roman numerals use seven symbols: `I = 1`, `V = 5`, `X = 10`, `L = 50`, `C = 100`, `D = 500`,
`M = 1000`. Symbols are normally written from largest to smallest and added up (`"XII"` is 12).
Six cases use **subtraction** instead: a smaller symbol written before a larger one is
subtracted from it:

- `I` before `V` or `X` (`IV = 4`, `IX = 9`);
- `X` before `L` or `C` (`XL = 40`, `XC = 90`);
- `C` before `D` or `M` (`CD = 400`, `CM = 900`).

Given a valid Roman numeral `s`, return its value.

Implement `Solution::roman_to_int(s: String) -> i32`.

## Examples

```text
Input:  s = "III"
Output: 3

Input:  s = "LVIII"
Output: 58          (L = 50, V = 5, III = 3)

Input:  s = "MCMXCIV"
Output: 1994        (M = 1000, CM = 900, XC = 90, IV = 4)
```

## Constraints

- `1 <= s.len() <= 15`
- `s` is a valid Roman numeral in the range `[1, 3999]`.

## Hints

1. Write `value(c: char) -> Option<i32>` with one `match`.
2. A symbol is subtracted exactly when the **next** symbol is larger. `chars().peekable()` lets
   you look at the next symbol without consuming it.

## Starter code

Write your code in [`src/practice/p0258_roman_to_integer.rs`](../src/practice/p0258_roman_to_integer.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0258
```

```rust
pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
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

    fn roman(s: &str) -> i32 {
        Solution::roman_to_int(s.to_string())
    }

    #[test]
    fn example_iii() {
        assert_eq!(roman("III"), 3);
    }

    #[test]
    fn example_lviii() {
        assert_eq!(roman("LVIII"), 58);
    }

    #[test]
    fn example_mcmxciv() {
        assert_eq!(roman("MCMXCIV"), 1994);
    }

    #[test]
    fn all_subtractive_pairs() {
        assert_eq!(roman("IV"), 4);
        assert_eq!(roman("IX"), 9);
        assert_eq!(roman("XL"), 40);
        assert_eq!(roman("XC"), 90);
        assert_eq!(roman("CD"), 400);
        assert_eq!(roman("CM"), 900);
    }

    #[test]
    fn extremes() {
        assert_eq!(roman("I"), 1);
        assert_eq!(roman("MMMCMXCIX"), 3999);
        assert_eq!(roman("MMMDCCCLXXXVIII"), 3888);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** walk the symbols from left to right. If the next symbol is larger than the current
one, the current one is subtracted; otherwise it is added. This one rule covers all six
subtractive pairs.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `while let Some(c) = chars.next()` loops over an iterator while still allowing
  `chars.peek()` inside the body.
- `chars.peek().and_then(|&n| value(n))` gives `Option<i32>` for the next symbol; comparing
  `Some(next) if next > v` happens in a `match` guard.
- Unknown characters are ignored by `let Some(v) = value(c) else { continue; };`.

```rust
pub struct Solution;

fn value(c: char) -> Option<i32> {
    match c {
        'I' => Some(1),
        'V' => Some(5),
        'X' => Some(10),
        'L' => Some(50),
        'C' => Some(100),
        'D' => Some(500),
        'M' => Some(1000),
        _ => None,
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut total = 0;
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            let Some(v) = value(c) else {
                continue;
            };
            match chars.peek().and_then(|&n| value(n)) {
                Some(next) if next > v => total -= v,
                _ => total += v,
            }
        }
        total
    }
}
```

Full tested solution: [`src/solutions/p0258_roman_to_integer.rs`](../src/solutions/p0258_roman_to_integer.rs)

</details>
