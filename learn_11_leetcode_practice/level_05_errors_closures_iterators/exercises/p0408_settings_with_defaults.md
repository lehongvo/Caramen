# Problem 0408: Settings with Defaults

- **Difficulty:** Easy
- **Topics:** `unwrap_or`, `unwrap_or_default`, `unwrap_or_else`, `map_or_else`, `as_deref`, `#[derive(Default)]`
- **Source:** Original · [Rust By Example: Unpacking options and defaults](https://doc.rust-lang.org/rust-by-example/error/option_unwrap/defaults.html)

## Problem

A program reads its settings from a file where every entry is optional. The parsed file is a
`RawSettings` whose fields are all `Option`s. Before the program starts it needs a complete
`Settings` value, with these defaults for missing entries:

| field     | default when `None`                            |
| --------- | ---------------------------------------------- |
| `width`   | `80`                                           |
| `title`   | the empty string (the `Default` of `String`)   |
| `verbose` | `false` (the `Default` of `bool`)              |
| `theme`   | `"light"`                                      |

Implement:

- `resolve(raw: RawSettings) -> Settings` with the defaults above.
- `RawSettings::title_label(&self) -> &str`: the title, or `"untitled"` when the title is
  missing **or** empty. It must not clone the `String`.
- `RawSettings::width_label(&self) -> String`: the width as text (`"120"`), or `"auto"`.

## Examples

```text
resolve(RawSettings { width: Some(120), ..Default::default() })
  -> Settings { width: 120, title: "", verbose: false, theme: "light" }
resolve(RawSettings { title: Some("Notes"), verbose: Some(true), theme: Some("dark"), width: None })
  -> Settings { width: 80, title: "Notes", verbose: true, theme: "dark" }

RawSettings { title: Some(""), .. }.title_label()  -> "untitled"
RawSettings { width: None, .. }.width_label()      -> "auto"
```

## Constraints

- No special constraints.

## Hints

1. `unwrap_or_default()` uses `Default::default()` for `None`: `""` for `String`, `false` for
   `bool`.
2. For `theme`, `unwrap_or_else(|| "light".to_string())` only allocates when the value is
   missing.
3. `Option<String>::as_deref()` gives an `Option<&str>` that borrows the string.

## Starter code

Write your code in [`src/practice/p0408_settings_with_defaults.rs`](../src/practice/p0408_settings_with_defaults.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0408
```

```rust
#[derive(Debug, PartialEq)]
pub struct Settings {
    pub width: u32,
    pub title: String,
    pub verbose: bool,
    pub theme: String,
}

#[derive(Debug, Default)]
pub struct RawSettings {
    pub width: Option<u32>,
    pub title: Option<String>,
    pub verbose: Option<bool>,
    pub theme: Option<String>,
}

pub fn resolve(raw: RawSettings) -> Settings {
    todo!("Write your solution here")
}

impl RawSettings {
    pub fn title_label(&self) -> &str {
        todo!("Write your solution here")
    }

    pub fn width_label(&self) -> String {
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
    fn all_defaults() {
        assert_eq!(
            resolve(RawSettings::default()),
            Settings {
                width: 80,
                title: String::new(),
                verbose: false,
                theme: "light".to_string(),
            }
        );
    }

    #[test]
    fn only_width_given() {
        let raw = RawSettings {
            width: Some(120),
            ..Default::default()
        };
        let s = resolve(raw);
        assert_eq!(s.width, 120);
        assert_eq!(s.theme, "light");
        assert!(!s.verbose);
    }

    #[test]
    fn everything_given_overrides_defaults() {
        let raw = RawSettings {
            width: None,
            title: Some("Notes".to_string()),
            verbose: Some(true),
            theme: Some("dark".to_string()),
        };
        assert_eq!(
            resolve(raw),
            Settings {
                width: 80,
                title: "Notes".to_string(),
                verbose: true,
                theme: "dark".to_string(),
            }
        );
    }

    #[test]
    fn title_label_handles_missing_and_empty() {
        let mut raw = RawSettings::default();
        assert_eq!(raw.title_label(), "untitled");
        raw.title = Some(String::new());
        assert_eq!(raw.title_label(), "untitled");
        raw.title = Some("Report".to_string());
        assert_eq!(raw.title_label(), "Report");
    }

    #[test]
    fn width_label_both_cases() {
        let mut raw = RawSettings::default();
        assert_eq!(raw.width_label(), "auto");
        raw.width = Some(0);
        assert_eq!(raw.width_label(), "0");
        raw.width = Some(120);
        assert_eq!(raw.width_label(), "120");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every field of `Settings` is the matching `Option` field "unwrapped" with the right
fallback. The labels borrow instead of moving: `as_deref` turns `&Option<String>` into
`Option<&str>`, and `map_or_else` produces a `String` in both cases.

**Rust concepts:**
- `unwrap_or(80)` for a cheap constant, `unwrap_or_default()` when the type's default is what
  you want, and `unwrap_or_else(closure)` when the fallback allocates.
- `#[derive(Default)]` on `RawSettings` makes every field `None`, so tests can write
  `RawSettings { width: Some(120), ..Default::default() }` (struct update syntax).
- `as_deref()` + `filter` + `unwrap_or` returns a `&str` that points either into the struct or
  to the literal `"untitled"`; both live long enough for the returned reference.

```rust
#[derive(Debug, PartialEq)]
pub struct Settings {
    pub width: u32,
    pub title: String,
    pub verbose: bool,
    pub theme: String,
}

#[derive(Debug, Default)]
pub struct RawSettings {
    pub width: Option<u32>,
    pub title: Option<String>,
    pub verbose: Option<bool>,
    pub theme: Option<String>,
}

pub fn resolve(raw: RawSettings) -> Settings {
    Settings {
        width: raw.width.unwrap_or(80),
        title: raw.title.unwrap_or_default(),
        verbose: raw.verbose.unwrap_or_default(),
        theme: raw.theme.unwrap_or_else(|| "light".to_string()),
    }
}

impl RawSettings {
    pub fn title_label(&self) -> &str {
        self.title
            .as_deref()
            .filter(|t| !t.is_empty())
            .unwrap_or("untitled")
    }

    pub fn width_label(&self) -> String {
        self.width
            .map_or_else(|| "auto".to_string(), |w| w.to_string())
    }
}
```

Full tested solution: [`src/solutions/p0408_settings_with_defaults.rs`](../src/solutions/p0408_settings_with_defaults.rs)

</details>
