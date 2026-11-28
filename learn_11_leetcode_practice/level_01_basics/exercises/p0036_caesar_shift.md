# Problem 0036: Caesar Shift

- **Difficulty:** Easy
- **Topics:** `char` arithmetic through `u8`, `rem_euclid`, avoiding overflow, `&[char]` to `Vec<char>`
- **Source:** Original · [`i32::rem_euclid`](https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid)

## Problem

The Caesar cipher replaces every letter with the letter `shift` positions later in the alphabet,
wrapping around after `z`. Write two functions:

1. `caesar_shift(c: char, shift: i32) -> char` shifts one character:
   - ASCII lowercase letters stay lowercase and ASCII uppercase letters stay uppercase;
   - after `'z'` comes `'a'` again (and after `'Z'` comes `'A'`);
   - a negative `shift` moves backwards (`'a'` shifted by `-1` is `'z'`);
   - every other character is returned unchanged.
2. `caesar_encode(text: &[char], shift: i32) -> Vec<char>` shifts every character of `text`.

`shift` can be **any** `i32`, including `i32::MIN` and `i32::MAX`.

## Examples

```text
caesar_shift('a', 3)    -> 'd'
caesar_shift('y', 3)    -> 'b'      y -> z -> a -> b
caesar_shift('B', -3)   -> 'Y'
caesar_shift('m', 26)   -> 'm'      a full turn
caesar_shift('!', 5)    -> '!'

caesar_encode(&['H', 'i', '!'], 1) -> ['I', 'j', '!']
```

## Constraints

- `i32::MIN <= shift <= i32::MAX`
- `0 <= text.len() <= 10^5`

## Hints

1. Work with the offset from `'a'` (`0..26`), shift it, and convert back.
2. `shift.rem_euclid(26)` reduces any shift (even a negative one) to `0..26`.
3. Reduce the shift **before** adding it to the offset, or `offset + i32::MAX` overflows.

## Starter code

Write your code in [`src/practice/p0036_caesar_shift.rs`](../src/practice/p0036_caesar_shift.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0036
```

```rust
pub fn caesar_shift(c: char, shift: i32) -> char {
    todo!("Write your solution here")
}

pub fn caesar_encode(text: &[char], shift: i32) -> Vec<char> {
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
    fn simple_shifts() {
        assert_eq!(caesar_shift('a', 3), 'd');
        assert_eq!(caesar_shift('y', 3), 'b');
        assert_eq!(caesar_shift('B', -3), 'Y');
        assert_eq!(caesar_shift('m', 26), 'm');
        assert_eq!(caesar_shift('a', -1), 'z');
    }

    #[test]
    fn non_letters_unchanged() {
        for c in ['!', ' ', '5', 'Ω', '[', '`'] {
            assert_eq!(caesar_shift(c, 5), c);
        }
    }

    #[test]
    fn extreme_shifts_do_not_overflow() {
        // i32::MAX = 2147483647 = 26 * 82595524 + 23, and i32::MIN leaves remainder 2.
        assert_eq!(caesar_shift('a', i32::MAX), 'x');
        assert_eq!(caesar_shift('z', i32::MAX), 'w');
        assert_eq!(caesar_shift('a', i32::MIN), 'c');
        assert_eq!(caesar_shift('Z', i32::MIN), 'B');
    }

    #[test]
    fn encode_text() {
        assert_eq!(caesar_encode(&['H', 'i', '!'], 1), vec!['I', 'j', '!']);
        assert_eq!(caesar_encode(&[], 7), Vec::<char>::new());
    }

    #[test]
    fn shifting_back_decodes() {
        let text: Vec<char> = "Hello, World! xyz ABC".chars().collect();
        for shift in [-100, -27, -1, 0, 1, 13, 25, 1000] {
            let encoded = caesar_encode(&text, shift);
            assert_eq!(caesar_encode(&encoded, -shift), text, "shift {shift}");
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** reduce the shift once with `rem_euclid(26)`, which makes it a value in `0..26`. For a
letter, compute `offset = c as u8 - base`, then `(offset + shift) % 26 + base` and cast to
`char`. Both values are below 26, so their sum is below 52 and fits easily in a `u8`.

**Rust concepts:**
- `rem_euclid` never returns a negative value, so a backwards shift of `-1` becomes `25`.
- The private helper `rotate` takes the base letter as a `u8` (`b'a'` or `b'A'`), so the same
  code handles both cases.
- Building the output: `Vec::with_capacity(n)` then `push` in a `for` loop over `&[char]`.

```rust
fn rotate(c: char, base: u8, shift: u8) -> char {
    let offset = c as u8 - base;
    ((offset + shift) % 26 + base) as char
}

pub fn caesar_shift(c: char, shift: i32) -> char {
    let shift = shift.rem_euclid(26) as u8; // 0..26, so the cast is lossless
    match c {
        'a'..='z' => rotate(c, b'a', shift),
        'A'..='Z' => rotate(c, b'A', shift),
        _ => c,
    }
}

pub fn caesar_encode(text: &[char], shift: i32) -> Vec<char> {
    let mut out = Vec::with_capacity(text.len());
    for &c in text {
        out.push(caesar_shift(c, shift));
    }
    out
}
```

Full tested solution: [`src/solutions/p0036_caesar_shift.rs`](../src/solutions/p0036_caesar_shift.rs)

</details>
