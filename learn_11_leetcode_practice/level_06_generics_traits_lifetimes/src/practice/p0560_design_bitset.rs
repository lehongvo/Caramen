// ==================================================================
// PRACTICE 0560: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0560_design_bitset.md
// Run the tests:          cargo test practice::p0560
// ==================================================================

//! # Problem 0560: Design Bitset
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct, lazy flip with an XOR flag, cached counts, an inherent `to_string` vs the `Display` trait
//! - **Source:** [LeetCode 2166. Design Bitset](https://leetcode.com/problems/design-bitset/)
//!
//! ## Problem
//!
//! A bitset of `size` bits, all initially `0`, supports:
//!
//! - `Bitset::new(size: i32) -> Self`
//! - `fix(&mut self, idx: i32)`: sets bit `idx` to `1` (no change if it already is).
//! - `unfix(&mut self, idx: i32)`: sets bit `idx` to `0`.
//! - `flip(&mut self)`: inverts **every** bit.
//! - `all(&self) -> bool`: whether every bit is `1`.
//! - `one(&self) -> bool`: whether at least one bit is `1`.
//! - `count(&self) -> i32`: the number of `1` bits.
//! - `to_string(&self) -> String`: the bits as `'0'`/`'1'` characters, bit `0` first.
//!
//! Every operation except `to_string` must run in `O(1)` time, including `flip`.
//!
//! This is a LeetCode *design* problem, so there is no `impl Solution`: paste the struct and its
//! `impl Bitset` block into LeetCode.
//!
//! ## Examples
//!
//! ```text
//! let mut b = Bitset::new(5);   // 00000
//! b.fix(3);                     // 00010
//! b.fix(1);                     // 01010
//! b.flip();                     // 10101
//! b.all()     => false
//! b.unfix(0);                   // 00101
//! b.flip();                     // 11010
//! b.one()     => true
//! b.unfix(0);                   // 01010
//! b.count()   => 2
//! b.to_string() => "01010"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= size <= 10^5`, `0 <= idx < size`
//! - At most `10^5` calls in total, at most `5` of them to `to_string`.
//!
//! ## Hints
//!
//! 1. Do not touch every bit on `flip`. Keep a flag `flipped`; the real value of bit `i` is
//!    `stored[i] ^ flipped`.
//! 2. Keep the number of ones up to date: after a flip it becomes `size - ones`.

/// A fixed-size bitset with O(1) flip.
pub struct Bitset {}

impl Bitset {
    pub fn new(size: i32) -> Self {
        todo!("Write your solution here")
    }

    pub fn fix(&mut self, idx: i32) {
        todo!("Write your solution here")
    }

    pub fn unfix(&mut self, idx: i32) {
        todo!("Write your solution here")
    }

    pub fn flip(&mut self) {
        todo!("Write your solution here")
    }

    pub fn all(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn one(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn count(&self) -> i32 {
        todo!("Write your solution here")
    }

    // LeetCode's API requires an inherent `to_string`; normally you would implement `Display`.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut b = Bitset::new(5);
        b.fix(3);
        b.fix(1);
        b.flip();
        assert!(!b.all());
        b.unfix(0);
        b.flip();
        assert!(b.one());
        b.unfix(0);
        assert_eq!(b.count(), 2);
        assert_eq!(b.to_string(), "01010");
    }

    #[test]
    fn fix_and_unfix_are_idempotent() {
        let mut b = Bitset::new(3);
        b.fix(1);
        b.fix(1);
        assert_eq!(b.count(), 1);
        b.unfix(2);
        b.unfix(1);
        b.unfix(1);
        assert_eq!(b.count(), 0);
        assert!(!b.one());
        assert_eq!(b.to_string(), "000");
    }

    #[test]
    fn flip_twice_and_all() {
        let mut b = Bitset::new(4);
        b.flip();
        assert!(b.all());
        assert_eq!(b.to_string(), "1111");
        b.unfix(2);
        assert!(!b.all());
        b.flip();
        b.flip();
        assert_eq!(b.to_string(), "1101");
        let mut one = Bitset::new(1);
        one.fix(0);
        assert!(one.all() && one.one());
    }

    #[test]
    fn matches_a_vec_model() {
        let n = 200;
        let mut b = Bitset::new(n as i32);
        let mut model = vec![false; n];
        for step in 0..20_000_usize {
            let idx = step * 7 % n;
            match step % 5 {
                0 | 1 => {
                    b.fix(idx as i32);
                    model[idx] = true;
                }
                2 | 3 => {
                    b.unfix(idx as i32);
                    model[idx] = false;
                }
                _ => {
                    b.flip();
                    model.iter_mut().for_each(|x| *x = !*x);
                }
            }
            assert_eq!(b.count() as usize, model.iter().filter(|&&x| x).count());
        }
        let expected: String = model.iter().map(|&x| if x { '1' } else { '0' }).collect();
        assert_eq!(b.to_string(), expected);
    }
}
