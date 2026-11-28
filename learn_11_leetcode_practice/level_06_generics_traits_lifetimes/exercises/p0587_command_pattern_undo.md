# Problem 0587: Undo and Redo with Commands

- **Difficulty:** Medium
- **Topics:** the command pattern with `Box<dyn Command>`, trait methods taking `&mut self` and another `&mut` argument, undo/redo stacks, moving boxes between vectors
- **Source:** Original · [The Rust Book 18.2](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)

## Problem

A text editor applies edits as **command objects** that know how to undo themselves:

```text
pub trait Command {
    fn execute(&mut self, doc: &mut String);
    fn undo(&mut self, doc: &mut String);
    fn describe(&self) -> String;
}
```

Implement three commands (positions are byte offsets; the tests only use ASCII text):

- `Insert { pos, text }`: inserts `text` at `pos` (clamped to the document length). Description:
  `"insert '{text}' at {pos}"`.
- `Delete::new(pos, len)`: deletes up to `len` bytes starting at `pos` (both clamped). It must
  **remember** the removed text (a private field) so that `undo` can put it back. Description:
  `"delete {len} at {pos}"`.
- `ReplaceAll::new(from, to)`: replaces every occurrence; `undo` restores the **previous document**
  (saved in a private field during `execute`). Description: `"replace '{from}' with '{to}'"`.

and the editor:

- `Editor::new(text: &str)`, `text(&self) -> &str`.
- `run(&mut self, command: Box<dyn Command>)`: executes it, pushes it on the undo stack and
  **clears** the redo stack.
- `undo(&mut self) -> bool`: undoes the latest command and moves it to the redo stack (`false` if
  there is nothing to undo).
- `redo(&mut self) -> bool`: executes the latest undone command again and moves it back.
- `history(&self) -> Vec<String>`: descriptions of the undo stack, oldest first.

## Examples

```text
let mut ed = Editor::new("hello world");
ed.run(Box::new(Insert { pos: 5, text: ",".into() }));      // "hello, world"
ed.run(Box::new(Delete::new(0, 7)));                         // "world"
ed.undo();                                                   // "hello, world"
ed.redo();                                                   // "world"
ed.history()  => ["insert ',' at 5", "delete 7 at 0"]
```

## Constraints

- Documents are ASCII in the tests.

## Hints

1. `String::insert_str`, `String::drain(range)` (it returns the removed characters) and `str::replace`.
2. `if let Some(mut cmd) = self.undo_stack.pop() { cmd.undo(&mut self.doc); self.redo_stack.push(cmd); }`

## Starter code

