//! # Problem 0924: Reading Panic Payloads with `dyn Any`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `std::any::Any`, `downcast_ref`, `Box<dyn Any + Send>`, `std::panic::catch_unwind`, `panic_any`, `UnwindSafe`
//! - **Source:** Original · [std: `any`](https://doc.rust-lang.org/std/any/index.html) · [std: `panic::catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)
//!
//! ## Problem
//!
//! When a thread panics, the value passed to `panic!` travels as a `Box<dyn Any + Send>`: the
//! **payload**. `panic!("literal")` produces a `&'static str` payload, `panic!("{x}")` a `String`,
//! and `std::panic::panic_any(value)` any `Send + 'static` value. Test harnesses and thread pools
//! downcast the payload to report a readable message.
//!
//! Implement:
//!
//! - `panic_message(payload: &(dyn Any + Send)) -> String`:
//!   - a `&'static str` or a `String` payload: the text itself;
//!   - an `i32` payload: `"panic with code <n>"`;
//!   - anything else: `"unknown panic payload"`.
//! - `run_catching<R>(f: impl FnOnce() -> R + UnwindSafe) -> Result<R, String>`: runs `f`; returns
//!   `Ok(result)`, or `Err(message)` if it panicked, using `catch_unwind` and `panic_message`.
//! - `describe(value: &dyn Any) -> String` for a few concrete types:
//!   `i32` → `"i32 7"`, `f64` → `"f64 2.5"`, `bool` → `"bool true"`, `String` → `"String \"hi\""`
//!   (Debug format), `Vec<i32>` → `"Vec<i32> of length 3"`, otherwise `"something else"`.
//! - `sum_numbers(values: &[Box<dyn Any>]) -> f64`: the sum of every `i32`, `i64` and `f64` in the
//!   slice (converted to `f64`), ignoring all other values.
//!
//! ## Examples
//!
//! ```text
//! run_catching(|| 2 + 2)                                => Ok(4)
//! run_catching(|| -> i32 { panic!("boom") })            => Err("boom")
//! run_catching(|| -> i32 { panic!("bad id {}", 7) })    => Err("bad id 7")
//! run_catching(|| -> i32 { std::panic::panic_any(42) }) => Err("panic with code 42")
//! describe(&vec![1, 2, 3])                               => "Vec<i32> of length 3"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints. (The default panic hook still prints the panic messages of the tests to
//!   standard error; that is expected.)
//!
//! ## Hints
//!
//! 1. `payload.downcast_ref::<&'static str>()` returns `Option<&&'static str>`.
//! 2. Try the candidate types one after another with `if let Some(x) = value.downcast_ref::<T>()`.
//! 3. `catch_unwind` returns `Result<R, Box<dyn Any + Send>>`; pass `&*payload` to `panic_message`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** downcasting is a runtime type check: `downcast_ref::<T>()` compares the value's
//! `TypeId` with `T`'s and returns a typed reference on a match. Each function tries the supported
//! types in turn. `run_catching` maps the `Err(payload)` of `catch_unwind` through `panic_message`.
//!
//! **Rust concepts:**
//! - `dyn Any` only supports `'static` types, because a `TypeId` cannot describe lifetimes.
//! - A string literal payload is a `&'static str`, a formatted one is a `String`: both must be
//!   handled, which is exactly what std's default panic hook does.
//! - `UnwindSafe` is a marker trait that `catch_unwind` requires; closures that only capture owned or
//!   `Copy` data implement it automatically, `AssertUnwindSafe` opts in explicitly.
//! - `Box<dyn Any>` in a slice is a heterogeneous collection; `&**boxed` or `boxed.as_ref()` gives
//!   the `&dyn Any` inside.
// @solution-end

use std::any::Any;
use std::panic::{self, UnwindSafe};

