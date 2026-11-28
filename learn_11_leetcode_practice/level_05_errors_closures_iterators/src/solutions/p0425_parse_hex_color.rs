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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** normalise both notations into three two-character channel strings, then convert each
//! channel with `u8::from_str_radix(.., 16)` after verifying it contains only hex digits.
//!
//! **Rust concepts:**
//! - `strip_prefix` returns `Option<&str>`, and `ok_or` turns it into a `Result` for `?`.
//! - Working on a `Vec<char>` avoids byte-index slicing, which could panic on multi-byte characters.
//! - `from_str_radix` accepts an optional sign, so validating the digits first is necessary.
//! - `{:02x}` formats a number in lower-case hexadecimal, padded with zeros to two digits.
//! - Averaging two `u8` values in `u16` avoids overflow (`255 + 255` does not fit in a `u8`).
// @solution-end

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
        // @solution-start
        match self {
            ColorError::MissingHash => write!(f, "a color must start with `#`"),
            ColorError::WrongLength(n) => write!(f, "expected 3 or 6 hex digits, found {n}"),
            ColorError::InvalidHex(c) => write!(f, "`{c}` is not a valid hex byte"),
        }
        // @solution-end
    }
}

impl Error for ColorError {}

// @solution-start remove
fn channel(text: &str) -> Result<u8, ColorError> {
    let invalid = || ColorError::InvalidHex(text.to_string());
    if !text.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(invalid());
    }
    u8::from_str_radix(text, 16).map_err(|_| invalid())
}
// @solution-end

pub fn parse_hex_color(s: &str) -> Result<Rgb, ColorError> {
    // @solution-start
    let digits: Vec<char> = s
        .strip_prefix('#')
        .ok_or(ColorError::MissingHash)?
        .chars()
        .collect();
    let channels: Vec<String> = match digits.len() {
        6 => digits.chunks(2).map(|pair| pair.iter().collect()).collect(),
        3 => digits.iter().map(|&c| [c, c].iter().collect()).collect(),
        n => return Err(ColorError::WrongLength(n)),
    };
    Ok(Rgb {
        r: channel(&channels[0])?,
        g: channel(&channels[1])?,
        b: channel(&channels[2])?,
    })
    // @solution-end
}

pub fn to_hex(color: Rgb) -> String {
    // @solution-start
    format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)
    // @solution-end
}

pub fn blend(a: &str, b: &str) -> Result<String, ColorError> {
    // @solution-start
    let a = parse_hex_color(a)?;
    let b = parse_hex_color(b)?;
    // The average of two u8 values always fits in a u8.
    let avg = |x: u8, y: u8| ((u16::from(x) + u16::from(y)) / 2) as u8;
    Ok(to_hex(Rgb {
        r: avg(a.r, b.r),
        g: avg(a.g, b.g),
        b: avg(a.b, b.b),
    }))
    // @solution-end
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
