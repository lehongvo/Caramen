//! # Problem 0610: Shared Tail Lists with Rc
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Rc<T>`, `Rc::clone`, `Rc::strong_count`, `Rc::ptr_eq`, sharing instead of copying
//! - **Source:** Original · [The Rust Book 15.4](https://doc.rust-lang.org/book/ch15-04-rc.html) · [`std::rc::Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)
//!
//! ## Problem
//!
//! With `Box`, every cell of a cons list has exactly one owner, so two lists cannot share a tail.
//! With `Rc` (reference counting), several lists can point at the same tail; the tail is freed
//! when its last owner goes away.
//!
//! ```text
//! pub enum List {
//!     Cons(i32, Rc<List>),
//!     Nil,
//! }
//! ```
//!
//! Implement:
//!
//! - `from_slice(values: &[i32]) -> Rc<List>`: a fresh list with the values in order;
//! - `prepend(value: i32, tail: &Rc<List>) -> Rc<List>`: a new list whose first value is `value`
//!   and whose rest **is** `tail` (shared, not copied);
//! - `tail(list: &Rc<List>) -> Option<Rc<List>>`: a shared handle to the rest of the list, or
//!   `None` for `Nil`;
//! - `to_vec(list: &List) -> Vec<i32>`: the values from front to back.
//!
//! The tests check the reference counts, so sharing must really happen.
//!
//! ## Examples
//!
//! ```text
//! let a = from_slice(&[5, 10]);       // strong_count(&a) == 1
//! let b = prepend(3, &a);             // strong_count(&a) == 2
//! let c = prepend(4, &a);             // strong_count(&a) == 3
//! to_vec(&b) == [3, 5, 10], to_vec(&c) == [4, 5, 10]
//! Rc::ptr_eq(&tail(&b).unwrap(), &a) == true   // the temporary handle is dropped right away
//! drop(c);                            // strong_count(&a) == 2
//! ```
//!
//! ## Constraints
//!
//! - `0 <= values.len() <= 10^4`
//!
//! ## Hints
//!
//! 1. `Rc::clone(&tail)` only increments a counter; it does not copy the list.
//! 2. Build `from_slice` from the back, like a `Box` cons list: start with `Rc::new(List::Nil)`.
//! 3. In `tail`, match on `&**list` (an `&List`) and clone the `Rc` you find in the `Cons` cell.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every function either creates exactly one new cell (`Rc::new(Cons(value, ...))`) or
//! hands out another `Rc` handle to an existing cell. Because `Rc<List>` derefs to `List`, the
//! walking code is the same as for a `Box` list.
//!
//! **Complexity:** `prepend` and `tail` are `O(1)`; `from_slice` and `to_vec` are `O(n)`.
//!
//! **Rust concepts:**
//! - `Rc::clone(&x)` is the idiomatic spelling of `x.clone()` for `Rc`: it signals a cheap counter
//!   increment, not a deep copy.
//! - `Rc::strong_count` reports how many `Rc` handles point at the allocation.
//! - `Rc` only gives shared (read-only) access; mutation needs interior mutability (`RefCell`).
// @solution-end

use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn from_slice(values: &[i32]) -> Rc<List> {
    // @solution-start
    values
        .iter()
        .rev()
        .fold(Rc::new(List::Nil), |tail, &value| {
            Rc::new(List::Cons(value, tail))
        })
    // @solution-end
}

pub fn prepend(value: i32, tail: &Rc<List>) -> Rc<List> {
    // @solution-start
    Rc::new(List::Cons(value, Rc::clone(tail)))
    // @solution-end
}

pub fn tail(list: &Rc<List>) -> Option<Rc<List>> {
    // @solution-start
    match &**list {
        List::Cons(_, rest) => Some(Rc::clone(rest)),
        List::Nil => None,
    }
    // @solution-end
}

pub fn to_vec(list: &List) -> Vec<i32> {
    // @solution-start
    let mut values = Vec::new();
    let mut current = list;
    while let List::Cons(value, rest) = current {
        values.push(*value);
        current = rest;
    }
    values
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_example_counts() {
        let a = from_slice(&[5, 10]);
        assert_eq!(Rc::strong_count(&a), 1);
        let b = prepend(3, &a);
        assert_eq!(Rc::strong_count(&a), 2);
        {
            let c = prepend(4, &a);
            assert_eq!(Rc::strong_count(&a), 3);
            assert_eq!(to_vec(&c), vec![4, 5, 10]);
        }
        assert_eq!(Rc::strong_count(&a), 2);
        assert_eq!(to_vec(&b), vec![3, 5, 10]);
    }

    #[test]
    fn tail_is_shared_not_copied() {
        let a = from_slice(&[5, 10]);
        let b = prepend(3, &a);
        let rest = tail(&b).unwrap();
        assert!(Rc::ptr_eq(&rest, &a));
        assert_eq!(Rc::strong_count(&a), 3);
        drop(b);
        assert_eq!(Rc::strong_count(&a), 2);
    }

    #[test]
    fn empty_list() {
        let empty = from_slice(&[]);
        assert_eq!(*empty, List::Nil);
        assert_eq!(tail(&empty), None);
        assert!(to_vec(&empty).is_empty());
        let one = prepend(1, &empty);
        assert_eq!(to_vec(&one), vec![1]);
        assert_eq!(*tail(&one).unwrap(), List::Nil);
    }

    #[test]
    fn many_lists_share_one_tail() {
        let shared = from_slice(&[7, 8, 9]);
        let heads: Vec<Rc<List>> = (0..100).map(|i| prepend(i, &shared)).collect();
        assert_eq!(Rc::strong_count(&shared), 101);
        assert_eq!(to_vec(&heads[42]), vec![42, 7, 8, 9]);
        drop(heads);
        assert_eq!(Rc::strong_count(&shared), 1);
    }
}
