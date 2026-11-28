// ==================================================================
// PRACTICE 0254: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0254_ignore_with_rest_patterns.md
// Run the tests:          cargo test practice::p0254
// ==================================================================

//! # Problem 0254: Ignoring Parts of a Value
//!
//! - **Difficulty:** Easy
//! - **Topics:** `_` wildcard, `..` rest pattern in tuples, structs and slices, `_name` vs `_`, patterns in `let`, `for` and parameters
//! - **Source:** Original · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#ignoring-values-in-a-pattern)
//!
//! ## Problem
//!
//! Often you need only part of a value. Patterns can skip the rest with `_` (one value) and `..`
//! (all remaining values). Implement each function with a pattern that names **only** what it uses:
//!
//! - `ends(t: (i32, i32, i32, i32, i32)) -> (i32, i32)`: the first and last elements of the tuple;
//! - `same_major(a: &Version, b: &Version) -> bool`: whether two `Version { major, minor, patch }`
//!   values have the same `major`;
//! - `second(nums: &[i32]) -> Option<i32>`: the second element, `None` if there are fewer than two;
//! - `outer_sum(triples: &[(i32, i32, i32)]) -> i32`: the sum of the first and third component of
//!   every triple (destructure in the `for` loop itself);
//! - `peek_pair(pair: (Option<String>, Option<String>)) -> (bool, (Option<String>, Option<String>))`:
//!   whether **both** options hold a value, plus the **same** pair given back. The check must not
//!   move the `String`s out.
//!
//! ## Examples
//!
//! ```text
//! ends((1, 2, 3, 4, 5))                     => (1, 5)
//! same_major(&Version { major: 2, minor: 1, patch: 0 }, &Version { major: 2, minor: 9, patch: 3 })
//!                                           => true
//! second(&[10, 20, 30])                     => Some(20)
//! second(&[10])                             => None
//! outer_sum(&[(1, 100, 2), (3, 100, 4)])    => 10
//! peek_pair((Some("x"), None))              => (false, (Some("x"), None))
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `let (first, .., last) = t;` skips the three middle values.
//! 2. `[_, second, ..]` matches slices of length 2 or more.
//! 3. `Some(_)` does not bind anything, so it does not move; `Some(_s)` **does** bind (and move) the
//!    value, it just silences the "unused variable" warning.

/// A semantic version number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

/// The first and last elements of a 5-tuple.
pub fn ends(t: (i32, i32, i32, i32, i32)) -> (i32, i32) {
    todo!("Write your solution here")
}

/// Whether both versions have the same major number.
pub fn same_major(a: &Version, b: &Version) -> bool {
    todo!("Write your solution here")
}

/// The second element of the slice.
pub fn second(nums: &[i32]) -> Option<i32> {
    todo!("Write your solution here")
}

/// The sum of the first and third components of every triple.
pub fn outer_sum(triples: &[(i32, i32, i32)]) -> i32 {
    todo!("Write your solution here")
}

/// Whether both options hold a value, and the pair itself.
pub fn peek_pair(
    pair: (Option<String>, Option<String>),
) -> (bool, (Option<String>, Option<String>)) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(major: u32, minor: u32, patch: u32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    #[test]
    fn tuple_ends() {
        assert_eq!(ends((1, 2, 3, 4, 5)), (1, 5));
        assert_eq!(ends((-7, 0, 0, 0, -7)), (-7, -7));
    }

    #[test]
    fn major_versions() {
        assert!(same_major(&v(2, 1, 0), &v(2, 9, 3)));
        assert!(!same_major(&v(1, 0, 0), &v(2, 0, 0)));
        assert!(same_major(&v(0, 0, 1), &v(0, 0, 1)));
    }

    #[test]
    fn second_elements() {
        assert_eq!(second(&[10, 20, 30]), Some(20));
        assert_eq!(second(&[10, 20]), Some(20));
        assert_eq!(second(&[10]), None);
        assert_eq!(second(&[]), None);
    }

    #[test]
    fn outer_sums() {
        assert_eq!(outer_sum(&[(1, 100, 2), (3, 100, 4)]), 10);
        assert_eq!(outer_sum(&[]), 0);
        let many = vec![(1, 999, 1); 1_000];
        assert_eq!(outer_sum(&many), 2_000);
    }

    #[test]
    fn peeking_does_not_move() {
        let x = || Some("x".to_string());
        assert_eq!(peek_pair((x(), x())), (true, (x(), x())));
        assert_eq!(peek_pair((x(), None)), (false, (x(), None)));
        assert_eq!(peek_pair((None, None)), (false, (None, None)));
    }
}
