// ==================================================================
// PRACTICE 0408: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0408_settings_with_defaults.md
// Run the tests:          cargo test practice::p0408
// ==================================================================

//! # Problem 0408: Settings with Defaults
//!
//! - **Difficulty:** Easy
//! - **Topics:** `unwrap_or`, `unwrap_or_default`, `unwrap_or_else`, `map_or_else`, `as_deref`, `#[derive(Default)]`
//! - **Source:** Original · [Rust By Example: Unpacking options and defaults](https://doc.rust-lang.org/rust-by-example/error/option_unwrap/defaults.html)
//!
//! ## Problem
//!
//! A program reads its settings from a file where every entry is optional. The parsed file is a
//! `RawSettings` whose fields are all `Option`s. Before the program starts it needs a complete
//! `Settings` value, with these defaults for missing entries:
//!
//! | field     | default when `None`                            |
//! | --------- | ---------------------------------------------- |
//! | `width`   | `80`                                           |
//! | `title`   | the empty string (the `Default` of `String`)   |
//! | `verbose` | `false` (the `Default` of `bool`)              |
//! | `theme`   | `"light"`                                      |
//!
//! Implement:
//!
//! - `resolve(raw: RawSettings) -> Settings` with the defaults above.
//! - `RawSettings::title_label(&self) -> &str`: the title, or `"untitled"` when the title is
//!   missing **or** empty. It must not clone the `String`.
//! - `RawSettings::width_label(&self) -> String`: the width as text (`"120"`), or `"auto"`.
//!
//! ## Examples
//!
//! ```text
//! resolve(RawSettings { width: Some(120), ..Default::default() })
//!   -> Settings { width: 120, title: "", verbose: false, theme: "light" }
//! resolve(RawSettings { title: Some("Notes"), verbose: Some(true), theme: Some("dark"), width: None })
//!   -> Settings { width: 80, title: "Notes", verbose: true, theme: "dark" }
//!
//! RawSettings { title: Some(""), .. }.title_label()  -> "untitled"
//! RawSettings { width: None, .. }.width_label()      -> "auto"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `unwrap_or_default()` uses `Default::default()` for `None`: `""` for `String`, `false` for
//!    `bool`.
//! 2. For `theme`, `unwrap_or_else(|| "light".to_string())` only allocates when the value is
//!    missing.
//! 3. `Option<String>::as_deref()` gives an `Option<&str>` that borrows the string.

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
