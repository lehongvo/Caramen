# Problem 0475: Run-Length Encoding with peekable

- **Difficulty:** Medium
- **Topics:** `Iterator::peekable`, `Peekable::next_if_eq`, `Peekable::next_if`, `while let`, decoding with errors
- **Source:** Original · [std `Peekable`](https://doc.rust-lang.org/std/iter/struct.Peekable.html) · [std `Peekable::next_if`](https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.next_if)

## Problem

*Run-length encoding* replaces each run of equal characters by its length and the character:
`"aaabccdddd"` becomes `"3a1b2c4d"`. Grouping a run needs to look at the next character
**without consuming it** when it is different. `peekable()` adds exactly that ability to any
iterator: `peek()` looks ahead, and `next_if_eq(&x)` / `next_if(pred)` consume the next item
only when it matches.

Implement:

- `encode(s: &str) -> String`. The input never contains ASCII digits.
- `decode(s: &str) -> Result<String, String>`: the inverse. A count is one or more digits and is
  followed by the character to repeat. Errors (first problem wins):
  - a character without a count before it -> ``"missing count before `C`"``;
  - a count of zero -> `"zero count"`;
  - digits at the end -> ``"missing character after `N`"`` (`N` = the digits as written);
  - a count that does not fit in a `usize` -> `"count too large"`.
- `runs(nums: &[i32]) -> Vec<(i32, usize)>`: the runs of a slice as `(value, length)` pairs.

## Examples

```text
encode("aaabccdddd")   -> "3a1b2c4d"
encode("")             -> ""
decode("3a1b2c4d")     -> Ok("aaabccdddd")
decode("12x")          -> Ok("xxxxxxxxxxxx")
decode("a")            -> Err("missing count before `a`")
decode("0a")           -> Err("zero count")
decode("3a12")         -> Err("missing character after `12`")
runs(&[5, 5, 1, 5, 5, 5]) -> [(5, 2), (1, 1), (5, 3)]
```

## Constraints

- Inputs have at most `10^4` characters; decoded outputs stay below `10^6` characters in the tests.

## Hints

1. `let mut it = s.chars().peekable(); while let Some(c) = it.next() { ... }` handles one run per
   loop iteration; inside, `while it.next_if_eq(&c).is_some() { count += 1; }`.
2. To read a number, collect digits with `next_if(|c| c.is_ascii_digit())` in a loop.
3. `str::repeat` or `iter::repeat(c).take(n)` produces the decoded run.

## Starter code

Write your code in [`src/practice/p0475_run_length_encoding_peekable.rs`](../src/practice/p0475_run_length_encoding_peekable.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0475
```

```rust
pub fn encode(s: &str) -> String {
    todo!("Write your solution here")
}

pub fn decode(s: &str) -> Result<String, String> {
    todo!("Write your solution here")
}

pub fn runs(nums: &[i32]) -> Vec<(i32, usize)> {
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
    fn encode_runs() {
        assert_eq!(encode("aaabccdddd"), "3a1b2c4d");
        assert_eq!(encode(""), "");
        assert_eq!(encode("abc"), "1a1b1c");
        assert_eq!(encode(&"z".repeat(12)), "12z");
        assert_eq!(encode("üüx"), "2ü1x");
    }

    #[test]
    fn decode_valid() {
        assert_eq!(decode("3a1b2c4d"), Ok("aaabccdddd".to_string()));
        assert_eq!(decode("12x"), Ok("x".repeat(12)));
        assert_eq!(decode(""), Ok(String::new()));
        assert_eq!(decode("2 1-"), Ok("  -".to_string()));
    }

    #[test]
    fn decode_errors() {
        assert_eq!(decode("a"), Err("missing count before `a`".to_string()));
        assert_eq!(decode("2ab"), Err("missing count before `b`".to_string()));
        assert_eq!(decode("0a"), Err("zero count".to_string()));
        assert_eq!(
            decode("3a12"),
            Err("missing character after `12`".to_string())
        );
        assert_eq!(
            decode("99999999999999999999999x"),
            Err("count too large".to_string())
        );
    }

    #[test]
    fn round_trip() {
        for s in ["aaabccdddd", "x", "abababab", "  ..!!", "mississippi"] {
            assert_eq!(decode(&encode(s)), Ok(s.to_string()));
        }
    }

    #[test]
    fn runs_of_numbers() {
        assert_eq!(runs(&[5, 5, 1, 5, 5, 5]), vec![(5, 2), (1, 1), (5, 3)]);
        assert_eq!(runs(&[]), vec![]);
        assert_eq!(runs(&[7]), vec![(7, 1)]);
        let long = vec![3; 10_000];
        assert_eq!(runs(&long), vec![(3, 10_000)]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** process the input one *group* at a time. `next()` starts a group and `next_if_eq` /
`next_if` keep extending it as long as the next item belongs to it; the first item that does not
belong stays in the iterator for the next group.

**Rust concepts:**
- `Peekable` stores at most one look-ahead item; `next_if_eq` (Rust 1.51) is "peek, compare,
  consume if equal" in one call.
- `while let Some(x) = it.next()` is the idiomatic manual-iteration loop; it lets the loop body
  advance the same iterator further.
- Parsing the digits with `parse::<usize>()` reports overflow for free.
- `slice::chunk_by` (Rust 1.77) is another way to get the runs of a slice:
  `nums.chunk_by(|a, b| a == b)`.

```rust
pub fn encode(s: &str) -> String {
    let mut it = s.chars().peekable();
    let mut out = String::new();
    while let Some(c) = it.next() {
        let mut count = 1;
        while it.next_if_eq(&c).is_some() {
            count += 1;
        }
        out.push_str(&count.to_string());
        out.push(c);
    }
    out
}

pub fn decode(s: &str) -> Result<String, String> {
    let mut it = s.chars().peekable();
    let mut out = String::new();
    while it.peek().is_some() {
        let mut digits = String::new();
        while let Some(d) = it.next_if(|c| c.is_ascii_digit()) {
            digits.push(d);
        }
        let Some(c) = it.next() else {
            return Err(format!("missing character after `{digits}`"));
        };
        if digits.is_empty() {
            return Err(format!("missing count before `{c}`"));
        }
        let count: usize = digits.parse().map_err(|_| "count too large".to_string())?;
        if count == 0 {
            return Err("zero count".to_string());
        }
        out.extend(std::iter::repeat_n(c, count));
    }
    Ok(out)
}

pub fn runs(nums: &[i32]) -> Vec<(i32, usize)> {
    let mut it = nums.iter().peekable();
    let mut out = Vec::new();
    while let Some(&x) = it.next() {
        let mut len = 1;
        while it.next_if(|&&y| y == x).is_some() {
            len += 1;
        }
        out.push((x, len));
    }
    out
}
```

Full tested solution: [`src/solutions/p0475_run_length_encoding_peekable.rs`](../src/solutions/p0475_run_length_encoding_peekable.rs)

</details>
