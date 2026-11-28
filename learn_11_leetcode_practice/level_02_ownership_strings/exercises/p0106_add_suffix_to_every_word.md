# Problem 0106: Add a Suffix to Every Word

- **Difficulty:** Easy
- **Topics:** `&mut [String]`, `iter_mut`, `push_str`, `ends_with`, `&str` parameters
- **Source:** Original · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)

## Problem

Implement `add_suffix(words: &mut [String], suffix: &str)`. It modifies the caller's strings
**in place**: every word that does not already end with `suffix` gets `suffix` appended to it.
Words that already end with `suffix` are left unchanged. The order of the words never changes and
no word is added or removed.

An empty `suffix` changes nothing (every string ends with the empty string).

The parameter is a mutable **slice** `&mut [String]` rather than `&mut Vec<String>`: the function
changes the elements but never the length, so it does not need the `Vec` itself. A caller with a
`Vec<String>` simply passes `&mut words`.

## Examples

```text
Input:  words = ["walk", "talked", "jump"], suffix = "ed"
Output: words = ["walked", "talked", "jumped"]

Input:  words = ["file.txt", "notes"], suffix = ".txt"
Output: words = ["file.txt", "notes.txt"]

Input:  words = [], suffix = "s"
Output: words = []
```

## Constraints

- `0 <= words.len() <= 10^4`
- `0 <= words[i].len(), suffix.len() <= 100`

## Hints

1. `words.iter_mut()` yields `&mut String`, one per element.
2. `String::ends_with(suffix)` checks the ending; `push_str(suffix)` appends a `&str`.

## Starter code

Write your code in [`src/practice/p0106_add_suffix_to_every_word.rs`](../src/practice/p0106_add_suffix_to_every_word.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0106
```

```rust
pub fn add_suffix(words: &mut [String], suffix: &str) {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn owned(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| w.to_string()).collect()
    }

    #[test]
    fn example_past_tense() {
        let mut words = owned(&["walk", "talked", "jump"]);
        add_suffix(&mut words, "ed");
        assert_eq!(words, owned(&["walked", "talked", "jumped"]));
    }

    #[test]
    fn example_file_extension() {
        let mut words = owned(&["file.txt", "notes"]);
        add_suffix(&mut words, ".txt");
        assert_eq!(words, owned(&["file.txt", "notes.txt"]));
    }

    #[test]
    fn example_empty_list() {
        let mut words: Vec<String> = Vec::new();
        add_suffix(&mut words, "s");
        assert!(words.is_empty());
    }

    #[test]
    fn empty_suffix_changes_nothing() {
        let mut words = owned(&["a", "", "bc"]);
        add_suffix(&mut words, "");
        assert_eq!(words, owned(&["a", "", "bc"]));
    }

    #[test]
    fn empty_word_gets_the_suffix() {
        let mut words = owned(&["", "s"]);
        add_suffix(&mut words, "s");
        assert_eq!(words, owned(&["s", "s"]));
    }

    #[test]
    fn works_on_part_of_a_vec() {
        let mut words = owned(&["a", "b", "c", "d"]);
        add_suffix(&mut words[1..3], "!");
        assert_eq!(words, owned(&["a", "b!", "c!", "d"]));
    }

    #[test]
    fn large_input() {
        let mut words: Vec<String> = (0..10_000).map(|i| format!("w{i}")).collect();
        add_suffix(&mut words, "0");
        assert_eq!(words[0], "w0");
        assert_eq!(words[1], "w10");
        assert_eq!(words[9_999], "w99990");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** walk over the slice with `iter_mut()` and, for each word, append the suffix unless the
word already ends with it. Each `&mut String` lets us grow that string where it lives.

**Complexity:** `O(total length)` time, no extra space besides the grown strings.

**Rust concepts:**
- `&mut [String]` gives mutable access to each element but not to the length; prefer it over
  `&mut Vec<String>` unless you need `push`/`remove` (Clippy's `ptr_arg` lint says the same).
- `for word in words.iter_mut()` (or `for word in words`) binds `word: &mut String`; method calls
  such as `word.push_str(..)` auto-dereference it.
- `suffix: &str` accepts string literals, `&String` and slices of other strings.

```rust
pub fn add_suffix(words: &mut [String], suffix: &str) {
    for word in words.iter_mut() {
        if !word.ends_with(suffix) {
            word.push_str(suffix);
        }
    }
}
```

Full tested solution: [`src/solutions/p0106_add_suffix_to_every_word.rs`](../src/solutions/p0106_add_suffix_to_every_word.rs)

</details>
