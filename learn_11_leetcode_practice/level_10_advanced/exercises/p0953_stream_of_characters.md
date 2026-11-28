# Problem 0953: Stream of Characters

- **Difficulty:** Hard
- **Topics:** a trie of **reversed** words, an arena of nodes in a `Vec` with `u32` child indices, bounded history buffer
- **Source:** [LeetCode 1032. Stream of Characters](https://leetcode.com/problems/stream-of-characters/)

## Problem

Letters arrive one at a time. After each letter, report whether some **suffix** of the letters
received so far is one of the given words.

Implement the `StreamChecker` struct:

- `StreamChecker::new(words: Vec<String>) -> Self`.
- `query(&mut self, letter: char) -> bool`: appends `letter` to the stream and returns `true` if a
  suffix of the stream (ending with this letter) is in `words`.

## Examples

```text
let mut s = StreamChecker::new(vec!["abc", "xyz"]);
s.query('a') => false
s.query('x') => false
s.query('y') => false
s.query('z') => true      ("xyz" is a suffix of "axyz")
```

## Constraints

- `1 <= words.len() <= 2000`, `1 <= words[i].len() <= 200`
- Words and letters are lowercase English letters.
- At most `4 * 10^4` calls to `query`.

## Hints

1. Insert every word **reversed** into a trie.
2. On each query, walk the trie from the newest letter backwards through the stream; stop at the
   first node that ends a word (return `true`) or when there is no child.
3. You never need more history than the longest word: trim the buffer from time to time.

## Starter code

Write your code in [`src/practice/p0953_stream_of_characters.rs`](../src/practice/p0953_stream_of_characters.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0953
```

```rust
pub struct StreamChecker {}

impl StreamChecker {
    pub fn new(words: Vec<String>) -> Self {
        todo!("Write your solution here")
    }

    pub fn query(&mut self, letter: char) -> bool {
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

    fn checker(words: &[&str]) -> StreamChecker {
        StreamChecker::new(words.iter().map(|w| w.to_string()).collect())
    }

    fn feed(checker: &mut StreamChecker, letters: &str) -> Vec<bool> {
        letters.chars().map(|c| checker.query(c)).collect()
    }

    #[test]
    fn example() {
        let mut s = checker(&["abc", "xyz"]);
        assert_eq!(feed(&mut s, "axyz"), [false, false, false, true]);
    }

    #[test]
    fn longer_example() {
        let mut s = checker(&["cd", "f", "kl"]);
        assert_eq!(
            feed(&mut s, "abcdefghijkl"),
            [
                false, false, false, true, false, true, false, false, false, false, false, true
            ]
        );
    }

    #[test]
    fn words_that_are_suffixes_of_each_other() {
        let mut s = checker(&["ab", "b", "aab"]);
        assert_eq!(feed(&mut s, "aab"), [false, false, true]);
        assert_eq!(feed(&mut s, "c"), [false]);
    }

    #[test]
    fn long_stream_keeps_working_after_trimming() {
        let mut s = checker(&["needle"]);
        let haystack = "x".repeat(10_000) + "needle" + &"y".repeat(10_000) + "needle";
        let hits: Vec<usize> = feed(&mut s, &haystack)
            .into_iter()
            .enumerate()
            .filter(|&(_, hit)| hit)
            .map(|(i, _)| i)
            .collect();
        assert_eq!(hits, vec![10_005, 20_011]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a suffix of the stream that equals a word is that word reversed read from the end of the
stream, so the reversed-word trie turns the question into one walk of at most `max_len` steps.
The nodes live in a `Vec` (an arena) and refer to children by index, which keeps the structure
compact and avoids recursive drops. The history buffer is trimmed to the last `max_len` letters
whenever it grows to twice that size, so trimming costs amortised `O(1)`.

**Complexity:** building is `O(total letters)`; each query is `O(max_len)`.

**Rust concepts:**
- An arena `Vec<Node>` with indices is a common alternative to `Box`-linked trees.
- `0` can mean "no child" because the root (index 0) is never anybody's child.
- `Vec::drain(..n)` removes a prefix of the buffer in one call.

```rust
struct Node {
    children: [u32; 26],
    is_word: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            children: [0; 26],
            is_word: false,
        }
    }
}

pub struct StreamChecker {
    nodes: Vec<Node>,
    stream: Vec<u8>,
    max_len: usize,
}

impl StreamChecker {
    pub fn new(words: Vec<String>) -> Self {
        let mut nodes = vec![Node::new()];
        let mut max_len = 0;
        for word in &words {
            max_len = max_len.max(word.len());
            let mut current = 0;
            for b in word.bytes().rev() {
                let slot = usize::from(b - b'a');
                if nodes[current].children[slot] == 0 {
                    nodes.push(Node::new());
                    nodes[current].children[slot] = (nodes.len() - 1) as u32;
                }
                current = nodes[current].children[slot] as usize;
            }
            nodes[current].is_word = true;
        }
        StreamChecker {
            nodes,
            stream: Vec::new(),
            max_len,
        }
    }

    pub fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter as u8);
        if self.stream.len() >= 2 * self.max_len.max(1) {
            let excess = self.stream.len() - self.max_len;
            self.stream.drain(..excess);
        }
        let mut current = 0;
        for &b in self.stream.iter().rev().take(self.max_len) {
            let next = self.nodes[current].children[usize::from(b - b'a')];
            if next == 0 {
                return false;
            }
            current = next as usize;
            if self.nodes[current].is_word {
                return true;
            }
        }
        false
    }
}
```

Full tested solution: [`src/solutions/p0953_stream_of_characters.rs`](../src/solutions/p0953_stream_of_characters.rs)

</details>
