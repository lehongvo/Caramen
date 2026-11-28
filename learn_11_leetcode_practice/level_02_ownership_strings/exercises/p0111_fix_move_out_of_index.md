# Problem 0111: Fix: Moving Out of an Index

- **Difficulty:** Easy
- **Topics:** fix the borrow checker, error E0507, keeping a `&String`, cloning once at the end
- **Source:** Original · [E0507](https://doc.rust-lang.org/error_codes/E0507.html) · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Problem

This function should return an owned copy of the longest word, but it does not compile:

```text
pub fn longest_owned(words: &Vec<String>) -> String {
    let mut best = words[0];
    for word in words {
        if word.len() > best.len() {
            best = *word;
        }
    }
    best
}

error[E0507]: cannot move out of index of `Vec<String>`
  |     let mut best = words[0];
  |                    ^^^^^^^^ move occurs because value has type `String`,
  |                             which does not implement the `Copy` trait
error[E0507]: cannot move out of `*word` which is behind a shared reference
```

The function only **borrows** the words, so it may not move any `String` out of them. It would
also panic on an empty vector.

Implement the corrected `longest_owned(words: &[String]) -> String`. It returns a new `String`
equal to the longest word (largest `len()`); on ties, the **first** longest word; for an empty
slice, an empty `String`. The caller's words must stay untouched.

## Examples

```text
Input:  words = ["pear", "banana", "fig"]
Output: "banana"

Input:  words = ["one", "two", "six"]
Output: "one"

Input:  words = []
Output: ""
```

## Constraints

- `0 <= words.len() <= 10^4`
- Words are ASCII.

## Hints

1. While searching, remember the best word as a **reference** (`&String` or `&str`); references
   are `Copy`, so `best = word` is fine.
2. Convert to an owned `String` only once, after the loop (`.clone()` or `.to_string()`).

## Starter code

Write your code in [`src/practice/p0111_fix_move_out_of_index.rs`](../src/practice/p0111_fix_move_out_of_index.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0111
```

```rust
pub fn longest_owned(words: &[String]) -> String {
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
    fn example_fruits() {
        assert_eq!(longest_owned(&owned(&["pear", "banana", "fig"])), "banana");
    }

    #[test]
    fn example_tie_keeps_first() {
        assert_eq!(longest_owned(&owned(&["one", "two", "six"])), "one");
    }

    #[test]
    fn example_empty() {
        assert_eq!(longest_owned(&[]), "");
    }

    #[test]
    fn caller_keeps_its_words() {
        let words = owned(&["a", "abc", "ab"]);
        let longest = longest_owned(&words);
        assert_eq!(longest, "abc");
        assert_eq!(words, owned(&["a", "abc", "ab"]));
    }

    #[test]
    fn result_is_an_independent_copy() {
        let words = owned(&["hello"]);
        let mut longest = longest_owned(&words);
        longest.push('!');
        assert_eq!(words[0], "hello");
        assert_eq!(longest, "hello!");
    }

    #[test]
    fn large_input() {
        let mut words: Vec<String> = (0..10_000).map(|i| "x".repeat(i % 10)).collect();
        words[4_321] = "y".repeat(10);
        assert_eq!(longest_owned(&words), "y".repeat(10));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** start with `best: &str = ""` and update it with any strictly longer word. All the
comparisons work on borrowed data; one allocation at the end produces the owned result.

**Complexity:** `O(n)` time; `O(L)` space for the returned copy of length `L`.

**Rust concepts:**
- `words[0]` is a place inside the vector; you can read it or borrow it (`&words[0]`) but you
  cannot move out of it, because the vector would be left with an invalid element.
- Shared references are `Copy`: assigning `best = word` copies the pointer, not the text.
- Starting from `""` (a `&'static str`) handles the empty input without indexing or panicking.
- Prefer `&[String]` over `&Vec<String>` for read-only parameters.

```rust
pub fn longest_owned(words: &[String]) -> String {
    let mut best: &str = "";
    for word in words {
        if word.len() > best.len() {
            best = word;
        }
    }
    best.to_string()
}
```

Full tested solution: [`src/solutions/p0111_fix_move_out_of_index.rs`](../src/solutions/p0111_fix_move_out_of_index.rs)

</details>
