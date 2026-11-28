# Problem 0562: Lifetime Elision Rules

- **Difficulty:** Easy
- **Topics:** the three lifetime elision rules, when explicit lifetimes are required, returning sub-slices, methods returning references tied to `&self`
- **Source:** Original · [The Rust Reference: lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html)

## Problem

Most functions that return references need no lifetime annotations, because the compiler applies
three **elision rules**:

1. Each elided input lifetime becomes a distinct lifetime parameter.
2. If there is exactly one input lifetime, it is assigned to all elided output lifetimes.
3. If there is a `&self` or `&mut self` parameter, its lifetime is assigned to all elided output
   lifetimes.

Implement the following, writing lifetimes **only where elision does not apply**:

- `split_key_value(line: &str) -> Option<(&str, &str)>`: splits `"key = value"` at the first `=`,
  trimming both parts; `None` without `=`. (Rule 2: one input.)
- `strip_comment(line: &str) -> &str`: the part before the first `#`, with trailing whitespace removed.
- `trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str`: `s` without `prefix` if it starts with it,
  otherwise `s`. Two inputs, so rule 2 does not apply: name the lifetime of `s` only.
- `choose<'a>(use_first: bool, a: &'a [i32], b: &'a [i32]) -> &'a [i32]`.
- A struct `WordList { words: Vec<String> }` with `longest(&self) -> Option<&str>` (rule 3) and
  `starting_with(&self, prefix: &str) -> Vec<&str>` (rule 3 again: the results borrow from `self`,
  not from `prefix`).

## Examples

```text
split_key_value(" name = Ada ")      => Some(("name", "Ada"))
split_key_value("no equals")         => None
strip_comment("port = 80   # http")  => "port = 80"
trim_prefix("v1.2.3", "v")           => "1.2.3"
choose(false, &[1], &[2, 3])         => [2, 3]
WordList { words: ["hi", "hello"] }.longest()            => Some("hello")
WordList { words: ["apple", "avocado", "kiwi"] }.starting_with("a") => ["apple", "avocado"]
```

## Constraints

- No special constraints.

## Hints

1. `line.split_once('=')` returns `Option<(&str, &str)>` borrowing from `line`.
2. `s.strip_prefix(prefix).unwrap_or(s)`.
3. For `starting_with`, the elided output lifetime comes from `&self` (rule 3), which is exactly
   what we want: `prefix` may be a temporary.

## Starter code

Write your code in [`src/practice/p0562_elision_rules.rs`](../src/practice/p0562_elision_rules.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0562
```

```rust
/// Splits `"key = value"` at the first `=`, trimming both parts.
pub fn split_key_value(line: &str) -> Option<(&str, &str)> {
    todo!("Write your solution here")
}

/// The text before the first `#`, without trailing whitespace.
pub fn strip_comment(line: &str) -> &str {
    todo!("Write your solution here")
}

/// `s` without `prefix` when it starts with it.
pub fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    todo!("Write your solution here")
}

/// `a` when `use_first`, otherwise `b`.
pub fn choose<'a>(use_first: bool, a: &'a [i32], b: &'a [i32]) -> &'a [i32] {
    todo!("Write your solution here")
}

pub struct WordList {
    pub words: Vec<String>,
}

impl WordList {
    /// The longest word (first on ties).
    pub fn longest(&self) -> Option<&str> {
        todo!("Write your solution here")
    }

    /// The words that start with `prefix`, in order.
    pub fn starting_with(&self, prefix: &str) -> Vec<&str> {
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

    fn list(words: &[&str]) -> WordList {
        WordList {
            words: words.iter().map(|w| w.to_string()).collect(),
        }
    }

    #[test]
    fn key_values_and_comments() {
        assert_eq!(split_key_value(" name = Ada "), Some(("name", "Ada")));
        assert_eq!(split_key_value("a=b=c"), Some(("a", "b=c")));
        assert_eq!(split_key_value("no equals"), None);
        assert_eq!(strip_comment("port = 80   # http"), "port = 80");
        assert_eq!(strip_comment("# only a comment"), "");
        assert_eq!(strip_comment("plain"), "plain");
    }

    #[test]
    fn prefix_with_a_temporary() {
        let version = String::from("v1.2.3");
        let trimmed;
        {
            let prefix = String::from("v");
            trimmed = trim_prefix(&version, &prefix);
        } // `prefix` is dropped; `trimmed` borrows only from `version`
        assert_eq!(trimmed, "1.2.3");
        assert_eq!(trim_prefix("1.0", "v"), "1.0");
    }

    #[test]
    fn choosing_slices() {
        assert_eq!(choose(false, &[1], &[2, 3]), &[2, 3]);
        assert_eq!(choose(true, &[1], &[2, 3]), &[1]);
    }

    #[test]
    fn word_list_methods() {
        let words = list(&["hi", "hello", "help"]);
        assert_eq!(words.longest(), Some("hello"));
        assert_eq!(list(&[]).longest(), None);
        let fruit = list(&["apple", "avocado", "kiwi"]);
        let found = fruit.starting_with(&String::from("a"));
        assert_eq!(found, vec!["apple", "avocado"]);
        assert!(fruit.starting_with("z").is_empty());
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each function returns sub-slices of an input. Where there is one reference input or a
`&self`, elision picks the right lifetime; `trim_prefix` and `choose` need an explicit `'a` because
the compiler cannot guess which input the result comes from.

**Rust concepts:**
- Elision is purely a shorthand: `fn strip_comment(line: &str) -> &str` means
  `fn strip_comment<'a>(line: &'a str) -> &'a str`.
- When rule 3 assigns `&self`'s lifetime to the output, the returned references keep `self`
  borrowed: you cannot mutate the `WordList` while you hold them.
- Writing `prefix: &str` without `'a` in `trim_prefix` documents that the result never points into
  `prefix`, so callers can pass a short-lived string.

```rust
/// Splits `"key = value"` at the first `=`, trimming both parts.
pub fn split_key_value(line: &str) -> Option<(&str, &str)> {
    line.split_once('=').map(|(k, v)| (k.trim(), v.trim()))
}

/// The text before the first `#`, without trailing whitespace.
pub fn strip_comment(line: &str) -> &str {
    line.split('#').next().unwrap_or("").trim_end()
}

/// `s` without `prefix` when it starts with it.
pub fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    s.strip_prefix(prefix).unwrap_or(s)
}

/// `a` when `use_first`, otherwise `b`.
pub fn choose<'a>(use_first: bool, a: &'a [i32], b: &'a [i32]) -> &'a [i32] {
    if use_first { a } else { b }
}

pub struct WordList {
    pub words: Vec<String>,
}

impl WordList {
    /// The longest word (first on ties).
    pub fn longest(&self) -> Option<&str> {
        let mut best: Option<&str> = None;
        for w in &self.words {
            if best.is_none_or(|b| w.len() > b.len()) {
                best = Some(w);
            }
        }
        best
    }

    /// The words that start with `prefix`, in order.
    pub fn starting_with(&self, prefix: &str) -> Vec<&str> {
        self.words
            .iter()
            .filter(|w| w.starts_with(prefix))
            .map(String::as_str)
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0562_elision_rules.rs`](../src/solutions/p0562_elision_rules.rs)

</details>
