# Problem 0261: UTF-8 Validation

- **Difficulty:** Medium
- **Topics:** binary literal range patterns, an `enum` state machine, `match` on `(state, byte)`, `fold`-like loop
- **Source:** [LeetCode 393. UTF-8 Validation](https://leetcode.com/problems/utf-8-validation/)

## Problem

In UTF-8, one character is encoded as 1 to 4 bytes, recognised by their leading bits:

```text
1 byte:  0xxxxxxx
2 bytes: 110xxxxx 10xxxxxx
3 bytes: 1110xxxx 10xxxxxx 10xxxxxx
4 bytes: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
```

Every byte after the first one of a character (a *continuation byte*) starts with `10`. You are
given `data`, a list of integers where only the lowest 8 bits of each element are used (each
element is one byte). Return `true` if `data` is a sequence of correctly formed characters by the
table above: every leading byte is followed by exactly the right number of continuation bytes,
and nothing is left unfinished at the end. (Only this bit structure is checked.)

Implement `Solution::valid_utf8(data: Vec<i32>) -> bool`.

## Examples

```text
Input:  data = [197, 130, 1]
Output: true      (11000101 10000010 = one 2-byte character, then 00000001)

Input:  data = [235, 140, 4]
Output: false     (11101011 starts a 3-byte character, but 00000100 is not a continuation byte)

Input:  data = [240, 162, 138, 147]
Output: true      (one 4-byte character)
```

## Constraints

- `1 <= data.len() <= 2 * 10^4`
- `0 <= data[i] <= 255`

## Hints

1. Range patterns work with binary literals: `0b1100_0000..=0b1101_1111` are exactly the bytes
   starting with `110`.
2. Keep a state: "expecting a new character" or "expecting `n` more continuation bytes".

## Starter code

Write your code in [`src/practice/p0261_utf_8_validation.rs`](../src/practice/p0261_utf_8_validation.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0261
```

```rust
pub struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
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
    fn example_two_byte_then_ascii() {
        assert!(Solution::valid_utf8(vec![197, 130, 1]));
    }

    #[test]
    fn example_broken_three_byte() {
        assert!(!Solution::valid_utf8(vec![235, 140, 4]));
    }

    #[test]
    fn example_four_byte() {
        assert!(Solution::valid_utf8(vec![240, 162, 138, 147]));
    }

    #[test]
    fn stray_continuation_and_bad_leaders() {
        assert!(!Solution::valid_utf8(vec![145]));
        assert!(!Solution::valid_utf8(vec![0b1111_1000, 128, 128, 128, 128]));
        assert!(!Solution::valid_utf8(vec![0b1110_0000, 128]));
    }

    #[test]
    fn real_utf8_strings() {
        let text = "Grüße, мир, 世界, 🦀";
        let data: Vec<i32> = text.bytes().map(i32::from).collect();
        assert!(Solution::valid_utf8(data.clone()));
        let mut cut = data;
        cut.pop();
        assert!(!Solution::valid_utf8(cut));
    }

    #[test]
    fn long_ascii() {
        assert!(Solution::valid_utf8(vec![65; 20_000]));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a two-state machine. In `Start`, the byte's leading bits say how many continuation
bytes must follow; in `Continue(n)`, the byte must be a continuation byte and `n` decreases.
The input is valid if no byte is rejected and the machine ends in `Start`.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- Each leading-bit class is a contiguous range of byte values, so it can be written as a range
  pattern with binary literals; `_` separators make the bit groups readable.
- Matching on the tuple `(state, byte)` lists the legal transitions; the final `_ => return false`
  rejects everything else.
- `State::Continue(1)` returning to `State::Start` is its own arm, before the general
  `State::Continue(n)` arm.
- `(x & 0xFF) as u8` keeps the lowest 8 bits, as the statement asks.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    /// Waiting for the first byte of a character.
    Start,
    /// This many continuation bytes are still expected.
    Continue(u8),
}

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut state = State::Start;
        for x in data {
            let byte = (x & 0xFF) as u8;
            state = match (state, byte) {
                (State::Start, 0b0000_0000..=0b0111_1111) => State::Start,
                (State::Start, 0b1100_0000..=0b1101_1111) => State::Continue(1),
                (State::Start, 0b1110_0000..=0b1110_1111) => State::Continue(2),
                (State::Start, 0b1111_0000..=0b1111_0111) => State::Continue(3),
                (State::Continue(1), 0b1000_0000..=0b1011_1111) => State::Start,
                (State::Continue(n), 0b1000_0000..=0b1011_1111) => State::Continue(n - 1),
                _ => return false,
            };
        }
        state == State::Start
    }
}
```

Full tested solution: [`src/solutions/p0261_utf_8_validation.rs`](../src/solutions/p0261_utf_8_validation.rs)

</details>
