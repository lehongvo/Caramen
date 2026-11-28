# Problem 0912: Unescape a String Literal Lazily

- **Difficulty:** Medium
- **Topics:** `Cow<'a, str>` in `Result` and `Vec`, borrowed sub-slices, `char::from_u32`, `Peekable<Chars>`
- **Source:** Original · [std: `Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html) · [Reference: character escapes](https://doc.rust-lang.org/reference/tokens.html#character-escapes)

## Problem

Parsers that read quoted text must turn escape sequences into the characters they stand for, but
most fields contain no escapes at all. Return borrowed slices of the input for those fields and
allocate only for the others.

1. `unescape(s: &str) -> Result<Cow<'_, str>, EscapeError>` replaces these escape sequences:

   | escape | result | escape | result |
   | --- | --- | --- | --- |
   | `\n` | newline | `\\` | backslash |
   | `\t` | tab | `\"` | `"` |
   | `\,` | `,` | `\u{1F600}` | the char with that hex code (1 to 6 hex digits) |

   If `s` contains no backslash, return `Cow::Borrowed(s)`. Errors:
   `EscapeError::TrailingBackslash` (a `\` at the very end), `EscapeError::UnknownEscape(c)` (a `\`
   followed by any other char `c`, including `u` without `{`), and
   `EscapeError::BadUnicode(text)` where `text` is what was found between the braces (missing `}`,
   no digits, more than 6 digits, a non-hex digit, or not a valid `char` such as a surrogate).
   For a missing `}`, `text` is everything after the `{`.
2. `split_fields(line: &str) -> Result<Vec<Cow<'_, str>>, EscapeError>` splits `line` at every
   comma that is **not** escaped, then unescapes each field. An escaped backslash does not escape
   the next character: in `a\\,b` the comma is a separator. `""` gives one empty field.

## Examples

```text
unescape("plain")            => Ok(Borrowed("plain"))
unescape(r"say \"hi\"\n")    => Ok(Owned("say \"hi\"\n"))
unescape(r"\u{48}\u{49}")    => Ok(Owned("HI"))
unescape(r"bad\q")           => Err(UnknownEscape('q'))
unescape(r"\u{D800}")        => Err(BadUnicode("D800"))
split_fields(r"a,b\,c,\\,d") => Ok([Borrowed("a"), Owned("b,c"), Owned("\\"), Borrowed("d")])
```

## Constraints

- `0 <= s.len() <= 10^5`

## Hints

1. `s.contains('\\')` decides between borrowing and building a new `String`.
2. Walk the characters with a `Peekable<Chars>`; after a backslash, `match` on the next char.
3. For `split_fields`, scan bytes and remember whether the previous byte started an escape; cut
   `&line[start..i]` at each unescaped comma, so the field is still a borrowed slice of `line`.

## Starter code

Write your code in [`src/practice/p0912_cow_unescape_string_literal.rs`](../src/practice/p0912_cow_unescape_string_literal.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0912
```

```rust
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscapeError {
    TrailingBackslash,
    UnknownEscape(char),
    BadUnicode(String),
}

pub fn unescape(s: &str) -> Result<Cow<'_, str>, EscapeError> {
    todo!("Write your solution here")
}

pub fn split_fields(line: &str) -> Result<Vec<Cow<'_, str>>, EscapeError> {
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

    fn owned(s: &str) -> Cow<'static, str> {
        Cow::Owned(s.to_string())
    }

    #[test]
    fn no_backslash_is_borrowed() {
        let input = "plain text, no escapes";
        let out = unescape(input).unwrap();
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(out.as_ptr(), input.as_ptr());
        assert!(matches!(unescape(""), Ok(Cow::Borrowed(""))));
    }

    #[test]
    fn simple_escapes() {
        assert_eq!(unescape(r#"say \"hi\"\n"#), Ok(owned("say \"hi\"\n")));
        assert_eq!(unescape(r"a\tb\\c\,d"), Ok(owned("a\tb\\c,d")));
        assert_eq!(unescape(r"\u{48}\u{49}"), Ok(owned("HI")));
        assert_eq!(unescape(r"\u{1F600}!"), Ok(owned("\u{1F600}!")));
    }

    #[test]
    fn errors() {
        assert_eq!(unescape(r"bad\q"), Err(EscapeError::UnknownEscape('q')));
        assert_eq!(unescape("end\\"), Err(EscapeError::TrailingBackslash));
        assert_eq!(unescape(r"\u0041"), Err(EscapeError::UnknownEscape('u')));
        assert_eq!(
            unescape(r"\u{D800}"),
            Err(EscapeError::BadUnicode("D800".to_string()))
        );
        assert_eq!(
            unescape(r"\u{}"),
            Err(EscapeError::BadUnicode(String::new()))
        );
        assert_eq!(
            unescape(r"\u{1234567}"),
            Err(EscapeError::BadUnicode("1234567".to_string()))
        );
        assert_eq!(
            unescape(r"\u{12G}"),
            Err(EscapeError::BadUnicode("12G".to_string()))
        );
        assert_eq!(
            unescape(r"\u{+41}"),
            Err(EscapeError::BadUnicode("+41".to_string()))
        );
        assert_eq!(
            unescape(r"\u{41"),
            Err(EscapeError::BadUnicode("41".to_string()))
        );
    }

    #[test]
    fn split_fields_borrows_plain_fields() {
        let line = r"a,b\,c,\\,d";
        let fields = split_fields(line).unwrap();
        assert_eq!(fields, vec!["a", "b,c", "\\", "d"]);
        let kinds: Vec<bool> = fields
            .iter()
            .map(|f| matches!(f, Cow::Borrowed(_)))
            .collect();
        assert_eq!(kinds, vec![true, false, false, true]);
        assert_eq!(split_fields("").unwrap(), vec![""]);
        assert_eq!(split_fields(",").unwrap(), vec!["", ""]);
    }

    #[test]
    fn split_fields_reports_errors_and_handles_unicode() {
        assert_eq!(
            split_fields(r"ok,bad\x"),
            Err(EscapeError::UnknownEscape('x'))
        );
        let fields = split_fields("caf\u{e9},na\\u{EF}ve").unwrap();
        assert_eq!(fields, vec!["caf\u{e9}", "na\u{ef}ve"]);
        let many = vec!["field"; 1_000].join(",");
        assert_eq!(split_fields(&many).unwrap().len(), 1_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `unescape` returns the input unchanged if it has no backslash. Otherwise it walks the
characters, copying ordinary ones and decoding each escape; `\u{...}` collects the characters up
to `}` and converts them with `u32::from_str_radix` and `char::from_u32`. `split_fields` finds the
separators with a tiny state machine (`escaped` is true right after an unescaped backslash) and
passes each `&line[start..i]` slice to `unescape`.

**Complexity:** `O(n)` time for both functions; borrowed fields use no extra memory.

**Rust concepts:**
- The `'_` in `Cow<'_, str>` says the result may borrow from the input; every borrowed field of
  `split_fields` points into `line`.
- Byte indices are safe to slice at here because `,` and `\` are ASCII, so they always sit on a
  `char` boundary in UTF-8.
- `char::from_u32` returns `None` for surrogates and values above `0x10FFFF`.

```rust
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscapeError {
    TrailingBackslash,
    UnknownEscape(char),
    BadUnicode(String),
}

fn decode_unicode(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<char, EscapeError> {
    // The caller already consumed `\u`.
    if chars.next_if_eq(&'{').is_none() {
        return Err(EscapeError::UnknownEscape('u'));
    }
    let mut digits = String::new();
    loop {
        match chars.next() {
            Some('}') => break,
            Some(c) => digits.push(c),
            None => return Err(EscapeError::BadUnicode(digits)),
        }
    }
    let valid_length = (1..=6).contains(&digits.len());
    u32::from_str_radix(&digits, 16)
        .ok()
        .filter(|_| valid_length && digits.chars().all(|c| c.is_ascii_hexdigit()))
        .and_then(char::from_u32)
        .ok_or(EscapeError::BadUnicode(digits))
}

pub fn unescape(s: &str) -> Result<Cow<'_, str>, EscapeError> {
    if !s.contains('\\') {
        return Ok(Cow::Borrowed(s));
    }
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        let decoded = match chars.next() {
            None => return Err(EscapeError::TrailingBackslash),
            Some('n') => '\n',
            Some('t') => '\t',
            Some('\\') => '\\',
            Some('"') => '"',
            Some(',') => ',',
            Some('u') => decode_unicode(&mut chars)?,
            Some(other) => return Err(EscapeError::UnknownEscape(other)),
        };
        out.push(decoded);
    }
    Ok(Cow::Owned(out))
}

pub fn split_fields(line: &str) -> Result<Vec<Cow<'_, str>>, EscapeError> {
    let mut fields = Vec::new();
    let mut start = 0;
    let mut escaped = false;
    for (i, byte) in line.bytes().enumerate() {
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == b',' {
            fields.push(unescape(&line[start..i])?);
            start = i + 1;
        }
    }
    fields.push(unescape(&line[start..])?);
    Ok(fields)
}
```

Full tested solution: [`src/solutions/p0912_cow_unescape_string_literal.rs`](../src/solutions/p0912_cow_unescape_string_literal.rs)

</details>
