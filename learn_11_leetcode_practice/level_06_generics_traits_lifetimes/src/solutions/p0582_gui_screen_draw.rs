//! # Problem 0582: Drawing a Screen of Components
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Vec<Box<dyn Draw>>`, open extension by library users, generic `add<C: Draw + 'static>` that boxes, `Box<dyn Draw>` means `Box<dyn Draw + 'static>`
//! - **Source:** Original · [The Rust Book 18.2](https://doc.rust-lang.org/book/ch18-02-trait-objects.html#defining-a-trait-for-common-behavior)
//!
//! ## Problem
//!
//! A tiny GUI library draws components as lines of text. Any type that implements
//!
//! ```text
//! pub trait Draw { fn draw(&self) -> Vec<String>; }
//! ```
//!
//! can be put on a `Screen`, including types written later by users of the library.
//!
//! Implement `Draw` for:
//!
//! - `Label { text }`: one line, the text itself.
//! - `Button { label, width }`: one line `"[" + label centred in width characters + "]"`. If the label
//!   is longer than `width`, the width is the label length. (Use the format spec `{:^width$}`.)
//! - `Checkbox { label, checked }`: one line, `"[x] label"` or `"[ ] label"`.
//!
//! and the screen:
//!
//! - `Screen { components: Vec<Box<dyn Draw>> }` (public field) with `new()`, `Default`,
//!   `add<C: Draw + 'static>(&mut self, component: C)` (boxes the component), `len(&self)`,
//!   `is_empty(&self)` and `render(&self) -> Vec<String>`: all lines of all components, in order.
//!
//! ## Examples
//!
//! ```text
//! let mut screen = Screen::new();
//! screen.add(Label { text: "Settings".into() });
//! screen.add(Checkbox { label: "Dark mode".into(), checked: true });
//! screen.add(Button { label: "OK".into(), width: 6 });
//! screen.render() => ["Settings", "[x] Dark mode", "[  OK  ]"]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `format!("[{:^w$}]", self.label, w = self.width)` centres the label (extra space goes to the
//!    right when the padding is odd).
//! 2. `Box<dyn Draw>` is short for `Box<dyn Draw + 'static>`: the boxed value may not borrow anything
//!    short-lived. That is why `add` requires `C: 'static`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each component formats its own lines; the screen does not know or care about concrete
//! types and just concatenates what every `Box<dyn Draw>` returns.
//!
//! **Rust concepts:**
//! - Trait objects allow *open* polymorphism: users can add new component types without changing the
//!   library (an enum would need a new variant).
//! - The default lifetime bound of `Box<dyn Trait>` is `'static`, so a generic helper that boxes a value
//!   must require `C: Draw + 'static`.
//! - `flat_map` concatenates the line vectors of all components.
// @solution-end

/// Anything that can draw itself as lines of text.
pub trait Draw {
    fn draw(&self) -> Vec<String>;
}

pub struct Label {
    pub text: String,
}

pub struct Button {
    pub label: String,
    pub width: usize,
}

pub struct Checkbox {
    pub label: String,
    pub checked: bool,
}

impl Draw for Label {
    fn draw(&self) -> Vec<String> {
        // @solution-start
        vec![self.text.clone()]
        // @solution-end
    }
}

impl Draw for Button {
    fn draw(&self) -> Vec<String> {
        // @solution-start
        vec![format!("[{:^w$}]", self.label, w = self.width)]
        // @solution-end
    }
}

impl Draw for Checkbox {
    fn draw(&self) -> Vec<String> {
        // @solution-start
        let mark = if self.checked { 'x' } else { ' ' };
        vec![format!("[{mark}] {}", self.label)]
        // @solution-end
    }
}

/// A list of components drawn from top to bottom.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new() -> Self {
        // @solution-start
        Screen {
            components: Vec::new(),
        }
        // @solution-end
    }

    pub fn add<C: Draw + 'static>(&mut self, component: C) {
        // @solution-start
        self.components.push(Box::new(component));
        // @solution-end
    }

    pub fn len(&self) -> usize {
        // @solution-start
        self.components.len()
        // @solution-end
    }

    pub fn is_empty(&self) -> bool {
        // @solution-start
        self.components.is_empty()
        // @solution-end
    }

    pub fn render(&self) -> Vec<String> {
        // @solution-start
        self.components.iter().flat_map(|c| c.draw()).collect()
        // @solution-end
    }
}

impl Default for Screen {
    fn default() -> Self {
        // @solution-start
        Self::new()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_screen() {
        let mut screen = Screen::new();
        screen.add(Label {
            text: "Settings".into(),
        });
        screen.add(Checkbox {
            label: "Dark mode".into(),
            checked: true,
        });
        screen.add(Button {
            label: "OK".into(),
            width: 6,
        });
        assert_eq!(screen.len(), 3);
        assert_eq!(
            screen.render(),
            vec!["Settings", "[x] Dark mode", "[  OK  ]"]
        );
    }

    #[test]
    fn button_widths() {
        let narrow = Button {
            label: "Cancel".into(),
            width: 2,
        };
        assert_eq!(narrow.draw(), vec!["[Cancel]"]);
        let odd = Button {
            label: "Go".into(),
            width: 5,
        };
        assert_eq!(odd.draw(), vec!["[ Go  ]"]);
        let unchecked = Checkbox {
            label: "Sync".into(),
            checked: false,
        };
        assert_eq!(unchecked.draw(), vec!["[ ] Sync"]);
    }

    #[test]
    fn user_defined_components() {
        // A component the library has never heard of, drawing several lines.
        struct Table {
            rows: Vec<(String, u32)>,
        }
        impl Draw for Table {
            fn draw(&self) -> Vec<String> {
                self.rows
                    .iter()
                    .map(|(k, v)| format!("{k:<6}|{v:>4}"))
                    .collect()
            }
        }
        let mut screen = Screen::default();
        assert!(screen.is_empty());
        screen.add(Table {
            rows: vec![("cpu".into(), 42), ("memory".into(), 7)],
        });
        screen
            .components
            .push(Box::new(Label { text: "--".into() }));
        assert_eq!(screen.render(), vec!["cpu   |  42", "memory|   7", "--"]);
    }
}
