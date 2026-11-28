# Problem 0153: Swap Case

- **Difficulty:** Easy
- **Topics:** case conversion, `char::is_uppercase`, `char::to_lowercase` returns an iterator, `extend`
- **Source:** Original · [`char::to_uppercase`](https://doc.rust-lang.org/std/primitive.char.html#method.to_uppercase) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)

## Problem

Implement `swap_case(text: &str) -> String`, which returns a new string where:

- every lowercase letter is replaced by its uppercase form;
- every uppercase letter is replaced by its lowercase form;
- every other character (digits, spaces, punctuation, letters without case such as `'日'`) is
  copied unchanged.

Use the full Unicode case rules of `char::to_uppercase` / `char::to_lowercase`. Some characters
map to **several** characters: the lowercase `'ß'` becomes `"SS"`, so the output can be longer
than the input.

## Examples

```text
Input:  text = "Hello World"
Output: "hELLO wORLD"

Input:  text = "Straße"
Output: "sTRASSE"

Input:  text = "ÜBER 42!"
Output: "über 42!"

Input:  text = "日本"
Output: "日本"
```

## Constraints

- `0 <= text.len() <= 10^5`

## Hints

1. `c.is_lowercase()` and `c.is_uppercase()` use the Unicode definitions.
2. `c.to_uppercase()` is an iterator of `char`; `out.extend(c.to_uppercase())` appends all of them.

## Starter code

Write your code in [`src/practice/p0153_swap_case.rs`](../src/practice/p0153_swap_case.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0153
```

```rust
pub fn swap_case(text: &str) -> String {
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
        assert_eq!(swap_case("Hello World"), "hELLO wORLD");
    }

    #[test]
    fn example_sharp_s_expands() {
        assert_eq!(swap_case("Straße"), "sTRASSE");
    }

    #[test]
    fn example_umlaut_and_digits() {
        assert_eq!(swap_case("ÜBER 42!"), "über 42!");
    }

    #[test]
    fn example_caseless_characters() {
        assert_eq!(swap_case("日本"), "日本");
    }

    #[test]
    fn empty_and_symbols() {
        assert_eq!(swap_case(""), "");
        assert_eq!(swap_case("123 !? 🦀"), "123 !? 🦀");
    }

    #[test]
    fn swapping_twice_is_not_always_identity() {
        assert_eq!(swap_case(&swap_case("aBc")), "aBc");
        assert_eq!(swap_case(&swap_case("ß")), "ss");
    }

    #[test]
    fn large_input() {
        let text = "aB".repeat(50_000);
        assert_eq!(swap_case(&text), "Ab".repeat(50_000));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** walk the characters and append, for each one, its converted form (an iterator of one or
more chars) or the character itself.

**Complexity:** `O(n)` time and space.

**Rust concepts:**
- `char::to_uppercase` / `to_lowercase` return iterators (`ToUppercase`, `ToLowercase`) because
  one character can map to up to three characters.
- `String` implements `Extend<char>`, so the iterator can be appended directly.
- Swapping twice does **not** always give the original back (`"ß"` to `"SS"` to `"ss"`): Unicode
  case mapping is not reversible in general.

```rust
pub fn swap_case(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_lowercase() {
            out.extend(c.to_uppercase());
        } else if c.is_uppercase() {
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}
```

Full tested solution: [`src/solutions/p0153_swap_case.rs`](../src/solutions/p0153_swap_case.rs)

</details>
