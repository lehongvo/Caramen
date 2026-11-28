# Problem 0105: Keep the Shorter String First

- **Difficulty:** Easy
- **Topics:** two `&mut String` parameters, `std::mem::swap`, comparing through references
- **Source:** Original · [`std::mem::swap`](https://doc.rust-lang.org/std/mem/fn.swap.html) · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Problem

Implement `shorter_first(a: &mut String, b: &mut String)`. After the call, `a` must hold the
shorter of the two strings and `b` the longer one (lengths are measured with `len()`). If both
have the same length, leave them as they are.

The function returns nothing: it works only through the two mutable references. You may not
clone either string. Exchange their contents instead, so that no text is copied.

The caller passes two different variables, so the two `&mut` references never point to the same
`String` (the borrow checker would reject `shorter_first(&mut x, &mut x)`).

## Examples

```text
Input:  a = "elephant", b = "cat"
Output: a = "cat", b = "elephant"

Input:  a = "ox", b = "zebra"
Output: a = "ox", b = "zebra"     (already in order)

Input:  a = "dog", b = "cat"
Output: a = "dog", b = "cat"      (same length: unchanged)
```

## Constraints

- `0 <= a.len(), b.len() <= 10^4`
- Do not call `.clone()`.

## Hints

1. Method calls auto-dereference, so `a.len()` works on a `&mut String`.
2. `let tmp = *a;` fails (you cannot move out of a reference). `std::mem::swap(a, b)` exchanges
   the two values behind the references in one step.

## Starter code

Write your code in [`src/practice/p0105_shorter_string_first.rs`](../src/practice/p0105_shorter_string_first.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0105
```

```rust
pub fn shorter_first(a: &mut String, b: &mut String) {
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

    fn run(a: &str, b: &str) -> (String, String) {
        let mut a = a.to_string();
        let mut b = b.to_string();
        shorter_first(&mut a, &mut b);
        (a, b)
    }

    #[test]
    fn example_longer_first_gets_swapped() {
        assert_eq!(run("elephant", "cat"), ("cat".into(), "elephant".into()));
    }

    #[test]
    fn example_already_in_order() {
        assert_eq!(run("ox", "zebra"), ("ox".into(), "zebra".into()));
    }

    #[test]
    fn example_same_length_unchanged() {
        assert_eq!(run("dog", "cat"), ("dog".into(), "cat".into()));
    }

    #[test]
    fn empty_strings() {
        assert_eq!(run("abc", ""), ("".into(), "abc".into()));
        assert_eq!(run("", ""), ("".into(), "".into()));
    }

    #[test]
    fn heap_buffers_are_exchanged_not_copied() {
        let mut a = "long text here".to_string();
        let mut b = "short".to_string();
        let (pa, pb) = (a.as_ptr(), b.as_ptr());
        shorter_first(&mut a, &mut b);
        // The buffers moved between the two variables; no new allocation was made.
        assert_eq!(a.as_ptr(), pb);
        assert_eq!(b.as_ptr(), pa);
    }

    #[test]
    fn large_strings() {
        let (a, b) = run(&"a".repeat(10_000), &"b".repeat(9_999));
        assert_eq!(a.len(), 9_999);
        assert_eq!(b.len(), 10_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** if `a` is longer than `b`, swap them with `std::mem::swap`. A `String` is a small
struct (pointer, length, capacity), so swapping exchanges those three fields and the heap text
stays where it is.

**Complexity:** `O(1)` time and extra space.

**Rust concepts:**
- Two `&mut` references may exist at the same time only if they point to different values; the
  function signature guarantees that to the compiler.
- `std::mem::swap(&mut T, &mut T)` exchanges two values without moving out of either reference.
- `a.swap(b)` does not exist for `String`; `mem::swap` works for every type.

```rust
pub fn shorter_first(a: &mut String, b: &mut String) {
    if a.len() > b.len() {
        std::mem::swap(a, b);
    }
}
```

Full tested solution: [`src/solutions/p0105_shorter_string_first.rs`](../src/solutions/p0105_shorter_string_first.rs)

</details>
