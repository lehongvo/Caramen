//! # Problem 0617: Runtime Borrow Checks
//!
//! - **Difficulty:** Medium
//! - **Topics:** `RefCell::try_borrow`, `try_borrow_mut`, `BorrowError`/`BorrowMutError`, borrow panics, guard lifetimes
//! - **Source:** Original · [The Rust Book 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) · [`RefCell::try_borrow_mut`](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.try_borrow_mut)
//!
//! ## Problem
//!
//! A `RefCell<T>` enforces the borrowing rules while the program runs: any number of `Ref` guards
//! (from `borrow`) **or** one `RefMut` guard (from `borrow_mut`) may exist at a time. Breaking the
//! rule with `borrow`/`borrow_mut` panics; the `try_` versions return an `Err` instead.
//!
//! Implement three functions:
//!
//! 1. `borrow_state<T>(cell: &RefCell<T>) -> BorrowState` reports what the cell allows right now:
//!    `Unused` (no guard alive), `Reading` (only shared guards alive) or `Writing` (a mutable guard
//!    is alive). It must not panic and must not keep any borrow after it returns.
//! 2. `try_update<T>(cell: &RefCell<T>, update: impl FnOnce(&mut T)) -> bool` applies `update` and
//!    returns `true` if the cell can be borrowed mutably right now; otherwise it changes nothing
//!    and returns `false`.
//! 3. `push_len(cell: &RefCell<Vec<usize>>)` appends the vector's current length to it. The
//!    one-liner `cell.borrow_mut().push(cell.borrow().len())` **panics** (the `RefMut` is alive
//!    while `borrow()` runs). Write a version that works.
//!
//! The tests also show the two panic messages of `RefCell` in Rust 1.87:
//! `"already borrowed: BorrowMutError"` and `"already mutably borrowed: BorrowError"`.
//!
//! ## Examples
//!
//! ```text
//! let cell = RefCell::new(5);
//! borrow_state(&cell)                         -> Unused
//! let r = cell.borrow();   borrow_state(&cell) -> Reading
//! try_update(&cell, |v| *v += 1)              -> false   (value still 5)
//! drop(r);
//! try_update(&cell, |v| *v += 1)              -> true    (value 6)
//! let w = cell.borrow_mut(); borrow_state(&cell) -> Writing
//!
//! let v = RefCell::new(vec![]);
//! push_len(&v); push_len(&v); push_len(&v);   // v == [0, 1, 2]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. If `try_borrow_mut` succeeds, nobody holds a guard. If it fails but `try_borrow` succeeds,
//!    only readers exist. If both fail, there is a writer. Drop the guard you obtained before
//!    returning (it is dropped automatically if you do not keep it).
//! 2. `if let Ok(mut value) = cell.try_borrow_mut() { ... }` keeps the guard alive exactly for the
//!    block.
//! 3. For `push_len`, finish the shared borrow (store the length in a local) **before** asking for
//!    the mutable one.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each function asks the `RefCell` for a guard with the `try_` methods and never keeps a
//! guard longer than needed. In `push_len`, the statement `let len = cell.borrow().len();` drops
//! its temporary `Ref` at the semicolon, so the following `borrow_mut` succeeds.
//!
//! **Complexity:** `O(1)` per call (amortized for `push`).
//!
//! **Rust concepts:**
//! - Guards (`Ref`, `RefMut`) release the borrow when they are dropped; temporaries are dropped at
//!   the end of the enclosing statement.
//! - In `a.borrow_mut().push(a.borrow().len())` the receiver guard is created first and is still
//!   alive while the argument is evaluated, hence the panic.
//! - `try_borrow` and `try_borrow_mut` (stable since 1.13) turn a would-be panic into a `Result`.
// @solution-end

use std::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowState {
    Unused,
    Reading,
    Writing,
}

pub fn borrow_state<T>(cell: &RefCell<T>) -> BorrowState {
    // @solution-start
    if cell.try_borrow_mut().is_ok() {
        BorrowState::Unused
    } else if cell.try_borrow().is_ok() {
        BorrowState::Reading
    } else {
        BorrowState::Writing
    }
    // @solution-end
}

pub fn try_update<T>(cell: &RefCell<T>, update: impl FnOnce(&mut T)) -> bool {
    // @solution-start
    match cell.try_borrow_mut() {
        Ok(mut value) => {
            update(&mut value);
            true
        }
        Err(_) => false,
    }
    // @solution-end
}

pub fn push_len(cell: &RefCell<Vec<usize>>) {
    // @solution-start
    let len = cell.borrow().len();
    cell.borrow_mut().push(len);
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_follows_the_live_guards() {
        let cell = RefCell::new(5);
        assert_eq!(borrow_state(&cell), BorrowState::Unused);
        {
            let _r1 = cell.borrow();
            let _r2 = cell.borrow();
            assert_eq!(borrow_state(&cell), BorrowState::Reading);
        }
        {
            let _w = cell.borrow_mut();
            assert_eq!(borrow_state(&cell), BorrowState::Writing);
        }
        assert_eq!(borrow_state(&cell), BorrowState::Unused);
    }

    #[test]
    fn try_update_respects_existing_borrows() {
        let cell = RefCell::new(5);
        let reader = cell.borrow();
        assert!(!try_update(&cell, |v| *v += 1));
        drop(reader);
        assert!(try_update(&cell, |v| *v += 1));
        assert_eq!(*cell.borrow(), 6);
        let writer = cell.borrow_mut();
        assert!(!try_update(&cell, |v| *v = 0));
        drop(writer);
        assert_eq!(*cell.borrow(), 6);
    }

    #[test]
    fn push_len_does_not_panic() {
        let v = RefCell::new(Vec::new());
        push_len(&v);
        push_len(&v);
        push_len(&v);
        assert_eq!(*v.borrow(), vec![0, 1, 2]);
        assert_eq!(borrow_state(&v), BorrowState::Unused);
    }

    #[test]
    #[should_panic(expected = "already borrowed: BorrowMutError")]
    fn borrow_mut_while_reading_panics() {
        let cell = RefCell::new(vec![1]);
        // Sanity check through your function first, so this test needs your code to pass.
        assert_eq!(borrow_state(&cell), BorrowState::Unused);
        let _reader = cell.borrow();
        cell.borrow_mut().push(2);
    }

    #[test]
    #[should_panic(expected = "already mutably borrowed: BorrowError")]
    fn borrow_while_writing_panics() {
        let cell = RefCell::new(0);
        assert!(try_update(&cell, |v| *v = 1));
        let _writer = cell.borrow_mut();
        let _reader = cell.borrow();
    }
}
