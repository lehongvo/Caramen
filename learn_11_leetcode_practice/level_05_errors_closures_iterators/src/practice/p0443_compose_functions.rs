// ==================================================================
// PRACTICE 0443: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0443_compose_functions.md
// Run the tests:          cargo test practice::p0443
// ==================================================================

//! # Problem 0443: Compose Functions
//!
//! - **Difficulty:** Medium
//! - **Topics:** generic closures `impl Fn(A) -> B`, returning a closure that captures closures, `fold` over `Vec<Box<dyn Fn>>`, collecting into `Option<Vec<_>>`
//! - **Source:** Original · [Rust By Example: Closures as output parameters](https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html) · [std `Iterator::fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)
//!
//! ## Problem
//!
//! *Composing* two functions `f` and `g` produces a new function "first `f`, then `g`":
//! `compose(f, g)(x) == g(f(x))`. With closures this is a one-liner, and it generalises to whole
//! chains of steps chosen at run time.
//!
//! Implement:
//!
//! - `compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C`.
//! - `repeat<F: Fn(i64) -> i64>(f: F, n: usize) -> impl Fn(i64) -> i64`: `f` applied `n` times.
//! - `compose_all(fs: Vec<Box<dyn Fn(i64) -> i64>>) -> Box<dyn Fn(i64) -> i64>`: the composition of
//!   all functions **in order** (`fs[0]` runs first). An empty list gives the identity function.
//! - `step(name: &str) -> Option<Box<dyn Fn(i64) -> i64>>` for `"inc"` (+1), `"double"` (*2),
//!   `"square"` (x*x) and `"negate"` (-x); `None` for any other name.
//! - `pipeline(names: &[&str]) -> Option<Box<dyn Fn(i64) -> i64>>`: the composition of the named
//!   steps in order, or `None` if any name is unknown.
//!
//! ## Examples
//!
//! ```text
//! let len_then_double = compose(|s: &str| s.len(), |n: usize| n * 2);
//! len_then_double("abc")                           -> 6
//! repeat(|x| x + 3, 4)(10)                         -> 22
//! compose_all(vec![Box::new(|x| x + 1), Box::new(|x| x * 10)])(4) -> 50     ((4 + 1) * 10)
//! compose_all(vec![])(7)                           -> 7
//! pipeline(&["inc", "square", "negate"]).map(|f| f(2))  -> Some(-9)
//! pipeline(&["inc", "cube"]).is_none()             -> true
//! ```
//!
//! ## Constraints
//!
//! - No special constraints; the tests never overflow.
//!
//! ## Hints
//!
//! 1. `move |x| g(f(x))`: the returned closure must own `f` and `g`.
//! 2. `compose_all` is a `fold` that starts with the identity `Box::new(|x| x)` and wraps the
//!    accumulated function in a new box at every step.
//! 3. `names.iter().map(|n| step(n)).collect::<Option<Vec<_>>>()` gives all steps or `None`.

pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |_| todo!()
}

pub fn repeat<F: Fn(i64) -> i64>(f: F, n: usize) -> impl Fn(i64) -> i64 {
    move |_| todo!()
}

pub fn compose_all(fs: Vec<Box<dyn Fn(i64) -> i64>>) -> Box<dyn Fn(i64) -> i64> {
    Box::new(|_| todo!())
}

pub fn step(name: &str) -> Option<Box<dyn Fn(i64) -> i64>> {
    todo!("Write your solution here")
}

pub fn pipeline(names: &[&str]) -> Option<Box<dyn Fn(i64) -> i64>> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_two_functions_of_different_types() {
        let len_then_double = compose(|s: &str| s.len(), |n: usize| n * 2);
        assert_eq!(len_then_double("abc"), 6);
        assert_eq!(len_then_double(""), 0);
        let describe = compose(|x: i32| x * x, |sq: i32| format!("square = {sq}"));
        assert_eq!(describe(-4), "square = 16");
    }

    #[test]
    fn compose_order_matters() {
        let add_then_mul = compose(|x: i64| x + 1, |x: i64| x * 10);
        let mul_then_add = compose(|x: i64| x * 10, |x: i64| x + 1);
        assert_eq!(add_then_mul(4), 50);
        assert_eq!(mul_then_add(4), 41);
    }

    #[test]
    fn repeat_n_times() {
        assert_eq!(repeat(|x| x + 3, 4)(10), 22);
        assert_eq!(repeat(|x| x * 2, 0)(5), 5);
        let twice_double = repeat(|x| x * 2, 2);
        assert_eq!(twice_double(1), 4);
        assert_eq!(twice_double(3), 12);
    }

    #[test]
    fn compose_all_in_order() {
        let f = compose_all(vec![Box::new(|x| x + 1), Box::new(|x| x * 10)]);
        assert_eq!(f(4), 50);
        assert_eq!(compose_all(vec![])(7), 7);
        let many: Vec<Box<dyn Fn(i64) -> i64>> = (0..1000)
            .map(|_| Box::new(|x: i64| x + 1) as Box<dyn Fn(i64) -> i64>)
            .collect();
        assert_eq!(compose_all(many)(0), 1000);
    }

    #[test]
    fn pipelines_by_name() {
        assert_eq!(
            pipeline(&["inc", "square", "negate"]).map(|f| f(2)),
            Some(-9)
        );
        assert_eq!(
            pipeline(&["double", "double", "inc"]).map(|f| f(5)),
            Some(21)
        );
        assert_eq!(pipeline(&[]).map(|f| f(8)), Some(8));
        assert!(pipeline(&["inc", "cube"]).is_none());
        assert!(step("cube").is_none());
        assert_eq!(step("negate").map(|f| f(3)), Some(-3));
    }
}
