// ==================================================================
// PRACTICE 0209: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0209_derive_debug_clone_partialeq.md
// Run the tests:          cargo test practice::p0209
// ==================================================================

//! # Problem 0209: Derive Debug, Clone and PartialEq
//!
//! - **Difficulty:** Easy
//! - **Topics:** `#[derive(Debug, Clone, PartialEq)]`, `{:?}` and `{:#?}`, `clone`, `==` on structs
//! - **Source:** Original · [The Rust Book, Appendix C](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
//!
//! ## Problem
//!
//! `#[derive(...)]` asks the compiler to write standard trait implementations for a struct:
//!
//! - `Debug` enables `{:?}` (one line) and `{:#?}` (pretty, multi-line) formatting;
//! - `Clone` adds `.clone()`, a deep copy (every field is cloned);
//! - `PartialEq` adds `==` and `!=`, comparing all fields in order.
//!
//! The types below already derive all three:
//!
//! ```text
//! pub struct Point3   { pub x: i32, pub y: i32, pub z: i32 }
//! pub struct Waypoint { pub name: String, pub position: Point3, pub tags: Vec<String> }
//! ```
//!
//! Implement, using the derived traits:
//!
//! - `debug_line(w: &Waypoint) -> String`: the `{:?}` representation of `w`.
//! - `pretty(p: &Point3) -> String`: the `{:#?}` representation of `p`.
//! - `renamed_copy(w: &Waypoint, new_name: &str) -> Waypoint`: a clone of `w` with a new name and the
//!   extra tag `"copy"` appended to `tags`. The original must not change.
//! - `count_equal(points: &[Point3], target: &Point3) -> usize`: how many points equal `target`.
//!
//! ## Examples
//!
//! ```text
//! w = Waypoint { name: "Camp", position: Point3 { x: 1, y: 2, z: 3 }, tags: ["base"] }
//!
//! debug_line(&w)
//!   => "Waypoint { name: \"Camp\", position: Point3 { x: 1, y: 2, z: 3 }, tags: [\"base\"] }"
//!
//! pretty(&Point3 { x: 1, y: 2, z: 3 })
//!   => "Point3 {\n    x: 1,\n    y: 2,\n    z: 3,\n}"
//!
//! renamed_copy(&w, "Camp 2").tags  => ["base", "copy"]
//! count_equal(&[p, q, p], &p)      => 2
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `format!("{:?}", w)` and `format!("{:#?}", p)` produce exactly the derived output.
//! 2. Clone first (`let mut copy = w.clone();`), then change the copy's fields.
//! 3. `points.iter().filter(|p| *p == target).count()` compares two `&Point3` values.

/// A point in 3D space.
#[derive(Debug, Clone, PartialEq)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// A named point on a map.
#[derive(Debug, Clone, PartialEq)]
pub struct Waypoint {
    pub name: String,
    pub position: Point3,
    pub tags: Vec<String>,
}

/// The one-line debug representation.
pub fn debug_line(w: &Waypoint) -> String {
    todo!("Write your solution here")
}

/// The pretty, multi-line debug representation.
pub fn pretty(p: &Point3) -> String {
    todo!("Write your solution here")
}

/// A renamed copy of `w`, tagged with `"copy"`.
pub fn renamed_copy(w: &Waypoint, new_name: &str) -> Waypoint {
    todo!("Write your solution here")
}

/// Counts the points equal to `target`.
pub fn count_equal(points: &[Point3], target: &Point3) -> usize {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn camp() -> Waypoint {
        Waypoint {
            name: String::from("Camp"),
            position: Point3 { x: 1, y: 2, z: 3 },
            tags: vec![String::from("base")],
        }
    }

    #[test]
    fn example_debug_line() {
        assert_eq!(
            debug_line(&camp()),
            "Waypoint { name: \"Camp\", position: Point3 { x: 1, y: 2, z: 3 }, tags: [\"base\"] }"
        );
    }

    #[test]
    fn example_pretty() {
        assert_eq!(
            pretty(&Point3 { x: 1, y: 2, z: 3 }),
            "Point3 {\n    x: 1,\n    y: 2,\n    z: 3,\n}"
        );
    }

    #[test]
    fn example_renamed_copy() {
        let original = camp();
        let copy = renamed_copy(&original, "Camp 2");
        assert_eq!(copy.name, "Camp 2");
        assert_eq!(copy.tags, vec!["base", "copy"]);
        assert_eq!(copy.position, original.position);
        // The original is unchanged.
        assert_eq!(original, camp());
        assert_ne!(copy, original);
    }

    #[test]
    fn example_count_equal() {
        let p = Point3 { x: 0, y: 0, z: 1 };
        let q = Point3 { x: 0, y: 1, z: 0 };
        assert_eq!(count_equal(&[p.clone(), q, p.clone()], &p), 2);
    }

    #[test]
    fn count_in_empty_slice() {
        assert_eq!(count_equal(&[], &Point3 { x: 1, y: 1, z: 1 }), 0);
    }

    #[test]
    fn debug_escapes_quotes_and_shows_empty_vec() {
        let w = Waypoint {
            name: String::from("say \"hi\""),
            position: Point3 { x: -1, y: 0, z: 0 },
            tags: Vec::new(),
        };
        assert_eq!(
            debug_line(&w),
            "Waypoint { name: \"say \\\"hi\\\"\", position: Point3 { x: -1, y: 0, z: 0 }, tags: [] }"
        );
    }
}
