# Problem 0945: Implement Trie (Prefix Tree)

- **Difficulty:** Medium
- **Topics:** tries, `[Option<Box<Node>>; 26]`, `Default` for arrays of `Option`, walking a tree with `&` and `&mut` references
- **Source:** [LeetCode 208. Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree/)

## Problem

A **trie** stores a set of words as a tree of characters: every path from the root spells a
prefix, and nodes are marked where a complete word ends. Lookups cost `O(length of the word)`,
independent of how many words are stored.

Implement the `Trie` struct:

- `Trie::new() -> Self`: an empty trie.
- `insert(&mut self, word: String)`: adds `word` (inserting it twice changes nothing).
- `search(&self, word: String) -> bool`: `true` if `word` was inserted.
- `starts_with(&self, prefix: String) -> bool`: `true` if some inserted word starts with `prefix`
  (a word is a prefix of itself).

## Examples

```text
let mut trie = Trie::new();
trie.insert("apple")
trie.search("apple")     => true
trie.search("app")       => false
trie.starts_with("app")  => true
trie.insert("app")
trie.search("app")       => true
```

## Constraints

- `1 <= word.len(), prefix.len() <= 2000`
- Words and prefixes consist only of lowercase English letters.
- At most `3 * 10^4` calls in total.

## Hints

1. Each node has 26 child slots, one per letter; `(b - b'a') as usize` is the slot of byte `b`.
2. `insert` walks down with a `&mut Node`, creating missing children with
   `get_or_insert_with(Box::default)`.
3. `search` and `starts_with` share a helper that returns the node reached by a prefix, if any.

## Starter code

Write your code in [`src/practice/p0945_implement_trie_prefix_tree.rs`](../src/practice/p0945_implement_trie_prefix_tree.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0945
```

```rust
pub struct Trie {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl Trie {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn insert(&mut self, word: String) {
        todo!("Write your solution here")
    }

    pub fn search(&self, word: String) -> bool {
        todo!("Write your solution here")
    }

    pub fn starts_with(&self, prefix: String) -> bool {
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

    fn s(text: &str) -> String {
        text.to_string()
    }

    #[test]
    fn example_sequence() {
        let mut trie = Trie::new();
        trie.insert(s("apple"));
        assert!(trie.search(s("apple")));
        assert!(!trie.search(s("app")));
        assert!(trie.starts_with(s("app")));
        trie.insert(s("app"));
        assert!(trie.search(s("app")));
    }

    #[test]
    fn prefixes_are_not_words() {
        let mut trie = Trie::new();
        trie.insert(s("banana"));
        trie.insert(s("band"));
        assert!(trie.starts_with(s("ban")));
        assert!(trie.starts_with(s("banana")));
        assert!(!trie.starts_with(s("bananas")));
        assert!(!trie.search(s("ban")));
        assert!(!trie.search(s("bananas")));
        assert!(!trie.starts_with(s("c")));
    }

    #[test]
    fn duplicates_and_single_letters() {
        let mut trie = Trie::new();
        trie.insert(s("a"));
        trie.insert(s("a"));
        assert!(trie.search(s("a")));
        assert!(!trie.search(s("b")));
        assert!(trie.starts_with(s("a")));
    }

    #[test]
    fn many_words_and_long_word() {
        let mut trie = Trie::new();
        for i in 0..2_000_u32 {
            let word: String = i
                .to_string()
                .bytes()
                .map(|d| char::from(b'a' + (d - b'0')))
                .collect();
            trie.insert(word);
        }
        assert!(trie.search(s("bjjj"))); // 1999
        assert!(!trie.search(s("caaa"))); // 2000 was not inserted
        assert!(trie.starts_with(s("bj")));
        let long = "z".repeat(2_000);
        trie.insert(long.clone());
        assert!(trie.search(long));
        assert!(trie.starts_with("z".repeat(1_999)));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every node owns its children in a fixed array indexed by letter. Insertion creates the
missing nodes along the word and marks the last one; a lookup follows existing children and
fails as soon as one is missing. `search` additionally requires the final node to be marked.

**Complexity:** `O(L)` time per operation for a word of length `L`; `O(total letters)` space.

**Rust concepts:**
- `#[derive(Default)]` works on `[Option<Box<Node>>; 26]` because `Option<T>: Default` and arrays
  of up to 32 `Default` elements implement `Default`.
- `Box` gives the recursive type a known size.
- Walking down with `node = child` (re-assigning a `&mut` or `&` variable) is the iterative way to
  descend a tree without recursion.

```rust
#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    is_word: bool,
}

pub struct Trie {
    root: Node,
}

impl Trie {
    fn find(&self, prefix: &str) -> Option<&Node> {
        let mut node = &self.root;
        for b in prefix.bytes() {
            node = node.children[usize::from(b - b'a')].as_deref()?;
        }
        Some(node)
    }
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Node::default(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for b in word.bytes() {
            node = node.children[usize::from(b - b'a')].get_or_insert_with(Box::default);
        }
        node.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        self.find(&word).is_some_and(|node| node.is_word)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.find(&prefix).is_some()
    }
}
```

Full tested solution: [`src/solutions/p0945_implement_trie_prefix_tree.rs`](../src/solutions/p0945_implement_trie_prefix_tree.rs)

</details>