Write your code in [`src/practice/p0587_command_pattern_undo.rs`](../src/practice/p0587_command_pattern_undo.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0587
```

```rust
/// An undoable edit of a text document.
pub trait Command {
    fn execute(&mut self, doc: &mut String);
    fn undo(&mut self, doc: &mut String);
    fn describe(&self) -> String;
}

pub struct Insert {
    pub pos: usize,
    pub text: String,
}

pub struct Delete {
    pos: usize,
    len: usize,
}

pub struct ReplaceAll {
    from: String,
    to: String,
}

impl Delete {
    pub fn new(pos: usize, len: usize) -> Self {
        todo!("Write your solution here")
    }
}

impl ReplaceAll {
    pub fn new(from: &str, to: &str) -> Self {
        todo!("Write your solution here")
    }
}

impl Command for Insert {
    fn execute(&mut self, doc: &mut String) {
        todo!("Write your solution here")
    }

    fn undo(&mut self, doc: &mut String) {
        todo!("Write your solution here")
    }

    fn describe(&self) -> String {
        todo!("Write your solution here")
    }
}

impl Command for Delete {
    fn execute(&mut self, doc: &mut String) {
        todo!("Write your solution here")
    }

    fn undo(&mut self, doc: &mut String) {
        todo!("Write your solution here")
    }

    fn describe(&self) -> String {
        todo!("Write your solution here")
    }
}

impl Command for ReplaceAll {
    fn execute(&mut self, doc: &mut String) {
        todo!("Write your solution here")
    }

    fn undo(&mut self, doc: &mut String) {
        todo!("Write your solution here")
    }

    fn describe(&self) -> String {
        todo!("Write your solution here")
    }
}

/// A document with undo and redo.
pub struct Editor {}

impl Editor {
    pub fn new(text: &str) -> Self {
        todo!("Write your solution here")
    }

    pub fn text(&self) -> &str {
        todo!("Write your solution here")
    }

    pub fn run(&mut self, mut command: Box<dyn Command>) {
        todo!("Write your solution here")
    }

    pub fn undo(&mut self) -> bool {
        todo!("Write your solution here")
    }

    pub fn redo(&mut self) -> bool {
        todo!("Write your solution here")
    }

    pub fn history(&self) -> Vec<String> {
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
    fn example_session() {
        let mut ed = Editor::new("hello world");
        ed.run(Box::new(Insert {
            pos: 5,
            text: ",".into(),
        }));
        assert_eq!(ed.text(), "hello, world");
        ed.run(Box::new(Delete::new(0, 7)));
        assert_eq!(ed.text(), "world");
        assert!(ed.undo());
        assert_eq!(ed.text(), "hello, world");
        assert!(ed.redo());
        assert_eq!(ed.text(), "world");
        assert_eq!(ed.history(), vec!["insert ',' at 5", "delete 7 at 0"]);
    }

    #[test]
    fn replace_all_and_full_undo() {
        let mut ed = Editor::new("a-b-c");
        ed.run(Box::new(ReplaceAll::new("-", "+")));
        ed.run(Box::new(Insert {
            pos: 100,
            text: "!".into(),
        }));
        assert_eq!(ed.text(), "a+b+c!");
        assert!(ed.undo());
        assert!(ed.undo());
        assert!(!ed.undo());
        assert_eq!(ed.text(), "a-b-c");
        assert_eq!(ed.history(), Vec::<String>::new());
    }

    #[test]
    fn new_command_clears_redo() {
        let mut ed = Editor::new("abc");
        ed.run(Box::new(Delete::new(1, 10)));
        assert_eq!(ed.text(), "a");
        ed.undo();
        assert_eq!(ed.text(), "abc");
        ed.run(Box::new(Insert {
            pos: 0,
            text: ">".into(),
        }));
        assert!(!ed.redo());
        assert_eq!(ed.text(), ">abc");
        assert_eq!(ed.history(), vec!["insert '>' at 0"]);
    }

    #[test]
    fn many_edits_round_trip() {
        let mut ed = Editor::new("");
        for i in 0..200 {
            ed.run(Box::new(Insert {
                pos: i,
                text: ((b'a' + (i % 26) as u8) as char).to_string(),
            }));
        }
        assert_eq!(ed.text().len(), 200);
        ed.run(Box::new(Delete::new(50, 100)));
        assert_eq!(ed.text().len(), 100);
        while ed.undo() {}
        assert_eq!(ed.text(), "");
        while ed.redo() {}
        assert_eq!(ed.text().len(), 100);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each command stores exactly what it needs to reverse itself. The editor only moves boxed
commands between two stacks and calls their methods; it never needs to know their concrete types.

**Rust concepts:**
- Trait objects with `&mut self` methods can update their own state (the removed text, the saved
  document) while also mutating the document passed in.
- Popping a `Box<dyn Command>` from one `Vec` and pushing it onto another moves ownership; nothing is
  cloned.
- Private fields (`removed`, `before`) plus constructors (`Delete::new`) keep the undo data out of the
  caller's hands.

```rust
/// An undoable edit of a text document.
pub trait Command {
    fn execute(&mut self, doc: &mut String);
    fn undo(&mut self, doc: &mut String);
    fn describe(&self) -> String;
}

pub struct Insert {
    pub pos: usize,
    pub text: String,
}

pub struct Delete {
    pos: usize,
    len: usize,
    removed: String,
    start: usize,
}

pub struct ReplaceAll {
    from: String,
    to: String,
    before: String,
}

impl Delete {
    pub fn new(pos: usize, len: usize) -> Self {
        Delete {
            pos,
            len,
            removed: String::new(),
            start: 0,
        }
    }
}

impl ReplaceAll {
    pub fn new(from: &str, to: &str) -> Self {
        ReplaceAll {
            from: from.to_string(),
            to: to.to_string(),
            before: String::new(),
        }
    }
}

impl Command for Insert {
    fn execute(&mut self, doc: &mut String) {
        let pos = self.pos.min(doc.len());
        doc.insert_str(pos, &self.text);
    }

    fn undo(&mut self, doc: &mut String) {
        let pos = self.pos.min(doc.len() - self.text.len());
        doc.replace_range(pos..pos + self.text.len(), "");
    }

    fn describe(&self) -> String {
        format!("insert '{}' at {}", self.text, self.pos)
    }
}

impl Command for Delete {
    fn execute(&mut self, doc: &mut String) {
        self.start = self.pos.min(doc.len());
        let end = (self.start + self.len).min(doc.len());
        self.removed = doc.drain(self.start..end).collect();
    }

    fn undo(&mut self, doc: &mut String) {
        doc.insert_str(self.start, &self.removed);
    }

    fn describe(&self) -> String {
        format!("delete {} at {}", self.len, self.pos)
    }
}

impl Command for ReplaceAll {
    fn execute(&mut self, doc: &mut String) {
        self.before = doc.clone();
        *doc = doc.replace(&self.from, &self.to);
    }

    fn undo(&mut self, doc: &mut String) {
        *doc = std::mem::take(&mut self.before);
    }

    fn describe(&self) -> String {
        format!("replace '{}' with '{}'", self.from, self.to)
    }
}

/// A document with undo and redo.
pub struct Editor {
    doc: String,
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}

impl Editor {
    pub fn new(text: &str) -> Self {
        Editor {
            doc: text.to_string(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn text(&self) -> &str {
        &self.doc
    }

    pub fn run(&mut self, mut command: Box<dyn Command>) {
        command.execute(&mut self.doc);
        self.undo_stack.push(command);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> bool {
        match self.undo_stack.pop() {
            Some(mut cmd) => {
                cmd.undo(&mut self.doc);
                self.redo_stack.push(cmd);
                true
            }
            None => false,
        }
    }

    pub fn redo(&mut self) -> bool {
        match self.redo_stack.pop() {
            Some(mut cmd) => {
                cmd.execute(&mut self.doc);
                self.undo_stack.push(cmd);
                true
            }
            None => false,
        }
    }

    pub fn history(&self) -> Vec<String> {
        self.undo_stack.iter().map(|c| c.describe()).collect()
    }
}
```

Full tested solution: [`src/solutions/p0587_command_pattern_undo.rs`](../src/solutions/p0587_command_pattern_undo.rs)

</details>
