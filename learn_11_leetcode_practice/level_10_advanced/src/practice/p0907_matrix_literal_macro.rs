// ==================================================================
// PRACTICE 0907: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0907_matrix_literal_macro.md
// Run the tests:          cargo test practice::p0907
// ==================================================================

//! # Problem 0907: Nested Repetitions: `matrix!`
//!
//! - **Difficulty:** Medium
//! - **Topics:** nested `$( $( ... ),+ );+` repetitions, separators, runtime checks in expansions
//! - **Source:** Original · [Reference: Macros By Example, transcribing](https://doc.rust-lang.org/reference/macros-by-example.html#transcribing) · [Rust by Example: DSL](https://doc.rust-lang.org/rust-by-example/macros/dsl.html)
//!
//! ## Problem
//!
//! Write two macros whose matchers contain a repetition **inside** another repetition.
//!
//! 1. `matrix![a, b, c; d, e, f]` builds a `Vec<Vec<T>>`: rows are separated by `;`, values inside a
//!    row by `,`. A trailing `;` is allowed, and `matrix![]` is an empty `Vec`. If the rows do not all
//!    have the same length, the expansion **panics at run time** with the message
//!    `"all rows must have the same length"`.
//! 2. `grouped! { key => [v1, v2, ...], key2 => [...], ... }` builds a
//!    `std::collections::BTreeMap<K, Vec<V>>`. A key may appear several times: its lists are
//!    **concatenated** in the order written. An empty list `key => []` still creates the key.
//!
//! Then use `matrix!` in `fn identity3() -> Vec<Vec<i32>>` (the 3 x 3 identity matrix) and write
//! `fn transpose(m: &[Vec<i32>]) -> Vec<Vec<i32>>` (an ordinary function, used by the tests to check
//! your matrices; `transpose(&[])` is empty).
//!
//! ## Examples
//!
//! ```text
//! matrix![1, 2, 3; 4, 5, 6]       => [[1, 2, 3], [4, 5, 6]]
//! matrix![7;]                     => [[7]]
//! matrix![1, 2; 3]                => panics: "all rows must have the same length"
//! grouped! { "a" => [1, 2], "b" => [], "a" => [3] }
//!                                 => {"a": [1, 2, 3], "b": []}
//! ```
//!
//! ## Constraints
//!
//! - Every row of `matrix!` has at least one value.
//!
//! ## Hints
//!
//! 1. Matcher for `matrix!`: `$( $( $x:expr ),+ );+ $(;)?`. In the transcriber, the outer `$( ... ),*`
//!    produces one inner `vec![...]` per row.
//! 2. Check the lengths inside the expansion with `assert!(rows.iter().all(...), "...")`.
//! 3. For `grouped!`, `map.entry(key).or_insert_with(Vec::new)` handles both new and repeated keys;
//!    `Vec::append(&mut vec![...])` then moves the listed values to the end.

macro_rules! matrix {
    ($($t:tt)*) => {
        todo!()
    };
}

// Only invoked by the tests below (in a real crate it would be `#[macro_export]`ed).
#[allow(unused_macros)]
macro_rules! grouped {
    ($($t:tt)*) => {
        todo!()
    };
}

/// The 3 x 3 identity matrix, written with `matrix!`.
pub fn identity3() -> Vec<Vec<i32>> {
    todo!("Write your solution here")
}

/// Rows become columns. `transpose(&[])` is empty.
pub fn transpose(m: &[Vec<i32>]) -> Vec<Vec<i32>> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn builds_rectangular_matrices() {
        let m: Vec<Vec<i32>> = matrix![1, 2, 3; 4, 5, 6];
        assert_eq!(m, vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(transpose(&m), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
        let single: Vec<Vec<char>> = matrix!['x';];
        assert_eq!(single, vec![vec!['x']]);
        let empty: Vec<Vec<u8>> = matrix![];
        assert!(empty.is_empty());
    }

    #[test]
    #[should_panic(expected = "all rows must have the same length")]
    fn ragged_rows_panic() {
        let _m: Vec<Vec<i32>> = matrix![1, 2; 3];
    }

    #[test]
    fn identity_is_its_own_transpose() {
        let id = identity3();
        assert_eq!(id, vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
        assert_eq!(transpose(&id), id);
        assert_eq!(transpose(&[]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn grouped_concatenates_repeated_keys() {
        let groups: BTreeMap<&str, Vec<i32>> = grouped! { "a" => [1, 2], "b" => [], "a" => [3] };
        let expected = BTreeMap::from([("a", vec![1, 2, 3]), ("b", vec![])]);
        assert_eq!(groups, expected);
        let none: BTreeMap<u8, Vec<u8>> = grouped! {};
        assert!(none.is_empty());
    }

    #[test]
    fn expressions_inside_cells() {
        let base = 10;
        let m: Vec<Vec<i32>> = matrix![base + 1, base * 2; -base, base / 3];
        assert_eq!(m, vec![vec![11, 20], vec![-10, 3]]);
        let fruits: BTreeMap<String, Vec<usize>> = grouped! {
            "kiwi".to_string() => ["kiwi".len()],
            String::from("fig") => [1, 2,],
        };
        assert_eq!(fruits["kiwi"], vec![4]);
        assert_eq!(fruits["fig"], vec![1, 2]);
    }
}
