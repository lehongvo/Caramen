// ==================================================================
// PRACTICE 0202: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0202_build_user.md
// Run the tests:          cargo test practice::p0202
// ==================================================================

//! # Problem 0202: Build a User with Field Init Shorthand
//!
//! - **Difficulty:** Easy
//! - **Topics:** struct literals, field init shorthand, mutable struct instances, `String` fields
//! - **Source:** Original · [The Rust Book 5.1](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
//!
//! ## Problem
//!
//! A user account is stored in this struct:
//!
//! ```text
//! pub struct User {
//!     pub active: bool,
//!     pub username: String,
//!     pub email: String,
//!     pub sign_in_count: u64,
//! }
//! ```
//!
//! Implement:
//!
//! - `build_user(email: String, username: String) -> User`: a new account that is `active`, has
//!   the given `email` and `username`, and `sign_in_count == 1`. When a parameter has the same name
//!   as a field you can write just `email` instead of `email: email` (*field init shorthand*).
//!   Use the shorthand for both string fields.
//! - `sign_in(user: &mut User)`: records one more sign-in. It adds `1` to `sign_in_count` and sets
//!   `active` back to `true` (signing in reactivates an account).
//!
//! ## Examples
//!
//! ```text
//! build_user("ana@example.com", "ana")
//!   => User { active: true, username: "ana", email: "ana@example.com", sign_in_count: 1 }
//!
//! let mut u = build_user("bo@example.com", "bo");
//! u.active = false;
//! sign_in(&mut u);
//!   => u.active == true, u.sign_in_count == 2
//! ```
//!
//! ## Constraints
//!
//! - `sign_in_count` never exceeds `10^9` in the tests.
//!
//! ## Hints
//!
//! 1. The struct literal `User { active: true, username, email, sign_in_count: 1 }` mixes the
//!    shorthand with ordinary `field: value` pairs; the order of fields does not matter.
//! 2. Through a `&mut User` you can assign to fields: `user.sign_in_count += 1;`.

/// A user account.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

/// Creates an active user that has signed in once.
pub fn build_user(email: String, username: String) -> User {
    todo!("Write your solution here")
}

/// Records one more sign-in and reactivates the account.
pub fn sign_in(user: &mut User) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_build_user() {
        let user = build_user(String::from("ana@example.com"), String::from("ana"));
        assert_eq!(
            user,
            User {
                active: true,
                username: String::from("ana"),
                email: String::from("ana@example.com"),
                sign_in_count: 1,
            }
        );
    }

    #[test]
    fn example_sign_in_reactivates() {
        let mut user = build_user(String::from("bo@example.com"), String::from("bo"));
        user.active = false;
        sign_in(&mut user);
        assert!(user.active);
        assert_eq!(user.sign_in_count, 2);
    }

    #[test]
    fn parameters_are_not_swapped() {
        let user = build_user(String::from("mail"), String::from("name"));
        assert_eq!(user.email, "mail");
        assert_eq!(user.username, "name");
    }

    #[test]
    fn many_sign_ins() {
        let mut user = build_user(String::from("c@x.org"), String::from("c"));
        for _ in 0..999 {
            sign_in(&mut user);
        }
        assert_eq!(user.sign_in_count, 1000);
        assert!(user.active);
    }

    #[test]
    fn empty_strings_are_allowed() {
        let user = build_user(String::new(), String::new());
        assert!(user.email.is_empty() && user.username.is_empty());
        assert_eq!(user.sign_in_count, 1);
    }
}
