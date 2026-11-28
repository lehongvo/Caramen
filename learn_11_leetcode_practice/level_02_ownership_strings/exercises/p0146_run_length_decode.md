# Problem 0146: Run-Length Decode

- **Difficulty:** Easy
- **Topics:** `chars()`, `char::to_digit`, accumulating a number, `Option::take`, `push` in a loop
- **Source:** Original · [`char::to_digit`](https://doc.rust-lang.org/std/primitive.char.html#method.to_digit) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)

## Problem

Run-length encoding writes repeated characters as a count followed by the character. Implement
the decoder `run_length_decode(encoded: &str) -> String`.

The input is a sequence of groups. Each group is an **optional** count, made of one or more ASCII
digits (`0`-`9`, possibly several, such as `12`), followed by exactly one character that is not
an ASCII digit. The group stands for that character repeated `count` times; a group without a
count stands for the character once. A count of `0` produces nothing.

Any non-digit character can be encoded, including spaces and non-ASCII characters.

## Examples

```text
Input:  encoded = "3a2bc"
Output: "aaabbc"

Input:  encoded = "12x"
Output: "xxxxxxxxxxxx"

Input:  encoded = "a2ü3 0q!"
Output: "aüü   !"        (three spaces; "0q" produces nothing)

Input:  encoded = ""
Output: ""
```

## Constraints

- `0 <= encoded.len() <= 10^4`
- `encoded` is valid: it never ends with a digit, and the decoded text has at most `10^5` chars.

## Hints

1. Iterate with `chars()` (not bytes), so that `'ü'` is one character.
2. `c.to_digit(10)` returns `Some(d)` for `'0'`-`'9'`; build multi-digit counts with
   `count = count * 10 + d`.
3. Store the pending count as an `Option<usize>`: `None` means "no count written, use 1".

## Starter code

Write your code in [`src/practice/p0146_run_length_decode.rs`](../src/practice/p0146_run_length_decode.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0146
```

```rust
pub fn run_length_decode(encoded: &str) -> String {
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

    #[test]
    fn example_simple_groups() {
        assert_eq!(run_length_decode("3a2bc"), "aaabbc");
    }

    #[test]
    fn example_multi_digit_count() {
        assert_eq!(run_length_decode("12x"), "x".repeat(12));
    }

    #[test]
    fn example_unicode_spaces_and_zero() {
        assert_eq!(run_length_decode("a2ü3 0q!"), "aüü   !");
    }

    #[test]
    fn example_empty() {
        assert_eq!(run_length_decode(""), "");
    }

    #[test]
    fn count_resets_after_each_group() {
        assert_eq!(run_length_decode("2ab"), "aab");
        assert_eq!(run_length_decode("10a1b"), "aaaaaaaaaab");
        assert_eq!(run_length_decode("2🦀"), "🦀🦀");
    }

    #[test]
    fn large_output() {
        let result = run_length_decode("50000a50000b");
        assert_eq!(result.len(), 100_000);
        assert!(result[..50_000].bytes().all(|b| b == b'a'));
        assert!(result[50_000..].bytes().all(|b| b == b'b'));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** read characters one by one. Digits extend the pending count. Any other character
closes a group: emit it `count` times (or once if no count was written) and reset the count.
`Option::take()` reads the pending count and resets it to `None` in one step.

**Complexity:** `O(n + m)` time for input length `n` and output length `m`; `O(m)` space.

**Rust concepts:**
- `char::to_digit(10)` converts an ASCII digit to its value and returns `None` for anything else.
- `Option::take(&mut self)` moves the value out and leaves `None` behind, like `mem::take`.
- `String::push(char)` encodes the character as UTF-8, so multi-byte characters just work.

```rust
pub fn run_length_decode(encoded: &str) -> String {
    let mut out = String::new();
    let mut count: Option<usize> = None;
    for c in encoded.chars() {
        match c.to_digit(10) {
            Some(d) => count = Some(count.unwrap_or(0) * 10 + d as usize),
            None => {
                for _ in 0..count.take().unwrap_or(1) {
                    out.push(c);
                }
            }
        }
    }
    out
}
```

Full tested solution: [`src/solutions/p0146_run_length_decode.rs`](../src/solutions/p0146_run_length_decode.rs)

</details>