pub fn panic_message(payload: &(dyn Any + Send)) -> String {
    // @solution-start
    if let Some(text) = payload.downcast_ref::<&'static str>() {
        text.to_string()
    } else if let Some(text) = payload.downcast_ref::<String>() {
        text.clone()
    } else if let Some(code) = payload.downcast_ref::<i32>() {
        format!("panic with code {code}")
    } else {
        "unknown panic payload".to_string()
    }
    // @solution-end
}

pub fn run_catching<R>(f: impl FnOnce() -> R + UnwindSafe) -> Result<R, String> {
    // @solution-start
    panic::catch_unwind(f).map_err(|payload| panic_message(&*payload))
    // @solution-end
}

pub fn describe(value: &dyn Any) -> String {
    // @solution-start
    if let Some(n) = value.downcast_ref::<i32>() {
        format!("i32 {n}")
    } else if let Some(x) = value.downcast_ref::<f64>() {
        format!("f64 {x}")
    } else if let Some(b) = value.downcast_ref::<bool>() {
        format!("bool {b}")
    } else if let Some(s) = value.downcast_ref::<String>() {
        format!("String {s:?}")
    } else if let Some(v) = value.downcast_ref::<Vec<i32>>() {
        format!("Vec<i32> of length {}", v.len())
    } else {
        "something else".to_string()
    }
    // @solution-end
}

pub fn sum_numbers(values: &[Box<dyn Any>]) -> f64 {
    // @solution-start
    values
        .iter()
        .map(|value| {
            let value: &dyn Any = value.as_ref();
            if let Some(&n) = value.downcast_ref::<i32>() {
                f64::from(n)
            } else if let Some(&n) = value.downcast_ref::<i64>() {
                n as f64
            } else {
                value.downcast_ref::<f64>().copied().unwrap_or(0.0)
            }
        })
        .sum()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_from_real_panics() {
        assert_eq!(run_catching(|| 2 + 2), Ok(4));
        assert_eq!(
            run_catching(|| -> i32 { panic!("boom") }),
            Err("boom".to_string())
        );
        assert_eq!(
            run_catching(|| -> i32 { panic!("bad id {}", 7) }),
            Err("bad id 7".to_string())
        );
        assert_eq!(
            run_catching(|| -> i32 { std::panic::panic_any(42) }),
            Err("panic with code 42".to_string())
        );
        assert_eq!(
            run_catching(|| -> i32 { std::panic::panic_any(1.5_f32) }),
            Err("unknown panic payload".to_string())
        );
    }

    #[test]
    fn payload_types_directly() {
        let literal: Box<dyn Any + Send> = Box::new("static text");
        let formatted: Box<dyn Any + Send> = Box::new(String::from("owned text"));
        assert_eq!(panic_message(&*literal), "static text");
        assert_eq!(panic_message(&*formatted), "owned text");
        assert_eq!(panic_message(&()), "unknown panic payload");
    }

    #[test]
    fn index_out_of_bounds_message() {
        let v: Vec<i32> = (1..=3).collect();
        let result = run_catching(move || v[10]);
        let message = result.unwrap_err();
        assert!(message.contains("index out of bounds"), "{message}");
    }

    #[test]
    fn describe_values() {
        assert_eq!(describe(&7_i32), "i32 7");
        assert_eq!(describe(&2.5_f64), "f64 2.5");
        assert_eq!(describe(&true), "bool true");
        assert_eq!(describe(&String::from("hi")), "String \"hi\"");
        assert_eq!(describe(&vec![1, 2, 3]), "Vec<i32> of length 3");
        assert_eq!(describe(&7_u8), "something else");
        assert_eq!(describe(&"a &str is not a String"), "something else");
    }

    #[test]
    fn heterogeneous_sum() {
        let values: Vec<Box<dyn Any>> = vec![
            Box::new(1_i32),
            Box::new(2_i64),
            Box::new(0.5_f64),
            Box::new("ignored"),
            Box::new(vec![100]),
            Box::new(10_u32),
        ];
        assert_eq!(sum_numbers(&values), 3.5);
        assert_eq!(sum_numbers(&[]), 0.0);
    }
}
