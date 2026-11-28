# Problem 0565: A Struct Holding `&str`

- **Difficulty:** Easy
- **Topics:** structs with a lifetime parameter, `impl<'a> Excerpt<'a>`, returning `&'a str` vs elided `&str`, constructors that borrow
- **Source:** Original · [The Rust Book 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions)

## Problem

A struct that stores a reference needs a lifetime parameter: `Excerpt<'a>` holds `part: &'a str`,
and an `Excerpt` can never outlive the text it points into.

Implement in `impl<'a> Excerpt<'a>`:

- `first_sentence(text: &'a str) -> Excerpt<'a>`: the text up to (not including) the first `'.'`,
  trimmed; the whole trimmed text if there is no `'.'`.
- `part(&self) -> &'a str`: the stored slice. Note the explicit `'a`: the result borrows from the
  **original text**, not from the `Excerpt`, so it stays usable after the `Excerpt` is dropped.
- `words(&self) -> Vec<&'a str>`: the whitespace-separated words of the part.
- `announce(&self, announcement: &str) -> String`: `"Attention please: {announcement} | {part}"`.

And a free function `sentences(text: &str) -> Vec<Excerpt<'_>>` that splits the text at every
`'.'`, trims each piece and skips empty pieces.

## Examples

```text
let novel = String::from("Call me Ishmael. Some years ago...");
let e = Excerpt::first_sentence(&novel);
e.part()                   => "Call me Ishmael"
e.words()                  => ["Call", "me", "Ishmael"]
e.announce("listen")       => "Attention please: listen | Call me Ishmael"
sentences("A b. C.  . D")  => ["A b", "C", "D"]
```

## Constraints

- No special constraints.

## Hints

1. `text.split('.').next()` always returns `Some(..)`; trim it.
2. If `part` returned an elided `&str`, rule 3 would tie it to `&self`, and the result could not
   outlive the `Excerpt`. Writing `-> &'a str` is more precise.

## Starter code

Write your code in [`src/practice/p0565_struct_holding_str.rs`](../src/practice/p0565_struct_holding_str.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0565
```

```rust
/// A borrowed piece of a longer text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn first_sentence(text: &'a str) -> Excerpt<'a> {
        todo!("Write your solution here")
    }

    pub fn part(&self) -> &'a str {
        todo!("Write your solution here")
    }

    pub fn words(&self) -> Vec<&'a str> {
        todo!("Write your solution here")
    }

    pub fn announce(&self, announcement: &str) -> String {
        todo!("Write your solution here")
    }
}

/// Every non-empty, trimmed sentence of `text`.
pub fn sentences(text: &str) -> Vec<Excerpt<'_>> {
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
    fn first_sentence_and_words() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let e = Excerpt::first_sentence(&novel);
        assert_eq!(e.part(), "Call me Ishmael");
        assert_eq!(e.words(), vec!["Call", "me", "Ishmael"]);
        assert_eq!(Excerpt::first_sentence("  no period  ").part(), "no period");
    }

    #[test]
    fn part_outlives_the_excerpt() {
        let text = String::from("First. Second.");
        let first_part;
        {
            let e = Excerpt::first_sentence(&text);
            first_part = e.part();
        } // `e` is dropped, but `first_part` borrows from `text`
        assert_eq!(first_part, "First");
    }

    #[test]
    fn announcements() {
        let e = Excerpt::first_sentence("Call me Ishmael. More.");
        assert_eq!(
            e.announce("listen"),
            "Attention please: listen | Call me Ishmael"
        );
    }

    #[test]
    fn splitting_into_sentences() {
        let parts: Vec<&str> = sentences("A b. C.  . D")
            .iter()
            .map(Excerpt::part)
            .collect();
        assert_eq!(parts, vec!["A b", "C", "D"]);
        assert!(sentences(" . . ").is_empty());
        let long = "word. ".repeat(1000);
        assert_eq!(sentences(&long).len(), 1000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every method hands out slices of the stored `&'a str`, so they can all carry the struct's
lifetime `'a`. Only `announce` allocates, because it builds new text.

**Rust concepts:**
- `struct Excerpt<'a> { part: &'a str }` reads "an `Excerpt` cannot outlive the `str` it borrows".
- In `impl<'a> Excerpt<'a>`, methods may return `&'a str` (tied to the text) or elided `&str` (tied
  to `&self`). Choosing `'a` gives callers more freedom.
- `Vec<Excerpt<'_>>` uses the anonymous lifetime `'_`, which elision ties to the only input `text`.

```rust
/// A borrowed piece of a longer text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn first_sentence(text: &'a str) -> Excerpt<'a> {
        Excerpt {
            part: text.split('.').next().unwrap_or("").trim(),
        }
    }

    pub fn part(&self) -> &'a str {
        self.part
    }

    pub fn words(&self) -> Vec<&'a str> {
        self.part.split_whitespace().collect()
    }

    pub fn announce(&self, announcement: &str) -> String {
        format!("Attention please: {announcement} | {}", self.part)
    }
}

/// Every non-empty, trimmed sentence of `text`.
pub fn sentences(text: &str) -> Vec<Excerpt<'_>> {
    text.split('.')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|part| Excerpt { part })
        .collect()
}
```

Full tested solution: [`src/solutions/p0565_struct_holding_str.rs`](../src/solutions/p0565_struct_holding_str.rs)

</details>
