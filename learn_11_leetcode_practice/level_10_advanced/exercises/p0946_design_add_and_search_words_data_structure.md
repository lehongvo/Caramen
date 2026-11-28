# Problem 0946: Design Add and Search Words Data Structure

- **Difficulty:** Medium
- **Topics:** tries, wildcard search with recursive DFS over `&Node`, byte slices, `Iterator::any`
- **Source:** [LeetCode 211. Design Add and Search Words Data Structure](https://leetcode.com/problems/design-add-and-search-words-data-structure/)

## Problem

Build a dictionary of words that supports searching with **wildcards**: in a search pattern, the
character `'.'` matches any single letter.

Implement the `WordDictionary` struct:

- `WordDictionary::new() -> Self`: an empty dictionary.
- `add_word(&mut self, word: String)`: stores `word`.
- `search(&self, word: String) -> bool`: `true` if some stored word has exactly the same length as
  the pattern `word` and matches it letter by letter, where `'.'` matches any letter.

## Examples

```text
add_word("bad"); add_word("dad"); add_word("mad");
search("pad")   => false
search("bad")   => true
search(".ad")   => true
search("b..")   => true
search("b.")    => false   (lengths differ)
```

## Constraints

- `1 <= word.len() <= 25`
- Added words contain only lowercase letters; search patterns contain lowercase letters or `'.'`.
- A search pattern contains at most 2 dots.
- At most `10^4` calls in total.

## Hints

1. Store the words in a trie (problem 0945).
2. Search recursively: `matches(node, rest)` follows one child for a letter and tries all
   children for a `'.'`.
3. The recursion ends when `rest` is empty: the answer is whether the node ends a word.

## Starter code

Write your code in [`src/practice/p0946_design_add_and_search_words_data_structure.rs`](../src/practice/p0946_design_add_and_search_words_data_structure.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0946
```

```rust
pub struct WordDictionary {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl WordDictionary {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn add_word(&mut self, word: String) {
        todo!("Write your solution here")
    }

    pub fn search(&self, word: String) -> bool {
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

    fn dictionary(words: &[&str]) -> WordDictionary {
        let mut dict = WordDictionary::new();
        for word in words {
            dict.add_word(word.to_string());
        }
        dict
    }

    #[test]
    fn example_sequence() {
        let dict = dictionary(&["bad", "dad", "mad"]);
        assert!(!dict.search("pad".to_string()));
        assert!(dict.search("bad".to_string()));
        assert!(dict.search(".ad".to_string()));
        assert!(dict.search("b..".to_string()));
        assert!(!dict.search("b.".to_string()));
    }

    #[test]
    fn lengths_must_match() {
        let dict = dictionary(&["a", "ab"]);
        assert!(dict.search(".".to_string()));
        assert!(dict.search("..".to_string()));
        assert!(!dict.search("...".to_string()));
        assert!(dict.search("a.".to_string()));
        assert!(!dict.search(".a".to_string()));
    }

    #[test]
    fn empty_dictionary() {
        let dict = WordDictionary::new();
        assert!(!dict.search("a".to_string()));
        assert!(!dict.search(".".to_string()));
    }

    #[test]
    fn many_words_with_wildcards() {
        let mut dict = WordDictionary::new();
        for a in b'a'..=b'z' {
            for b in b'a'..=b'z' {
                if (a + b) % 3 == 0 {
                    dict.add_word(String::from_utf8(vec![a, b, b'x']).unwrap());
                }
            }
        }
        assert!(dict.search("..x".to_string()));
        assert!(!dict.search("..y".to_string()));
        // 'a' + 'b' = 97 + 98 = 195, divisible by 3.
        assert!(dict.search("abx".to_string()));
        assert!(!dict.search("aax".to_string()));
        assert!(dict.search("a.x".to_string()));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a trie of the added words, plus a recursive search over the pattern's bytes. A letter
moves to one child; a dot branches into every existing child, and `any` stops at the first
branch that matches.

**Complexity:** `add_word` is `O(L)`; `search` is `O(L)` without dots and `O(26^d * L)` with `d`
dots in the worst case (at most `26^2` here).

**Rust concepts:**
- Slice patterns `match rest { [] => ..., [first, tail @ ..] => ... }` split the pattern cleanly.
- `children.iter().flatten()` iterates over the existing children only (skipping `None`).
- The recursive helper takes `&Node` and `&[u8]`, so the search never allocates.

```rust
#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    is_word: bool,
}

fn matches(node: &Node, rest: &[u8]) -> bool {
    match rest {
        [] => node.is_word,
        [b'.', tail @ ..] => node
            .children
            .iter()
            .flatten()
            .any(|child| matches(child, tail)),
        [letter, tail @ ..] => node.children[usize::from(letter - b'a')]
            .as_deref()
            .is_some_and(|child| matches(child, tail)),
    }
}

pub struct WordDictionary {
    root: Node,
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary {
            root: Node::default(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for b in word.bytes() {
            node = node.children[usize::from(b - b'a')].get_or_insert_with(Box::default);
        }
        node.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        matches(&self.root, word.as_bytes())
    }
}
```

Full tested solution: [`src/solutions/p0946_design_add_and_search_words_data_structure.rs`](../src/solutions/p0946_design_add_and_search_words_data_structure.rs)

</details>
