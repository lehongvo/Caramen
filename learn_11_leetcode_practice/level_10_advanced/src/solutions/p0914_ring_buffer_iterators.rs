//! # Problem 0914: A Ring Buffer with Double-Ended Iterators
//!
//! - **Difficulty:** Hard
//! - **Topics:** `DoubleEndedIterator`, `ExactSizeIterator`, `FusedIterator`, `IntoIterator` for `T` and `&T`, modular indexing
//! - **Source:** Original · [std: `DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html) · [std: `ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html) · [std: `FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html)
//!
//! ## Problem
//!
//! A ring buffer keeps the **last `capacity` values** pushed into it: pushing into a full buffer
//! overwrites the oldest value. It is stored in a fixed `Vec<Option<T>>` of length `capacity`; the
//! oldest value sits at index `head` and the `len` live values continue from there, wrapping around
//! the end of the vector.
//!
//! Implement `RingBuffer<T>`:
//!
//! - `with_capacity(capacity)`: panics with `"capacity must be positive"` if `capacity == 0`.
//! - `capacity()`, `len()`, `is_empty()`.
//! - `push(&mut self, value: T) -> Option<T>`: appends as the newest value; when the buffer is full,
//!   the oldest value is removed and returned.
//! - `pop_front(&mut self) -> Option<T>` (oldest) and `pop_back(&mut self) -> Option<T>` (newest).
//! - `iter(&self) -> Iter<'_, T>`: borrows the values from oldest to newest.
//!
//! Then the iterators:
//!
//! - `Iter<'a, T>` yields `&'a T`. It implements `Iterator` with an **exact** `size_hint`,
//!   `DoubleEndedIterator` (`next_back` yields from the newest end; the two ends never cross),
//!   `ExactSizeIterator` and `FusedIterator` (once it returns `None` it keeps returning `None`).
//! - `IntoIter<T>` (from `RingBuffer::into_iter`, by value) yields owned `T`, oldest first, and is
//!   also double-ended, exact-size and fused.
//! - `impl IntoIterator for &RingBuffer<T>` (so `for x in &ring` works) and for `RingBuffer<T>`.
//!
//! ## Examples
//!
//! ```text
//! let mut ring = RingBuffer::with_capacity(3);
//! ring.push(1); ring.push(2); ring.push(3);   // each returns None
//! ring.push(4)                                // => Some(1); ring is now 2, 3, 4
//! ring.iter().collect::<Vec<_>>()             // => [&2, &3, &4]
//! ring.iter().rev().collect::<Vec<_>>()       // => [&4, &3, &2]
//! ring.iter().len()                           // => 3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= capacity <= 10^5`
//!
//! ## Hints
//!
//! 1. The `i`-th oldest live value is at index `(head + i) % capacity`.
//! 2. Let `Iter` keep the range of logical positions it still has to yield: `front..back`. `next`
//!    takes `front`, `next_back` takes `back - 1`, and both stop when `front == back`.
//! 3. `IntoIter` can simply wrap the buffer and call `pop_front` / `pop_back`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** slots are `Option<T>`, so values can be moved out with `take()`. `push` writes at
//! logical position `len` (or overwrites position `0` and advances `head` when full). `Iter` stores a
//! reference to the buffer and the logical range `front..back`; both ends shrink the same range, so
//! they cannot cross and the remaining length is always `back - front`.
//!
//! **Complexity:** every operation is `O(1)`; iterating is `O(len)`.
//!
//! **Rust concepts:**
//! - `DoubleEndedIterator` only adds `next_back`; `rev`, `rfind`, `nth_back`... come for free.
//! - `ExactSizeIterator` has no required method: its `len()` uses `size_hint`, which must therefore
//!   be exact (`(n, Some(n))`).
//! - `FusedIterator` is a marker trait: a promise that lets `Iterator::fuse` skip its extra checks.
//! - `IntoIterator for &'a RingBuffer<T>` with `Item = &'a T` is what a `for` loop over `&ring` uses.
// @solution-end

use std::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct RingBuffer<T> {
    slots: Vec<Option<T>>,
    head: usize,
    len: usize,
}

impl<T> RingBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        // @solution-start
        assert!(capacity > 0, "capacity must be positive");
        RingBuffer {
            slots: (0..capacity).map(|_| None).collect(),
            head: 0,
            len: 0,
        }
        // @solution-end
    }

    pub fn capacity(&self) -> usize {
        // @solution-start
        self.slots.len()
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

    // @solution-start remove
    /// Physical index of the logical position `i` (0 = oldest).
    fn index(&self, i: usize) -> usize {
        (self.head + i) % self.slots.len()
    }
    // @solution-end

    pub fn push(&mut self, value: T) -> Option<T> {
        // @solution-start
        if self.len == self.capacity() {
            // Full: the oldest slot becomes the newest one.
            let oldest = self.slots[self.head].replace(value);
            self.head = self.index(1);
            oldest
        } else {
            let index = self.index(self.len);
            self.slots[index] = Some(value);
            self.len += 1;
            None
        }
        // @solution-end
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // @solution-start
        if self.len == 0 {
            return None;
        }
        let value = self.slots[self.head].take();
        self.head = self.index(1);
        self.len -= 1;
        value
        // @solution-end
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // @solution-start
        if self.len == 0 {
            return None;
        }
        let index = self.index(self.len - 1);
        self.len -= 1;
        self.slots[index].take()
        // @solution-end
    }

    pub fn iter(&self) -> Iter<'_, T> {
        // @solution-start
        Iter {
            ring: self,
            front: 0,
            back: self.len,
        }
        // @solution-end
    }
}

