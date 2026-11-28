// ==================================================================
// PRACTICE 0509: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0509_most_frequent_item.md
// Run the tests:          cargo test practice::p0509
// ==================================================================

//! # Problem 0509: Most Frequent Item
//!
//! - **Difficulty:** Medium
//! - **Topics:** multiple trait bounds, `where` clauses, `T: Hash + Eq + Ord + Clone`, `HashMap<&T, usize>`
//! - **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax)
//!
//! ## Problem
//!
//! Write two generic counting functions.
//!
//! - `most_frequent<T>(items: &[T]) -> Option<(T, usize)>` returns the element that appears most
//!   often together with its count. When several elements share the highest count, return the
//!   **smallest** of them (by `Ord`). Return `None` for an empty slice.
//!   Bounds: `T: Hash + Eq + Ord + Clone`.
//! - `top_n<T>(items: &[T], n: usize) -> Vec<(T, usize)>` returns the `n` most frequent elements
//!   (fewer if there are fewer distinct elements), sorted by count **descending**, then by value
//!   **ascending**. Same bounds.
//!
//! Write the bounds in a `where` clause.
//!
//! ## Examples
//!
//! ```text
//! most_frequent(&[3, 1, 3, 2, 1, 3])          => Some((3, 3))
//! most_frequent(&["b", "a", "b", "a"])        => Some(("a", 2))    tie: "a" < "b"
//! most_frequent::<u8>(&[])                    => None
//! top_n(&['x', 'y', 'x', 'z', 'y', 'x'], 2)   => [('x', 3), ('y', 2)]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= items.len() <= 10^5`
//! - `0 <= n <= 10^5`
//!
//! ## Hints
//!
//! 1. Count with a `HashMap<&T, usize>`: the keys borrow from `items`, so nothing is cloned while
//!    counting. `Hash + Eq` are exactly what a `HashMap` key needs.
//! 2. Clone a value only when you put it in the result.
//! 3. Sorting by `(Reverse(count), value)` gives "count descending, then value ascending".

use std::cmp::Reverse;
use std::collections::HashMap;
use std::hash::Hash;

/// The most frequent element and its count; ties go to the smallest element.
pub fn most_frequent<T>(items: &[T]) -> Option<(T, usize)>
where
    T: Hash + Eq + Ord + Clone,
{
    todo!("Write your solution here")
}

/// The `n` most frequent elements, by count descending then value ascending.
pub fn top_n<T>(items: &[T], n: usize) -> Vec<(T, usize)>
where
    T: Hash + Eq + Ord + Clone,
{
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_most_frequent() {
        assert_eq!(most_frequent(&[3, 1, 3, 2, 1, 3]), Some((3, 3)));
        assert_eq!(most_frequent(&["b", "a", "b", "a"]), Some(("a", 2)));
        assert_eq!(most_frequent::<u8>(&[]), None);
    }

    #[test]
    fn example_top_n() {
        assert_eq!(
            top_n(&['x', 'y', 'x', 'z', 'y', 'x'], 2),
            vec![('x', 3), ('y', 2)]
        );
        assert_eq!(
            top_n(&['x', 'y', 'x', 'z', 'y', 'x'], 10),
            vec![('x', 3), ('y', 2), ('z', 1)]
        );
        assert_eq!(top_n(&[1, 2, 3], 0), Vec::<(i32, usize)>::new());
    }

    #[test]
    fn works_with_owned_strings() {
        let words: Vec<String> = "to be or not to be".split(' ').map(String::from).collect();
        assert_eq!(most_frequent(&words), Some(("be".to_string(), 2)));
        assert_eq!(
            top_n(&words, 3),
            vec![
                ("be".to_string(), 2),
                ("to".to_string(), 2),
                ("not".to_string(), 1)
            ]
        );
    }

    #[test]
    fn large_input() {
        let items: Vec<u32> = (0..100_000).map(|i| i % 97).collect();
        // 100_000 = 97 * 1030 + 90, so values 0..90 appear 1031 times and 90..97 appear 1030 times.
        assert_eq!(most_frequent(&items), Some((0, 1031)));
        let top = top_n(&items, 91);
        assert_eq!(top[89], (89, 1031));
        assert_eq!(top[90], (90, 1030));
    }
}
