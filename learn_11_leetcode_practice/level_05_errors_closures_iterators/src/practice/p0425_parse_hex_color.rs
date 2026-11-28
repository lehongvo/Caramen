// ==================================================================
// PRACTICE 0425: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0425_parse_hex_color.md
// Run the tests:          cargo test practice::p0425
// ==================================================================

//! # Problem 0425: Parse a Hex Color
//!
//! - **Difficulty:** Easy
//! - **Topics:** `u8::from_str_radix`, `strip_prefix`, `chunks`, custom error enum, `impl Display`, `?`
//! - **Source:** Original · [std `u8::from_str_radix`](https://doc.rust-lang.org/std/primitive.u8.html#method.from_str_radix)
//!
//! ## Problem
//!
//! CSS colors are often written as `#RRGGBB` (two hex digits per channel, e.g. `#ff8800`) or in the
//! short form `#RGB` where each digit is doubled (`#f80` means `#ff8800`).
//!
//! Implement `parse_hex_color(s: &str) -> Result<Rgb, ColorError>` (no trimming):
//!
//! 1. `s` must start with `'#'`, otherwise `MissingHash`.
//! 2. The rest must have exactly 3 or 6 **characters**, otherwise `WrongLength(n)` with `n` the
//!    number of characters after `'#'`.
//! 3. Split the rest into the three channels (two characters each, or one doubled character in the
//!    short form). A channel containing anything other than `0-9`, `a-f`, `A-F` gives
//!    `InvalidHex(channel)` where `channel` is the two-character text (`"zz"`, or `"gg"` for the
//!    short-form digit `g`). Report the first bad channel (red, then green, then blue).
//!
//! Also implement:
//!
//! - `Display` for `ColorError`:
//!   `MissingHash` -> ``"a color must start with `#`"``,
//!   `WrongLength(n)` -> `"expected 3 or 6 hex digits, found N"`,
//!   `InvalidHex(c)` -> ``"`C` is not a valid hex byte"``.
//! - `to_hex(color: Rgb) -> String`: the long lower-case form, e.g. `"#ff8800"`.
//! - `blend(a: &str, b: &str) -> Result<String, ColorError>`: parse both colors (first `a`) and
//!   return the channel-wise average, rounded down, in the long form.
//!
//! ## Examples
//!
//! ```text
//! parse_hex_color("#ff8800") -> Ok(Rgb { r: 255, g: 136, b: 0 })
//! parse_hex_color("#F80")    -> Ok(Rgb { r: 255, g: 136, b: 0 })
//! parse_hex_color("ff8800")  -> Err(MissingHash)
//! parse_hex_color("#ff88")   -> Err(WrongLength(4))
//! parse_hex_color("#ff88zz") -> Err(InvalidHex("zz"))
//! parse_hex_color("#+f0000") -> Err(InvalidHex("+f"))
//! to_hex(Rgb { r: 1, g: 2, b: 255 }) -> "#0102ff"
//! blend("#000000", "#ffffff")        -> Ok("#7f7f7f")
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `s.strip_prefix('#').ok_or(ColorError::MissingHash)?` handles rule 1.
//! 2. Collect the characters into a `Vec<char>` so you can count them and use `chunks(2)`.
//! 3. `u8::from_str_radix("+f", 16)` returns `Ok(15)`! Check `char::is_ascii_hexdigit` yourself first.

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, PartialEq)]
pub enum ColorError {
    MissingHash,
    WrongLength(usize),
    InvalidHex(String),
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Error for ColorError {}

pub fn parse_hex_color(s: &str) -> Result<Rgb, ColorError> {
    todo!("Write your solution here")
}

pub fn to_hex(color: Rgb) -> String {
    todo!("Write your solution here")
}

pub fn blend(a: &str, b: &str) -> Result<String, ColorError> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_and_short_forms() {
        let orange = Rgb {
            r: 255,
            g: 136,
            b: 0,
        };
        assert_eq!(parse_hex_color("#ff8800"), Ok(orange));
        assert_eq!(parse_hex_color("#FF8800"), Ok(orange));
        assert_eq!(parse_hex_color("#F80"), Ok(orange));
        assert_eq!(parse_hex_color("#000"), Ok(Rgb { r: 0, g: 0, b: 0 }));
    }

    #[test]
    fn hash_and_length_errors() {
        assert_eq!(parse_hex_color("ff8800"), Err(ColorError::MissingHash));
        assert_eq!(parse_hex_color(" #ff8800"), Err(ColorError::MissingHash));
        assert_eq!(parse_hex_color("#ff88"), Err(ColorError::WrongLength(4)));
        assert_eq!(parse_hex_color("#"), Err(ColorError::WrongLength(0)));
        assert_eq!(parse_hex_color("#üü"), Err(ColorError::WrongLength(2)));
    }

    #[test]
    fn invalid_channels() {
        assert_eq!(
            parse_hex_color("#ff88zz"),
            Err(ColorError::InvalidHex("zz".to_string()))
        );
        assert_eq!(
            parse_hex_color("#+f0000"),
            Err(ColorError::InvalidHex("+f".to_string()))
        );
        assert_eq!(
            parse_hex_color("#gg0"),
            Err(ColorError::InvalidHex("gg".to_string()))
        );
        assert_eq!(
            parse_hex_color("#xyzxyz"),
            Err(ColorError::InvalidHex("xy".to_string()))
        );
    }

    #[test]
    fn display_messages() {
        assert_eq!(
            ColorError::MissingHash.to_string(),
            "a color must start with `#`"
        );
        assert_eq!(
            ColorError::WrongLength(4).to_string(),
            "expected 3 or 6 hex digits, found 4"
        );
        assert_eq!(
            ColorError::InvalidHex("zz".to_string()).to_string(),
            "`zz` is not a valid hex byte"
        );
    }

    #[test]
    fn to_hex_and_blend() {
        assert_eq!(to_hex(Rgb { r: 1, g: 2, b: 255 }), "#0102ff");
        assert_eq!(blend("#000000", "#ffffff"), Ok("#7f7f7f".to_string()));
        assert_eq!(blend("#f00", "#00f"), Ok("#7f007f".to_string()));
        assert_eq!(blend("#f00", "blue"), Err(ColorError::MissingHash));
    }
}
