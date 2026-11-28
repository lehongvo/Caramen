// ==================================================================
// PRACTICE 0024: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0024_integer_casts_with_as.md
// Run the tests:          cargo test practice::p0024
// ==================================================================

//! # Problem 0024: Integer Casts with `as`
//!
//! - **Difficulty:** Easy
//! - **Topics:** `as` casts between integer types, truncation, two's complement, bit shifts `>>` and `<<`
//! - **Source:** Original · [Rust Reference: type cast expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions)
//!
//! ## Problem
//!
//! The `as` operator converts between integer types and **never fails**, even when the value does
//! not fit. This problem makes you predict exactly what it does. Write three functions:
//!
//! 1. `cast_report(x: i32) -> (u8, i8, u16, i64)` returns `(x as u8, x as i8, x as u16, x as i64)`.
//! 2. `split_u16(x: u16) -> (u8, u8)` returns the `(high, low)` bytes of `x`: for `0xABCD` that is
//!    `(0xAB, 0xCD)`.
//! 3. `join_u16(high: u8, low: u8) -> u16` does the opposite: `join_u16(0xAB, 0xCD) == 0xABCD`.
//!
//! ## Examples
//!
//! ```text
//! cast_report(300)  -> (44, 44, 300, 300)       300 = 0x012C, the low byte is 0x2C = 44
//! cast_report(200)  -> (200, -56, 200, 200)     the byte 0xC8 read as signed is 200 - 256 = -56
//! cast_report(-1)   -> (255, -1, 65535, -1)     -1 is all ones in two's complement
//!
//! split_u16(0xABCD)       -> (0xAB, 0xCD)
//! join_u16(0x12, 0x34)    -> 0x1234
//! ```
//!
//! ## Constraints
//!
//! - Any value of the parameter types.
//!
//! ## Hints
//!
//! 1. Casting to a **smaller** integer type keeps only the lowest bits; casting between signed and
//!    unsigned types of the **same** size keeps the bits and reinterprets them.
//! 2. Casting to a **larger** signed type sign-extends, so the numeric value is preserved.
//! 3. `x >> 8` moves the high byte into the low position. `u16::from(high) << 8` moves it back.

pub fn cast_report(x: i32) -> (u8, i8, u16, i64) {
    todo!("Write your solution here")
}

pub fn split_u16(x: u16) -> (u8, u8) {
    todo!("Write your solution here")
}

pub fn join_u16(high: u8, low: u8) -> u16 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cast_examples() {
        assert_eq!(cast_report(300), (44, 44, 300, 300));
        assert_eq!(cast_report(200), (200, -56, 200, 200));
        assert_eq!(cast_report(-1), (255, -1, 65_535, -1));
    }

    #[test]
    fn cast_extremes() {
        assert_eq!(cast_report(0), (0, 0, 0, 0));
        assert_eq!(cast_report(128), (128, -128, 128, 128));
        assert_eq!(cast_report(i32::MAX), (255, -1, 65_535, 2_147_483_647));
        assert_eq!(cast_report(i32::MIN), (0, 0, 0, -2_147_483_648));
        assert_eq!(cast_report(-300), (212, -44, 65_236, -300));
    }

    #[test]
    fn split_examples() {
        assert_eq!(split_u16(0xABCD), (0xAB, 0xCD));
        assert_eq!(split_u16(0x00FF), (0x00, 0xFF));
        assert_eq!(split_u16(0xFF00), (0xFF, 0x00));
    }

    #[test]
    fn join_examples() {
        assert_eq!(join_u16(0x12, 0x34), 0x1234);
        assert_eq!(join_u16(0xFF, 0xFF), u16::MAX);
        assert_eq!(join_u16(0, 0), 0);
    }

    #[test]
    fn split_then_join_round_trips() {
        for x in (0..=u16::MAX).step_by(97) {
            let (high, low) = split_u16(x);
            assert_eq!(join_u16(high, low), x);
        }
    }
}
