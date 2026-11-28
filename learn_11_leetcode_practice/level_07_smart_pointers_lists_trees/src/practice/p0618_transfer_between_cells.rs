// ==================================================================
// PRACTICE 0618: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0618_transfer_between_cells.md
// Run the tests:          cargo test practice::p0618
// ==================================================================

//! # Problem 0618: Transfer Between RefCells
//!
//! - **Difficulty:** Medium
//! - **Topics:** aliasing `&RefCell<T>` arguments, `std::ptr::eq`, `RefCell::swap`, `Vec::drain`, short-lived guards
//! - **Source:** Original · [The Rust Book 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) · [`std::ptr::eq`](https://doc.rust-lang.org/std/ptr/fn.eq.html)
//!
//! ## Problem
//!
//! With plain references, the compiler guarantees that two `&mut Vec<i32>` arguments are different
//! vectors. With `&RefCell<Vec<i32>>` it does not: both parameters may point at the **same** cell,
//! and borrowing it mutably twice panics at run time.
//!
//! Implement two functions that work in every case, including when both arguments are the same
//! cell:
//!
//! 1. `transfer(from: &RefCell<Vec<i32>>, to: &RefCell<Vec<i32>>, count: usize) -> usize` moves
//!    the **last** `count` items of `from` (all of them if it has fewer) to the end of `to`,
//!    keeping their order, and returns how many items were moved. When `from` and `to` are the same
//!    cell, moving items from its end to its end changes nothing; return the number of items that
//!    would have moved.
//! 2. `swap_contents(a: &RefCell<Vec<i32>>, b: &RefCell<Vec<i32>>)` exchanges the two vectors.
//!    `RefCell::swap` panics when both are the same cell; in that case do nothing.
//!
//! ## Examples
//!
//! ```text
//! from = [1, 2, 3, 4], to = [9]
//! transfer(&from, &to, 2) -> 2       from = [1, 2], to = [9, 3, 4]
//! transfer(&from, &to, 5) -> 2       from = [],     to = [9, 3, 4, 1, 2]
//!
//! one = [1, 2, 3]
//! transfer(&one, &one, 2) -> 2       one = [1, 2, 3]  (no panic)
//! swap_contents(&one, &one)          one = [1, 2, 3]  (no panic)
//! ```
//!
//! ## Constraints
//!
//! - Vectors hold at most `10^5` items.
//!
//! ## Hints
//!
//! 1. `std::ptr::eq(from, to)` is `true` when both references point at the same `RefCell`.
//! 2. `vec.drain(start..)` removes a tail range and yields the removed items in order.
//! 3. For different cells, holding one `borrow_mut()` of each at the same time is fine.

use std::cell::RefCell;

pub fn transfer(from: &RefCell<Vec<i32>>, to: &RefCell<Vec<i32>>, count: usize) -> usize {
    todo!("Write your solution here")
}

pub fn swap_contents(a: &RefCell<Vec<i32>>, b: &RefCell<Vec<i32>>) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_moves_the_tail_in_order() {
        let from = RefCell::new(vec![1, 2, 3, 4]);
        let to = RefCell::new(vec![9]);
        assert_eq!(transfer(&from, &to, 2), 2);
        assert_eq!(*from.borrow(), vec![1, 2]);
        assert_eq!(*to.borrow(), vec![9, 3, 4]);
        assert_eq!(transfer(&from, &to, 5), 2);
        assert!(from.borrow().is_empty());
        assert_eq!(*to.borrow(), vec![9, 3, 4, 1, 2]);
        assert_eq!(transfer(&from, &to, 1), 0);
    }

    #[test]
    fn transfer_to_the_same_cell_changes_nothing() {
        let one = RefCell::new(vec![1, 2, 3]);
        assert_eq!(transfer(&one, &one, 2), 2);
        assert_eq!(transfer(&one, &one, 10), 3);
        assert_eq!(*one.borrow(), vec![1, 2, 3]);
    }

    #[test]
    fn swap_contents_of_two_cells() {
        let a = RefCell::new(vec![1]);
        let b = RefCell::new(vec![2, 3]);
        swap_contents(&a, &b);
        assert_eq!(*a.borrow(), vec![2, 3]);
        assert_eq!(*b.borrow(), vec![1]);
    }

    #[test]
    fn swap_with_itself_is_a_no_op() {
        let a = RefCell::new(vec![7, 8]);
        swap_contents(&a, &a);
        assert_eq!(*a.borrow(), vec![7, 8]);
    }

    #[test]
    fn large_transfer() {
        let from = RefCell::new((0..100_000).collect::<Vec<i32>>());
        let to = RefCell::new(Vec::new());
        assert_eq!(transfer(&from, &to, 60_000), 60_000);
        assert_eq!(from.borrow().len(), 40_000);
        assert_eq!(to.borrow()[0], 40_000);
        assert_eq!(*to.borrow().last().unwrap(), 99_999);
    }
}
