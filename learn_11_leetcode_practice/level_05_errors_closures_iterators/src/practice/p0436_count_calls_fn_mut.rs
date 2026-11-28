// ==================================================================
// PRACTICE 0436: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0436_count_calls_fn_mut.md
// Run the tests:          cargo test practice::p0436
// ==================================================================

//! # Problem 0436: Count Calls with FnMut
//!
//! - **Difficulty:** Easy
//! - **Topics:** `FnMut`, closures capturing by mutable reference, `mut f: F`, returning `impl FnMut`, `move`
//! - **Source:** Original · [The Rust Book 13.1: Moving captured values out of closures and the `Fn` traits](https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures-and-the-fn-traits) · [std `FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html)
//!
//! ## Problem
//!
//! A closure that **changes** something it captured (`|| count += 1`) does not implement `Fn`,
//! only `FnMut`: calling it needs mutable access to the closure itself. A function that accepts
//! such closures uses the bound `F: FnMut(...)` and must hold the closure in a `mut` binding.
//!
//! Implement:
//!
//! - `call_n_times<F: FnMut()>(n: usize, f: F)`: call `f` exactly `n` times.
//! - `call_with_indexes<F: FnMut(usize)>(n: usize, f: F)`: call `f(0)`, `f(1)`, ..., `f(n - 1)`.
//! - `collect_until_none<F: FnMut() -> Option<i32>>(next: F) -> Vec<i32>`: keep calling `next`
//!   and collect the values until it returns `None` (the `None` is not stored).
//! - `make_counter(start: u32) -> impl FnMut() -> u32`: each call returns the next number,
//!   beginning with `start`: `start`, `start + 1`, ...
//! - `running_sum(values: &[i32]) -> Vec<i32>`: the prefix sums, computed with a closure passed to
//!   `map` that updates a captured `total`.
//!
//! ## Examples
//!
//! ```text
//! let mut count = 0;
//! call_n_times(3, || count += 1);           // count == 3
//! let mut seen = Vec::new();
//! call_with_indexes(4, |i| seen.push(i * i)); // seen == [0, 1, 4, 9]
//! let mut n = 0;
//! collect_until_none(|| { n += 1; (n <= 3).then_some(n * 10) })   -> [10, 20, 30]
//! let mut c = make_counter(5);  c() -> 5,  c() -> 6,  c() -> 7
//! running_sum(&[1, 2, 3, 4])                -> [1, 3, 6, 10]
//! ```
//!
//! ## Constraints
//!
//! - `n <= 10^5`.
//!
//! ## Hints
//!
//! 1. Write the parameter as `mut f: F` so that you can call `f()` (calling an `FnMut` mutates it).
//! 2. `while let Some(v) = next() { ... }` handles `collect_until_none`.
//! 3. `make_counter` returns `move || { let current = n; n += 1; current }`, where `n` is a
//!    `let mut` copy of `start` owned by the closure.

pub fn call_n_times<F: FnMut()>(n: usize, mut f: F) {
    todo!("Write your solution here")
}

pub fn call_with_indexes<F: FnMut(usize)>(n: usize, f: F) {
    todo!("Write your solution here")
}

pub fn collect_until_none<F: FnMut() -> Option<i32>>(mut next: F) -> Vec<i32> {
    todo!("Write your solution here")
}

pub fn make_counter(start: u32) -> impl FnMut() -> u32 {
    move || todo!()
}

pub fn running_sum(values: &[i32]) -> Vec<i32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_n_times_updates_captured_state() {
        let mut count = 0;
        call_n_times(3, || count += 1);
        assert_eq!(count, 3);
        let mut log = String::new();
        call_n_times(2, || log.push_str("hi "));
        assert_eq!(log, "hi hi ");
        let mut never = 0;
        call_n_times(0, || never += 1);
        assert_eq!(never, 0);
    }

    #[test]
    fn call_with_indexes_passes_each_index() {
        let mut seen = Vec::new();
        call_with_indexes(4, |i| seen.push(i * i));
        assert_eq!(seen, vec![0, 1, 4, 9]);
        let mut total = 0;
        call_with_indexes(100_000, |i| total += i);
        assert_eq!(total, 4_999_950_000);
    }

    #[test]
    fn collect_until_none_stops_at_the_first_none() {
        let mut n = 0;
        let values = collect_until_none(|| {
            n += 1;
            (n <= 3).then_some(n * 10)
        });
        assert_eq!(values, vec![10, 20, 30]);
        assert_eq!(n, 4);
        assert_eq!(collect_until_none(|| None), Vec::<i32>::new());
    }

    #[test]
    fn counters_are_independent() {
        let mut a = make_counter(5);
        let mut b = make_counter(0);
        assert_eq!(a(), 5);
        assert_eq!(a(), 6);
        assert_eq!(b(), 0);
        assert_eq!(a(), 7);
        assert_eq!(b(), 1);
    }

    #[test]
    fn running_sums() {
        assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(running_sum(&[5, -5, 5]), vec![5, 0, 5]);
        assert_eq!(running_sum(&[]), Vec::<i32>::new());
    }
}