/// Borrowing iterator over a ring buffer, oldest to newest.
#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    ring: &'a RingBuffer<T>,
    front: usize,
    back: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        // @solution-start
        if self.front == self.back {
            return None;
        }
        let index = self.ring.index(self.front);
        self.front += 1;
        self.ring.slots[index].as_ref()
        // @solution-end
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // @solution-start
        let remaining = self.back - self.front;
        (remaining, Some(remaining))
        // @solution-end
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // @solution-start
        if self.front == self.back {
            return None;
        }
        self.back -= 1;
        self.ring.slots[self.ring.index(self.back)].as_ref()
        // @solution-end
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}
impl<T> FusedIterator for Iter<'_, T> {}

/// Owning iterator over a ring buffer, oldest to newest.
#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    ring: RingBuffer<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // @solution-start
        self.ring.pop_front()
        // @solution-end
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // @solution-start
        (self.ring.len(), Some(self.ring.len()))
        // @solution-end
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        // @solution-start
        self.ring.pop_back()
        // @solution-end
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}
impl<T> FusedIterator for IntoIter<T> {}

impl<'a, T> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        // @solution-start
        self.iter()
        // @solution-end
    }
}

impl<T> IntoIterator for RingBuffer<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        // @solution-start
        IntoIter { ring: self }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ring_of(capacity: usize, values: &[i32]) -> RingBuffer<i32> {
        let mut ring = RingBuffer::with_capacity(capacity);
        for &v in values {
            ring.push(v);
        }
        ring
    }

    #[test]
    fn push_overwrites_the_oldest() {
        let mut ring = RingBuffer::with_capacity(3);
        assert_eq!(
            (ring.push(1), ring.push(2), ring.push(3)),
            (None, None, None)
        );
        assert_eq!(ring.push(4), Some(1));
        assert_eq!(ring.iter().collect::<Vec<_>>(), vec![&2, &3, &4]);
        assert_eq!(
            (ring.len(), ring.capacity(), ring.is_empty()),
            (3, 3, false)
        );
    }

    #[test]
    fn iterates_from_both_ends() {
        let ring = ring_of(4, &[1, 2, 3, 4, 5, 6]);
        assert_eq!(
            ring.iter().rev().copied().collect::<Vec<_>>(),
            vec![6, 5, 4, 3]
        );
        let mut it = ring.iter();
        assert_eq!(it.len(), 4);
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next_back(), Some(&6));
        assert_eq!(it.len(), 2);
        assert_eq!(it.next_back(), Some(&5));
        assert_eq!(it.next(), Some(&4));
        assert_eq!((it.next(), it.next_back(), it.next()), (None, None, None));
        assert_eq!(it.size_hint(), (0, Some(0)));
    }

    #[test]
    fn pops_and_owned_iteration() {
        let mut ring = ring_of(3, &[10, 20, 30, 40]);
        assert_eq!(ring.pop_front(), Some(20));
        assert_eq!(ring.pop_back(), Some(40));
        ring.push(50);
        ring.push(60);
        assert_eq!(ring.push(70), Some(30));
        let owned: Vec<i32> = ring.clone().into_iter().collect();
        assert_eq!(owned, vec![50, 60, 70]);
        let mut it = ring.into_iter();
        assert_eq!(it.len(), 3);
        assert_eq!(it.next_back(), Some(70));
        assert_eq!(it.collect::<Vec<_>>(), vec![50, 60]);
    }

    #[test]
    fn for_loops_and_empty_buffer() {
        let ring = ring_of(5, &[1, 2, 3]);
        let mut sum = 0;
        for value in &ring {
            sum += value;
        }
        assert_eq!(sum, 6);
        let mut empty: RingBuffer<String> = RingBuffer::with_capacity(2);
        assert!(empty.is_empty());
        assert_eq!(empty.iter().next(), None);
        assert_eq!((empty.pop_front(), empty.pop_back()), (None, None));
    }

    #[test]
    #[should_panic(expected = "capacity must be positive")]
    fn zero_capacity_panics() {
        let _ring: RingBuffer<u8> = RingBuffer::with_capacity(0);
    }

    #[test]
    fn many_wraparounds_with_strings() {
        let mut ring = RingBuffer::with_capacity(7);
        for i in 0..10_000 {
            ring.push(i.to_string());
        }
        let last: Vec<&str> = ring.iter().map(String::as_str).collect();
        assert_eq!(
            last,
            ["9993", "9994", "9995", "9996", "9997", "9998", "9999"]
        );
        assert_eq!(ring.iter().rev().nth(2).map(String::as_str), Some("9997"));
        assert_eq!(ring.iter().rposition(|s| s == "9994"), Some(1));
    }
}
