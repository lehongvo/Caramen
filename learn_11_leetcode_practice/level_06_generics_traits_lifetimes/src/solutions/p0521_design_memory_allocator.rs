//! # Problem 0521: Design Memory Allocator
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct, `Vec<i32>` as memory, scanning for runs, `iter_mut`, `retain`-style counting
//! - **Source:** [LeetCode 2502. Design Memory Allocator](https://leetcode.com/problems/design-memory-allocator/)
//!
//! ## Problem
//!
//! Simulate a tiny memory of `n` units, indexed `0..n`. Every unit is either free or owned by a block
//! with some id `m_id` (several blocks may share the same id).
//!
//! Implement `Allocator`:
//!
//! - `Allocator::new(n: i32) -> Self`: all `n` units start free.
//! - `allocate(&mut self, size: i32, m_id: i32) -> i32`: finds the **leftmost** run of `size`
//!   consecutive free units, marks them as owned by `m_id`, and returns the index of the first unit
//!   of the run. Returns `-1` (and changes nothing) if no such run exists.
//! - `free_memory(&mut self, m_id: i32) -> i32`: frees **every** unit owned by `m_id` and returns how
//!   many units were freed (possibly `0`).
//!
//! This is a LeetCode *design* problem, so there is no `impl Solution`: paste the struct and its
//! `impl Allocator` block into LeetCode.
//!
//! ## Examples
//!
//! ```text
//! let mut a = Allocator::new(10);
//! a.allocate(1, 1)   => 0      memory: 1 . . . . . . . . .
//! a.allocate(1, 2)   => 1      memory: 1 2 . . . . . . . .
//! a.allocate(1, 3)   => 2      memory: 1 2 3 . . . . . . .
//! a.free_memory(2)   => 1      memory: 1 . 3 . . . . . . .
//! a.allocate(3, 4)   => 3      memory: 1 . 3 4 4 4 . . . .   (the hole at 1 is too small)
//! a.allocate(1, 1)   => 1      memory: 1 1 3 4 4 4 . . . .
//! a.allocate(1, 1)   => 6      memory: 1 1 3 4 4 4 1 . . .
//! a.free_memory(1)   => 3      memory: . . 3 4 4 4 . . . .
//! a.allocate(10, 2)  => -1
//! a.free_memory(7)   => 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n, size, m_id <= 1000`
//! - At most `1000` calls to `allocate` and `free_memory` in total.
//!
//! ## Hints
//!
//! 1. Store one `i32` per unit, using `0` for "free" (ids are always positive).
//! 2. To allocate, scan left to right while counting the current run of free units; as soon as it
//!    reaches `size`, you know where the run started.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** memory is a `Vec<i32>` where `0` means free. `allocate` scans once, tracking the length
//! of the current free run; when the run reaches `size`, it fills that run with `m_id`. `free_memory`
//! resets every unit equal to `m_id` and counts them.
//!
//! **Complexity:** `O(n)` per call, `O(n)` space.
//!
//! **Rust concepts:**
//! - `self.memory[start..=i].fill(m_id)` writes a whole sub-slice at once.
//! - `iter_mut().filter(|u| **u == m_id)` yields `&mut i32` references that can be overwritten in
//!   place; `**u` dereferences the `&&mut i32` given to the filter closure.
//! - The struct has no `new()` without arguments, so no `Default` impl is needed here.
// @solution-end

/// A first-fit memory allocator over `n` units.
pub struct Allocator {
    // @solution-start remove
    memory: Vec<i32>,
    // @solution-end
}

impl Allocator {
    pub fn new(n: i32) -> Self {
        // @solution-start
        Allocator {
            memory: vec![0; n as usize],
        }
        // @solution-end
    }

    pub fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        // @solution-start
        let size = size as usize;
        let mut run = 0;
        for i in 0..self.memory.len() {
            if self.memory[i] == 0 {
                run += 1;
                if run == size {
                    let start = i + 1 - size;
                    self.memory[start..=i].fill(m_id);
                    return start as i32;
                }
            } else {
                run = 0;
            }
        }
        -1
        // @solution-end
    }

    pub fn free_memory(&mut self, m_id: i32) -> i32 {
        // @solution-start
        let mut freed = 0;
        for unit in self.memory.iter_mut().filter(|u| **u == m_id) {
            *unit = 0;
            freed += 1;
        }
        freed
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut a = Allocator::new(10);
        assert_eq!(a.allocate(1, 1), 0);
        assert_eq!(a.allocate(1, 2), 1);
        assert_eq!(a.allocate(1, 3), 2);
        assert_eq!(a.free_memory(2), 1);
        assert_eq!(a.allocate(3, 4), 3);
        assert_eq!(a.allocate(1, 1), 1);
        assert_eq!(a.allocate(1, 1), 6);
        assert_eq!(a.free_memory(1), 3);
        assert_eq!(a.allocate(10, 2), -1);
        assert_eq!(a.free_memory(7), 0);
    }

    #[test]
    fn exact_fit_and_full_memory() {
        let mut a = Allocator::new(5);
        assert_eq!(a.allocate(5, 9), 0);
        assert_eq!(a.allocate(1, 1), -1);
        assert_eq!(a.free_memory(9), 5);
        assert_eq!(a.allocate(2, 1), 0);
        assert_eq!(a.allocate(3, 2), 2);
    }

    #[test]
    fn failed_allocation_changes_nothing() {
        let mut a = Allocator::new(4);
        assert_eq!(a.allocate(1, 1), 0);
        assert_eq!(a.allocate(1, 2), 1);
        assert_eq!(a.free_memory(1), 1);
        // Free units: 0, 2, 3. A run of 3 does not exist.
        assert_eq!(a.allocate(3, 5), -1);
        assert_eq!(a.free_memory(5), 0);
        assert_eq!(a.allocate(2, 5), 2);
    }

    #[test]
    fn many_small_blocks() {
        let mut a = Allocator::new(1000);
        for i in 0..1000 {
            assert_eq!(a.allocate(1, i % 10 + 1), i);
        }
        assert_eq!(a.allocate(1, 1), -1);
        assert_eq!(a.free_memory(3), 100);
        // Units 2, 12, 22, ... are free now: only single holes.
        assert_eq!(a.allocate(2, 3), -1);
        assert_eq!(a.allocate(1, 3), 2);
    }
}
