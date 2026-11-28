# Problem 0034: Classify a Character

- **Difficulty:** Easy
- **Topics:** the `char` type, `match` on `char` ranges, `|` patterns, `&'static str`
- **Source:** Original · [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type)

## Problem

A Rust `char` is a single Unicode scalar value: it can be `'a'`, `'7'`, `' '`, but also `'Ω'`
or `'中'`. Write `classify(c: char) -> &'static str` that puts a character into exactly one
category:

| Characters                                   | Result           |
| -------------------------------------------- | ---------------- |
| `'a'` to `'z'`                               | `"lowercase"`    |
| `'A'` to `'Z'`                               | `"uppercase"`    |
| `'0'` to `'9'`                               | `"digit"`        |
| space `' '`, tab `'\t'`, newline `'\n'`, carriage return `'\r'` | `"whitespace"` |
| anything else (punctuation, non-ASCII, ...)  | `"other"`        |

Only the ASCII letters and digits listed above count; for example `'Ω'` is `"other"`.

## Examples

```text
classify('q')  -> "lowercase"
classify('Q')  -> "uppercase"
classify('7')  -> "digit"
classify('\t') -> "whitespace"
classify('?')  -> "other"
classify('Ω')  -> "other"
```

## Constraints

- Any `char`.

## Hints

1. Range patterns work with `char`: `'a'..='z' => ...`.
2. Several characters can share an arm: `' ' | '\t' | '\n' | '\r' => ...`.

## Starter code

Write your code in [`src/practice/p0034_classify_a_character.rs`](../src/practice/p0034_classify_a_character.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0034
```

```rust
pub fn classify(c: char) -> &'static str {
    todo!("Write your solution here")
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
    fn letters_and_digits() {
        assert_eq!(classify('q'), "lowercase");
        assert_eq!(classify('a'), "lowercase");
        assert_eq!(classify('z'), "lowercase");
        assert_eq!(classify('Q'), "uppercase");
        assert_eq!(classify('A'), "uppercase");
        assert_eq!(classify('Z'), "uppercase");
        assert_eq!(classify('0'), "digit");
        assert_eq!(classify('9'), "digit");
    }

    #[test]
    fn whitespace() {
        for c in [' ', '\t', '\n', '\r'] {
            assert_eq!(classify(c), "whitespace", "{c:?}");
        }
    }

    #[test]
    fn other_characters() {
        for c in ['?', '_', '@', '[', '`', '{', '/', ':', 'Ω', '中', '\u{0}'] {
            assert_eq!(classify(c), "other", "{c:?}");
        }
    }

    #[test]
    fn counts_over_all_ascii() {
        let mut lower = 0;
        let mut upper = 0;
        let mut digits = 0;
        for code in 0u8..=127 {
            match classify(code as char) {
                "lowercase" => lower += 1,
                "uppercase" => upper += 1,
                "digit" => digits += 1,
                _ => {}
            }
        }
        assert_eq!((lower, upper, digits), (26, 26, 10));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a single `match` with three range arms, one `|` arm for the whitespace characters and
a final `_` arm.

**Rust concepts:**
- `char` is 4 bytes and holds any Unicode scalar value. Character literals use single quotes and
  support escapes such as `'\n'`, `'\t'` and `'\u{3A9}'` (which is `'Ω'`).
- `char` values are ordered by their code point, so `'a'..='z'` contains exactly the 26 ASCII
  lowercase letters.
- The standard library has ready-made tests such as `c.is_ascii_lowercase()`, `c.is_ascii_digit()`
  and `c.is_whitespace()` (the last one also accepts Unicode spaces, so it is not identical to the
  rule above).

```rust
pub fn classify(c: char) -> &'static str {
    match c {
        'a'..='z' => "lowercase",
        'A'..='Z' => "uppercase",
        '0'..='9' => "digit",
        ' ' | '\t' | '\n' | '\r' => "whitespace",
        _ => "other",
    }
}
```

Full tested solution: [`src/solutions/p0034_classify_a_character.rs`](../src/solutions/p0034_classify_a_character.rs)

</details>
