// ==================================================================
// PRACTICE 0533: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0533_display_for_types.md
// Run the tests:          cargo test practice::p0533
// ==================================================================

//! # Problem 0533: Implementing `Display`
//!
//! - **Difficulty:** Easy
//! - **Topics:** `std::fmt::Display`, `fmt::Formatter`, `write!`, format specs (`{:02}`, `{:02X}`), `to_string` for free
//! - **Source:** Original · [std::fmt::Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)
//!
//! ## Problem
//!
//! `Display` controls how a value looks with `{}` and what `to_string()` returns. Implement it for:
//!
//! - `Rgb { r: u8, g: u8, b: u8 }`: `"#RRGGBB"` with two **uppercase** hex digits per channel, e.g.
//!   `"#FF8000"`.
//! - `Clock { hours: u8, minutes: u8 }`: `"HH:MM"` with two digits each, e.g. `"09:05"`.
//! - `Card { rank: u8, suit: Suit }` where `Suit` is `Spades`, `Hearts`, `Diamonds` or `Clubs`. Implement
//!   `Display` for `Suit` as `"S"`, `"H"`, `"D"`, `"C"`, and for `Card` as rank then suit. The rank is
//!   `"A"` for `1`, `"J"`, `"Q"`, `"K"` for `11`, `12`, `13`, and the number itself for `2..=10`
//!   (e.g. `"10H"`, `"QS"`, `"AD"`).
//! - `Hand(Vec<Card>)`: `"[AS, 10H, QD]"`, the cards separated by `", "` inside square brackets
//!   (`"[]"` when empty).
//!
//! Every `impl` must use the `Display` impl of the inner parts (for example, `Hand` formats each card
//! with `{}`), not duplicate their logic.
//!
//! ## Examples
//!
//! ```text
//! Rgb { r: 255, g: 128, b: 0 }.to_string()     => "#FF8000"
//! format!("{}", Clock { hours: 9, minutes: 5 }) => "09:05"
//! Card { rank: 10, suit: Suit::Hearts }          => "10H"
//! Hand(vec![AS, 10H, QD])                        => "[AS, 10H, QD]"
//! ```
//!
//! ## Constraints
//!
//! - `0 <= hours <= 23`, `0 <= minutes <= 59`, `1 <= rank <= 13`.
//!
//! ## Hints
//!
//! 1. The method to write is `fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result`; use
//!    `write!(f, "...", ...)` inside it.
//! 2. `{:02X}` prints a number in uppercase hexadecimal, padded with zeros to 2 digits.
//! 3. For `Hand`, write `"["`, then each card (with `", "` before every card except the first), then
//!    `"]"`. Propagate errors with `?`.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clock {
    pub hours: u8,
    pub minutes: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hand(pub Vec<Card>);

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors() {
        assert_eq!(
            Rgb {
                r: 255,
                g: 128,
                b: 0
            }
            .to_string(),
            "#FF8000"
        );
        assert_eq!(Rgb { r: 0, g: 0, b: 0 }.to_string(), "#000000");
        assert_eq!(
            format!(
                "color: {}",
                Rgb {
                    r: 10,
                    g: 171,
                    b: 205
                }
            ),
            "color: #0AABCD"
        );
    }

    #[test]
    fn clocks() {
        assert_eq!(
            format!(
                "{}",
                Clock {
                    hours: 9,
                    minutes: 5
                }
            ),
            "09:05"
        );
        assert_eq!(
            Clock {
                hours: 23,
                minutes: 59
            }
            .to_string(),
            "23:59"
        );
        assert_eq!(
            Clock {
                hours: 0,
                minutes: 0
            }
            .to_string(),
            "00:00"
        );
    }

    #[test]
    fn cards_and_suits() {
        assert_eq!(Suit::Clubs.to_string(), "C");
        let cases = [
            (1, Suit::Spades, "AS"),
            (10, Suit::Hearts, "10H"),
            (12, Suit::Spades, "QS"),
            (13, Suit::Diamonds, "KD"),
            (2, Suit::Clubs, "2C"),
        ];
        for (rank, suit, expected) in cases {
            assert_eq!(Card { rank, suit }.to_string(), expected);
        }
    }

    #[test]
    fn hands() {
        let hand = Hand(vec![
            Card {
                rank: 1,
                suit: Suit::Spades,
            },
            Card {
                rank: 10,
                suit: Suit::Hearts,
            },
            Card {
                rank: 12,
                suit: Suit::Diamonds,
            },
        ]);
        assert_eq!(hand.to_string(), "[AS, 10H, QD]");
        assert_eq!(Hand(vec![]).to_string(), "[]");
        assert_eq!(
            Hand(vec![Card {
                rank: 11,
                suit: Suit::Clubs
            }])
            .to_string(),
            "[JC]"
        );
    }
}
