// ==================================================================
// PRACTICE 0027: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0027_wrapping_fnv_hash.md
// Run the tests:          cargo test practice::p0027
// ==================================================================

//! # Problem 0027: Wrapping FNV-1a Hash
//!
//! - **Difficulty:** Easy
//! - **Topics:** `wrapping_mul`, `u32`, XOR `^`, `u8` to `u32` conversion, byte string literals
//! - **Source:** Original · [`u32::wrapping_mul`](https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_mul)
//!
//! ## Problem
//!
//! Hash functions **want** arithmetic that wraps around: they mix bits, and the overflowing high
//! bits are thrown away on purpose. Implement the 32-bit FNV-1a hash of a byte slice:
//!
//! ```text
//! hash = 2166136261                     (the "offset basis", 0x811C9DC5)
//! for each byte b in the input:
//!     hash = hash XOR b
//!     hash = hash * 16777619 (mod 2^32) (the "FNV prime", 0x01000193)
//! return hash
//! ```
//!
//! Write `fnv1a_32(bytes: &[u8]) -> u32`.
//!
//! ## Examples
//!
//! ```text
//! fnv1a_32(b"")       -> 0x811C9DC5
//! fnv1a_32(b"a")      -> 0xE40C292C
//! fnv1a_32(b"foobar") -> 0xBF9CF968
//! ```
//!
//! `b"foobar"` is a **byte string literal**: a `&[u8; 6]` holding the ASCII codes of the letters.
//!
//! ## Constraints
//!
//! - `0 <= bytes.len() <= 10^6`
//!
//! ## Hints
//!
//! 1. A plain `hash * 16777619` panics with "attempt to multiply with overflow" in tests.
//! 2. `u32::wrapping_mul` computes the product modulo `2^32`.
//! 3. XOR needs both sides to have the same type: convert the byte with `u32::from(b)`.

pub fn fnv1a_32(bytes: &[u8]) -> u32 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_is_the_offset_basis() {
        assert_eq!(fnv1a_32(b""), 0x811C_9DC5);
    }

    #[test]
    fn single_letter() {
        assert_eq!(fnv1a_32(b"a"), 0xE40C_292C);
    }

    #[test]
    fn known_vectors() {
        assert_eq!(fnv1a_32(b"foobar"), 0xBF9C_F968);
        assert_eq!(fnv1a_32(b"hello"), 0x4F9F_2CAB);
    }

    #[test]
    fn order_matters() {
        assert_ne!(fnv1a_32(b"ab"), fnv1a_32(b"ba"));
    }

    #[test]
    fn long_input_does_not_panic() {
        let data = vec![0xFFu8; 1_000_000];
        let h = fnv1a_32(&data);
        assert_eq!(h, fnv1a_32(&data));
        assert_ne!(h, fnv1a_32(&data[..999_999]));
    }
}
