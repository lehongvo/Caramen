//! # Problem 0622: Josephus Circle with Rc
//!
//! - **Difficulty:** Medium
//! - **Topics:** circular `Rc<RefCell<..>>` list, unlinking nodes with `Option::take`, `std::iter::from_fn`, breaking a cycle in `Drop`
//! - **Source:** Original · [The Rust Book 15.6](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
//!
//! ## Problem
//!
//! `n` people, numbered `1` to `n`, sit in a circle. Starting from person `1`, they count `1, 2,
//! ..., k` around the circle; person number `k` in the count leaves, and the next count starts
//! with the person after them. This repeats until nobody is left.
//!
//! Model the circle as a **circular** singly linked list of shared seats. The layout is given:
//!
//! ```text
//! pub struct Circle {
//!     before: Option<Rc<RefCell<Seat>>>,   // the seat just BEFORE the one that counts "1"
//!     len: usize,
//! }
//!
//! struct Seat {
//!     id: usize,
//!     next: Option<Rc<RefCell<Seat>>>,     // never None while the seat is in the circle
//! }
//! ```
//!
//! Implement:
//!
//! - `Circle::new(n: usize) -> Circle`: seats `1..=n` linked in order, the last one pointing back
//!   to the first; `before` is seat `n` (so seat `1` counts first). `new(0)` is an empty circle;
//! - `len(&self)` and `is_empty(&self)`;
//! - `eliminate(&mut self, k: usize) -> Option<usize>`: run one round of counting (`k >= 1`), unlink
//!   the chosen seat and return its id; `None` if the circle is empty;
//! - `impl Drop for Circle`: the seats form a cycle of strong `Rc`s, which would never be freed.
//!   Break every link, with a loop;
//! - `josephus(n: usize, k: usize) -> Vec<usize>`: the order in which people leave, using `Circle`.
//!
//! ## Examples
//!
//! ```text
//! josephus(7, 3) -> [3, 6, 2, 7, 5, 1, 4]
//! josephus(5, 2) -> [2, 4, 1, 5, 3]
//! josephus(4, 1) -> [1, 2, 3, 4]
//! josephus(1, 9) -> [1]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 5000`, `1 <= k <= 10^4`
//!
//! ## Hints
//!
//! 1. Build the seats in a `Vec`, link `seats[i].next = seats[i + 1]`, then the last to the first.
//! 2. In `eliminate`, move `before` forward `(k - 1) % len` times; the victim is `before.next`.
//!    Re-link `before.next = victim.next` and clear the victim's `next`.
//! 3. When the last person leaves, `before` and the victim are the same seat (it points to
//!    itself): clear its `next` and set `before` to `None`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** keep a handle to the seat *before* the counting start, because a singly linked list
//! can only unlink a node through its predecessor. Each round walks `(k - 1) % len` steps, splices
//! the victim out and leaves `before` where it is, so the next count starts right after the victim.
//! The destructor takes `before.next` and follows the chain, taking each `next`, until it reaches
//! the seat whose link was already cleared.
//!
//! **Complexity:** `josephus` is `O(n * min(k, n))` time and `O(n)` space.
//!
//! **Rust concepts:**
//! - A cycle of `Rc`s keeps every member's strong count above zero forever; it must be broken by
//!   hand (here) or avoided with `Weak` (next problems).
//! - `let next = seat.borrow().next.clone();` ends the borrow before the variable is reassigned,
//!   which keeps the `RefCell` guards short.
//! - `Option::take` on `next` both unlinks and hands over ownership of the following seat.
//! - `std::iter::from_fn(|| circle.eliminate(k))` turns the repeated call into an iterator that
//!   ends at the first `None`, so `josephus` is a single `collect`.
// @solution-end

use std::cell::RefCell;
use std::rc::Rc;

pub struct Circle {
    before: Option<Rc<RefCell<Seat>>>,
    len: usize,
}

struct Seat {
    id: usize,
    next: Option<Rc<RefCell<Seat>>>,
}

