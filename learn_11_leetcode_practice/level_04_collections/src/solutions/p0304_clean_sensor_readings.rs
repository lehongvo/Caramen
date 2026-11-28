//! # Problem 0304: Clean Sensor Readings
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Vec::retain`, `&mut Vec<T>` parameters, in-place filtering
//! - **Source:** Original · [`Vec::retain`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain)
//!
//! ## Problem
//!
//! A sensor sometimes reports impossible values, and a log file contains blank lines. Clean both
//! **in place** (modify the vector you are given instead of building a new one):
//!
//! 1. `clean_readings(readings: &mut Vec<i32>, low: i32, high: i32) -> usize` keeps only the
//!    readings `r` with `low <= r <= high`, in their original order, and returns how many readings
//!    were removed.
//! 2. `drop_blank_lines(lines: &mut Vec<String>)` removes every line that is empty or contains only
//!    whitespace (spaces, tabs, ...). The remaining lines keep their order and their content
//!    (do not trim them).
//!
//! ## Examples
//!
//! ```text
//! readings = [20, -999, 21, 500, 19], low = -40, high = 60
//! clean_readings(...) => 2, readings becomes [20, 21, 19]
//!
//! lines = ["a", "", "  ", "\tb ", "\t"]
//! drop_blank_lines(...) => lines becomes ["a", "\tb "]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= readings.len(), lines.len() <= 10^5`
//! - `low <= high`
//!
//! ## Hints
//!
//! 1. `retain(|x| ...)` keeps the elements for which the closure returns `true` and removes the
//!    rest, preserving order.
//! 2. Remember the length before calling `retain` to compute how many were removed.
//! 3. `str::trim` returns a slice without leading and trailing whitespace.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `Vec::retain` is the in-place filter. It visits every element once, moves the kept
//! elements to the front and drops the others, so it runs in `O(n)` without allocating.
//!
//! **Rust concepts:**
//! - `retain` gives the closure a `&T`, so `|&r| ...` destructures the reference for `i32`.
//! - `(low..=high).contains(&r)` is a readable way to test an inclusive range.
//! - Removing elements one by one with `remove(i)` inside a loop would be `O(n^2)` and easy to get
//!   wrong (indices shift); `retain` avoids both problems.
//! - `line.trim().is_empty()` checks "only whitespace" without changing the line.
// @solution-end

/// Keeps the readings in `low..=high` and returns how many were removed.
pub fn clean_readings(readings: &mut Vec<i32>, low: i32, high: i32) -> usize {
    // @solution-start
    let before = readings.len();
    readings.retain(|&r| (low..=high).contains(&r));
    before - readings.len()
    // @solution-end
}

/// Removes the lines that are empty or only whitespace.
pub fn drop_blank_lines(lines: &mut Vec<String>) {
    // @solution-start
    lines.retain(|line| !line.trim().is_empty());
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_readings() {
        let mut readings = vec![20, -999, 21, 500, 19];
        assert_eq!(clean_readings(&mut readings, -40, 60), 2);
        assert_eq!(readings, vec![20, 21, 19]);
    }

    #[test]
    fn bounds_are_inclusive() {
        let mut readings = vec![-40, -41, 60, 61, 0];
        assert_eq!(clean_readings(&mut readings, -40, 60), 2);
        assert_eq!(readings, vec![-40, 60, 0]);
    }

    #[test]
    fn empty_and_all_removed() {
        let mut empty: Vec<i32> = Vec::new();
        assert_eq!(clean_readings(&mut empty, 0, 1), 0);
        let mut all_bad = vec![5, 6, 7];
        assert_eq!(clean_readings(&mut all_bad, 0, 1), 3);
        assert!(all_bad.is_empty());
    }

    #[test]
    fn example_blank_lines() {
        let mut lines: Vec<String> = ["a", "", "  ", "\tb ", "\t"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        drop_blank_lines(&mut lines);
        assert_eq!(lines, vec!["a".to_string(), "\tb ".to_string()]);
    }

    #[test]
    fn large_input_keeps_order() {
        let mut readings: Vec<i32> = (0..100_000).collect();
        assert_eq!(clean_readings(&mut readings, 10, 19), 99_990);
        assert_eq!(readings, (10..20).collect::<Vec<i32>>());
    }
}
