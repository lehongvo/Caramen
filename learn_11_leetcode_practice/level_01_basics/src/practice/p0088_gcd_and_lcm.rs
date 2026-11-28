// ==================================================================
// PRACTICE 0088: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0088_gcd_and_lcm.md
// Run the tests:          cargo test practice::p0088
// ==================================================================

//! # Problem 0088: GCD and LCM
//!
//! - **Difficulty:** Easy
//! - **Topics:** Euclid's algorithm with `while`, tuple assignment `(a, b) = (b, a % b)`, `checked_mul`, `Option`
//! - **Source:** Original · [`u64::checked_mul`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_mul)
//!
//! ## Problem
//!
//! Write two classic number-theory functions on `u64`:
//!
//! 1. `gcd(a: u64, b: u64) -> u64` returns the **greatest common divisor**: the largest number that
//!    divides both `a` and `b`. By convention `gcd(a, 0) == a`, and `gcd(0, 0) == 0`.
//! 2. `lcm(a: u64, b: u64) -> Option<u64>` returns the **least common multiple**: the smallest
//!    positive number divisible by both. If either input is `0`, the result is `Some(0)`. Return
//!    `None` when the result does not fit in a `u64`.
//!
//! ## Examples
//!
//! ```text
//! gcd(12, 18)  -> 6
//! gcd(17, 5)   -> 1          17 and 5 are coprime
//! gcd(0, 9)    -> 9
//!
//! lcm(4, 6)    -> Some(12)
//! lcm(0, 6)    -> Some(0)
//! lcm(u64::MAX, u64::MAX - 1) -> None
//! ```
//!
//! ## Constraints
//!
//! - Any `u64` values.
//!
//! ## Hints
//!
//! 1. Euclid: `gcd(a, b) == gcd(b, a % b)`, and the process stops when the second number is 0.
//! 2. `lcm(a, b) == a / gcd(a, b) * b`. Dividing **before** multiplying keeps intermediate values small.
//! 3. The multiplication can still overflow: use `checked_mul`.

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    todo!("Write your solution here")
}

pub fn lcm(a: u64, b: u64) -> Option<u64> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_examples() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(18, 12), 6);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(0, 9), 9);
        assert_eq!(gcd(9, 0), 9);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn gcd_large_values() {
        assert_eq!(gcd(u64::MAX, u64::MAX), u64::MAX);
        assert_eq!(gcd(1 << 63, 1 << 40), 1 << 40);
        // Consecutive Fibonacci numbers are coprime (and are Euclid's worst case).
        assert_eq!(
            gcd(12_200_160_415_121_876_738, 7_540_113_804_746_346_429),
            1
        );
    }

    #[test]
    fn lcm_examples() {
        assert_eq!(lcm(4, 6), Some(12));
        assert_eq!(lcm(7, 7), Some(7));
        assert_eq!(lcm(0, 6), Some(0));
        assert_eq!(lcm(6, 0), Some(0));
        assert_eq!(lcm(1, u64::MAX), Some(u64::MAX));
    }

    #[test]
    fn lcm_overflow() {
        assert_eq!(lcm(u64::MAX, u64::MAX - 1), None);
        assert_eq!(lcm(1 << 32, 3 << 32), Some(3 << 32));
        assert_eq!(lcm(1 << 33, (1 << 33) - 1), None);
    }

    #[test]
    fn lcm_of_one_to_twenty() {
        let mut acc = 1;
        for k in 1..=20 {
            acc = lcm(acc, k).expect("fits in u64");
        }
        assert_eq!(acc, 232_792_560);
    }
}
