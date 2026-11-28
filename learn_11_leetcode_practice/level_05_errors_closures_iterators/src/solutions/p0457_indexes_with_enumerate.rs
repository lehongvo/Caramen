//! # Problem 0457: Indexes Where Things Happen
//!
//! - **Difficulty:** Easy
//! - **Topics:** `enumerate`, `position`, `rposition`, `max_by_key` tie-breaking, `rev`, `filter_map`
//! - **Source:** Original · [std `Iterator::enumerate`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate) · [std `Iterator::position`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position)
//!
//! ## Problem
//!
//! `enumerate()` pairs every item with its index, `(0, a), (1, b), ...`, so iterator chains can
//! answer "**where**" questions, not only "what" questions.
//!
//! Implement:
//!
//! - `indexes_of(nums: &[i32], target: i32) -> Vec<usize>`: every index where `target` appears.
//! - `first_negative(nums: &[i32]) -> Option<usize>`: index of the first negative number.
//! - `last_negative(nums: &[i32]) -> Option<usize>`: index of the last negative number.
//! - `index_of_max(nums: &[i32]) -> Option<usize>`: index of the largest value; if it appears
//!   several times, the **first** one. `None` for an empty slice.
//! - `numbered(items: &[&str]) -> Vec<String>`: `"1. first"`, `"2. second"`, ... (numbering from 1).
//!
//! ## Examples
//!
//! ```text
//! indexes_of(&[4, 1, 4, 4, 2], 4)   -> [0, 2, 3]
//! first_negative(&[3, -1, 5, -2])   -> Some(1)
//! last_negative(&[3, -1, 5, -2])    -> Some(3)
//! first_negative(&[1, 2])           -> None
//! index_of_max(&[2, 9, 4, 9])       -> Some(1)
//! numbered(&["tea", "milk"])        -> ["1. tea", "2. milk"]
//! ```
//!
//! ## Constraints
//!
//! - Slices have at most `10^5` elements.
//!
//! ## Hints
//!
//! 1. `enumerate().filter(...).map(|(i, _)| i)` (or a single `filter_map`) collects indexes.
//! 2. `rposition` searches from the end of a slice iterator.
//! 3. `max_by_key` returns the **last** maximum when there are ties. Iterating in reverse with
//!    `rev()` makes that last one the element with the smallest index.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each question is one chain on top of `iter()`: `enumerate` provides the index,
//! `position`/`rposition` search from either end, and `max_by_key` finds the maximum. The tie rule
//! of `max_by_key` (last maximum wins) is turned into "first maximum" by iterating backwards.
//!
//! **Rust concepts:**
//! - `enumerate` yields `(usize, item)` tuples; closure patterns like `|(i, &v)|` destructure them.
//! - `position` and `rposition` return `Option<usize>` and stop at the first match.
//! - `Iterator::max_by_key` / `max` return the last of several equal maxima; `min_by_key` / `min`
//!   return the first of several equal minima. Knowing this avoids subtle bugs.
//! - `Enumerate` over a slice iterator is double-ended, so `rev()` is allowed and keeps the
//!   original indexes.
// @solution-end

pub fn indexes_of(nums: &[i32], target: i32) -> Vec<usize> {
    // @solution-start
    nums.iter()
        .enumerate()
        .filter_map(|(i, &v)| (v == target).then_some(i))
        .collect()
    // @solution-end
}

pub fn first_negative(nums: &[i32]) -> Option<usize> {
    // @solution-start
    nums.iter().position(|&v| v < 0)
    // @solution-end
}

pub fn last_negative(nums: &[i32]) -> Option<usize> {
    // @solution-start
    nums.iter().rposition(|&v| v < 0)
    // @solution-end
}

pub fn index_of_max(nums: &[i32]) -> Option<usize> {
    // @solution-start
    nums.iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, &v)| v)
        .map(|(i, _)| i)
    // @solution-end
}

pub fn numbered(items: &[&str]) -> Vec<String> {
    // @solution-start
    items
        .iter()
        .enumerate()
        .map(|(i, item)| format!("{}. {item}", i + 1))
        .collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_of_target() {
        assert_eq!(indexes_of(&[4, 1, 4, 4, 2], 4), vec![0, 2, 3]);
        assert_eq!(indexes_of(&[4, 1], 7), Vec::<usize>::new());
        assert_eq!(indexes_of(&[], 7), Vec::<usize>::new());
    }

    #[test]
    fn first_and_last_negative() {
        assert_eq!(first_negative(&[3, -1, 5, -2]), Some(1));
        assert_eq!(last_negative(&[3, -1, 5, -2]), Some(3));
        assert_eq!(first_negative(&[1, 2]), None);
        assert_eq!(last_negative(&[]), None);
        assert_eq!(last_negative(&[-5, 0]), Some(0));
    }

    #[test]
    fn index_of_max_prefers_the_first() {
        assert_eq!(index_of_max(&[2, 9, 4, 9]), Some(1));
        assert_eq!(index_of_max(&[7, 7, 7]), Some(0));
        assert_eq!(index_of_max(&[-3, -1, -2]), Some(1));
        assert_eq!(index_of_max(&[]), None);
    }

    #[test]
    fn numbered_items() {
        assert_eq!(numbered(&["tea", "milk"]), vec!["1. tea", "2. milk"]);
        assert_eq!(numbered(&[]), Vec::<String>::new());
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..100_000).map(|i| (i * 7919) % 1000).collect();
        assert_eq!(index_of_max(&nums), nums.iter().position(|&v| v == 999));
        assert_eq!(indexes_of(&nums, 0).len(), 100);
    }
}
