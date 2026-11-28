# Problem 0170: Fix: Returning a Slice of a Local String

- **Difficulty:** Easy
- **Topics:** fix the borrow checker, error E0515, slices of temporaries, choosing `String` as the return type
- **Source:** Original · [E0515](https://doc.rust-lang.org/error_codes/E0515.html) · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references)

## Problem

This function should return the first word of a text in upper case, but it does not compile:

```text
pub fn shout_first_word(text: &str) -> &str {
    let upper = text.to_uppercase();
    upper.split_whitespace().next().unwrap_or("")
}

error[E0515]: cannot return value referencing local variable `upper`
  |     upper.split_whitespace().next().unwrap_or("")
  |     -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |
  |     returns a value referencing data owned by the current function
  |     `upper` is borrowed here
```

`upper` is a new `String` owned by the function and dropped when it returns, so no slice of it
can be returned. The upper-case text does not exist in `text` either. The result is new text, and
new text must be returned as an owned `String`.

Implement the corrected `shout_first_word(text: &str) -> String`: it returns the first
whitespace-separated word of `text` converted to upper case (Unicode rules), or an empty string
if `text` has no word. Upper-case only that word, not the whole text.

## Examples

```text
Input:  text = "hello big world"
Output: "HELLO"

Input:  text = "  straße links"
Output: "STRASSE"

Input:  text = " \n "
Output: ""
```

## Constraints

- `0 <= text.len() <= 10^5`

## Hints

1. Find the first word in `text` itself (`split_whitespace().next()`): that slice borrows from the
   caller's data, which outlives the function.
2. Then convert just that word with `to_uppercase()` and return the resulting `String`.

## Starter code

Write your code in [`src/practice/p0170_fix_return_slice_of_local.rs`](../src/practice/p0170_fix_return_slice_of_local.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0170
```

```rust
pub fn shout_first_word(text: &str) -> String {
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
    fn example_ascii() {
        assert_eq!(shout_first_word("hello big world"), "HELLO");
    }

    #[test]
    fn example_unicode_expansion() {
        assert_eq!(shout_first_word("  straße links"), "STRASSE");
    }

    #[test]
    fn example_no_word() {
        assert_eq!(shout_first_word(" \n "), "");
        assert_eq!(shout_first_word(""), "");
    }

    #[test]
    fn result_outlives_the_input() {
        let result = {
            let text = String::from("temporary text");
            shout_first_word(&text)
        };
        assert_eq!(result, "TEMPORARY");
    }

    #[test]
    fn only_the_first_word_is_converted() {
        let text = format!("go {}", "x".repeat(99_000));
        assert_eq!(shout_first_word(&text), "GO");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** reverse the order of the two steps. First borrow the word from `text` (cheap, valid for
the caller), then build the upper-case `String` from that word only. Returning the `String` moves
it to the caller, so nothing dangles, and the rest of the text is never upper-cased.

**Complexity:** `O(n)` time to find the word, `O(w)` space for the returned word.

**Rust concepts:**
- E0515: a function may return references only to data that outlives it (its arguments or
  `'static` data), never to its own locals.
- Changing the return type from `&str` to `String` is the standard fix when the function creates
  the text it returns.
- `Option::map_or_else(String::new, str::to_uppercase)` turns `Option<&str>` into a `String`.

```rust
pub fn shout_first_word(text: &str) -> String {
    text.split_whitespace()
        .next()
        .map_or_else(String::new, str::to_uppercase)
}
```

Full tested solution: [`src/solutions/p0170_fix_return_slice_of_local.rs`](../src/solutions/p0170_fix_return_slice_of_local.rs)

</details>
