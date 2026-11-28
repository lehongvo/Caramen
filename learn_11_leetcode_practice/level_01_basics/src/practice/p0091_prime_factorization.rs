// ==================================================================
// PRACTICE 0091: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0091_prime_factorization.md
// Run the tests:          cargo test practice::p0091
// ==================================================================

//! # Problem 0091: Prime Factorization
//!
//! - **Difficulty:** Easy
//! - **Topics:** trial division, `while` with an overflow-safe condition, `u64`, building a `Vec`
//! - **Source:** Original · [The Rust Book 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html#conditional-loops-with-while)
//!
//! ## Problem
//!
//! A **prime** is an integer greater than 1 whose only positive divisors are 1 and itself. Every
//! integer `n >= 2` can be written as a product of primes in exactly one way (up to order). Write:
//!
//! 1. `is_prime(n: u64) -> bool`, which returns whether `n` is prime (`0` and `1` are not).
//! 2. `prime_factors(n: u64) -> Vec<u64>`, which returns the prime factors of `n` in increasing order,
//!    each repeated as many times as it divides `n`. For `n = 0` and `n = 1`, return an empty vector.
//!
//! ## Examples
//!
//! ```text
//! is_prime(2)            -> true
//! is_prime(91)           -> false     91 = 7 * 13
//! is_prime(1_000_000_007) -> true
//!
//! prime_factors(12)            -> [2, 2, 3]
//! prime_factors(97)            -> [97]
//! prime_factors(600851475143)  -> [71, 839, 1471, 6857]
//! prime_factors(1)             -> []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 10^12`
//!
//! ## Hints
//!
//! 1. If `n` has a divisor other than 1 and itself, it has one that is at most `sqrt(n)`.
//! 2. To factor, divide out `d = 2, 3, 4, ...` as many times as possible while `d * d <= n`. Whatever
//!    is left above 1 at the end is itself a prime.
//! 3. `d * d <= n` can overflow for huge `n`; the equivalent test `d <= n / d` cannot.

pub fn is_prime(n: u64) -> bool {
    todo!("Write your solution here")
}

pub fn prime_factors(n: u64) -> Vec<u64> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_primes() {
        let primes: Vec<u64> = (0..50).filter(|&n| is_prime(n)).collect();
        assert_eq!(
            primes,
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
        );
    }

    #[test]
    fn large_primes_and_composites() {
        assert!(!is_prime(91));
        assert!(is_prime(1_000_000_007));
        assert!(is_prime(999_999_999_989));
        assert!(!is_prime(999_999_999_999));
    }

    #[test]
    fn factor_examples() {
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
        assert_eq!(prime_factors(97), vec![97]);
        assert_eq!(prime_factors(600_851_475_143), vec![71, 839, 1471, 6857]);
    }

    #[test]
    fn zero_and_one_have_no_factors() {
        assert_eq!(prime_factors(0), Vec::<u64>::new());
        assert_eq!(prime_factors(1), Vec::<u64>::new());
    }

    #[test]
    fn powers_and_large_inputs() {
        assert_eq!(prime_factors(1 << 40), vec![2; 40]);
        let mut expected = vec![2; 12];
        expected.extend([5; 12]);
        assert_eq!(prime_factors(1_000_000_000_000), expected);
        assert_eq!(
            prime_factors(999_999_999_999),
            vec![3, 3, 3, 7, 11, 13, 37, 101, 9901]
        );
        assert_eq!(prime_factors(999_999_999_989), vec![999_999_999_989]);
    }

    #[test]
    fn product_of_factors_is_the_number() {
        for n in 2..2000u64 {
            let factors = prime_factors(n);
            assert_eq!(factors.iter().product::<u64>(), n);
            assert!(factors.iter().all(|&p| is_prime(p)));
        }
    }
}
