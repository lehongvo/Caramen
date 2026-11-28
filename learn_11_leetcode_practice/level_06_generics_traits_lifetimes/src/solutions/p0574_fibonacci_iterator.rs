//! # Problem 0574: A Fibonacci Iterator
//!
//! - **Difficulty:** Easy
//! - **Topics:** implementing `Iterator` (`type Item`, `fn next`), ending an infinite sequence on overflow with `checked_add`, `ExactSizeIterator`, adapters for free
//! - **Source:** Original · [std::iter: implementing Iterator](https://doc.rust-lang.org/std/iter/index.html#implementing-iterator)
//!
//! ## Problem
//!
//! Implement two iterators by hand:
//!
//! 1. `Fibonacci`, created with `fibonacci()`, yields `0, 1, 1, 2, 3, 5, 8, ...` as `u64` and **stops**
//!    after the largest Fibonacci number that fits in a `u64` (F(93) = 12200160415121876738), instead of
//!    overflowing. Use `checked_add`.
//! 2. `Countdown`, created with `Countdown::new(n: u32)`, yields `n, n - 1, ..., 1`. Also implement
//!    `ExactSizeIterator` for it (which needs an exact `size_hint`), so `.len()` works.
//!
//! Once `next` exists, every adapter (`take`, `filter`, `sum`, `zip`, ...) is available for free.
//!
//! ## Examples
//!
//! ```text
//! fibonacci().take(10).collect::<Vec<_>>()    => [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
//! fibonacci().count()                         => 94          (F(0) to F(93))
//! fibonacci().last()                          => Some(12200160415121876738)
//! Countdown::new(3).collect::<Vec<_>>()       => [3, 2, 1]
//! Countdown::new(5).len()                     => 5
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Keep the next two values `(a, b)`; yield `a`, then move to `(b, a + b)`.
//! 2. When `a + b` overflows you still have to yield `b` one more time, so store the "next" value as an
//!    `Option<u64>`.
//! 3. `size_hint` returns `(lower, Some(upper))`; for an exact size both are the remaining count.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `Fibonacci` stores the current value and an optional next value. Each step yields the
//! current value and computes the following one with `checked_add`; `None` marks the end of
//! representable values. `Countdown` simply decrements a counter.
//!
//! **Rust concepts:**
//! - Implementing `Iterator` only requires `type Item` and `next`; dozens of provided methods are
//!   built on top of `next`.
//! - `checked_add` returns `None` on overflow instead of panicking (debug) or wrapping (release).
//! - `ExactSizeIterator` has no required methods but relies on `size_hint` being exact; it enables
//!   `len()` and some optimisations in `collect`.
// @solution-end

/// The Fibonacci numbers that fit in a `u64`.
pub struct Fibonacci {
    // @solution-start remove
    current: Option<u64>,
    next: Option<u64>,
    // @solution-end
}

/// A new Fibonacci iterator starting at 0.
pub fn fibonacci() -> Fibonacci {
    // @solution-start
    Fibonacci {
        current: Some(0),
        next: Some(1),
    }
    // @solution-end
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // @solution-start
        let value = self.current?;
        self.current = self.next;
        self.next = self.next.and_then(|n| n.checked_add(value));
        Some(value)
        // @solution-end
    }
}

/// Counts down from `n` to 1.
pub struct Countdown {
    // @solution-start remove
    remaining: u32,
    // @solution-end
}

impl Countdown {
    pub fn new(n: u32) -> Self {
        // @solution-start
        Countdown { remaining: n }
        // @solution-end
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        // @solution-start
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(self.remaining + 1)
        }
        // @solution-end
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // @solution-start
        let n = self.remaining as usize;
        (n, Some(n))
        // @solution-end
    }
}

impl ExactSizeIterator for Countdown {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_fibonacci_numbers() {
        assert_eq!(
            fibonacci().take(10).collect::<Vec<_>>(),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        );
    }

    #[test]
    fn stops_before_overflow() {
        assert_eq!(fibonacci().count(), 94);
        assert_eq!(fibonacci().last(), Some(12_200_160_415_121_876_738));
        let mut f = fibonacci().skip(93);
        assert_eq!(f.next(), Some(12_200_160_415_121_876_738));
        assert_eq!(f.next(), None);
        assert_eq!(f.next(), None);
    }

    #[test]
    fn adapters_come_for_free() {
        let even: Vec<u64> = fibonacci().filter(|x| x % 2 == 0).take(5).collect();
        assert_eq!(even, vec![0, 2, 8, 34, 144]);
        let below_100: u64 = fibonacci().take_while(|&x| x < 100).sum();
        assert_eq!(below_100, 232);
    }

    #[test]
    fn countdowns() {
        assert_eq!(Countdown::new(3).collect::<Vec<_>>(), vec![3, 2, 1]);
        assert_eq!(Countdown::new(0).next(), None);
        let mut c = Countdown::new(5);
        assert_eq!(c.len(), 5);
        c.next();
        assert_eq!(c.len(), 4);
        let pairs: Vec<(u32, u64)> = Countdown::new(3).zip(fibonacci()).collect();
        assert_eq!(pairs, vec![(3, 0), (2, 1), (1, 1)]);
    }
}
