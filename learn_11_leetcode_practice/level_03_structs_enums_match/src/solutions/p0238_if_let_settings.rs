//! # Problem 0238: Settings with if let
//!
//! - **Difficulty:** Easy
//! - **Topics:** `if let`, `if let ... else`, matching one variant out of many, `if let` with `&mut`, nested patterns
//! - **Source:** Original · [The Rust Book 6.3](https://doc.rust-lang.org/book/ch06-03-if-let.html)
//!
//! ## Problem
//!
//! An app stores its settings as a list of `Setting { key: String, value: Value }`, where
//!
//! ```text
//! enum Value { Bool(bool), Int(i64), Text(String) }
//! ```
//!
//! Keys may repeat; the **first** setting with a key wins. Implement, using `if let` (not a full
//! `match`) wherever you only care about one shape:
//!
//! - `find<'a>(settings: &'a [Setting], key: &str) -> Option<&'a Value>`: the value of the first
//!   setting with that key;
//! - `volume(settings: &[Setting]) -> i64`: the `"volume"` setting if it is an `Int`, clamped to
//!   `0..=100`; otherwise (missing, or the wrong type) `50`;
//! - `is_enabled(settings: &[Setting], key: &str) -> bool`: `true` only if the key exists and holds
//!   `Bool(true)`;
//! - `username(settings: &[Setting]) -> String`: the `"user"` setting if it is non-empty `Text`,
//!   otherwise `"guest"`;
//! - `bump_ints(settings: &mut [Setting], by: i64) -> usize`: add `by` to **every** `Int` setting
//!   (all of them, not only the first per key) and return how many were changed.
//!
//! ## Examples
//!
//! ```text
//! settings = [volume: Int(140), dark_mode: Bool(true), user: Text("ana"), volume: Int(10)]
//! volume(&settings)                  => 100    (first "volume" wins, clamped)
//! is_enabled(&settings, "dark_mode") => true
//! is_enabled(&settings, "user")      => false  (not a Bool)
//! username(&settings)                => "ana"
//! username(&[])                      => "guest"
//! bump_ints(&mut settings, 5)        => 2      (volumes become 145 and 15)
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` settings; integer values stay far from `i64` limits.
//!
//! ## Hints
//!
//! 1. `if let Some(Value::Int(v)) = find(settings, "volume") { ... } else { ... }` checks two
//!    layers at once.
//! 2. `i64::clamp(v, 0, 100)` limits a value to a range.
//! 3. In `bump_ints`, `if let Value::Int(n) = &mut s.value { *n += by; }` changes the value in place.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `find` does the lookup once; the other functions destructure its result with a nested
//! `if let` pattern and fall back in the `else` branch.
//!
//! **Rust concepts:**
//! - `if let PATTERN = EXPR { ... } else { ... }` is a `match` with one interesting arm and a
//!   catch-all. It trades exhaustiveness checking for brevity.
//! - Patterns nest: `Some(Value::Text(name))` looks through the `Option` and the `Value` at once,
//!   and a guard-like condition (`!name.is_empty()`) can follow inside the block.
//! - The lifetime `'a` in `find` says the returned `&Value` borrows from `settings`, not from `key`.
//! - Matching on `&mut s.value` binds `n: &mut i64`, so `*n += by` updates the setting in place.
// @solution-end

/// The value of a setting.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Text(String),
}

/// One key/value pair.
#[derive(Debug, Clone, PartialEq)]
pub struct Setting {
    pub key: String,
    pub value: Value,
}

/// The value of the first setting with `key`.
pub fn find<'a>(settings: &'a [Setting], key: &str) -> Option<&'a Value> {
    // @solution-start
    settings.iter().find(|s| s.key == key).map(|s| &s.value)
    // @solution-end
}

/// The volume clamped to `0..=100`, or `50`.
pub fn volume(settings: &[Setting]) -> i64 {
    // @solution-start
    if let Some(Value::Int(v)) = find(settings, "volume") {
        (*v).clamp(0, 100)
    } else {
        50
    }
    // @solution-end
}

/// Whether the key holds `Bool(true)`.
pub fn is_enabled(settings: &[Setting], key: &str) -> bool {
    // @solution-start
    if let Some(Value::Bool(b)) = find(settings, key) {
        *b
    } else {
        false
    }
    // @solution-end
}

/// The non-empty `"user"` text, or `"guest"`.
pub fn username(settings: &[Setting]) -> String {
    // @solution-start
    if let Some(Value::Text(name)) = find(settings, "user") {
        if !name.is_empty() {
            return name.clone();
        }
    }
    String::from("guest")
    // @solution-end
}

/// Adds `by` to every `Int` setting; returns how many changed.
pub fn bump_ints(settings: &mut [Setting], by: i64) -> usize {
    // @solution-start
    let mut changed = 0;
    for s in settings.iter_mut() {
        if let Value::Int(n) = &mut s.value {
            *n += by;
            changed += 1;
        }
    }
    changed
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(key: &str, value: Value) -> Setting {
        Setting {
            key: key.to_string(),
            value,
        }
    }

    fn sample() -> Vec<Setting> {
        vec![
            set("volume", Value::Int(140)),
            set("dark_mode", Value::Bool(true)),
            set("user", Value::Text("ana".to_string())),
            set("volume", Value::Int(10)),
        ]
    }

    #[test]
    fn find_returns_first_match() {
        let s = sample();
        assert_eq!(find(&s, "volume"), Some(&Value::Int(140)));
        assert_eq!(find(&s, "missing"), None);
    }

    #[test]
    fn volume_rules() {
        assert_eq!(volume(&sample()), 100);
        assert_eq!(volume(&[set("volume", Value::Int(-3))]), 0);
        assert_eq!(volume(&[set("volume", Value::Int(42))]), 42);
        assert_eq!(volume(&[set("volume", Value::Text("loud".into()))]), 50);
        assert_eq!(volume(&[]), 50);
    }

    #[test]
    fn enabled_flags() {
        let s = sample();
        assert!(is_enabled(&s, "dark_mode"));
        assert!(!is_enabled(&s, "user"));
        assert!(!is_enabled(&s, "missing"));
        assert!(!is_enabled(&[set("beta", Value::Bool(false))], "beta"));
    }

    #[test]
    fn usernames() {
        assert_eq!(username(&sample()), "ana");
        assert_eq!(username(&[]), "guest");
        assert_eq!(
            username(&[set("user", Value::Text(String::new()))]),
            "guest"
        );
        assert_eq!(username(&[set("user", Value::Int(7))]), "guest");
    }

    #[test]
    fn bumping_all_ints() {
        let mut s = sample();
        assert_eq!(bump_ints(&mut s, 5), 2);
        assert_eq!(s[0].value, Value::Int(145));
        assert_eq!(s[3].value, Value::Int(15));
        assert_eq!(s[1].value, Value::Bool(true));
    }

    #[test]
    fn bumping_many() {
        let mut s: Vec<Setting> = (0..10_000)
            .map(|i| {
                if i % 2 == 0 {
                    set("n", Value::Int(i))
                } else {
                    set("b", Value::Bool(true))
                }
            })
            .collect();
        assert_eq!(bump_ints(&mut s, -1), 5_000);
        assert_eq!(s[9_998].value, Value::Int(9_997));
    }
}
