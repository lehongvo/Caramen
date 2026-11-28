# Problem 0530: An Extension Trait for `str`

- **Difficulty:** Easy
- **Topics:** implementing your own trait for std types (`str`, `[i32]`), the orphan rule, method calls through `Deref`
- **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type)

## Problem

You cannot add inherent methods to `str`, but you can define **your own trait** and implement it
for `str`. Once the trait is in scope, its methods can be called like built-in ones. This is
called an *extension trait*.

1. Trait `TextExt`, implemented for `str`:
   - `word_count(&self) -> usize`: the number of whitespace-separated words.
   - `is_palindrome_loose(&self) -> bool`: `true` if the string reads the same forwards and
     backwards after keeping only alphanumeric characters and ignoring ASCII case. An empty result
     counts as a palindrome.
   - `title_case(&self) -> String`: every word with its first character uppercased and the rest
     lowercased, words joined by single spaces.
2. Trait `Stats`, implemented for `[i32]`:
   - `mean(&self) -> Option<f64>`: the average, `None` when empty.
   - `spread(&self) -> Option<i64>`: `max - min` as `i64`, `None` when empty.

Because `String` dereferences to `str` and `Vec<i32>` dereferences to `[i32]`, the methods also
work on `String` and `Vec<i32>` values.

## Examples

```text
"  the quick  brown fox ".word_count()          => 4
"A man, a plan, a canal: Panama".is_palindrome_loose() => true
"hELLO wORLD".title_case()                      => "Hello World"
vec![1, 2, 6].mean()                            => Some(3.0)
[-5, 10, 3].spread()                            => Some(15)
```

## Constraints

- Strings may contain any Unicode text; only ASCII letters change case in `is_palindrome_loose`.

## Hints

1. Write `impl TextExt for str { ... }` (not `for &str`): methods taking `&self` then receive a
   `&str`.
2. The orphan rule allows `impl MyTrait for ForeignType` and `impl ForeignTrait for MyType`, but not
   `impl ForeignTrait for ForeignType`.
3. For `title_case`, `char::to_uppercase` returns an iterator because one character may become
   several.

## Starter code

Write your code in [`src/practice/p0530_extension_trait_for_str.rs`](../src/practice/p0530_extension_trait_for_str.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0530
```

```rust
/// Extra methods for string slices.
pub trait TextExt {
    fn word_count(&self) -> usize;
    fn is_palindrome_loose(&self) -> bool;
    fn title_case(&self) -> String;
}

impl TextExt for str {
    fn word_count(&self) -> usize {
        todo!("Write your solution here")
    }

    fn is_palindrome_loose(&self) -> bool {
        todo!("Write your solution here")
    }

    fn title_case(&self) -> String {
        todo!("Write your solution here")
    }
}

/// Simple statistics for integer slices.
pub trait Stats {
    fn mean(&self) -> Option<f64>;
    fn spread(&self) -> Option<i64>;
}

impl Stats for [i32] {
    fn mean(&self) -> Option<f64> {
        todo!("Write your solution here")
    }

    fn spread(&self) -> Option<i64> {
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

    #[test]
    fn word_counts() {
        assert_eq!("  the quick  brown fox ".word_count(), 4);
        assert_eq!("".word_count(), 0);
        assert_eq!(String::from("one").word_count(), 1);
    }

    #[test]
    fn loose_palindromes() {
        assert!("A man, a plan, a canal: Panama".is_palindrome_loose());
        assert!("".is_palindrome_loose());
        assert!(!"race a car".is_palindrome_loose());
        assert!(String::from("No 'x' in Nixon").is_palindrome_loose());
    }

    #[test]
    fn title_cases() {
        assert_eq!("hELLO wORLD".title_case(), "Hello World");
        assert_eq!("  rust   is fun ".title_case(), "Rust Is Fun");
        assert_eq!("\u{e9}cole".title_case(), "\u{c9}cole");
        assert_eq!("".title_case(), "");
    }

    #[test]
    fn slice_stats() {
        assert_eq!(Vec::from([1, 2, 6]).mean(), Some(3.0)); // Vec<i32> derefs to [i32]
        assert_eq!([-5, 10, 3].spread(), Some(15));
        let empty: [i32; 0] = [];
        assert_eq!(empty.mean(), None);
        assert_eq!(empty.spread(), None);
        assert_eq!([i32::MIN, i32::MAX].spread(), Some(4_294_967_295));
        assert_eq!([i32::MAX, i32::MAX].mean(), Some(i32::MAX as f64));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each method is a short iterator chain over the receiver. The palindrome check collects
the normalised characters once and compares them with their reverse.

**Rust concepts:**
- Implementing a local trait on a foreign type (`str`, `[i32]`) is allowed by the orphan rule
  because the trait is defined in this crate.
- Method resolution auto-derefs: `String` → `str` and `Vec<i32>` → `[i32]`, so one impl on the
  unsized type covers the owned types too.
- `iter().min()` / `iter().max()` return `Option<&i32>`; combining them with `zip` produces `None`
  for an empty slice without special cases.

```rust
/// Extra methods for string slices.
pub trait TextExt {
    fn word_count(&self) -> usize;
    fn is_palindrome_loose(&self) -> bool;
    fn title_case(&self) -> String;
}

impl TextExt for str {
    fn word_count(&self) -> usize {
        self.split_whitespace().count()
    }

    fn is_palindrome_loose(&self) -> bool {
        let chars: Vec<char> = self
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        chars.iter().eq(chars.iter().rev())
    }

    fn title_case(&self) -> String {
        self.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first
                        .to_uppercase()
                        .chain(chars.flat_map(char::to_lowercase))
                        .collect(),
                    None => String::new(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/// Simple statistics for integer slices.
pub trait Stats {
    fn mean(&self) -> Option<f64>;
    fn spread(&self) -> Option<i64>;
}

impl Stats for [i32] {
    fn mean(&self) -> Option<f64> {
        if self.is_empty() {
            return None;
        }
        let total: i64 = self.iter().map(|&x| i64::from(x)).sum();
        Some(total as f64 / self.len() as f64)
    }

    fn spread(&self) -> Option<i64> {
        self.iter()
            .min()
            .zip(self.iter().max())
            .map(|(&lo, &hi)| i64::from(hi) - i64::from(lo))
    }
}
```

Full tested solution: [`src/solutions/p0530_extension_trait_for_str.rs`](../src/solutions/p0530_extension_trait_for_str.rs)

</details>
