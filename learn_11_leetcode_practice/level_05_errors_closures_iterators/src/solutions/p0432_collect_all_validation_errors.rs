//! # Problem 0432: Collect Every Validation Error
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Result<(), Vec<E>>`, checks as `(bool, error)` pairs, `filter_map` + `bool::then_some`, `Result::err`, `impl Display`
//! - **Source:** Original · [std `bool::then_some`](https://doc.rust-lang.org/std/primitive.bool.html#method.then_some)
//!
//! ## Problem
//!
//! `?` stops at the first error, which is right for most code but frustrating in a sign-up form: the
//! user fixes one field, submits, and only then learns about the next problem. A form validator
//! should report **every** problem at once, so its error type is a `Vec` of errors.
//!
//! Implement `validate(r: &Registration) -> Result<(), Vec<FieldError>>`. Run all the checks below
//! and collect the errors **in this order** (several may come from the same field):
//!
//! 1. `username` has at least 3 characters, else `TooShort { field: "username", min: 3 }`;
//! 2. `username` contains only ASCII letters, ASCII digits and `'_'`, else `InvalidChars("username")`;
//! 3. `password` has at least 8 characters, else `TooShort { field: "password", min: 8 }`;
//! 4. `password` contains at least one ASCII digit, else `MissingDigit("password")`;
//! 5. `confirm` equals `password`, else `Mismatch`;
//! 6. `age` is in `13..=120`, else `OutOfRange { field: "age", min: 13, max: 120 }`.
//!
//! Return `Ok(())` when the list is empty. "Characters" means `chars().count()`.
//!
//! Also implement:
//!
//! - `invalid_entries(rs: &[Registration]) -> Vec<(usize, Vec<FieldError>)>`: for each invalid
//!   registration, its index and its errors, in input order.
//! - `Display` for `FieldError`:
//!   - `TooShort { field, min }` -> ``"`FIELD` must have at least MIN characters"``
//!   - `InvalidChars(field)` -> ``"`FIELD` may only contain letters, digits and `_`"``
//!   - `MissingDigit(field)` -> ``"`FIELD` must contain a digit"``
//!   - `Mismatch` -> `"passwords do not match"`
//!   - `OutOfRange { field, min, max }` -> ``"`FIELD` must be between MIN and MAX"``
//!
//! ## Examples
//!
//! ```text
//! validate(username "ann_1", password "s3cretpass", confirm "s3cretpass", age 30) -> Ok(())
//! validate(username "a!", password "short", confirm "shorter", age 7)
//!   -> Err([TooShort { field: "username", min: 3 }, InvalidChars("username"),
//!           TooShort { field: "password", min: 8 }, MissingDigit("password"),
//!           Mismatch, OutOfRange { field: "age", min: 13, max: 120 }])
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Write the checks as an array of `(passed: bool, error: FieldError)` pairs.
//! 2. `passed_checks.into_iter().filter_map(|(ok, err)| (!ok).then_some(err))` keeps the failures.
//! 3. `validate(r).err()` turns a `Result` into an `Option` of the error list.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** evaluate every rule up front into a table of `(passed, error)` pairs; the failing
//! rows are the answer. Nothing short-circuits, so the user sees every problem at once.
//! `invalid_entries` reuses `validate` and keeps only the `Err` cases.
//!
//! **Rust concepts:**
//! - `Result<(), Vec<FieldError>>` is a perfectly valid result type: success carries nothing,
//!   failure carries the full list.
//! - `bool::then_some(value)` returns `Some(value)` when the bool is `true`; it is evaluated
//!   eagerly, which is fine because every error value is cheap to build.
//! - `enumerate` + `filter_map` + `Result::err` + `Option::map` collects `(index, errors)` pairs.
//! - Building all errors (instead of stopping) is a design choice; `?` is still the right tool when
//!   later steps depend on earlier ones.
// @solution-end

use std::fmt;

#[derive(Debug, Clone)]
pub struct Registration {
    pub username: String,
    pub password: String,
    pub confirm: String,
    pub age: u32,
}

