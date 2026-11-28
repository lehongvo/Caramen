// ==================================================================
// PRACTICE 0335: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0335_compare_course_enrollments.md
// Run the tests:          cargo test practice::p0335
// ==================================================================

//! # Problem 0335: Compare Course Enrollments
//!
//! - **Difficulty:** Easy
//! - **Topics:** `HashSet::union`, `intersection`, `difference`, `symmetric_difference`, `is_subset`, `is_disjoint`
//! - **Source:** Original · [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html) · [Rust By Example: HashSet](https://doc.rust-lang.org/rust-by-example/std/hash/hashset.html)
//!
//! ## Problem
//!
//! A school compares the students enrolled in a math class and in an art class. The rosters are
//! given as slices of names, and a name may be listed more than once by mistake. Implement:
//!
//! 1. `enrollment_report(math: &[&str], art: &[&str]) -> Report`, where
//!
//!    ```text
//!    pub struct Report {
//!        pub both: Vec<String>,        // in math AND in art
//!        pub either: Vec<String>,      // in math OR in art (or both)
//!        pub only_math: Vec<String>,   // in math but NOT in art
//!        pub exactly_one: Vec<String>, // in exactly one of the two classes
//!    }
//!    ```
//!
//!    Every list contains each name once and is **sorted alphabetically**.
//! 2. `meets_prerequisites(required: &[&str], taken: &[&str]) -> bool` returns `true` when every
//!    course in `required` also appears in `taken`.
//! 3. `no_overlap(a: &[&str], b: &[&str]) -> bool` returns `true` when no name appears in both.
//!
//! ## Examples
//!
//! ```text
//! math = ["Ana", "Ben", "Cal", "Ben"], art = ["Cal", "Dee", "Ana"]
//! enrollment_report => both:        ["Ana", "Cal"]
//!                      either:      ["Ana", "Ben", "Cal", "Dee"]
//!                      only_math:   ["Ben"]
//!                      exactly_one: ["Ben", "Dee"]
//!
//! meets_prerequisites(["algebra"], ["algebra", "geometry"]) => true
//! meets_prerequisites(["algebra", "logic"], ["algebra"])     => false
//! no_overlap(["a", "b"], ["c"]) => true
//! ```
//!
//! ## Constraints
//!
//! - Each slice has at most `10^4` names.
//!
//! ## Hints
//!
//! 1. Build a `HashSet<&str>` from each slice with `iter().copied().collect()`.
//! 2. `HashSet` has a method for each of the four lists, plus `is_subset` and `is_disjoint`.
//! 3. The set operations iterate in an unspecified order; write a small helper that collects
//!    names into a sorted `Vec<String>`.

use std::collections::HashSet;

/// The four comparisons between the two rosters, each sorted alphabetically.
#[derive(Debug, PartialEq)]
pub struct Report {
    pub both: Vec<String>,
    pub either: Vec<String>,
    pub only_math: Vec<String>,
    pub exactly_one: Vec<String>,
}

/// Compares the two rosters.
pub fn enrollment_report(math: &[&str], art: &[&str]) -> Report {
    todo!("Write your solution here")
}

/// Returns `true` when every required course has been taken.
pub fn meets_prerequisites(required: &[&str], taken: &[&str]) -> bool {
    todo!("Write your solution here")
}

/// Returns `true` when no name appears in both slices.
pub fn no_overlap(a: &[&str], b: &[&str]) -> bool {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn report_example() {
        let report = enrollment_report(&["Ana", "Ben", "Cal", "Ben"], &["Cal", "Dee", "Ana"]);
        assert_eq!(
            report,
            Report {
                both: strings(&["Ana", "Cal"]),
                either: strings(&["Ana", "Ben", "Cal", "Dee"]),
                only_math: strings(&["Ben"]),
                exactly_one: strings(&["Ben", "Dee"]),
            }
        );
    }

    #[test]
    fn report_with_an_empty_class() {
        let report = enrollment_report(&["Zed", "Amy"], &[]);
        assert_eq!(report.both, Vec::<String>::new());
        assert_eq!(report.either, strings(&["Amy", "Zed"]));
        assert_eq!(report.only_math, strings(&["Amy", "Zed"]));
        assert_eq!(report.exactly_one, strings(&["Amy", "Zed"]));
    }

    #[test]
    fn prerequisites_examples() {
        assert!(meets_prerequisites(&["algebra"], &["algebra", "geometry"]));
        assert!(!meets_prerequisites(&["algebra", "logic"], &["algebra"]));
        assert!(meets_prerequisites(&[], &[]));
        assert!(meets_prerequisites(&["x", "x"], &["x"]));
    }

    #[test]
    fn no_overlap_examples() {
        assert!(no_overlap(&["a", "b"], &["c"]));
        assert!(!no_overlap(&["a", "b"], &["b"]));
        assert!(no_overlap(&[], &["b"]));
    }

    #[test]
    fn large_rosters() {
        let math: Vec<String> = (0..10_000).map(|i| format!("s{:05}", i)).collect();
        let art: Vec<String> = (5_000..15_000).map(|i| format!("s{:05}", i)).collect();
        let math: Vec<&str> = math.iter().map(String::as_str).collect();
        let art: Vec<&str> = art.iter().map(String::as_str).collect();
        let report = enrollment_report(&math, &art);
        assert_eq!(report.both.len(), 5_000);
        assert_eq!(report.both[0], "s05000");
        assert_eq!(report.either.len(), 15_000);
        assert_eq!(report.only_math.last().map(String::as_str), Some("s04999"));
        assert_eq!(report.exactly_one.len(), 10_000);
        assert!(!no_overlap(&math, &art));
    }
}
