// ==================================================================
// PRACTICE 0218: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0218_message_enum.md
// Run the tests:          cargo test practice::p0218
// ==================================================================

//! # Problem 0218: Four Kinds of Messages
//!
//! - **Difficulty:** Easy
//! - **Topics:** unit, tuple and struct-like enum variants, destructuring variants in `match`, `&mut self`
//! - **Source:** Original · [The Rust Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
//!
//! ## Problem
//!
//! A tiny drawing program receives messages of four shapes:
//!
//! ```text
//! enum Message {
//!     Quit,                       // unit variant: no data
//!     Move { x: i32, y: i32 },    // struct-like variant: named fields
//!     Write(String),              // tuple variant with one field
//!     ChangeColor(u8, u8, u8),    // tuple variant with three fields
//! }
//! ```
//!
//! The program state is a `Screen` with public fields `cursor: (i32, i32)`, `text: String`,
//! `color: (u8, u8, u8)` and `running: bool`. Implement:
//!
//! - `Screen::new() -> Screen`: cursor `(0, 0)`, empty text, color `(0, 0, 0)`, `running = true`;
//! - `apply(&mut self, msg: Message)`:
//!   - `Quit` sets `running` to `false`;
//!   - `Move { x, y }` moves the cursor **by** `(x, y)` (relative move);
//!   - `Write(s)` appends `s` to `text`;
//!   - `ChangeColor(r, g, b)` replaces the color;
//!   - once `running` is `false`, every further message is ignored;
//! - `run(messages: Vec<Message>) -> Screen`: starts from `Screen::new()` and applies every message
//!   in order.
//!
//! ## Examples
//!
//! ```text
//! run(vec![Write("hi".into()), Move { x: 2, y: -1 }, ChangeColor(255, 0, 0), Write("!".into())])
//!   => Screen { cursor: (2, -1), text: "hi!", color: (255, 0, 0), running: true }
//!
//! run(vec![Move { x: 1, y: 1 }, Quit, Move { x: 5, y: 5 }, Write("lost".into())])
//!   => Screen { cursor: (1, 1), text: "", color: (0, 0, 0), running: false }
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` messages; the cursor stays within `i32`.
//!
//! ## Hints
//!
//! 1. Each variant shape has its own pattern: `Message::Quit`, `Message::Move { x, y }`,
//!    `Message::Write(s)`, `Message::ChangeColor(r, g, b)`.
//! 2. Check `running` once at the top of `apply` and return early.

/// A message sent to the drawing program.
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

/// The state of the drawing program.
#[derive(Debug, Clone, PartialEq)]
pub struct Screen {
    pub cursor: (i32, i32),
    pub text: String,
    pub color: (u8, u8, u8),
    pub running: bool,
}

impl Screen {
    /// A fresh, running screen.
    // `new` takes no arguments but `running` starts as `true`, so `Default` would be misleading.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Screen {
        todo!("Write your solution here")
    }

    /// Applies one message (ignored after `Quit`).
    pub fn apply(&mut self, msg: Message) {
        todo!("Write your solution here")
    }
}

/// Applies every message to a fresh screen.
pub fn run(messages: Vec<Message>) -> Screen {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::Message::*;
    use super::*;

    #[test]
    fn first_example() {
        let s = run(vec![
            Write("hi".into()),
            Move { x: 2, y: -1 },
            ChangeColor(255, 0, 0),
            Write("!".into()),
        ]);
        assert_eq!(
            s,
            Screen {
                cursor: (2, -1),
                text: "hi!".to_string(),
                color: (255, 0, 0),
                running: true,
            }
        );
    }

    #[test]
    fn messages_after_quit_are_ignored() {
        let s = run(vec![
            Move { x: 1, y: 1 },
            Quit,
            Move { x: 5, y: 5 },
            Write("lost".into()),
            ChangeColor(1, 2, 3),
        ]);
        assert_eq!(s.cursor, (1, 1));
        assert_eq!(s.text, "");
        assert_eq!(s.color, (0, 0, 0));
        assert!(!s.running);
    }

    #[test]
    fn new_screen_and_no_messages() {
        let s = run(vec![]);
        assert_eq!(s, Screen::new());
        assert!(s.running);
        assert_eq!(s.cursor, (0, 0));
    }

    #[test]
    fn apply_directly_with_mut_self() {
        let mut s = Screen::new();
        s.apply(ChangeColor(10, 20, 30));
        s.apply(ChangeColor(40, 50, 60));
        s.apply(Move { x: -3, y: 4 });
        s.apply(Move { x: 3, y: -4 });
        assert_eq!(s.color, (40, 50, 60));
        assert_eq!(s.cursor, (0, 0));
    }

    #[test]
    fn many_moves_and_writes() {
        let mut msgs = Vec::new();
        for _ in 0..5_000 {
            msgs.push(Move { x: 1, y: -2 });
            msgs.push(Write("ab".into()));
        }
        let s = run(msgs);
        assert_eq!(s.cursor, (5_000, -10_000));
        assert_eq!(s.text.len(), 10_000);
    }
}