impl Circle {
    pub fn new(n: usize) -> Circle {
        // @solution-start
        let seats: Vec<Rc<RefCell<Seat>>> = (1..=n)
            .map(|id| Rc::new(RefCell::new(Seat { id, next: None })))
            .collect();
        for (i, seat) in seats.iter().enumerate() {
            let next = Rc::clone(&seats[(i + 1) % n]);
            seat.borrow_mut().next = Some(next);
        }
        Circle {
            before: seats.last().cloned(),
            len: n,
        }
        // @solution-end
    }

    pub fn len(&self) -> usize {
        // @solution-start
        self.len
        // @solution-end
    }

    pub fn is_empty(&self) -> bool {
        // @solution-start
        self.len == 0
        // @solution-end
    }

    pub fn eliminate(&mut self, k: usize) -> Option<usize> {
        // @solution-start
        let mut before = self.before.take()?;
        for _ in 0..(k - 1) % self.len {
            let next = before.borrow().next.clone()?;
            before = next;
        }
        let victim = before.borrow_mut().next.take()?;
        let id = victim.borrow().id;
        self.len -= 1;
        if self.len > 0 {
            let after = victim.borrow_mut().next.take();
            before.borrow_mut().next = after;
            self.before = Some(before);
        }
        // When the last seat leaves, `victim` is `before` itself and its self-link was taken above.
        Some(id)
        // @solution-end
    }
}

impl Drop for Circle {
    fn drop(&mut self) {
        // @solution-start replace: // TODO: break the cycle of strong links, with a loop.
        if let Some(start) = self.before.take() {
            let mut link = start.borrow_mut().next.take();
            while let Some(seat) = link {
                link = seat.borrow_mut().next.take();
            }
        }
        // @solution-end
    }
}

pub fn josephus(n: usize, k: usize) -> Vec<usize> {
    // @solution-start
    let mut circle = Circle::new(n);
    std::iter::from_fn(|| circle.eliminate(k)).collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Weak;

    #[test]
    fn classic_examples() {
        assert_eq!(josephus(7, 3), vec![3, 6, 2, 7, 5, 1, 4]);
        assert_eq!(josephus(5, 2), vec![2, 4, 1, 5, 3]);
    }

    #[test]
    fn k_of_one_and_tiny_circles() {
        assert_eq!(josephus(4, 1), vec![1, 2, 3, 4]);
        assert_eq!(josephus(1, 9), vec![1]);
        assert!(josephus(0, 3).is_empty());
    }

    #[test]
    fn eliminate_step_by_step() {
        let mut circle = Circle::new(3);
        assert_eq!(circle.len(), 3);
        assert_eq!(circle.eliminate(2), Some(2));
        // Counting restarts at 3: 3, 1, 3, 1, 3.
        assert_eq!(circle.eliminate(5), Some(3));
        assert_eq!(circle.len(), 1);
        assert_eq!(circle.eliminate(1), Some(1));
        assert!(circle.is_empty());
        assert_eq!(circle.eliminate(1), None);
    }

    #[test]
    fn dropping_a_circle_frees_every_seat() {
        let circle = Circle::new(50);
        let watcher: Weak<RefCell<Seat>> = Rc::downgrade(circle.before.as_ref().unwrap());
        assert!(watcher.upgrade().is_some());
        drop(circle);
        assert!(watcher.upgrade().is_none());
    }

    #[test]
    fn partially_emptied_circle_is_freed() {
        let mut circle = Circle::new(10);
        circle.eliminate(4);
        circle.eliminate(4);
        let watcher = Rc::downgrade(circle.before.as_ref().unwrap());
        drop(circle);
        assert!(watcher.upgrade().is_none());
    }

    #[test]
    fn large_circle() {
        let order = josephus(5_000, 7);
        assert_eq!(order.len(), 5_000);
        assert_eq!(order[0], 7);
        assert_eq!(order[1], 14);
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (1..=5_000).collect::<Vec<_>>());
    }
}
