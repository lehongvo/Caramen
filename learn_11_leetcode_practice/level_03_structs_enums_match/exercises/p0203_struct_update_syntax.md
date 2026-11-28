# Problem 0203: Copy a Profile with Struct Update Syntax

- **Difficulty:** Easy
- **Topics:** struct update syntax `..base`, moves of non-`Copy` fields, `clone`
- **Source:** Original · [The Rust Book 5.1](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

## Problem

An application stores user preferences in a `Profile`:

```text
pub struct Profile {
    pub username: String,
    pub email: String,
    pub theme: String,
    pub font_size: u32,
    pub notifications: bool,
}
```

*Struct update syntax* builds a new value from an old one: `Profile { theme: t, ..base }` takes
`theme` from `t` and **every other field** from `base`. Fields of non-`Copy` types such as
`String` are *moved* out of `base`.

Implement three functions:

1. `default_profile(username: &str) -> Profile`: username `username`, email
   `"<username>@example.com"`, theme `"light"`, font size `14`, notifications on.
2. `with_email(base: Profile, email: String) -> Profile`: takes ownership of `base` and returns
   the same profile with a new email. Use `..base`.
3. `dark_copy(base: &Profile) -> Profile`: only **borrows** `base`. Returns a copy whose theme is
   `"dark"` and whose font size is `base.font_size + 2`; every other field is the same. Because
   you cannot move fields out of a borrowed value, use `..base.clone()`.

## Examples

```text
default_profile("kim")
  => Profile { username: "kim", email: "kim@example.com", theme: "light",
               font_size: 14, notifications: true }

with_email(default_profile("kim"), "kim@work.io")
  => same as above but email: "kim@work.io"

dark_copy(&default_profile("kim"))
  => same as the default but theme: "dark", font_size: 16
```

## Constraints

- `username` is a non-empty ASCII string; `font_size <= 100`.

## Hints

1. `format!("{username}@example.com")` builds the default email.
2. In `with_email`, after `Profile { email, ..base }` the old `base` is partially moved and can
   no longer be used, which is fine because the function owns it.
3. `..expr` must come **last** in the literal, and `expr` can be any expression of type `Profile`.

## Starter code

Write your code in [`src/practice/p0203_struct_update_syntax.rs`](../src/practice/p0203_struct_update_syntax.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0203
```

```rust
/// User preferences.
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub username: String,
    pub email: String,
    pub theme: String,
    pub font_size: u32,
    pub notifications: bool,
}

/// The profile a new user starts with.
pub fn default_profile(username: &str) -> Profile {
    todo!("Write your solution here")
}

/// Returns `base` with a different email (consumes `base`).
pub fn with_email(base: Profile, email: String) -> Profile {
    todo!("Write your solution here")
}

/// Returns a dark-theme copy of `base` with a font two points larger.
pub fn dark_copy(base: &Profile) -> Profile {
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

    fn kim() -> Profile {
        Profile {
            username: String::from("kim"),
            email: String::from("kim@example.com"),
            theme: String::from("light"),
            font_size: 14,
            notifications: true,
        }
    }

    #[test]
    fn example_default_profile() {
        assert_eq!(default_profile("kim"), kim());
    }

    #[test]
    fn example_with_email() {
        let updated = with_email(default_profile("kim"), String::from("kim@work.io"));
        let expected = Profile {
            email: String::from("kim@work.io"),
            ..kim()
        };
        assert_eq!(updated, expected);
    }

    #[test]
    fn example_dark_copy() {
        let base = default_profile("kim");
        let dark = dark_copy(&base);
        assert_eq!(dark.theme, "dark");
        assert_eq!(dark.font_size, 16);
        assert_eq!(dark.username, "kim");
        assert_eq!(dark.email, "kim@example.com");
        assert!(dark.notifications);
    }

    #[test]
    fn dark_copy_leaves_the_original_untouched() {
        let base = default_profile("lee");
        let _dark = dark_copy(&base);
        assert_eq!(base, default_profile("lee"));
        assert_eq!(base.theme, "light");
    }

    #[test]
    fn with_email_keeps_custom_settings() {
        let custom = Profile {
            notifications: false,
            font_size: 30,
            ..kim()
        };
        let moved = with_email(custom, String::from("k@k.k"));
        assert_eq!(moved.font_size, 30);
        assert!(!moved.notifications);
        assert_eq!(moved.email, "k@k.k");
    }

    #[test]
    fn chained_updates() {
        let p = dark_copy(&dark_copy(&default_profile("max")));
        assert_eq!(p.font_size, 18);
        assert_eq!(p.email, "max@example.com");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each function is a single struct literal. `with_email` moves the remaining fields out of
the owned `base`; `dark_copy` clones the borrowed profile and overrides two fields of the clone.

**Rust concepts:**
- `..base` copies `Copy` fields (`u32`, `bool`) and *moves* the others (`String`). After a move,
  `base` as a whole cannot be used, although its `Copy` fields still can.
- With only a `&Profile`, moving is impossible (error E0507), so the update source must be an
  owned value: `base.clone()` creates one.
- Overridden fields are not taken from the update source, so `Profile { theme, ..base }` does not
  move `base.theme`.

```rust
/// User preferences.
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub username: String,
    pub email: String,
    pub theme: String,
    pub font_size: u32,
    pub notifications: bool,
}

/// The profile a new user starts with.
pub fn default_profile(username: &str) -> Profile {
    Profile {
        username: username.to_string(),
        email: format!("{username}@example.com"),
        theme: String::from("light"),
        font_size: 14,
        notifications: true,
    }
}

/// Returns `base` with a different email (consumes `base`).
pub fn with_email(base: Profile, email: String) -> Profile {
    Profile { email, ..base }
}

/// Returns a dark-theme copy of `base` with a font two points larger.
pub fn dark_copy(base: &Profile) -> Profile {
    Profile {
        theme: String::from("dark"),
        font_size: base.font_size + 2,
        ..base.clone()
    }
}
```

Full tested solution: [`src/solutions/p0203_struct_update_syntax.rs`](../src/solutions/p0203_struct_update_syntax.rs)

</details>
