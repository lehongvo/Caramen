# Problem 0171: Slugify a Title

- **Difficulty:** Medium
- **Topics:** choosing `&str` in and `String` out, a "pending separator" flag, `is_ascii_alphanumeric`, validating with `bytes().all`
- **Source:** Original · [`char::is_ascii_alphanumeric`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_alphanumeric) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)

## Problem

Blogs turn article titles into URL *slugs*, such as `"hello-world"` for `"Hello, World!"`.
Implement two functions:

1. `slugify(title: &str) -> String` builds the slug:
   - ASCII letters are kept and converted to lowercase; ASCII digits are kept;
   - every run of other characters (spaces, punctuation, non-ASCII characters, ...) between two
     kept characters becomes a **single** `'-'`;
   - the slug never starts or ends with `'-'`; a title with no ASCII letter or digit gives `""`.
2. `is_slug(text: &str) -> bool` returns `true` if `text` is a valid slug: non-empty, made only of
   `a`-`z`, `0`-`9` and `'-'`, not starting or ending with `'-'`, and without `"--"`.

The parameter types are `&str` because both functions only read their input; `slugify` returns a
`String` because it produces new text, while `is_slug` just answers a question.

## Examples

```text
Input:  slugify("Hello, World!")
Output: "hello-world"

Input:  slugify("  Rust 1.87 -- Released!  ")
Output: "rust-1-87-released"

Input:  slugify("Grüße aus Köln")
Output: "gr-e-aus-k-ln"      (non-ASCII letters are separators)

Input:  slugify("¿?")
Output: ""

Input:  is_slug("rust-1-87")   -> true
Input:  is_slug("rust--lang")  -> false
Input:  is_slug("-rust")       -> false
```

## Constraints

- `0 <= title.len(), text.len() <= 10^5`

## Hints

1. Do not push a `'-'` as soon as you see a separator: remember that one is *pending* and push it
   only when the next kept character arrives (and only if the slug is not empty yet).
2. That rule alone avoids leading, trailing and doubled dashes.
3. For `is_slug`, combine `starts_with`, `ends_with`, `contains("--")` and `bytes().all(...)`.

## Starter code

Write your code in [`src/practice/p0171_slugify.rs`](../src/practice/p0171_slugify.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0171
```

```rust
pub fn slugify(title: &str) -> String {
    todo!("Write your solution here")
}

pub fn is_slug(text: &str) -> bool {
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
    fn example_hello_world() {
        assert_eq!(slugify("Hello, World!"), "hello-world");
    }

    #[test]
    fn example_digits_and_runs_of_separators() {
        assert_eq!(slugify("  Rust 1.87 -- Released!  "), "rust-1-87-released");
    }

    #[test]
    fn example_non_ascii_letters_are_separators() {
        assert_eq!(slugify("Grüße aus Köln"), "gr-e-aus-k-ln");
    }

    #[test]
    fn example_nothing_to_keep() {
        assert_eq!(slugify("¿?"), "");
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn example_is_slug() {
        assert!(is_slug("rust-1-87"));
        assert!(!is_slug("rust--lang"));
        assert!(!is_slug("-rust"));
    }

    #[test]
    fn is_slug_rejects_bad_characters_and_edges() {
        assert!(!is_slug(""));
        assert!(!is_slug("rust-"));
        assert!(!is_slug("Rust"));
        assert!(!is_slug("rust lang"));
        assert!(!is_slug("über"));
        assert!(is_slug("a"));
        assert!(is_slug("2024"));
    }

    #[test]
    fn every_non_empty_slug_is_valid() {
        for title in ["A -- B", "...x...", "C++ & Rust", "日本 2 go", "--a--b--"] {
            let slug = slugify(title);
            assert!(is_slug(&slug), "{title:?} gave {slug:?}");
        }
    }

    #[test]
    fn large_title() {
        let title = "Word! ".repeat(10_000);
        let slug = slugify(&title);
        assert_eq!(slug.len(), 10_000 * 5 - 1);
        assert!(is_slug(&slug));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a single pass with a `pending_dash` flag. Separators only set the flag. A kept character
first emits the pending dash (unless the slug is still empty), then itself in lowercase. Because
dashes are emitted only *before* a kept character, the slug can never start or end with one or
contain two in a row. `is_slug` checks the same properties directly.

**Complexity:** `O(n)` time; `O(n)` space for the slug, `O(1)` for `is_slug`.

**Rust concepts:**
- Signature design: borrow inputs you only read (`&str`), return `String` for newly built text and
  plain values (`bool`) for answers.
- `char::is_ascii_alphanumeric` and `to_ascii_lowercase` deliberately ignore non-ASCII
  characters; `char::is_alphanumeric` would accept `'ü'` as well.
- Deferring an action with a flag is a common way to handle separators without special cases at
  the ends.

```rust
pub fn slugify(title: &str) -> String {
    let mut slug = String::with_capacity(title.len());
    let mut pending_dash = false;
    for c in title.chars() {
        if c.is_ascii_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            pending_dash = false;
            slug.push(c.to_ascii_lowercase());
        } else {
            pending_dash = true;
        }
    }
    slug
}

pub fn is_slug(text: &str) -> bool {
    !text.is_empty()
        && !text.starts_with('-')
        && !text.ends_with('-')
        && !text.contains("--")
        && text
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}
```

Full tested solution: [`src/solutions/p0171_slugify.rs`](../src/solutions/p0171_slugify.rs)

</details>
