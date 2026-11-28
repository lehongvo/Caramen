// ==================================================================
// PRACTICE 0208: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0208_consuming_self_ticket.md
// Run the tests:          cargo test practice::p0208
// ==================================================================

//! # Problem 0208: Redeem a Ticket by Value
//!
//! - **Difficulty:** Easy
//! - **Topics:** methods taking `self` by value, moving fields out of a struct, consuming APIs
//! - **Source:** Original · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
//!
//! ## Problem
//!
//! A method whose first parameter is plain `self` (not `&self`) **takes ownership** of the value. After
//! the call the caller can no longer use it. This is perfect for actions that may only happen once,
//! such as redeeming a concert ticket: once redeemed, the ticket is gone and a receipt exists instead.
//!
//! ```text
//! pub struct Ticket  { pub event: String, pub seat: u32 }
//! pub struct Receipt { pub event: String, pub seat: u32, pub code: String }
//! ```
//!
//! Implement:
//!
//! - `Ticket::new(event: &str, seat: u32) -> Self`.
//! - `Ticket::upgrade(self, new_seat: u32) -> Ticket`: consumes the ticket and returns a ticket for
//!   the same event with the new seat.
//! - `Ticket::redeem(self) -> Receipt`: consumes the ticket. The receipt has the same event and seat,
//!   and `code` is the event name in uppercase, a `#`, and the seat number padded with zeros to at
//!   least 3 digits, e.g. `"JAZZ NIGHT#007"`.
//! - `Ticket::into_parts(self) -> (String, u32)`: consumes the ticket and returns its two fields.
//!
//! Move the `event` string into the result; do not clone it.
//!
//! ## Examples
//!
//! ```text
//! Ticket::new("Jazz Night", 7).redeem()
//!   => Receipt { event: "Jazz Night", seat: 7, code: "JAZZ NIGHT#007" }
//!
//! Ticket::new("Opera", 12).upgrade(1).redeem().code   => "OPERA#001"
//! Ticket::new("Rock", 1234).into_parts()              => ("Rock", 1234)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= seat <= 10^6`; `event` is non-empty.
//!
//! ## Hints
//!
//! 1. `format!("{:03}", seat)` pads a number with zeros to width 3.
//! 2. Compute the code **before** moving `self.event` into the receipt, or destructure first:
//!    `let Ticket { event, seat } = self;`.

/// A ticket that can be redeemed once.
#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    pub event: String,
    pub seat: u32,
}

/// Proof that a ticket was redeemed.
#[derive(Debug, Clone, PartialEq)]
pub struct Receipt {
    pub event: String,
    pub seat: u32,
    pub code: String,
}

impl Ticket {
    /// Creates a ticket.
    pub fn new(event: &str, seat: u32) -> Self {
        todo!("Write your solution here")
    }

    /// Exchanges the ticket for one with another seat.
    pub fn upgrade(self, new_seat: u32) -> Ticket {
        todo!("Write your solution here")
    }

    /// Uses up the ticket and returns a receipt.
    pub fn redeem(self) -> Receipt {
        todo!("Write your solution here")
    }

    /// Splits the ticket into its event name and seat.
    pub fn into_parts(self) -> (String, u32) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_redeem() {
        let receipt = Ticket::new("Jazz Night", 7).redeem();
        assert_eq!(
            receipt,
            Receipt {
                event: String::from("Jazz Night"),
                seat: 7,
                code: String::from("JAZZ NIGHT#007"),
            }
        );
    }

    #[test]
    fn example_upgrade_then_redeem() {
        let receipt = Ticket::new("Opera", 12).upgrade(1).redeem();
        assert_eq!(receipt.code, "OPERA#001");
        assert_eq!(receipt.seat, 1);
        assert_eq!(receipt.event, "Opera");
    }

    #[test]
    fn example_into_parts() {
        assert_eq!(
            Ticket::new("Rock", 1234).into_parts(),
            (String::from("Rock"), 1234)
        );
    }

    #[test]
    fn long_seat_numbers_are_not_truncated() {
        assert_eq!(Ticket::new("x", 1_000_000).redeem().code, "X#1000000");
    }

    #[test]
    fn upgrade_keeps_event() {
        let t = Ticket::new("Ballet", 5).upgrade(9).upgrade(3);
        assert_eq!(
            t,
            Ticket {
                event: String::from("Ballet"),
                seat: 3
            }
        );
    }
}
