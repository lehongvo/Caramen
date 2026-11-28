//! # Problem 0549: Typed IDs with `PhantomData`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `PhantomData<T>`, manual `Clone`/`Copy`/`PartialEq`/`Eq`/`Hash`/`Debug` impls without bounds on `T`, why `#[derive]` adds bounds
//! - **Source:** Original · [std::marker::PhantomData](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
//!
//! ## Problem
//!
//! Using plain `u32` ids makes it easy to pass a user id where an order id is expected. A **typed id**
//! `Id<T>` stores a `u32` but carries the entity type `T` in its type, so `Id<User>` and `Id<Order>`
//! are different types:
//!
//! ```text
//! pub struct Id<T> { raw: u32, _marker: PhantomData<T> }
//! ```
//!
//! `PhantomData<T>` is a zero-sized field that "uses" `T` without storing one.
//!
//! The entity types `User` and `Order` are plain marker structs that implement **no traits at all**.
//! `#[derive(Clone, Copy, PartialEq, ...)]` on `Id<T>` would generate `impl<T: Clone> Clone for Id<T>`,
//! which is useless here because `User` is not `Clone`. So implement by hand, **without any bound on
//! `T`**: `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash` (all based on `raw` only) and
//! `Debug` (printed as `Id(7)`).
//!
//! Also implement:
//!
//! - `Id::<T>::new(raw: u32) -> Self` and `raw(self) -> u32`.
//! - `IdGenerator<T>` handing out `Id(1)`, `Id(2)`, ... with `new()`, `next_id(&mut self) -> Id<T>` and
//!   `Default`.
//! - `unique_sorted<T>(ids: &[Id<T>]) -> Vec<Id<T>>`: the distinct ids in increasing order (use a
//!   `HashSet<Id<T>>` or sort and dedup: both need your impls).
//!
//! ## Examples
//!
//! ```text
//! let mut users: IdGenerator<User> = IdGenerator::new();
//! let a = users.next_id();          // Id(1)
//! let b = a;                        // Copy, even though User is not Copy
//! a == b                            => true
//! format!("{:?}", users.next_id())  => "Id(2)"
//! unique_sorted(&[Id::new(3), Id::new(1), Id::new(3)]) => [Id(1), Id(3)]
//! // let wrong: Id<Order> = a;      // does not compile: mismatched types
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `impl<T> Clone for Id<T> { fn clone(&self) -> Self { *self } }` together with
//!    `impl<T> Copy for Id<T> {}`.
//! 2. `impl<T> Hash for Id<T>` hashes only `self.raw`, matching `PartialEq` which compares only `raw`.
//! 3. `PhantomData` is constructed with the unit-like expression `PhantomData`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every trait impl looks only at `raw`, and none of them mentions `T` in a bound. The
//! generator keeps a counter and wraps it in `Id::new`.
//!
//! **Rust concepts:**
//! - `#[derive]` on a generic type adds a bound for every type parameter (`T: Clone`, `T: PartialEq`, ...)
//!   even if the field that uses `T` would not need it. Manual impls avoid over-constrained types.
//! - `PhantomData<T>` makes the struct generic over `T` at zero runtime cost: `size_of::<Id<User>>()`
//!   equals `size_of::<u32>()`.
//! - `Copy` requires `Clone`; the idiomatic manual `Clone` for a `Copy` type is `*self`.
//! - Consistency rules still apply: `Hash` and `Ord` agree with `PartialEq` because all of them use the
//!   same field.
// @solution-end

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// A user entity (implements no traits on purpose).
pub struct User;

/// An order entity (implements no traits on purpose).
pub struct Order;

/// An id for entities of type `T`.
pub struct Id<T> {
    raw: u32,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(raw: u32) -> Self {
        // @solution-start
        Id {
            raw,
            _marker: PhantomData,
        }
        // @solution-end
    }

    pub fn raw(self) -> u32 {
        // @solution-start
        self.raw
        // @solution-end
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        // @solution-start
        *self
        // @solution-end
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        // @solution-start
        self.raw == other.raw
        // @solution-end
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // @solution-start
        Some(self.cmp(other))
        // @solution-end
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // @solution-start
        self.raw.cmp(&other.raw)
        // @solution-end
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // @solution-start
        self.raw.hash(state);
        // @solution-end
    }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // @solution-start
        write!(f, "Id({})", self.raw)
        // @solution-end
    }
}

/// Hands out increasing ids starting at 1.
pub struct IdGenerator<T> {
    next: u32,
    _marker: PhantomData<T>,
}

impl<T> IdGenerator<T> {
    pub fn new() -> Self {
        // @solution-start
        IdGenerator {
            next: 1,
            _marker: PhantomData,
        }
        // @solution-end
    }

    pub fn next_id(&mut self) -> Id<T> {
        // @solution-start
        let id = Id::new(self.next);
        self.next += 1;
        id
        // @solution-end
    }
}

impl<T> Default for IdGenerator<T> {
    fn default() -> Self {
        // @solution-start
        Self::new()
        // @solution-end
    }
}

/// The distinct ids in increasing order.
pub fn unique_sorted<T>(ids: &[Id<T>]) -> Vec<Id<T>> {
    // @solution-start
    let set: HashSet<Id<T>> = ids.iter().copied().collect();
    let mut out: Vec<Id<T>> = set.into_iter().collect();
    out.sort();
    out
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_and_copy() {
        let mut users: IdGenerator<User> = IdGenerator::new();
        let a = users.next_id();
        let b = a; // `a` is still usable: Id<User> is Copy
        assert_eq!(a, b);
        assert_eq!(a.raw(), 1);
        assert_eq!(users.next_id().raw(), 2);
        let mut orders: IdGenerator<Order> = IdGenerator::default();
        assert_eq!(orders.next_id().raw(), 1);
    }

    #[test]
    fn debug_and_ordering() {
        let id: Id<Order> = Id::new(7);
        assert_eq!(format!("{id:?}"), "Id(7)");
        assert!(Id::<User>::new(2) < Id::<User>::new(10));
        assert_eq!(Id::<User>::new(4).cmp(&Id::new(4)), Ordering::Equal);
        assert_ne!(Id::<User>::new(4), Id::new(5));
        #[allow(clippy::clone_on_copy)]
        let cloned = id.clone();
        assert_eq!(cloned, id);
    }

    #[test]
    fn hash_set_and_unique_sorted() {
        let ids: Vec<Id<User>> = [3, 1, 3, 2, 1].into_iter().map(Id::new).collect();
        assert_eq!(
            unique_sorted(&ids),
            vec![Id::new(1), Id::new(2), Id::new(3)]
        );
        let set: HashSet<Id<User>> = ids.into_iter().collect();
        assert_eq!(set.len(), 3);
        assert!(set.contains(&Id::new(2)));
    }

    #[test]
    fn zero_cost_and_many_ids() {
        assert_eq!(std::mem::size_of::<Id<User>>(), std::mem::size_of::<u32>());
        let mut generator: IdGenerator<Order> = IdGenerator::new();
        let ids: Vec<Id<Order>> = (0..100_000).map(|_| generator.next_id()).collect();
        let mut doubled = ids.clone();
        doubled.extend(ids.iter().rev().copied());
        let unique = unique_sorted(&doubled);
        assert_eq!(unique.len(), 100_000);
        assert_eq!(unique[99_999].raw(), 100_000);
    }
}
