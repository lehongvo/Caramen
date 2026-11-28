// ==================================================================
// PRACTICE 0204: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0204_tuple_struct_newtypes.md
// Run the tests:          cargo test practice::p0204
// ==================================================================

//! # Problem 0204: Tuple Structs: Colors and Units
//!
//! - **Difficulty:** Easy
//! - **Topics:** tuple structs, `.0` / `.1` field access, destructuring with `let`, newtypes
//! - **Source:** Original · [The Rust Book 5.1](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
//!
//! ## Problem
//!
//! A *tuple struct* has a name but its fields have no names, only positions:
//!
//! ```text
//! pub struct Rgb(pub u8, pub u8, pub u8);   // red, green, blue
//! pub struct Meters(pub f64);
//! pub struct Feet(pub f64);
//! ```
//!
//! A one-field tuple struct such as `Meters` is called a *newtype*: it costs nothing at run time, but
//! the compiler will not let you pass `Feet` where `Meters` is expected.
//!
//! Implement:
//!
//! - `to_hex(color: Rgb) -> String`: the color as `"#RRGGBB"` with two **uppercase** hexadecimal
//!   digits per channel.
//! - `grayscale(color: Rgb) -> Rgb`: a gray color whose three channels all equal the integer average
//!   `(r + g + b) / 3` (rounded down).
//! - `feet_to_meters(length: Feet) -> Meters`: one foot is exactly `0.3048` meters.
//! - `total_meters(parts: &[Meters]) -> Meters`: the sum of all lengths (`Meters(0.0)` if empty).
//!
//! ## Examples
//!
//! ```text
//! to_hex(Rgb(255, 99, 71))     => "#FF6347"
//! to_hex(Rgb(0, 0, 0))         => "#000000"
//! grayscale(Rgb(10, 20, 31))   => Rgb(20, 20, 20)      (61 / 3 = 20)
//! feet_to_meters(Feet(10.0))   => Meters(3.048)
//! total_meters(&[Meters(1.5), Meters(2.0)]) => Meters(3.5)
//! ```
//!
//! ## Constraints
//!
//! - Channel values are `0..=255`; lengths are finite and between `-10^6` and `10^6`.
//!
//! ## Hints
//!
//! 1. Destructure in one line: `let Rgb(r, g, b) = color;`.
//! 2. `format!("{:02X}", value)` prints a byte as two uppercase hex digits.
//! 3. `r + g + b` overflows a `u8`; convert to `u32` first and back with `as u8`.

/// A color with red, green and blue channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// A length in meters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meters(pub f64);

/// A length in feet.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Feet(pub f64);

/// Formats a color as `#RRGGBB`.
pub fn to_hex(color: Rgb) -> String {
    todo!("Write your solution here")
}

/// Returns the gray color with the same average brightness.
pub fn grayscale(color: Rgb) -> Rgb {
    todo!("Write your solution here")
}

/// Converts feet to meters.
pub fn feet_to_meters(length: Feet) -> Meters {
    todo!("Write your solution here")
}

/// Adds up several lengths.
pub fn total_meters(parts: &[Meters]) -> Meters {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn example_tomato_hex() {
        assert_eq!(to_hex(Rgb(255, 99, 71)), "#FF6347");
    }

    #[test]
    fn example_black_hex_keeps_leading_zeros() {
        assert_eq!(to_hex(Rgb(0, 0, 0)), "#000000");
        assert_eq!(to_hex(Rgb(1, 10, 171)), "#010AAB");
    }

    #[test]
    fn example_grayscale_rounds_down() {
        assert_eq!(grayscale(Rgb(10, 20, 31)), Rgb(20, 20, 20));
    }

    #[test]
    fn grayscale_of_white_does_not_overflow() {
        assert_eq!(grayscale(Rgb(255, 255, 255)), Rgb(255, 255, 255));
    }

    #[test]
    fn example_feet_to_meters() {
        let Meters(m) = feet_to_meters(Feet(10.0));
        assert!(close(m, 3.048));
    }

    #[test]
    fn example_total_meters() {
        assert!(close(total_meters(&[Meters(1.5), Meters(2.0)]).0, 3.5));
    }

    #[test]
    fn total_of_empty_slice_is_zero() {
        assert_eq!(total_meters(&[]), Meters(0.0));
    }

    #[test]
    fn convert_then_sum() {
        let parts = [
            feet_to_meters(Feet(1.0)),
            feet_to_meters(Feet(2.0)),
            Meters(-0.9144),
        ];
        assert!(close(total_meters(&parts).0, 0.0));
    }
}