#[derive(Debug, PartialEq)]
pub enum FieldError {
    TooShort {
        field: &'static str,
        min: usize,
    },
    InvalidChars(&'static str),
    MissingDigit(&'static str),
    Mismatch,
    OutOfRange {
        field: &'static str,
        min: u32,
        max: u32,
    },
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // @solution-start
        match self {
            FieldError::TooShort { field, min } => {
                write!(f, "`{field}` must have at least {min} characters")
            }
            FieldError::InvalidChars(field) => {
                write!(f, "`{field}` may only contain letters, digits and `_`")
            }
            FieldError::MissingDigit(field) => write!(f, "`{field}` must contain a digit"),
            FieldError::Mismatch => write!(f, "passwords do not match"),
            FieldError::OutOfRange { field, min, max } => {
                write!(f, "`{field}` must be between {min} and {max}")
            }
        }
        // @solution-end
    }
}

pub fn validate(r: &Registration) -> Result<(), Vec<FieldError>> {
    // @solution-start
    let checks = [
        (
            r.username.chars().count() >= 3,
            FieldError::TooShort {
                field: "username",
                min: 3,
            },
        ),
        (
            r.username
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_'),
            FieldError::InvalidChars("username"),
        ),
        (
            r.password.chars().count() >= 8,
            FieldError::TooShort {
                field: "password",
                min: 8,
            },
        ),
        (
            r.password.chars().any(|c| c.is_ascii_digit()),
            FieldError::MissingDigit("password"),
        ),
        (r.confirm == r.password, FieldError::Mismatch),
        (
            (13..=120).contains(&r.age),
            FieldError::OutOfRange {
                field: "age",
                min: 13,
                max: 120,
            },
        ),
    ];
    let errors: Vec<FieldError> = checks
        .into_iter()
        .filter_map(|(passed, error)| (!passed).then_some(error))
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
    // @solution-end
}

pub fn invalid_entries(rs: &[Registration]) -> Vec<(usize, Vec<FieldError>)> {
    // @solution-start
    rs.iter()
        .enumerate()
        .filter_map(|(i, r)| validate(r).err().map(|errors| (i, errors)))
        .collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reg(username: &str, password: &str, confirm: &str, age: u32) -> Registration {
        Registration {
            username: username.to_string(),
            password: password.to_string(),
            confirm: confirm.to_string(),
            age,
        }
    }

    #[test]
    fn valid_registration() {
        assert_eq!(
            validate(&reg("ann_1", "s3cretpass", "s3cretpass", 30)),
            Ok(())
        );
        assert_eq!(validate(&reg("abc", "12345678", "12345678", 13)), Ok(()));
    }

    #[test]
    fn every_rule_fails() {
        assert_eq!(
            validate(&reg("a!", "short", "shorter", 7)),
            Err(vec![
                FieldError::TooShort {
                    field: "username",
                    min: 3
                },
                FieldError::InvalidChars("username"),
                FieldError::TooShort {
                    field: "password",
                    min: 8
                },
                FieldError::MissingDigit("password"),
                FieldError::Mismatch,
                FieldError::OutOfRange {
                    field: "age",
                    min: 13,
                    max: 120
                },
            ])
        );
    }

    #[test]
    fn single_failures() {
        assert_eq!(
            validate(&reg("bob", "longpassword", "longpassword", 40)),
            Err(vec![FieldError::MissingDigit("password")])
        );
        assert_eq!(
            validate(&reg("bob", "longpass1", "longpass2", 121)),
            Err(vec![
                FieldError::Mismatch,
                FieldError::OutOfRange {
                    field: "age",
                    min: 13,
                    max: 120
                }
            ])
        );
        assert_eq!(
            validate(&reg("bøb", "longpass1", "longpass1", 20)),
            Err(vec![FieldError::InvalidChars("username")])
        );
    }

    #[test]
    fn invalid_entries_keep_indexes() {
        let all = vec![
            reg("good_one", "passw0rd!", "passw0rd!", 20),
            reg("x", "passw0rd!", "passw0rd!", 20),
            reg("good_two", "passw0rd!", "passw0rd!", 20),
            reg("ok_name", "passw0rd!", "passw0rd!", 5),
        ];
        let bad = invalid_entries(&all);
        assert_eq!(bad.len(), 2);
        assert_eq!(bad[0].0, 1);
        assert_eq!(bad[1].0, 3);
        assert_eq!(
            bad[1].1,
            vec![FieldError::OutOfRange {
                field: "age",
                min: 13,
                max: 120
            }]
        );
    }

    #[test]
    fn display_messages() {
        assert_eq!(
            FieldError::TooShort {
                field: "username",
                min: 3
            }
            .to_string(),
            "`username` must have at least 3 characters"
        );
        assert_eq!(
            FieldError::InvalidChars("username").to_string(),
            "`username` may only contain letters, digits and `_`"
        );
        assert_eq!(
            FieldError::MissingDigit("password").to_string(),
            "`password` must contain a digit"
        );
        assert_eq!(FieldError::Mismatch.to_string(), "passwords do not match");
        assert_eq!(
            FieldError::OutOfRange {
                field: "age",
                min: 13,
                max: 120
            }
            .to_string(),
            "`age` must be between 13 and 120"
        );
    }
}
