//! # Problem 0909: Function Composition with `compose!`
//!
//! - **Difficulty:** Medium
//! - **Topics:** recursive macros that build closures, `move` closures, `=>` separators, closure type inference
//! - **Source:** Original · [The Rust Book 20.5](https://doc.rust-lang.org/book/ch20-05-macros.html) · [The Rust Book 13.1: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
//!
//! ## Problem
//!
//! A generic `compose(f, g)` function is easy; composing **any number** of functions is not, because
//! every intermediate type may differ. A macro side-steps the problem: it writes the nested calls
//! for you. Implement two macros:
//!
//! - `compose!(f1, f2, ..., fn)` evaluates to a **closure** `x -> fn(...f2(f1(x)))`: the functions
//!   are applied **left to right**. `compose!(f)` is just `f`. Each function expression is evaluated
//!   exactly once, when `compose!` runs (not on every call).
//! - `pipe!(value => f1 => f2 => ...)` evaluates to `...f2(f1(value))` immediately;
//!   `pipe!(value)` is `value`.
//!
//! Then use them:
//!
//! - `slugify() -> impl Fn(&str) -> String`: trim, lowercase, then replace every run of
//!   non-alphanumeric characters with a single `-` and drop leading/trailing dashes. Build it with
//!   `compose!` from small closures.
//! - `digits_sum_squared(n: u64) -> u64`: the square of the sum of the decimal digits of `n`,
//!   written as a `pipe!`.
//!
//! ## Examples
//!
//! ```text
//! let f = compose!(|x: i32| x + 1, |x| x * 2, |x: i32| x.to_string());
//! f(3)                                   => "8"
//! pipe!(3 => |x: i32| x + 1 => |x| x * 2) => 8
//! slugify()("  Hello, World!! ")         => "hello-world"
//! digits_sum_squared(1234)               => 100
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Recursive arm for `compose!`: bind `let first = $f;` and `let rest = compose!(...);`, then return
//!    `move |x| rest(first(x))`.
//! 2. `pipe!` needs no recursion: `let v = $value; $( let v = ($f)(v); )* v`.
//! 3. Wrap each function in parentheses when calling it, so `(|x| x + 1)(v)` parses.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `compose!` peels off the first function, composes the rest recursively, and returns a
//! closure that feeds the output of the first into the composition of the rest. Both pieces are
//! moved into the closure, so they are evaluated once. `pipe!` simply shadows a variable once per
//! stage.
//!
//! **Rust concepts:**
//! - The macro produces nested **closures**, each with its own anonymous type; no generic `compose`
//!   function with dozens of type parameters is needed.
//! - Closure parameter types are inferred from how the closure is used later in the same function,
//!   which is why `|x| x * 2` works after `|x: i32| x + 1`.
//! - `$value:expr` may be followed by `=>` (it is in the follow set of `expr`).
//! - Returning `impl Fn(&str) -> String` hides the concrete closure type from the caller.
// @solution-end

macro_rules! compose {
    // @solution-start replace: ($($t:tt)*) => { |_| todo!() };
    ($f:expr $(,)?) => {
        $f
    };
    ($f:expr, $($rest:expr),+ $(,)?) => {{
        let first = $f;
        let rest = compose!($($rest),+);
        move |x| rest(first(x))
    }};
    // @solution-end
}

macro_rules! pipe {
    // @solution-start replace: ($($t:tt)*) => { todo!() };
    ($value:expr $(=> $f:expr)*) => {{
        let value = $value;
        $( let value = ($f)(value); )*
        value
    }};
    // @solution-end
}

/// Builds a slug: `"  Hello, World!! "` becomes `"hello-world"`.
pub fn slugify() -> impl Fn(&str) -> String {
    // @solution-start replace: |_: &str| todo!()
    compose!(|s: &str| s.trim().to_lowercase(), |s: String| {
        s.split(|c: char| !c.is_alphanumeric())
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    },)
    // @solution-end
}

/// `(sum of the decimal digits of n)^2`, written with `pipe!`.
pub fn digits_sum_squared(n: u64) -> u64 {
    // @solution-start
    pipe!(n
        => |n: u64| n.to_string()
        => |s: String| s.bytes().map(|b| u64::from(b - b'0')).sum::<u64>()
        => |sum: u64| sum * sum)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn compose_applies_left_to_right() {
        let f = compose!(|x: i32| x + 1, |x| x * 2, |x: i32| x.to_string());
        let out: String = f(3);
        assert_eq!(out, "8");
        let g = compose!(|x: i32| x * 2, |x: i32| x + 1);
        let out: i32 = g(3);
        assert_eq!(out, 7);
    }

    #[test]
    fn compose_single_function_and_trailing_comma() {
        let neg = compose!(|x: i64| -x,);
        let out: i64 = neg(5);
        assert_eq!(out, -5);
        let twice = compose!(str::len, |n: usize| n * 2);
        let out: usize = twice("abc");
        assert_eq!(out, 6);
    }

    #[test]
    fn function_expressions_are_evaluated_once() {
        let built = Cell::new(0);
        let make_adder = |k: i32| {
            built.set(built.get() + 1);
            move |x: i32| x + k
        };
        let f = compose!(make_adder(1), make_adder(10), make_adder(100));
        assert_eq!(built.get(), 3);
        let results: Vec<i32> = (0..5).map(&f).collect();
        assert_eq!(results, vec![111, 112, 113, 114, 115]);
        assert_eq!(built.get(), 3);
    }

    #[test]
    fn pipe_runs_immediately() {
        let eight: i32 = pipe!(3 => |x: i32| x + 1 => |x| x * 2);
        let same: &str = pipe!("unchanged");
        let shout: String = pipe!("hi" => str::to_uppercase => |s: String| s + "!");
        assert_eq!((eight, same, shout.as_str()), (8, "unchanged", "HI!"));
    }

    #[test]
    fn slugify_and_digits() {
        let slug = slugify();
        assert_eq!(slug("  Hello, World!! "), "hello-world");
        assert_eq!(slug("Rust 2024 -- Edition"), "rust-2024-edition");
        assert_eq!(slug("***"), "");
        assert_eq!(digits_sum_squared(1234), 100);
        assert_eq!(digits_sum_squared(0), 0);
        assert_eq!(digits_sum_squared(u64::MAX), 87 * 87);
    }
}
