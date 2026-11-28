// ==================================================================
// PRACTICE 0566: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0566_static_lifetime.md
// Run the tests:          cargo test practice::p0566
// ==================================================================

//! # Problem 0566: The `'static` Lifetime
//!
//! - **Difficulty:** Easy
//! - **Topics:** `&'static str` from literals and `static` tables, structs storing `&'static str`, the `T: 'static` bound with `std::thread::spawn`
//! - **Source:** Original · [The Rust Book 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-static-lifetime)
//!
//! ## Problem
//!
//! `'static` appears in two different places:
//!
//! - As a **reference** lifetime, `&'static str` means "this string lives for the whole program", which
//!   is true for string literals and `static` items.
//! - As a **bound**, `T: 'static` means "`T` contains no references shorter than `'static`". Owned types
//!   such as `String` or `Vec<i32>` satisfy it, even though their values are dropped eventually.
//!
//! Implement:
//!
//! - `weekday_name(n: u8) -> Option<&'static str>`: `1` is `"Monday"`, ..., `7` is `"Sunday"`; read the
//!   names from a `static` array.
//! - `http_reason(code: u16) -> &'static str`: `200` `"OK"`, `201` `"Created"`, `204` `"No Content"`,
//!   `301` `"Moved Permanently"`, `400` `"Bad Request"`, `404` `"Not Found"`, `500`
//!   `"Internal Server Error"`, anything else `"Unknown"`.
//! - `Registry { names: Vec<&'static str> }` with `new()`, `register(&mut self, name: &'static str)`
//!   (ignores duplicates) and `names(&self) -> &[&'static str]`. Because the struct stores
//!   `&'static str`, it needs no lifetime parameter, but only literals (or other `'static` strings) can
//!   be registered.
//! - `run_in_thread<T, F>(value: T, f: F) -> T where T: Send + 'static, F: FnOnce(T) -> T + Send +
//!   'static`: runs `f(value)` on a new thread (`std::thread::spawn`), waits for it with `join` and
//!   returns the result. `spawn` requires `'static` because the thread might outlive the caller.
//!
//! ## Examples
//!
//! ```text
//! weekday_name(1)     => Some("Monday")
//! weekday_name(9)     => None
//! http_reason(404)    => "Not Found"
//! let mut r = Registry::new(); r.register("cache"); r.register("cache"); r.names() => ["cache"]
//! run_in_thread(String::from("hi"), |s| s + "!")   => "hi!"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `static DAYS: [&str; 7] = [...]`; inside a `static`, `&str` already means `&'static str`.
//! 2. `std::thread::spawn(move || f(value)).join()` returns a `Result`; a panic in the thread is the
//!    only error, so `expect` is fine here.

use std::thread;

/// The English name of weekday `n` (`1` is Monday).
pub fn weekday_name(n: u8) -> Option<&'static str> {
    todo!("Write your solution here")
}

/// The standard reason phrase of a few HTTP status codes.
pub fn http_reason(code: u16) -> &'static str {
    todo!("Write your solution here")
}

/// A list of component names that live for the whole program.
#[derive(Debug, Default)]
pub struct Registry {
    names: Vec<&'static str>,
}

impl Registry {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn register(&mut self, name: &'static str) {
        todo!("Write your solution here")
    }

    pub fn names(&self) -> &[&'static str] {
        todo!("Write your solution here")
    }
}

/// Runs `f(value)` on another thread and returns its result.
pub fn run_in_thread<T, F>(value: T, f: F) -> T
where
    T: Send + 'static,
    F: FnOnce(T) -> T + Send + 'static,
{
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weekday_names() {
        assert_eq!(weekday_name(1), Some("Monday"));
        assert_eq!(weekday_name(7), Some("Sunday"));
        assert_eq!(weekday_name(0), None);
        assert_eq!(weekday_name(9), None);
        // A `&'static str` can be stored anywhere, for as long as we like.
        let kept: &'static str = weekday_name(3).unwrap();
        assert_eq!(kept, "Wednesday");
    }

    #[test]
    fn http_reasons() {
        assert_eq!(http_reason(200), "OK");
        assert_eq!(http_reason(404), "Not Found");
        assert_eq!(http_reason(500), "Internal Server Error");
        assert_eq!(http_reason(418), "Unknown");
    }

    #[test]
    fn registry_of_static_names() {
        let mut r = Registry::new();
        r.register("cache");
        r.register("logger");
        r.register("cache");
        assert_eq!(r.names(), &["cache", "logger"]);
        assert!(Registry::default().names().is_empty());
    }

    #[test]
    fn owned_values_are_static() {
        assert_eq!(run_in_thread(String::from("hi"), |s| s + "!"), "hi!");
        let doubled = run_in_thread(vec![1, 2, 3], |v| v.into_iter().map(|x| x * 2).collect());
        assert_eq!(doubled, vec![2, 4, 6]);
        let offset = 10;
        assert_eq!(run_in_thread(5, move |x| x + offset), 15);
    }
}
