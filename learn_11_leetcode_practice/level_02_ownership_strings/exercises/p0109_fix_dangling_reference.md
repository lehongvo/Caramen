# Problem 0109: Fix: Returning a Reference to a Local

- **Difficulty:** Easy
- **Topics:** fix the borrow checker, error E0106, dangling references, returning `String`, `format!`
- **Source:** Original · [E0106](https://doc.rust-lang.org/error_codes/E0106.html) · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Problem

This function tries to build a label such as `"item-007"` and return a reference to it:

```text
pub fn make_label(n: u32) -> &str {
    let label = format!("item-{n:03}");
    &label
}

error[E0106]: missing lifetime specifier
  | pub fn make_label(n: u32) -> &str {
  |                              ^ expected named lifetime parameter
  = help: this function's return type contains a borrowed value, but there is
          no value for it to be borrowed from
```

`label` is dropped when the function returns, so a reference to it would point to freed memory
(a *dangling reference*). No lifetime annotation can fix that: the data must outlive the call.

Implement the corrected `make_label(n: u32) -> String`. It returns `"item-"` followed by `n`
written in decimal and padded with leading zeros to **at least** 3 digits. Numbers with more than
3 digits are written in full.

## Examples

```text
Input:  n = 7
Output: "item-007"

Input:  n = 42
Output: "item-042"

Input:  n = 12345
Output: "item-12345"
```

## Constraints

- `0 <= n <= u32::MAX`

## Hints

1. Return the owned `String` itself: ownership moves to the caller, so nothing dangles.
2. In `format!`, `{n:03}` means "width 3, pad with zeros".

## Starter code

Write your code in [`src/practice/p0109_fix_dangling_reference.rs`](../src/practice/p0109_fix_dangling_reference.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0109
```

```rust
pub fn make_label(n: u32) -> String {
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
    fn example_one_digit() {
        assert_eq!(make_label(7), "item-007");
    }

    #[test]
    fn example_two_digits() {
        assert_eq!(make_label(42), "item-042");
    }

    #[test]
    fn example_many_digits() {
        assert_eq!(make_label(12_345), "item-12345");
    }

    #[test]
    fn zero_and_three_digits() {
        assert_eq!(make_label(0), "item-000");
        assert_eq!(make_label(999), "item-999");
    }

    #[test]
    fn largest_value() {
        assert_eq!(make_label(u32::MAX), "item-4294967295");
    }

    #[test]
    fn labels_outlive_the_calls() {
        let labels: Vec<String> = (1..=3).map(make_label).collect();
        assert_eq!(labels, vec!["item-001", "item-002", "item-003"]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a function that *creates* text must return an owned `String`. `format!` builds it and
the return moves it to the caller.

**Rust concepts:**
- A returned reference must borrow from one of the inputs (or be `'static`). With no reference
  parameter there is nothing to borrow from, which is what E0106 is telling you here.
- Returning `String` transfers ownership, so the caller decides when the text is dropped.
- Format specs: `{:03}` pads to width 3 with zeros; wider values are never truncated.

```rust
pub fn make_label(n: u32) -> String {
    format!("item-{n:03}")
}
```

Full tested solution: [`src/solutions/p0109_fix_dangling_reference.rs`](../src/solutions/p0109_fix_dangling_reference.rs)

</details>
