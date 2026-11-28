// ==================================================================
// PRACTICE 0919: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0919_const_fn_lookup_tables.md
// Run the tests:          cargo test practice::p0919
// ==================================================================

//! # Problem 0919: Compile-Time Tables with `const fn`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `const fn`, `while` loops in const context, `const` items computed at compile time, const generic return arrays, CRC tables
//! - **Source:** Original · [Reference: Constant evaluation](https://doc.rust-lang.org/reference/const_eval.html) · [Reference: Const functions](https://doc.rust-lang.org/reference/const_eval.html#const-functions)
//!
//! ## Problem
//!
//! A `const fn` can run at **compile time**: use it to compute lookup tables once, while compiling,
//! instead of at every program start. In a `const fn` you cannot use `for` loops or iterator
//! adapters (they call trait methods), but `while`, `if`, `match`, arithmetic and array indexing all
//! work.
//!
//! Implement these `const fn`s:
//!
//! - `gcd(a: u64, b: u64) -> u64` (Euclid; `gcd(0, 0) == 0`).
//! - `is_prime(n: u32) -> bool` (trial division; `0` and `1` are not prime).
//! - `first_primes<const N: usize>() -> [u32; N]`: the first `N` primes in increasing order.
//! - `crc8_table() -> [u8; 256]`: the table for CRC-8 with polynomial `0x07`. Entry `i` is obtained
//!   by starting from `crc = i` and repeating 8 times: if the top bit is set,
//!   `crc = (crc << 1) ^ 0x07`, else `crc = crc << 1` (all in `u8`, so the shifted-out bit is lost).
//!
//! And two ordinary functions that **must** use tables computed at compile time, declared inside
//! their bodies as `const` items:
//!
//! - `nth_prime(n: usize) -> Option<u32>`: the `n`-th prime (0-based) among the first 100 primes,
//!   `None` for `n >= 100`. Use `const PRIMES: [u32; 100] = first_primes::<100>();`.
//! - `crc8(data: &[u8]) -> u8`: start with `crc = 0`; for each byte, `crc = TABLE[(crc ^ byte) as
//!   usize]`, where `const TABLE: [u8; 256] = crc8_table();`.
//!
//! ## Examples
//!
//! ```text
//! gcd(48, 18)                => 6
//! is_prime(97)               => true
//! first_primes::<5>()        => [2, 3, 5, 7, 11]
//! crc8_table()[1]            => 0x07
//! crc8(b"123456789")         => 0xF4     (the standard check value of CRC-8)
//! nth_prime(99)              => Some(541)
//! ```
//!
//! ## Constraints
//!
//! - `first_primes` is called with `N <= 1000`.
//!
//! ## Hints
//!
//! 1. Replace `for i in 0..n` by `let mut i = 0; while i < n { ...; i += 1; }`.
//! 2. Start with `let mut table = [0u8; 256];` and fill it in place.
//! 3. `u8::wrapping_shl` is not needed: `crc << 1` on a `u8` already drops the top bit (shifting by
//!    less than the bit width never overflows).

pub const fn gcd(a: u64, b: u64) -> u64 {
    todo!()
}

pub const fn is_prime(n: u32) -> bool {
    todo!()
}

pub const fn first_primes<const N: usize>() -> [u32; N] {
    todo!()
}

pub const fn crc8_table() -> [u8; 256] {
    todo!()
}

pub fn nth_prime(n: usize) -> Option<u32> {
    todo!("Write your solution here")
}

pub fn crc8(data: &[u8]) -> u8 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_values() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(0, 9), 9);
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(u64::MAX, u64::MAX - 1), 1);
    }

    #[test]
    fn primes() {
        assert!(is_prime(97));
        assert!(!is_prime(1) && !is_prime(0) && !is_prime(91));
        assert!(is_prime(2_147_483_647));
        assert!(is_prime(4_294_967_291));
        assert!(!is_prime(u32::MAX));
        assert_eq!(first_primes::<5>(), [2, 3, 5, 7, 11]);
        assert_eq!(first_primes::<0>(), []);
        assert_eq!(first_primes::<1000>()[999], 7919);
    }

    #[test]
    fn nth_prime_uses_the_table() {
        assert_eq!(nth_prime(0), Some(2));
        assert_eq!(nth_prime(24), Some(97));
        assert_eq!(nth_prime(99), Some(541));
        assert_eq!(nth_prime(100), None);
    }

    #[test]
    fn crc8_table_entries() {
        let table = crc8_table();
        assert_eq!(table[0], 0x00);
        assert_eq!(table[1], 0x07);
        assert_eq!(table[2], 0x0E);
        assert_eq!(table[0x80], 0x89);
        assert_eq!(table[255], 0xF3);
    }

    #[test]
    fn crc8_check_values() {
        assert_eq!(crc8(b"123456789"), 0xF4);
        assert_eq!(crc8(b""), 0x00);
        assert_eq!(crc8(&[0x01]), 0x07);
        let big = vec![0xAB_u8; 100_000];
        assert_eq!(crc8(&big), crc8(&big));
        assert_ne!(crc8(b"hello"), crc8(b"hellp"));
    }
}
