//! # Problem 0635: Iterators for a Box List
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Iterator` for a linked list, `IntoIter`/`Iter<'a>`/`IterMut<'a>`, `Option::as_deref_mut`, `IntoIterator` for `&List` and `&mut List`
//! - **Source:** Original · [`std::iter`](https://doc.rust-lang.org/std/iter/index.html) · [The Rust Book 13.2](https://doc.rust-lang.org/book/ch13-02-iterators.html)
//!
//! ## Problem
//!
//! Collections in Rust usually offer three iterators: by value (`into_iter`), by shared reference
//! (`iter`) and by mutable reference (`iter_mut`). Write all three for a singly linked stack.
//!
//! The data types are given:
//!
//! ```text
//! pub struct List<T> { head: Option<Box<Node<T>>> }
//! struct Node<T> { elem: T, next: Option<Box<Node<T>>> }
//!
//! pub struct IntoIter<T>(List<T>);
//! pub struct Iter<'a, T> { next: Option<&'a Node<T>> }
//! pub struct IterMut<'a, T> { next: Option<&'a mut Node<T>> }
//! ```
//!
//! Implement:
//!
//! - `List::new`, `push(&mut self, elem: T)` (to the front) and `pop(&mut self) -> Option<T>`;
//! - `into_iter`, `iter(&self) -> Iter<'_, T>`, `iter_mut(&mut self) -> IterMut<'_, T>`;
//! - `Iterator` for `IntoIter<T>` (items `T`), `Iter<'a, T>` (items `&'a T`) and
//!   `IterMut<'a, T>` (items `&'a mut T`), each yielding from the front (the most recently pushed
//!   element first);
//! - `IntoIterator` for `List<T>`, `&'a List<T>` and `&'a mut List<T>`, so that all three `for`
//!   loop forms work.
//!
//! ## Examples
//!
//! ```text
//! let mut list = List::new();
//! list.push(1); list.push(2); list.push(3);
//! list.iter().copied().collect::<Vec<_>>()   -> [3, 2, 1]
//! for x in &mut list { *x *= 10; }
//! list.into_iter().collect::<Vec<_>>()       -> [30, 20, 10]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `IntoIter::next` is just `self.0.pop()`.
//! 2. `Iter::next`: `self.next.map(|node| { self.next = node.next.as_deref(); &node.elem })`.
//!    `Option<&T>` is `Copy`, so `map` does not need `take()`.
//! 3. `IterMut::next`: `&mut` references are not `Copy`, so start with `self.next.take()`, then use
//!    `node.next.as_deref_mut()` for the following node.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each iterator stores "where to continue". The owning iterator wraps the list and pops.
//! The borrowing iterators hold an `Option` of a reference to the next node; `next` hands out a
//! reference to the node's element and advances to `node.next`.
//!
//! **Complexity:** every `next` call is `O(1)`; a full iteration is `O(n)`.
//!
//! **Rust concepts:**
//! - The lifetime `'a` in `Iter<'a, T>` ties every yielded `&'a T` to the borrow of the list, so the
//!   list cannot be changed while an iterator (or an item) is alive.
//! - `take()` on `Option<&mut Node<T>>` is required because two live `&mut` to the same node are
//!   not allowed; moving the reference out avoids that.
//! - Implementing `IntoIterator for &List<T>` is what makes `for x in &list` compile.
// @solution-end

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

// Clippy would like a `Default` impl next to `new()`; it is not the point of this problem.
#[allow(clippy::new_without_default)]
impl<T> List<T> {
    pub fn new() -> Self {
        // @solution-start
        List { head: None }
        // @solution-end
    }

    pub fn push(&mut self, elem: T) {
        // @solution-start
        let next = self.head.take();
        self.head = Some(Box::new(Node { elem, next }));
        // @solution-end
    }

    pub fn pop(&mut self) -> Option<T> {
        // @solution-start
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
        // @solution-end
    }

    pub fn iter(&self) -> Iter<'_, T> {
        // @solution-start
        Iter {
            next: self.head.as_deref(),
        }
        // @solution-end
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        // @solution-start
        IterMut {
            next: self.head.as_deref_mut(),
        }
        // @solution-end
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // @solution-start
        self.0.pop()
        // @solution-end
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        // @solution-start
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
        // @solution-end
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        // @solution-start
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
        // @solution-end
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        // @solution-start
        IntoIter(self)
        // @solution-end
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        // @solution-start
        self.iter()
        // @solution-end
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        // @solution-start
        self.iter_mut()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_of(values: &[i32]) -> List<i32> {
        let mut list = List::new();
        for &v in values {
            list.push(v);
        }
        list
    }

    #[test]
    fn iter_yields_front_first() {
        let list = list_of(&[1, 2, 3]);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![3, 2, 1]);
        // The list is still usable after borrowing iteration.
        assert_eq!(list.iter().count(), 3);
    }

    #[test]
    fn iter_mut_changes_elements() {
        let mut list = list_of(&[1, 2, 3]);
        for x in list.iter_mut() {
            *x *= 10;
        }
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![30, 20, 10]);
    }

    #[test]
    fn into_iter_moves_owned_values() {
        let mut list = List::new();
        list.push(String::from("a"));
        list.push(String::from("b"));
        let owned: Vec<String> = list.into_iter().collect();
        assert_eq!(owned, vec!["b", "a"]);
    }

    #[test]
    fn for_loops_over_all_three_forms() {
        let mut list = list_of(&[1, 2, 3]);
        let mut sum = 0;
        for x in &list {
            sum += x;
        }
        for x in &mut list {
            *x += 1;
        }
        let mut collected = Vec::new();
        for x in list {
            collected.push(x);
        }
        assert_eq!(sum, 6);
        assert_eq!(collected, vec![4, 3, 2]);
    }

    #[test]
    fn empty_list_and_pop() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.iter().next(), None);
        assert_eq!(list.iter_mut().next(), None);
        assert_eq!(list.pop(), None);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.into_iter().next(), None);
    }
}
