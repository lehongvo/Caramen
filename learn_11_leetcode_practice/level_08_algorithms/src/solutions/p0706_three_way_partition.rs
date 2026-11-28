//! # Problem 0706: Three-Way Partition
//!
//! - **Difficulty:** Medium
//! - **Topics:** Dutch national flag (three pointers), generic `T: Ord`, `slice::swap`, loop invariants
//! - **Source:** Original · [`slice::swap`](https://doc.rust-lang.org/std/primitive.slice.html#method.swap)
//!
//! ## Problem
//!
//! Write a generic function that rearranges a mutable slice **in place** around a `pivot` value,
//! in a single pass, so that it becomes three consecutive groups:
//!
//! 1. every element **smaller** than `pivot`,
//! 2. then every element **equal** to `pivot`,
//! 3. then every element **greater** than `pivot`.
//!
//! Return `(lt, gt)` such that `items[..lt]` are all `< pivot`, `items[lt..gt]` are all `== pivot`
//! and `items[gt..]` are all `> pivot`. The order of elements **inside** each group does not
//! matter. Use only `O(1)` extra memory: move elements with `swap`, do not allocate a new vector
//! and do not sort.
//!
//! This is the technique behind LeetCode's "Sort Colors" problem (sorting `0`s, `1`s and `2`s with
//! pivot `1`), and the partition step of three-way quicksort.
//!
//! Implement `three_way_partition<T: Ord>(items: &mut [T], pivot: &T) -> (usize, usize)`.
//!
//! ## Examples
//!
//! ```text
//! Input:  items = [2, 0, 2, 1, 1, 0], pivot = 1
//! Output: (2, 4), items becomes [0, 0, 1, 1, 2, 2]
//!
//! Input:  items = [5, 1, 9, 3], pivot = 4
//! Output: (2, 2)        (no element equals 4; items is [1, 3] then [9, 5] in some order)
//!
//! Input:  items = [], pivot = 7
//! Output: (0, 0)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= items.len() <= 10^5`
//! - `T` only implements `Ord`: you cannot copy or clone elements, only compare and swap them.
//!
//! ## Hints
//!
//! 1. Keep three boundaries: the end of the "smaller" zone, the current position, and the start of
//!    the "greater" zone. Everything between the current position and the "greater" zone is unknown.
//! 2. When you swap an element in from the "greater" side, you have not looked at it yet: do not
//!    advance the current position.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** Dutch national flag, three pointers: maintain the invariant
//! `items[..lt] < pivot`, `items[lt..i] == pivot`, `items[i..gt]` unknown, `items[gt..] > pivot`.
//! Look at `items[i]`:
//! - smaller: swap it to position `lt` (which holds an "equal" element, or is `i` itself), then
//!   advance both `lt` and `i`;
//! - equal: it is already in the right zone, advance `i`;
//! - greater: shrink the "greater" zone with `gt -= 1` and swap `items[i]` there. The element that
//!   comes back to `i` is unknown, so `i` stays.
//!
//! The unknown zone shrinks by one every step, so the loop ends after `n` steps with the three
//! groups in place.
//!
//! **Complexity:** `O(n)` time (one pass, at most `n` swaps), `O(1)` extra space.
//!
//! **Rust concepts:**
//! - The generic bound `T: Ord` is all the function needs: `cmp` returns an `Ordering` to `match`.
//!   No `Clone` or `Copy` is required because elements are only moved with `swap`.
//! - [`slice::swap`](https://doc.rust-lang.org/std/primitive.slice.html#method.swap) exchanges two
//!   positions of a `&mut [T]` without needing two mutable borrows at the same time.
//! - Writing the loop invariant down first is the reliable way to get pointer algorithms right.
// @solution-end

use std::cmp::Ordering;

pub fn three_way_partition<T: Ord>(items: &mut [T], pivot: &T) -> (usize, usize) {
    // @solution-start
    let (mut lt, mut i, mut gt) = (0, 0, items.len());
    while i < gt {
        match items[i].cmp(pivot) {
            Ordering::Less => {
                items.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Equal => i += 1,
            Ordering::Greater => {
                gt -= 1;
                items.swap(i, gt); // the element swapped in is unknown: do not advance `i`
            }
        }
    }
    (lt, gt)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the three groups and that no element was lost or duplicated.
    fn check<T: Ord + Clone + std::fmt::Debug>(original: &[T], pivot: T) -> (usize, usize) {
        let mut items = original.to_vec();
        let (lt, gt) = three_way_partition(&mut items, &pivot);
        assert!(lt <= gt && gt <= items.len());
        assert!(items[..lt].iter().all(|x| *x < pivot), "{items:?}");
        assert!(items[lt..gt].iter().all(|x| *x == pivot), "{items:?}");
        assert!(items[gt..].iter().all(|x| *x > pivot), "{items:?}");
        let (mut a, mut b) = (original.to_vec(), items);
        a.sort();
        b.sort();
        assert_eq!(a, b);
        (lt, gt)
    }

    #[test]
    fn example_colors() {
        let mut items = vec![2, 0, 2, 1, 1, 0];
        assert_eq!(three_way_partition(&mut items, &1), (2, 4));
        assert_eq!(items, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn example_pivot_missing() {
        assert_eq!(check(&[5, 1, 9, 3], 4), (2, 2));
    }

    #[test]
    fn example_empty() {
        let mut items: Vec<i32> = Vec::new();
        assert_eq!(three_way_partition(&mut items, &7), (0, 0));
    }

    #[test]
    fn all_elements_equal() {
        assert_eq!(check(&[3, 3, 3], 3), (0, 3));
    }

    #[test]
    fn works_with_strings_without_clone_in_the_function() {
        let words = ["pear", "apple", "fig", "kiwi", "fig"].map(String::from);
        assert_eq!(check(&words, "fig".to_string()), (1, 3));
    }

    #[test]
    fn large_input() {
        let items: Vec<u32> = (0..100_000).map(|i| (i * 7919) % 11).collect();
        let (lt, gt) = check(&items, 5);
        // Values 0..=4 are smaller: 5 of every 11 residues.
        assert_eq!(lt, items.iter().filter(|&&x| x < 5).count());
        assert_eq!(gt - lt, items.iter().filter(|&&x| x == 5).count());
    }
}
