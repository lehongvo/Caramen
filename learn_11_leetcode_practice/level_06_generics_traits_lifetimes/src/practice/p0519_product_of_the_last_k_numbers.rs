// ==================================================================
// PRACTICE 0519: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0519_product_of_the_last_k_numbers.md
// Run the tests:          cargo test practice::p0519
// ==================================================================

//! # Problem 0519: Product of the Last K Numbers
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct, prefix products, handling zeros by resetting, `i64` intermediates, `Default`
//! - **Source:** [LeetCode 1352. Product of the Last K Numbers](https://leetcode.com/problems/product-of-the-last-k-numbers/)
//!
//! ## Problem
//!
//! Numbers arrive one at a time. At any moment you may be asked for the product of the **last `k`**
//! numbers received.
//!
//! Implement `ProductOfNumbers`:
//!
//! - `ProductOfNumbers::new() -> Self`: no numbers yet.
//! - `add(&mut self, num: i32)`: appends `num` to the stream.
//! - `get_product(&self, k: i32) -> i32`: the product of the last `k` numbers. The stream always
//!   holds at least `k` numbers when this is called.
//!
//! Also implement `Default`. This is a LeetCode *design* problem, so there is no `impl Solution`:
//! paste the struct and its `impl ProductOfNumbers` block into LeetCode.
//!
//! ## Examples
//!
//! ```text
//! add(3), add(0), add(2), add(5), add(4)      stream = [3, 0, 2, 5, 4]
//! get_product(2) => 20     (5 * 4)
//! get_product(3) => 40     (2 * 5 * 4)
//! get_product(4) => 0      (0 * 2 * 5 * 4)
//! add(8)                                      stream = [3, 0, 2, 5, 4, 8]
//! get_product(2) => 32     (4 * 8)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= num <= 100`
//! - `1 <= k <= 4 * 10^4`
//! - At most `4 * 10^4` calls to `add` and `get_product` in total.
//! - The product of any contiguous run of numbers fits in an `i32`.
//!
//! ## Hints
//!
//! 1. With prefix products `p[i] = a[0] * ... * a[i - 1]`, the product of the last `k` numbers is
//!    `p[n] / p[n - k]`, as long as no zero is involved.
//! 2. A zero makes every product that contains it equal to `0`. When a zero arrives, forget
//!    everything before it.

/// Answers "product of the last k numbers" queries.
pub struct ProductOfNumbers {}

impl ProductOfNumbers {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn add(&mut self, num: i32) {
        todo!("Write your solution here")
    }

    pub fn get_product(&self, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

impl Default for ProductOfNumbers {
    fn default() -> Self {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut p = ProductOfNumbers::new();
        for x in [3, 0, 2, 5, 4] {
            p.add(x);
        }
        assert_eq!(p.get_product(2), 20);
        assert_eq!(p.get_product(3), 40);
        assert_eq!(p.get_product(4), 0);
        p.add(8);
        assert_eq!(p.get_product(2), 32);
    }

    #[test]
    fn no_zeros() {
        let mut p = ProductOfNumbers::default();
        for x in [2, 3, 4] {
            p.add(x);
        }
        assert_eq!(p.get_product(1), 4);
        assert_eq!(p.get_product(3), 24);
    }

    #[test]
    fn zero_as_last_number() {
        let mut p = ProductOfNumbers::new();
        p.add(7);
        p.add(0);
        assert_eq!(p.get_product(1), 0);
        assert_eq!(p.get_product(2), 0);
        p.add(9);
        assert_eq!(p.get_product(1), 9);
        assert_eq!(p.get_product(2), 0);
    }

    #[test]
    fn many_ones_and_twos() {
        let mut p = ProductOfNumbers::new();
        let mut stream = Vec::new();
        for i in 0..40_000 {
            // Mostly ones, with a two every 1000 numbers and a zero every 9000, so products stay small.
            let x = if i % 9000 == 8999 {
                0
            } else if i % 1000 == 0 {
                2
            } else {
                1
            };
            p.add(x);
            stream.push(x);
        }
        for k in [1, 10, 1_000, 5_000, 8_999, 9_000, 39_000] {
            let expected: i64 = stream[stream.len() - k..]
                .iter()
                .map(|&x| i64::from(x))
                .product();
            assert_eq!(i64::from(p.get_product(k as i32)), expected, "k = {k}");
        }
    }
}
