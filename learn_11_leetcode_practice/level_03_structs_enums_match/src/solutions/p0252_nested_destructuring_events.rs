//! # Problem 0252: Nested Destructuring of UI Events
//!
//! - **Difficulty:** Medium
//! - **Topics:** nested enum and struct patterns, literals inside nested patterns, `..` in struct patterns, `if let` with nested patterns
//! - **Source:** Original · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#destructuring-nested-structs-and-enums)
//!
//! ## Problem
//!
//! A GUI toolkit delivers events of these types:
//!
//! ```text
//! struct Point { x: i32, y: i32 }
//! enum Button { Left, Right, Middle }
//! enum Key { Char(char), Enter, Escape, Backspace }
//! enum Event {
//!     Click { button: Button, at: Point },
//!     KeyPress { key: Key, shift: bool },
//!     Scroll { delta: i32 },
//!     Resize(u32, u32),
//! }
//! ```
//!
//! Implement `describe(event: &Event) -> String` with **one** `match`, checking these rules in order:
//!
//! 1. a left click at `(0, 0)`: `"left click at origin"`;
//! 2. a left click anywhere else: `"left click at (x, y)"`;
//! 3. a right click: `"context menu at (x, y)"`;
//! 4. any other click: `"middle click"`;
//! 5. a character key with shift: `"typed 'C'"` where `C` is the character in uppercase
//!    (use `to_ascii_uppercase`); without shift: `"typed 'c'"` as is;
//! 6. `Enter`: `"submit"`; `Escape`: `"cancel"`; `Backspace`: `"erase"` (shift does not matter);
//! 7. a scroll of `0`: `"no scroll"`; a positive delta `d`: `"scroll down d"`; a negative delta:
//!    `"scroll up d"` with `d` the absolute value;
//! 8. a resize with equal sides: `"square window n"`; otherwise `"window w x h"`.
//!
//! Then write `typed_text(events: &[Event]) -> String`: replay the key presses only. Character keys
//! add their character (uppercase with shift), `Enter` adds `'\n'`, `Backspace` removes the last
//! character (if any), and every other event is ignored.
//!
//! ## Examples
//!
//! ```text
//! describe(&Click { button: Left, at: Point { x: 0, y: 0 } })   => "left click at origin"
//! describe(&Click { button: Right, at: Point { x: 3, y: -2 } }) => "context menu at (3, -2)"
//! describe(&KeyPress { key: Char('q'), shift: true })           => "typed 'Q'"
//! describe(&Scroll { delta: -3 })                               => "scroll up 3"
//! describe(&Resize(800, 800))                                   => "square window 800"
//! typed_text(&[KeyPress h (shift), KeyPress i, Scroll, KeyPress Backspace, KeyPress o, KeyPress Enter])
//!                                                               => "Ho\n"
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` events; scroll deltas are never `i32::MIN`.
//!
//! ## Hints
//!
//! 1. Patterns nest as deep as the data: `Event::Click { button: Button::Left, at: Point { x: 0, y: 0 } }`.
//! 2. `Event::KeyPress { key: Key::Enter, .. }` ignores the `shift` field.
//! 3. For scrolls: `Event::Scroll { delta: 0 }`, then `Event::Scroll { delta: d @ 1.. }`, then the rest.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** list the rules as arms from most specific to most general. Literal values inside the
//! nested pattern (`x: 0, y: 0`, `shift: true`, `delta: 0`) select the special cases; bindings pull
//! out the data the message needs. `typed_text` uses the same nested patterns on key events only.
//!
//! **Rust concepts:**
//! - A pattern can mirror the full structure of a value: enum variant, then struct fields, then
//!   the variants and fields inside those.
//! - `..` in a struct pattern ignores the remaining fields; `_` ignores one value.
//! - Matching on `&Event` binds fields by reference (`x: &i32`, `c: &char`), so `*c` or `format!`
//!   (which accepts references) are used in the arm bodies.
//! - `@` and ranges also work inside nested patterns: `Event::Scroll { delta: d @ 1.. }`.
//! - `Resize(w, h) if w == h` uses a guard because a pattern cannot compare two fields.
// @solution-end

/// A position on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// A mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Left,
    Right,
    Middle,
}

/// A keyboard key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Escape,
    Backspace,
}

/// A user-interface event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Click { button: Button, at: Point },
    KeyPress { key: Key, shift: bool },
    Scroll { delta: i32 },
    Resize(u32, u32),
}

/// A one-line description of the event.
pub fn describe(event: &Event) -> String {
    // @solution-start
    match event {
        Event::Click {
            button: Button::Left,
            at: Point { x: 0, y: 0 },
        } => String::from("left click at origin"),
        Event::Click {
            button: Button::Left,
            at: Point { x, y },
        } => format!("left click at ({x}, {y})"),
        Event::Click {
            button: Button::Right,
            at: Point { x, y },
        } => format!("context menu at ({x}, {y})"),
        Event::Click { .. } => String::from("middle click"),
        Event::KeyPress {
            key: Key::Char(c),
            shift: true,
        } => format!("typed '{}'", c.to_ascii_uppercase()),
        Event::KeyPress {
            key: Key::Char(c),
            shift: false,
        } => format!("typed '{c}'"),
        Event::KeyPress {
            key: Key::Enter, ..
        } => String::from("submit"),
        Event::KeyPress {
            key: Key::Escape, ..
        } => String::from("cancel"),
        Event::KeyPress {
            key: Key::Backspace,
            ..
        } => String::from("erase"),
        Event::Scroll { delta: 0 } => String::from("no scroll"),
        Event::Scroll { delta: d @ 1.. } => format!("scroll down {d}"),
        Event::Scroll { delta } => format!("scroll up {}", delta.unsigned_abs()),
        Event::Resize(w, h) if w == h => format!("square window {w}"),
        Event::Resize(w, h) => format!("window {w} x {h}"),
    }
    // @solution-end
}

/// Replays the key presses into text.
pub fn typed_text(events: &[Event]) -> String {
    // @solution-start
    let mut text = String::new();
    for event in events {
        if let Event::KeyPress { key, shift } = event {
            match key {
                Key::Char(c) if *shift => text.push(c.to_ascii_uppercase()),
                Key::Char(c) => text.push(*c),
                Key::Enter => text.push('\n'),
                Key::Backspace => {
                    text.pop();
                }
                Key::Escape => {}
            }
        }
    }
    text
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::Button::*;
    use super::Event::*;
    use super::Key::*;
    use super::*;

    fn click(button: Button, x: i32, y: i32) -> Event {
        Click {
            button,
            at: Point { x, y },
        }
    }

    fn key(key: Key, shift: bool) -> Event {
        KeyPress { key, shift }
    }

    #[test]
    fn clicks() {
        assert_eq!(describe(&click(Left, 0, 0)), "left click at origin");
        assert_eq!(describe(&click(Left, 0, 5)), "left click at (0, 5)");
        assert_eq!(describe(&click(Right, 3, -2)), "context menu at (3, -2)");
        assert_eq!(describe(&click(Right, 0, 0)), "context menu at (0, 0)");
        assert_eq!(describe(&click(Middle, 9, 9)), "middle click");
    }

    #[test]
    fn keys() {
        assert_eq!(describe(&key(Char('q'), true)), "typed 'Q'");
        assert_eq!(describe(&key(Char('q'), false)), "typed 'q'");
        assert_eq!(describe(&key(Char('7'), true)), "typed '7'");
        assert_eq!(describe(&key(Enter, true)), "submit");
        assert_eq!(describe(&key(Escape, false)), "cancel");
        assert_eq!(describe(&key(Backspace, false)), "erase");
    }

    #[test]
    fn scrolls_and_resizes() {
        assert_eq!(describe(&Scroll { delta: 0 }), "no scroll");
        assert_eq!(describe(&Scroll { delta: 4 }), "scroll down 4");
        assert_eq!(describe(&Scroll { delta: -3 }), "scroll up 3");
        assert_eq!(describe(&Resize(800, 800)), "square window 800");
        assert_eq!(describe(&Resize(1024, 768)), "window 1024 x 768");
    }

    #[test]
    fn replay_typed_text() {
        let events = [
            key(Char('h'), true),
            key(Char('i'), false),
            Scroll { delta: 1 },
            key(Backspace, false),
            key(Char('o'), false),
            key(Enter, false),
        ];
        assert_eq!(typed_text(&events), "Ho\n");
    }

    #[test]
    fn backspace_on_empty_and_ignored_events() {
        let events = [
            key(Backspace, false),
            click(Left, 1, 1),
            key(Escape, true),
            key(Char('a'), false),
            Resize(1, 2),
        ];
        assert_eq!(typed_text(&events), "a");
        assert_eq!(typed_text(&[]), "");
    }

    #[test]
    fn many_key_presses() {
        let events: Vec<Event> = (0..10_000).map(|i| key(Char('x'), i % 2 == 0)).collect();
        let text = typed_text(&events);
        assert_eq!(text.len(), 10_000);
        assert_eq!(text.matches('X').count(), 5_000);
    }
}
