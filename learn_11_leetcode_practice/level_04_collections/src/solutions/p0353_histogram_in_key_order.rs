//! # Problem 0353: Histogram in Key Order
//!
//! - **Difficulty:** Easy
//! - **Topics:** `BTreeMap` with sorted iteration, `entry().or_insert`, `str::repeat`, `rev()` on a `BTreeMap`
//! - **Source:** Original · [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) · [`std::collections`](https://doc.rust-lang.org/std/collections/index.html)
//!
//! ## Problem
//!
//! A `std::collections::BTreeMap` stores its keys **in sorted order**: iterating over it always
//! visits the keys from smallest to largest. It has the same entry API as `HashMap`. Use it to
//! implement:
//!
//! 1. `histogram(values: &[i32]) -> BTreeMap<i32, usize>` maps each distinct value to its count.
//! 2. `render(values: &[i32]) -> String` draws the histogram as text: one line per distinct value,
//!    from the smallest value to the largest, of the form `"<value> | <count stars>"`, lines joined
//!    with `'\n'` (no trailing newline, `""` for no values).
//! 3. `smallest_mode(values: &[i32]) -> Option<i32>` returns the most frequent value; if several
//!    values are equally frequent, the **smallest** of them. `None` for an empty slice.
//!
//! ## Examples
//!
//! ```text
//! values = [3, 1, 3, -2, 3, 1]
//! histogram     => {-2: 1, 1: 2, 3: 3}
//! render        => "-2 | *\n1 | **\n3 | ***"
//! smallest_mode => Some(3)
//!
//! smallest_mode([5, 4, 5, 4]) => Some(4)     (both occur twice; 4 is smaller)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= values.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. Build the `BTreeMap` exactly like a `HashMap` count; only the type changes.
//! 2. `"*".repeat(n)` builds a string of `n` stars.
//! 3. `max_by_key` returns the **last** maximum it sees. Iterating the map in reverse with `rev()`
//!    makes that last maximum the smallest key.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** count into a `BTreeMap`; its ordered iteration gives `render` the lines in the
//! required order for free. For the mode, scan the entries from the largest key down to the smallest
//! with `max_by_key` on the count: among equal counts it keeps the last one visited, which is the
//! smallest key.
//!
//! **Rust concepts:**
//! - `BTreeMap` operations are `O(log n)` (a B-tree), compared with `O(1)` on average for
//!   `HashMap`; you pay a little speed for a deterministic, sorted order.
//! - `BTreeMap` keys need `Ord` (not `Hash`).
//! - `iter()` on a `BTreeMap` is a `DoubleEndedIterator`, so `rev()` walks it backwards without
//!   collecting.
// @solution-end

use std::collections::BTreeMap;

/// Maps each value to its count, in key order.
pub fn histogram(values: &[i32]) -> BTreeMap<i32, usize> {
    // @solution-start
    let mut counts = BTreeMap::new();
    for &v in values {
        *counts.entry(v).or_insert(0) += 1;
    }
    counts
    // @solution-end
}

/// Draws one `"value | ***"` line per distinct value, smallest value first.
pub fn render(values: &[i32]) -> String {
    // @solution-start
    histogram(values)
        .iter()
        .map(|(value, &count)| format!("{value} | {}", "*".repeat(count)))
        .collect::<Vec<String>>()
        .join("\n")
    // @solution-end
}

/// The most frequent value (the smallest one on ties).
pub fn smallest_mode(values: &[i32]) -> Option<i32> {
    // @solution-start
    histogram(values)
        .iter()
        .rev()
        .max_by_key(|&(_, &count)| count)
        .map(|(&value, _)| value)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn histogram_example() {
        let h = histogram(&[3, 1, 3, -2, 3, 1]);
        let pairs: Vec<(i32, usize)> = h.into_iter().collect();
        assert_eq!(pairs, vec![(-2, 1), (1, 2), (3, 3)]);
    }

    #[test]
    fn render_example() {
        assert_eq!(render(&[3, 1, 3, -2, 3, 1]), "-2 | *\n1 | **\n3 | ***");
        assert_eq!(render(&[]), "");
        assert_eq!(render(&[0]), "0 | *");
    }

    #[test]
    fn smallest_mode_examples() {
        assert_eq!(smallest_mode(&[3, 1, 3, -2, 3, 1]), Some(3));
        assert_eq!(smallest_mode(&[5, 4, 5, 4]), Some(4));
        assert_eq!(smallest_mode(&[]), None);
        assert_eq!(smallest_mode(&[9, -9, 0]), Some(-9));
    }

    #[test]
    fn large_input() {
        let values: Vec<i32> = (0..100_000).map(|i| (i * 7) % 100 - 50).collect();
        let h = histogram(&values);
        assert_eq!(h.len(), 100);
        assert_eq!(h.keys().next(), Some(&-50));
        assert!(h.values().all(|&c| c == 1000));
        assert_eq!(smallest_mode(&values), Some(-50));
    }
}
