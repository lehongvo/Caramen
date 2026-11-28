//! # Problem 0406: Required Form Fields with ok_or
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Option::ok_or`, `Option::ok_or_else`, `Option::filter`, `?` on `Result`, `map_err`
//! - **Source:** Original · [std `Option::ok_or_else`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else)
//!
//! ## Problem
//!
//! A web form arrives as a list of `(key, value)` pairs. A *missing* field is an `Option` problem
//! ("there is no value"), but to report it to the user you want a `Result` with an error message.
//! `ok_or` and `ok_or_else` convert `Option<T>` into `Result<T, E>`.
//!
//! Implement these methods on `Form`:
//!
//! - `get(&self, key: &str) -> Option<&str>`: the value of the **first** pair whose key equals
//!   `key`. A pair whose value is the empty string `""` counts as missing.
//! - `require(&self, key: &str) -> Result<&str, String>`: like `get`, but a missing field becomes
//!   the error ``"missing field `KEY`"`` (backticks included, `KEY` replaced by the key).
//! - `first_key(&self) -> Result<&str, &'static str>`: the key of the first pair, or the error
//!   `"the form is empty"` when there are no pairs at all.
//! - `to_signup(&self) -> Result<Signup, String>`: read the required fields `username`, `email`
//!   and `age` (in that order, returning the first error). `age` must parse as a `u32`; otherwise
//!   the error is ``"field `age` must be a whole number, got `VALUE`"``.
//!
//! ## Examples
//!
//! ```text
//! form = [("username", "ann"), ("email", "ann@x.org"), ("age", "31")]
//! form.get("email")       -> Some("ann@x.org")
//! form.require("phone")   -> Err("missing field `phone`")
//! form.first_key()        -> Ok("username")
//! form.to_signup()        -> Ok(Signup { username: "ann", email: "ann@x.org", age: 31 })
//!
//! form = [("username", ""), ("age", "x")]
//! form.to_signup()        -> Err("missing field `username`")   (an empty value is missing)
//!
//! form = [("username", "bo"), ("email", "b@y.org"), ("age", "-3")]
//! form.to_signup()        -> Err("field `age` must be a whole number, got `-3`")
//! ```
//!
//! ## Constraints
//!
//! - At most `100` pairs. Keys and values are arbitrary strings.
//!
//! ## Hints
//!
//! 1. `get` is `find` + `map` + `filter(|v| !v.is_empty())`.
//! 2. `ok_or(error)` builds the error value immediately; `ok_or_else(|| ...)` builds it only when
//!    the option is `None`. Use the lazy version when the error needs `format!`.
//! 3. In `to_signup`, `self.require("username")?` returns the error early.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `get` answers "is there a value?". `require` converts that answer into a `Result`
//! with a message, and `to_signup` strings several `require` calls together with `?`, so the first
//! missing or invalid field stops the conversion.
//!
//! **Rust concepts:**
//! - `ok_or("the form is empty")` is fine because a string literal costs nothing to create.
//!   `ok_or_else(|| format!(...))` avoids allocating a `String` on the success path.
//! - `Option::filter` turns an empty value into `None`, so `require` does not need a special case.
//! - `map_err` replaces the `ParseIntError` with a message that names the field and the value.
//! - `"-3".parse::<u32>()` fails (a `u32` cannot be negative), which is exactly what we want here.
// @solution-end

#[derive(Debug, PartialEq)]
pub struct Signup {
    pub username: String,
    pub email: String,
    pub age: u32,
}

pub struct Form {
    pub fields: Vec<(String, String)>,
}

impl Form {
    pub fn get(&self, key: &str) -> Option<&str> {
        // @solution-start
        self.fields
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .filter(|v| !v.is_empty())
        // @solution-end
    }

    pub fn require(&self, key: &str) -> Result<&str, String> {
        // @solution-start
        self.get(key)
            .ok_or_else(|| format!("missing field `{key}`"))
        // @solution-end
    }

    pub fn first_key(&self) -> Result<&str, &'static str> {
        // @solution-start
        self.fields
            .first()
            .map(|(k, _)| k.as_str())
            .ok_or("the form is empty")
        // @solution-end
    }

    pub fn to_signup(&self) -> Result<Signup, String> {
        // @solution-start
        let username = self.require("username")?.to_string();
        let email = self.require("email")?.to_string();
        let raw_age = self.require("age")?;
        let age = raw_age
            .parse::<u32>()
            .map_err(|_| format!("field `age` must be a whole number, got `{raw_age}`"))?;
        Ok(Signup {
            username,
            email,
            age,
        })
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn form(pairs: &[(&str, &str)]) -> Form {
        Form {
            fields: pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }

    #[test]
    fn get_and_require() {
        let f = form(&[("username", "ann"), ("email", "ann@x.org"), ("note", "")]);
        assert_eq!(f.get("email"), Some("ann@x.org"));
        assert_eq!(f.get("note"), None);
        assert_eq!(f.require("username"), Ok("ann"));
        assert_eq!(f.require("phone"), Err("missing field `phone`".to_string()));
        assert_eq!(f.require("note"), Err("missing field `note`".to_string()));
    }

    #[test]
    fn first_key_of_empty_and_non_empty_forms() {
        assert_eq!(form(&[("a", "1"), ("b", "2")]).first_key(), Ok("a"));
        assert_eq!(form(&[]).first_key(), Err("the form is empty"));
    }

    #[test]
    fn complete_signup() {
        let f = form(&[("username", "ann"), ("email", "ann@x.org"), ("age", "31")]);
        assert_eq!(
            f.to_signup(),
            Ok(Signup {
                username: "ann".to_string(),
                email: "ann@x.org".to_string(),
                age: 31,
            })
        );
    }

    #[test]
    fn first_error_wins() {
        let f = form(&[("username", ""), ("age", "x")]);
        assert_eq!(f.to_signup(), Err("missing field `username`".to_string()));
        let f = form(&[("username", "bo"), ("age", "7")]);
        assert_eq!(f.to_signup(), Err("missing field `email`".to_string()));
    }

    #[test]
    fn invalid_age_reports_the_value() {
        let f = form(&[("username", "bo"), ("email", "b@y.org"), ("age", "-3")]);
        assert_eq!(
            f.to_signup(),
            Err("field `age` must be a whole number, got `-3`".to_string())
        );
    }
}
