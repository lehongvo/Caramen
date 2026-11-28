# Problem 0165: Trim Each Line

- **Difficulty:** Easy
- **Topics:** `lines()`, `trim`, returning `Vec<&str>` that borrows from the input
- **Source:** Original · [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines) · [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim)

## Problem

Implement `trim_lines(text: &str) -> Vec<&str>`, which splits `text` into lines, removes the
leading and trailing whitespace of every line, and drops the lines that become empty. The
remaining lines are returned in their original order.

Lines are separated by `"\n"` or `"\r\n"`. Every returned `&str` must be a slice of `text`: the
function copies no text at all.

## Examples

```text
Input:  text = "  first line  \n\n\tsecond\n   \nthird"
Output: ["first line", "second", "third"]

Input:  text = "a\r\nb\r\n"
Output: ["a", "b"]

Input:  text = ""
Output: []
```

## Constraints

- `0 <= text.len() <= 10^5`

## Hints

1. `text.lines()` yields every line without its line terminator (`"\n"` or `"\r\n"`).
2. `trim()` returns a sub-slice; `filter(|line| !line.is_empty())` drops blank lines.

## Starter code

Write your code in [`src/practice/p0165_trim_each_line.rs`](../src/practice/p0165_trim_each_line.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0165
```

```rust
pub fn trim_lines(text: &str) -> Vec<&str> {
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
    fn example_mixed_lines() {
        assert_eq!(
            trim_lines("  first line  \n\n\tsecond\n   \nthird"),
            vec!["first line", "second", "third"]
        );
    }

    #[test]
    fn example_windows_line_endings() {
        assert_eq!(trim_lines("a\r\nb\r\n"), vec!["a", "b"]);
    }

    #[test]
    fn example_empty() {
        assert!(trim_lines("").is_empty());
    }

    #[test]
    fn only_blank_lines() {
        assert!(trim_lines("\n \n\t\n").is_empty());
    }

    #[test]
    fn results_point_into_the_text() {
        let text = String::from("  alpha\nbeta  ");
        let lines = trim_lines(&text);
        assert_eq!(lines, vec!["alpha", "beta"]);
        assert_eq!(lines[0].as_ptr(), text[2..].as_ptr());
        assert_eq!(lines[1].as_ptr(), text[8..].as_ptr());
    }

    #[test]
    fn large_input() {
        let text = "  item  \n\n".repeat(10_000);
        let lines = trim_lines(&text);
        assert_eq!(lines.len(), 10_000);
        assert!(lines.iter().all(|&l| l == "item"));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** chain three borrowed views: `lines()` slices the text into lines, `trim()` slices each
line again, and `filter` keeps the non-empty ones. Collecting gives a `Vec<&str>` of pointers into
the original text.

**Complexity:** `O(n)` time; `O(k)` space for the `k` returned slices.

**Rust concepts:**
- The signature `fn(&str) -> Vec<&str>` means every element borrows from `text` (lifetime
  elision); the caller must keep `text` alive while using the result.
- `str::lines` handles both `"\n"` and `"\r\n"`, and does not produce a final empty line after a
  trailing newline.
- Chained iterator adapters are lazy: nothing happens until `collect`.

```rust
pub fn trim_lines(text: &str) -> Vec<&str> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}
```

Full tested solution: [`src/solutions/p0165_trim_each_line.rs`](../src/solutions/p0165_trim_each_line.rs)

</details>
