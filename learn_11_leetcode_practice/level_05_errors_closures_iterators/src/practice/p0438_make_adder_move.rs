// ==================================================================
// PRACTICE 0438: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0438_make_adder_move.md
// Run the tests:          cargo test practice::p0438
// ==================================================================

//! # Problem 0438: Make Adders and Multipliers with move
//!
//! - **Difficulty:** Easy
//! - **Topics:** returning closures with `impl Fn`, the `move` keyword, closures owning a `String`, precise capturing `use<>`
//! - **Source:** Original · [Rust By Example: Closures as output parameters](https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html) · [The Rust Book 13.1: Capturing references or moving ownership](https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-references-or-moving-ownership)
//!
//! ## Problem
//!
//! A function can **return** a closure. Because the closure's type has no name, the return type
//! is written `impl Fn(i32) -> i32` ("some type that implements this closure trait"). The closure
//! usually needs data from the function's parameters, but those die when the function returns, so
//! the closure must take ownership of them: that is what `move` does.
//!
//! Implement these *closure factories*:
//!
//! - `make_adder(n: i32) -> impl Fn(i32) -> i32`: a closure that adds `n`.
//! - `make_multiplier(n: i32) -> impl Fn(i32) -> i32`: a closure that multiplies by `n`.
//! - `make_clamp(lo: i32, hi: i32) -> impl Fn(i32) -> i32`: a closure that forces its argument into
//!   `lo..=hi` (you may assume `lo <= hi`).
//! - `make_greeter(greeting: &str) -> impl Fn(&str) -> String + use<>`: a closure that returns
//!   `"GREETING, NAME!"`. The closure must own its own copy of the greeting, so that it can be
//!   used after the caller's string is gone.
//!
//! About `+ use<>`: in edition 2024, a returned `impl Trait` is assumed to capture **every**
//! lifetime of the function's parameters, here the lifetime of `greeting`. The compiler would then
//! treat the closure as borrowing `greeting` even though it does not. The precise-capturing syntax
//! `use<>` (stable since Rust 1.82) lists what the hidden type may capture: nothing at all.
//!
//! ## Examples
//!
//! ```text
//! let add5 = make_adder(5);          add5(10) -> 15,  add5(-5) -> 0
//! let triple = make_multiplier(3);   triple(7) -> 21
//! let pct = make_clamp(0, 100);      pct(150) -> 100,  pct(-3) -> 0,  pct(42) -> 42
//! let hello = make_greeter("Hello"); hello("Ann") -> "Hello, Ann!"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints; the tests never overflow.
//!
//! ## Hints
//!
//! 1. `move |x| x + n` copies `n` into the closure.
//! 2. `i32::clamp(self, min, max)` already exists.
//! 3. In `make_greeter`, convert the `&str` into a `String` first, then `move` that `String` into the
//!    closure. Returning a closure that borrows `greeting` would tie it to the caller's string.

pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |_| todo!()
}

pub fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |_| todo!()
}

pub fn make_clamp(lo: i32, hi: i32) -> impl Fn(i32) -> i32 {
    move |_| todo!()
}

pub fn make_greeter(greeting: &str) -> impl Fn(&str) -> String + use<> {
    move |_| todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adders() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(-5), 0);
        let sub3 = make_adder(-3);
        assert_eq!(sub3(add5(1)), 3);
    }

    #[test]
    fn multipliers() {
        let triple = make_multiplier(3);
        assert_eq!(triple(7), 21);
        assert_eq!(make_multiplier(0)(123), 0);
        let fns = [make_multiplier(2), make_multiplier(10)];
        assert_eq!(fns.iter().map(|f| f(4)).collect::<Vec<_>>(), vec![8, 40]);
    }

    #[test]
    fn clamps() {
        let pct = make_clamp(0, 100);
        assert_eq!(pct(150), 100);
        assert_eq!(pct(-3), 0);
        assert_eq!(pct(42), 42);
        assert_eq!(make_clamp(5, 5)(-100), 5);
    }

    #[test]
    fn greeters_own_their_text() {
        let hello = {
            let word = String::from("Hello");
            make_greeter(&word)
            // `word` is dropped here; the closure has its own copy.
        };
        assert_eq!(hello("Ann"), "Hello, Ann!");
        let hi = make_greeter("Hi");
        assert_eq!(hi("Bob"), "Hi, Bob!");
        assert_eq!(hello("Cy"), "Hello, Cy!");
    }
}
