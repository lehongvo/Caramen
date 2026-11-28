//! # Problem 0502: Remove Consecutive Duplicates Generically
//!
//! - **Difficulty:** Easy
//! - **Topics:** generic functions, `T: PartialEq + Clone`, `Vec<T>`, `&mut Vec<T>`
//! - **Source:** Original · [The Rust Book 10.1](https://doc.rust-lang.org/book/ch10-01-syntax.html)
//!
//! ## Problem
//!
//! Write two generic functions that collapse **runs** of equal neighbours into a single element.
//! Elements that are equal but not next to each other are kept.
//!
//! - `dedup_consecutive<T: PartialEq + Clone>(items: &[T]) -> Vec<T>` returns a **new** vector and
//!   leaves the input untouched.
//! - `dedup_in_place<T: PartialEq>(items: &mut Vec<T>)` modifies the vector itself. It needs no
//!   `Clone` bound because it only moves elements around. Do **not** call `Vec::dedup`: write the
//!   loop yourself.
//!
//! Both functions keep the **first** element of every run and preserve the original order.
//!
//! ## Examples
//!
//! ```text
//! dedup_consecutive(&[1, 1, 2, 2, 2, 1, 3, 3]) => [1, 2, 1, 3]
//! dedup_consecutive(&["a", "a", "b"])          => ["a", "b"]
//! dedup_consecutive::<i32>(&[])                => []
//!
//! let mut v = vec!['x', 'x', 'y', 'x'];
//! dedup_in_place(&mut v);                      // v is now ['x', 'y', 'x']
//! ```
//!
//! ## Constraints
//!
//! - `0 <= items.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. An element is kept when it differs from the element **just before it** (or when it is the
//!    first one). `windows(2)` or comparing with the last pushed element both work.
//! 2. For the in-place version, keep a "write" index: copy each kept element to `write` with
//!    `swap`, then `truncate` the vector.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `dedup_consecutive` pushes an element when it differs from the last element already in
//! the output. `dedup_in_place` uses two indices: `read` scans every element, `write` marks the end of
//! the kept prefix. Kept elements are swapped forward, and the tail is truncated at the end.
//!
//! **Rust concepts:**
//! - `T: PartialEq` provides `==`/`!=`; `T: Clone` is only needed when building a new `Vec<T>` from
//!   borrowed elements (`item.clone()`).
//! - `last()` on a `Vec<T>` returns `Option<&T>`; comparing `Some(item)` with it avoids special-casing
//!   the empty output.
//! - `slice::swap` moves values without cloning them, which is why the in-place version has a weaker
//!   bound than the copying version.
// @solution-end

/// Returns a new vector where every run of equal neighbours is collapsed into one element.
pub fn dedup_consecutive<T: PartialEq + Clone>(items: &[T]) -> Vec<T> {
    // @solution-start
    let mut out: Vec<T> = Vec::with_capacity(items.len());
    for item in items {
        if out.last() != Some(item) {
            out.push(item.clone());
        }
    }
    out
    // @solution-end
}

/// Collapses every run of equal neighbours in place, keeping the first element of each run.
pub fn dedup_in_place<T: PartialEq>(items: &mut Vec<T>) {
    // @solution-start
    if items.is_empty() {
        return;
    }
    let mut write = 1;
    for read in 1..items.len() {
        if items[read] != items[write - 1] {
            items.swap(read, write);
            write += 1;
        }
    }
    items.truncate(write);
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_numbers() {
        assert_eq!(
            dedup_consecutive(&[1, 1, 2, 2, 2, 1, 3, 3]),
            vec![1, 2, 1, 3]
        );
        let mut v = vec![1, 1, 2, 2, 2, 1, 3, 3];
        dedup_in_place(&mut v);
        assert_eq!(v, vec![1, 2, 1, 3]);
    }

    #[test]
    fn works_for_strings_and_chars() {
        assert_eq!(dedup_consecutive(&["a", "a", "b"]), vec!["a", "b"]);
        let mut owned = vec![
            "x".to_string(),
            "x".to_string(),
            "y".to_string(),
            "x".to_string(),
        ];
        dedup_in_place(&mut owned);
        assert_eq!(owned, vec!["x", "y", "x"]);
        let mut chars = vec!['x', 'x', 'y', 'x'];
        dedup_in_place(&mut chars);
        assert_eq!(chars, vec!['x', 'y', 'x']);
    }

    #[test]
    fn empty_and_single() {
        assert_eq!(dedup_consecutive::<i32>(&[]), Vec::<i32>::new());
        assert_eq!(dedup_consecutive(&[42]), vec![42]);
        let mut empty: Vec<u8> = Vec::new();
        dedup_in_place(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn all_equal_and_no_duplicates() {
        assert_eq!(dedup_consecutive(&[7; 5]), vec![7]);
        let mut v = vec![1, 2, 3, 4];
        dedup_in_place(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn large_input_matches_std() {
        let data: Vec<u32> = (0..100_000).map(|i| (i / 3) % 10).collect();
        let mut expected = data.clone();
        expected.dedup();
        assert_eq!(dedup_consecutive(&data), expected);
        let mut v = data;
        dedup_in_place(&mut v);
        assert_eq!(v, expected);
    }
}
