# Problem 0240: Undo Stack with while let

- **Difficulty:** Easy
- **Topics:** `while let Some(x) = stack.pop()`, an enum recording changes, `String::truncate`, `std::mem::take`
- **Source:** Original · [The Rust Book 19.1](https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html) · [Rust by Example: while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)

## Problem

A one-line text editor supports two edits and an undo history:

- `append(&mut self, s: &str)`: add `s` at the end of the text;
- `clear(&mut self)`: erase the whole text.

Every edit that changes the text pushes a record onto a history stack, so that it can be undone.
Edits that change nothing (`append("")`, or `clear()` on an empty text) are **not** recorded.

Implement `Editor` with:

- `new() -> Editor` (empty text, empty history) and `text(&self) -> &str`;
- `append` and `clear` as described above;
- `undo(&mut self) -> bool`: revert the most recent recorded edit; `false` if there is none;
- `undo_all(&mut self) -> usize`: undo everything, newest first, and return how many edits were
  undone. Write it with a `while let` loop over the history.

Choose the history type yourself; a good one is `enum Change { Appended(usize), Cleared(String) }`
(how many bytes were appended, or the text that was erased).

## Examples

```text
let mut e = Editor::new();
e.append("hello");      // "hello"
e.append(" world");     // "hello world"
e.clear();              // ""
e.append("bye");        // "bye"
e.undo()      => true   // ""
e.undo()      => true   // "hello world"
e.undo_all()  => 2      // ""
e.undo()      => false
```

## Constraints

- At most `10^4` edits; texts are ASCII.

## Hints

1. Undoing `Appended(n)` means cutting the text back to `len - n` bytes: `String::truncate`.
2. Undoing `Cleared(old)` means putting `old` back; `clear` can move the old text into the record
   with `std::mem::take(&mut self.text)`.
3. `while let Some(change) = self.history.pop() { ... }` stops when the stack is empty.

## Starter code

Write your code in [`src/practice/p0240_while_let_undo_stack.rs`](../src/practice/p0240_while_let_undo_stack.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0240
```

```rust
/// A one-line editor with undo.
#[derive(Debug, Default)]
pub struct Editor {}

impl Editor {
    /// An empty editor.
    pub fn new() -> Editor {
        todo!("Write your solution here")
    }

    /// The current text.
    pub fn text(&self) -> &str {
        todo!("Write your solution here")
    }

    /// Appends `s` (recorded only if `s` is not empty).
    pub fn append(&mut self, s: &str) {
        todo!("Write your solution here")
    }

    /// Erases the text (recorded only if it was not empty).
    pub fn clear(&mut self) {
        todo!("Write your solution here")
    }

    /// Reverts the latest edit.
    pub fn undo(&mut self) -> bool {
        todo!("Write your solution here")
    }

    /// Reverts every edit, newest first; returns how many.
    pub fn undo_all(&mut self) -> usize {
        todo!("Write your solution here")
    }
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_walkthrough() {
        let mut e = Editor::new();
        e.append("hello");
        e.append(" world");
        e.clear();
        e.append("bye");
        assert_eq!(e.text(), "bye");
        assert!(e.undo());
        assert_eq!(e.text(), "");
        assert!(e.undo());
        assert_eq!(e.text(), "hello world");
        assert_eq!(e.undo_all(), 2);
        assert_eq!(e.text(), "");
        assert!(!e.undo());
    }

    #[test]
    fn no_op_edits_are_not_recorded() {
        let mut e = Editor::new();
        e.clear();
        e.append("");
        assert!(!e.undo());
        e.append("x");
        e.append("");
        assert!(e.undo());
        assert!(!e.undo());
    }

    #[test]
    fn undo_all_on_empty_history() {
        let mut e = Editor::new();
        assert_eq!(e.undo_all(), 0);
        assert_eq!(e.text(), "");
    }

    #[test]
    fn edits_after_undo_continue_normally() {
        let mut e = Editor::new();
        e.append("ab");
        e.append("cd");
        assert!(e.undo());
        e.append("XY");
        assert_eq!(e.text(), "abXY");
        e.clear();
        e.clear();
        assert!(e.undo());
        assert_eq!(e.text(), "abXY");
    }

    #[test]
    fn many_edits() {
        let mut e = Editor::new();
        for i in 0..10_000 {
            if i % 100 == 99 {
                e.clear();
            } else {
                e.append("z");
            }
        }
        // i = 9_999 is a clear, so the text ends empty; every edit changed the text.
        assert_eq!(e.text(), "");
        assert!(e.undo());
        assert_eq!(e.text().len(), 99);
        assert_eq!(e.undo_all(), 9_999);
        assert_eq!(e.text(), "");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** store the *inverse information* for each edit. `undo` pops one record and applies its
inverse; `undo_all` repeats that with `while let` until the stack is empty.

**Rust concepts:**
- `while let Some(x) = v.pop()` loops while the pattern matches: `pop` returns `None` on an empty
  vector, which ends the loop. It is the idiomatic way to drain a stack.
- `std::mem::take(&mut self.text)` moves the `String` out of the field and leaves an empty one,
  so the erased text is saved without a clone.
- The `Change` enum is private: the history is an implementation detail.
- `undo` returns `bool` by matching on `self.history.pop()` and calling a shared helper
  `revert`, used by `undo_all` too.

```rust
/// How to revert one edit.
#[derive(Debug, Clone, PartialEq)]
enum Change {
    /// `n` bytes were appended.
    Appended(usize),
    /// The text was cleared; this is what it contained.
    Cleared(String),
}

/// A one-line editor with undo.
#[derive(Debug, Default)]
pub struct Editor {
    text: String,
    history: Vec<Change>,
}

impl Editor {
    /// An empty editor.
    pub fn new() -> Editor {
        Editor::default()
    }

    /// The current text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Appends `s` (recorded only if `s` is not empty).
    pub fn append(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        self.text.push_str(s);
        self.history.push(Change::Appended(s.len()));
    }

    /// Erases the text (recorded only if it was not empty).
    pub fn clear(&mut self) {
        if self.text.is_empty() {
            return;
        }
        let old = std::mem::take(&mut self.text);
        self.history.push(Change::Cleared(old));
    }

    /// Reverts the latest edit.
    pub fn undo(&mut self) -> bool {
        match self.history.pop() {
            Some(change) => {
                self.revert(change);
                true
            }
            None => false,
        }
    }

    /// Reverts every edit, newest first; returns how many.
    pub fn undo_all(&mut self) -> usize {
        let mut count = 0;
        while let Some(change) = self.history.pop() {
            self.revert(change);
            count += 1;
        }
        count
    }

    fn revert(&mut self, change: Change) {
        match change {
            Change::Appended(n) => {
                let keep = self.text.len() - n;
                self.text.truncate(keep);
            }
            Change::Cleared(old) => self.text = old,
        }
    }
}
```

Full tested solution: [`src/solutions/p0240_while_let_undo_stack.rs`](../src/solutions/p0240_while_let_undo_stack.rs)

</details>
