# Problem 0926: Memoized Fields with `OnceCell`

- **Difficulty:** Medium
- **Topics:** `std::cell::OnceCell`, `get_or_init`, `set`/`get`, resetting caches with `take`, `Cell` counters, returning references into a cache
- **Source:** Original · [std: `OnceCell`](https://doc.rust-lang.org/std/cell/struct.OnceCell.html)

## Problem

A `Document` computes expensive derived data (a word count, a word index) only when first asked
and then reuses it, even though the getters take `&self`. `OnceCell<T>` is made for this: it is
empty until the first `get_or_init(|| ...)` stores a value, and from then on it hands out
references to that value.

Implement `Document` (choose the private fields):

- `new(text: &str) -> Document`.
- `text(&self) -> &str`.
- `word_count(&self) -> usize`: the number of words, where a word is a whitespace-separated piece
  with leading and trailing non-alphanumeric characters removed, ignoring pieces that become
  empty (`"--"` is not a word). Computed once.
- `positions(&self, word: &str) -> &[usize]`: the 0-based indices, among those words, of every
  occurrence of `word`, compared in lowercase (`word` is lowercased too). The whole index
  (`HashMap<String, Vec<usize>>`) is built once, on the first call; absent words give `&[]`.
- `computations(&self) -> usize`: how many times a cached value was **computed** so far (the
  count and the index each add 1 when they are built).
- `set_title(&self, title: &str) -> Result<(), String>`: the title can be set only once; a second
  call returns `Err(rejected_title)`. `title(&self) -> Option<&str>`.
- `set_text(&mut self, text: &str)`: replaces the text and **clears** the count and the index
  (they will be recomputed on demand). The title is kept.

## Examples

```text
let doc = Document::new("The cat saw the other cat -- twice.");
doc.word_count()         => 7      computations() => 1
doc.word_count()         => 7      computations() => 1  (cached)
doc.positions("cat")     => [1, 5]
doc.positions("THE")     => [0, 3] computations() => 2
doc.set_title("Cats")    => Ok(());  doc.set_title("Dogs") => Err("Dogs")
```

## Constraints

- No special constraints.

## Hints

1. `self.count.get_or_init(|| { self.computations.set(...); compute() })` returns `&usize`.
2. `OnceCell::set` returns `Err(value)` if the cell was already full.
3. `OnceCell::take(&mut self)` empties a cell; it needs `&mut self`, which `set_text` has.

## Starter code

Write your code in [`src/practice/p0926_oncecell_memoized_fields.rs`](../src/practice/p0926_oncecell_memoized_fields.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0926
```

```rust
use std::cell::{Cell, OnceCell};
use std::collections::HashMap;

pub struct Document {}

impl Document {
    pub fn new(text: &str) -> Document {
        todo!("Write your solution here")
    }

    pub fn text(&self) -> &str {
        todo!("Write your solution here")
    }

    pub fn word_count(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn positions(&self, word: &str) -> &[usize] {
        todo!("Write your solution here")
    }

    pub fn computations(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn set_title(&self, title: &str) -> Result<(), String> {
        todo!("Write your solution here")
    }

    pub fn title(&self) -> Option<&str> {
        todo!("Write your solution here")
    }

    pub fn set_text(&mut self, text: &str) {
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
    fn word_count_is_computed_once() {
        let doc = Document::new("The cat saw the other cat -- twice.");
        assert_eq!(doc.computations(), 0);
        assert_eq!(doc.word_count(), 7);
        assert_eq!(doc.computations(), 1);
        assert_eq!(doc.word_count(), 7);
        assert_eq!(doc.computations(), 1);
        assert_eq!(doc.text(), "The cat saw the other cat -- twice.");
    }

    #[test]
    fn positions_use_a_lazily_built_index() {
        let doc = Document::new("The cat saw the other cat -- twice.");
        assert_eq!(doc.positions("cat"), &[1, 5]);
        assert_eq!(doc.computations(), 1);
        assert_eq!(doc.positions("THE"), &[0, 3]);
        assert_eq!(doc.positions("twice"), &[6]);
        assert!(doc.positions("dog").is_empty());
        assert_eq!(doc.computations(), 1);
        assert_eq!(doc.word_count(), 7);
        assert_eq!(doc.computations(), 2);
    }

    #[test]
    fn title_can_be_set_once() {
        let doc = Document::new("");
        assert_eq!(doc.title(), None);
        assert_eq!(doc.set_title("Cats"), Ok(()));
        assert_eq!(doc.set_title("Dogs"), Err("Dogs".to_string()));
        assert_eq!(doc.title(), Some("Cats"));
        assert_eq!(doc.word_count(), 0);
    }

    #[test]
    fn set_text_clears_the_caches() {
        let mut doc = Document::new("one two");
        doc.set_title("Numbers").unwrap();
        assert_eq!(doc.word_count(), 2);
        assert_eq!(doc.positions("two"), &[1]);
        doc.set_text("two two two");
        assert_eq!(doc.word_count(), 3);
        assert_eq!(doc.positions("two"), &[0, 1, 2]);
        assert_eq!(doc.computations(), 4);
        assert_eq!(doc.title(), Some("Numbers"));
    }

    #[test]
    fn large_document() {
        let text = "alpha beta, gamma! ".repeat(10_000);
        let doc = Document::new(&text);
        assert_eq!(doc.word_count(), 30_000);
        assert_eq!(doc.positions("gamma").len(), 10_000);
        assert_eq!(doc.positions("gamma")[9_999], 29_999);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** both caches are `OnceCell`s initialised by closures that bump a `Cell<usize>` counter,
so each computation is visible exactly once. `positions` borrows from the cached map, which is
fine because the map lives as long as `&self` and is never replaced while borrowed (`set_text`
needs `&mut self`). A shared helper splits the text into normalised words.

**Complexity:** the first call of each cache is `O(n)`; later calls are `O(1)` (`positions` costs
one hash lookup).

**Rust concepts:**
- `OnceCell<T>` gives write-once interior mutability: `get_or_init` only needs `&self`, and the
  returned `&T` stays valid because the value is never overwritten through `&self`.
- Resetting requires `&mut self` (`take` or assigning a new `OnceCell`), so the borrow checker
  guarantees no reference into the old value survives.
- `OnceCell` is not `Sync`; the thread-safe version is `std::sync::OnceLock` (problem 0927).

```rust
use std::cell::{Cell, OnceCell};
use std::collections::HashMap;

pub struct Document {
    text: String,
    title: OnceCell<String>,
    count: OnceCell<usize>,
    index: OnceCell<HashMap<String, Vec<usize>>>,
    computations: Cell<usize>,
}

fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split_whitespace()
        .map(|piece| piece.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
}

impl Document {
    pub fn new(text: &str) -> Document {
        Document {
            text: text.to_string(),
            title: OnceCell::new(),
            count: OnceCell::new(),
            index: OnceCell::new(),
            computations: Cell::new(0),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn word_count(&self) -> usize {
        *self.count.get_or_init(|| {
            self.computations.set(self.computations.get() + 1);
            words(&self.text).count()
        })
    }

    pub fn positions(&self, word: &str) -> &[usize] {
        let index = self.index.get_or_init(|| {
            self.computations.set(self.computations.get() + 1);
            let mut index: HashMap<String, Vec<usize>> = HashMap::new();
            for (i, w) in words(&self.text).enumerate() {
                index.entry(w).or_default().push(i);
            }
            index
        });
        index
            .get(&word.to_lowercase())
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn computations(&self) -> usize {
        self.computations.get()
    }

    pub fn set_title(&self, title: &str) -> Result<(), String> {
        self.title.set(title.to_string())
    }

    pub fn title(&self) -> Option<&str> {
        self.title.get().map(String::as_str)
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.count.take();
        self.index.take();
    }
}
```

Full tested solution: [`src/solutions/p0926_oncecell_memoized_fields.rs`](../src/solutions/p0926_oncecell_memoized_fields.rs)

</details>
