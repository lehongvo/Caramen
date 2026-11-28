# Problem 0108: Fix: Use After a Consuming Loop

- **Difficulty:** Easy
- **Topics:** fix the borrow checker, error E0382, `for x in v` vs `for x in &v`
- **Source:** Original · [E0382](https://doc.rust-lang.org/error_codes/E0382.html) · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Problem

This function should return the total length of all words and the longest word, but it does not
compile:

```text
pub fn total_and_longest(words: Vec<String>) -> (usize, String) {
    let mut total = 0;
    for word in words {
        total += word.len();
    }
    let mut longest = String::new();
    for word in words {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    (total, longest)
}

error[E0382]: use of moved value: `words`
  |     for word in words {
  |                 ----- `words` moved due to this implicit call to `.into_iter()`
  ...
  |     for word in words {
  |                 ^^^^^ value used here after move
```

Implement the corrected `total_and_longest(words: Vec<String>) -> (usize, String)`:

- the first value is the sum of `word.len()` over all words;
- the second value is the longest word (the **first** one if several have the maximum length),
  or an empty `String` if `words` is empty.

Keep the signature, and do not clone any word: the longest word can be moved out of `words`.

## Examples

```text
Input:  words = ["red", "green", "blue"]
Output: (12, "green")

Input:  words = ["ab", "cd"]
Output: (4, "ab")       (tie: the first longest word wins)

Input:  words = []
Output: (0, "")
```

## Constraints

- `0 <= words.len() <= 10^4`
- Words are ASCII.

## Hints

1. `for word in words` calls `into_iter()` and consumes the vector. Which of the two loops really
   needs to own the words?
2. `for word in &words` borrows the vector and yields `&String`, leaving `words` usable afterwards.

## Starter code

Write your code in [`src/practice/p0108_fix_use_after_consuming_loop.rs`](../src/practice/p0108_fix_use_after_consuming_loop.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0108
```

```rust
pub fn total_and_longest(words: Vec<String>) -> (usize, String) {
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
    fn example_colors() {
        assert_eq!(total_and_longest(owned(&["red", "green", "blue"])), (12, "green".to_string()));
    }

    #[test]
    fn example_tie_keeps_first() {
        assert_eq!(total_and_longest(owned(&["ab", "cd"])), (4, "ab".to_string()));
    }

    #[test]
    fn example_empty() {
        assert_eq!(total_and_longest(Vec::new()), (0, String::new()));
    }

    #[test]
    fn only_empty_words() {
        assert_eq!(total_and_longest(owned(&["", ""])), (0, String::new()));
    }

    #[test]
    fn longest_at_the_end() {
        assert_eq!(total_and_longest(owned(&["a", "bb", "ccc"])), (6, "ccc".to_string()));
    }

    #[test]
    fn large_input() {
        let mut words: Vec<String> = (0..10_000).map(|_| "abc".to_string()).collect();
        words[7_000] = "abcdef".to_string();
        assert_eq!(total_and_longest(words), (30_003, "abcdef".to_string()));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the first loop only reads lengths, so it can borrow (`for word in &words`). The second
loop moves the winning word into `longest`, so it keeps consuming the vector. The strict `>`
keeps the first word of maximum length.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `for x in v` uses `IntoIterator for Vec<T>` and moves `v`; `for x in &v` uses
  `IntoIterator for &Vec<T>` and yields `&T`; `for x in &mut v` yields `&mut T`.
- Choose the weakest access each loop needs: borrow to read, consume only when you move out.
- Reassigning `longest = word` drops the previous `String` stored in `longest` automatically.

```rust
pub fn total_and_longest(words: Vec<String>) -> (usize, String) {
    let mut total = 0;
    // Only reading here, so borrow instead of consuming.
    for word in &words {
        total += word.len();
    }
    let mut longest = String::new();
    // This loop moves the winner out, so it may consume the vector.
    for word in words {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    (total, longest)
}
```

Full tested solution: [`src/solutions/p0108_fix_use_after_consuming_loop.rs`](../src/solutions/p0108_fix_use_after_consuming_loop.rs)

</details>
