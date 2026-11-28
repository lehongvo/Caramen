//! # Problem 0214: A 24-Hour Clock Struct
//!
//! - **Difficulty:** Easy
//! - **Topics:** normalising constructor, private fields, `rem_euclid`, `#[derive(Clone, Copy, PartialEq, Eq)]`
//! - **Source:** Original · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) · [`i32::rem_euclid`](https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid)
//!
//! ## Problem
//!
//! Model a wall clock that shows hours and minutes, with no date. Every `Clock` value must always
//! hold a valid time between `00:00` and `23:59`, so that two clocks showing the same time compare
//! equal with `==`.
//!
//! Implement the `Clock` struct (its fields are private; choose them yourself) and these methods:
//!
//! - `Clock::new(hours: i32, minutes: i32) -> Clock`: any numbers are accepted, including negative
//!   ones and values past the end of the day. The clock "rolls over" like a real one:
//!   `new(25, 0)` shows `01:00`, `new(0, -1)` shows `23:59`, `new(1, 90)` shows `02:30`.
//! - `hours(&self) -> u32` and `minutes(&self) -> u32`: the two parts of the displayed time.
//! - `add_minutes(self, minutes: i32) -> Clock`: a new clock moved forward (or backward if
//!   `minutes` is negative). The struct is `Copy`, so the original stays usable.
//! - `to_hhmm(&self) -> String`: the time as `"HH:MM"` with two digits each, e.g. `"07:05"`.
//! - `minutes_until(&self, later: Clock) -> u32`: how many minutes you must wait, moving forward,
//!   for this clock to show the same time as `later` (`0` if they already agree).
//!
//! ## Examples
//!
//! ```text
//! Clock::new(8, 5).to_hhmm()                  => "08:05"
//! Clock::new(25, 160).to_hhmm()               => "03:40"   (25:00 + 2h40 rolls over)
//! Clock::new(-1, -15).to_hhmm()               => "22:45"
//! Clock::new(23, 30).add_minutes(45).to_hhmm()=> "00:15"
//! Clock::new(24, 0) == Clock::new(0, 0)       => true
//! Clock::new(22, 0).minutes_until(Clock::new(1, 30)) => 210
//! ```
//!
//! ## Constraints
//!
//! - `-10^6 <= hours, minutes <= 10^6` (so `hours * 60 + minutes` fits in an `i32`).
//!
//! ## Hints
//!
//! 1. Store a single number: the minutes since midnight, always in `0..1440`.
//! 2. `%` keeps the sign of the left operand in Rust (`-1 % 1440 == -1`); `rem_euclid` always
//!    returns a non-negative remainder (`(-1i32).rem_euclid(1440) == 1439`).
//! 3. Once the representation is normalised, `#[derive(PartialEq)]` gives the right `==` for free.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** keep one private field, `minutes_since_midnight: u32`, and route every construction
//! through `new`, which reduces `hours * 60 + minutes` with `rem_euclid(1440)`. All other methods
//! are simple arithmetic on that number, and `add_minutes` just calls `new(0, current + delta)`.
//!
//! **Rust concepts:**
//! - A constructor that *normalises* its input is how a struct keeps an invariant: because the
//!   field is private, code outside the module cannot build an invalid `Clock`.
//! - `rem_euclid` is the right "wrap around" operator for negative numbers.
//! - `#[derive(Clone, Copy)]` makes the struct behave like an integer: passing it by value copies it,
//!   so `add_minutes(self, ...)` does not consume the caller's clock.
//! - `format!("{:02}:{:02}", h, m)` pads with zeros to width 2.
// @solution-end

/// A time of day between 00:00 and 23:59.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    // @solution-start remove
    minutes_since_midnight: u32,
    // @solution-end
}

impl Clock {
    /// Builds a clock, rolling over any out-of-range hours and minutes.
    pub fn new(hours: i32, minutes: i32) -> Clock {
        // @solution-start
        let total = (hours * 60 + minutes).rem_euclid(24 * 60);
        // `rem_euclid` with a positive divisor never returns a negative number.
        Clock {
            minutes_since_midnight: total as u32,
        }
        // @solution-end
    }

    /// The hour shown, `0..=23`.
    pub fn hours(&self) -> u32 {
        // @solution-start
        self.minutes_since_midnight / 60
        // @solution-end
    }

    /// The minute shown, `0..=59`.
    pub fn minutes(&self) -> u32 {
        // @solution-start
        self.minutes_since_midnight % 60
        // @solution-end
    }

    /// A new clock moved by `minutes` (negative moves backward).
    pub fn add_minutes(self, minutes: i32) -> Clock {
        // @solution-start
        Clock::new(0, self.minutes_since_midnight as i32 + minutes)
        // @solution-end
    }

    /// The time as `"HH:MM"`.
    pub fn to_hhmm(&self) -> String {
        // @solution-start
        format!("{:02}:{:02}", self.hours(), self.minutes())
        // @solution-end
    }

    /// Minutes to wait, moving forward, until this clock shows `later`'s time.
    pub fn minutes_until(&self, later: Clock) -> u32 {
        // @solution-start
        (later.minutes_since_midnight + 24 * 60 - self.minutes_since_midnight) % (24 * 60)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_times_format_with_two_digits() {
        assert_eq!(Clock::new(8, 5).to_hhmm(), "08:05");
        assert_eq!(Clock::new(0, 0).to_hhmm(), "00:00");
        assert_eq!(Clock::new(23, 59).to_hhmm(), "23:59");
        let c = Clock::new(14, 7);
        assert_eq!((c.hours(), c.minutes()), (14, 7));
    }

    #[test]
    fn rolls_over_forward() {
        assert_eq!(Clock::new(25, 0).to_hhmm(), "01:00");
        assert_eq!(Clock::new(1, 90).to_hhmm(), "02:30");
        assert_eq!(Clock::new(25, 160).to_hhmm(), "03:40");
        assert_eq!(Clock::new(100, 0).to_hhmm(), "04:00");
    }

    #[test]
    fn rolls_over_backward() {
        assert_eq!(Clock::new(0, -1).to_hhmm(), "23:59");
        assert_eq!(Clock::new(-1, -15).to_hhmm(), "22:45");
        assert_eq!(Clock::new(-25, 0).to_hhmm(), "23:00");
        // -61_000_000 minutes = -42_362 days + 1_280 minutes.
        assert_eq!(Clock::new(-1_000_000, -1_000_000).to_hhmm(), "21:20");
    }

    #[test]
    fn add_minutes_copies_and_wraps() {
        let start = Clock::new(23, 30);
        let later = start.add_minutes(45);
        assert_eq!(later.to_hhmm(), "00:15");
        // `start` is still usable because `Clock` is `Copy`.
        assert_eq!(start.to_hhmm(), "23:30");
        assert_eq!(start.add_minutes(-24 * 60 * 3), start);
        assert_eq!(Clock::new(0, 10).add_minutes(-20).to_hhmm(), "23:50");
    }

    #[test]
    fn equality_uses_normalised_time() {
        assert_eq!(Clock::new(24, 0), Clock::new(0, 0));
        assert_eq!(Clock::new(0, 1500), Clock::new(1, 0));
        assert_ne!(Clock::new(10, 0), Clock::new(10, 1));
    }

    #[test]
    fn minutes_until_moves_forward_only() {
        assert_eq!(Clock::new(22, 0).minutes_until(Clock::new(1, 30)), 210);
        assert_eq!(Clock::new(1, 30).minutes_until(Clock::new(22, 0)), 1230);
        assert_eq!(Clock::new(5, 5).minutes_until(Clock::new(5, 5)), 0);
    }
}
