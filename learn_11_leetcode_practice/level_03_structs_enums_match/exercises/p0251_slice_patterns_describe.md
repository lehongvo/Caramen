# Problem 0251: Describe a List with Slice Patterns

- **Difficulty:** Easy
- **Topics:** slice patterns `[]`, `[x]`, `[first, .., last]`, rest bindings `rest @ ..`, recursion on slices
- **Source:** Original · [The Rust Reference: Slice patterns](https://doc.rust-lang.org/reference/patterns.html#slice-patterns)

## Problem

Slice patterns match on the **shape** of a slice: its length and some of its elements. Use them
(not indexing, not `len()`) in the following functions:

- `describe(v: &[i32]) -> String`:
  - `[]` gives `"empty"`;
  - one element `x` gives `"one: x"`;
  - two elements gives `"pair: a, b"`;
  - longer slices give `"from <first> to <last> (<n> items)"`.
- `sum(v: &[i32]) -> i64`: recursive: the sum of an empty slice is `0`, otherwise the head plus
  the sum of the tail.
- `is_palindrome(v: &[i32]) -> bool`: recursive: slices of length 0 or 1 are palindromes;
  otherwise the first and last must be equal and the middle must be a palindrome.
- `starts_and_ends_with_zero(v: &[i32]) -> bool`: `true` if the slice has at least two elements
  and both the first and the last are `0`.

## Examples

```text
describe(&[])               => "empty"
describe(&[7])              => "one: 7"
describe(&[1, 2])           => "pair: 1, 2"
describe(&[3, 1, 4, 1, 5])  => "from 3 to 5 (5 items)"
sum(&[1, 2, 3])             => 6
is_palindrome(&[1, 2, 1])   => true
is_palindrome(&[1, 2])      => false
starts_and_ends_with_zero(&[0, 5, 0]) => true
starts_and_ends_with_zero(&[0])       => false
```

## Constraints

- Slices have at most `1000` elements (the recursion depth stays small).

## Hints

1. `[first, .., last]` matches any slice with at least two elements; `..` skips the middle.
2. `[head, tail @ ..]` binds `tail` to the rest of the slice (a `&[i32]`).
3. Literals work inside slice patterns: `[0, .., 0]`.

## Starter code

Write your code in [`src/practice/p0251_slice_patterns_describe.rs`](../src/practice/p0251_slice_patterns_describe.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0251
```

```rust
/// A short description of the slice's shape.
pub fn describe(v: &[i32]) -> String {
    todo!("Write your solution here")
}

/// The sum of the slice, computed recursively.
pub fn sum(v: &[i32]) -> i64 {
    todo!("Write your solution here")
}

/// Whether the slice reads the same backwards, checked recursively.
pub fn is_palindrome(v: &[i32]) -> bool {
    todo!("Write your solution here")
}

/// Whether the slice has at least two elements and starts and ends with 0.
pub fn starts_and_ends_with_zero(v: &[i32]) -> bool {
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
    fn descriptions() {
        assert_eq!(describe(&[]), "empty");
        assert_eq!(describe(&[7]), "one: 7");
        assert_eq!(describe(&[1, 2]), "pair: 1, 2");
        assert_eq!(describe(&[3, 1, 4, 1, 5]), "from 3 to 5 (5 items)");
        assert_eq!(describe(&[-1, 0, -1]), "from -1 to -1 (3 items)");
    }

    #[test]
    fn recursive_sums() {
        assert_eq!(sum(&[]), 0);
        assert_eq!(sum(&[1, 2, 3]), 6);
        assert_eq!(sum(&[i32::MAX, i32::MAX]), 2 * i64::from(i32::MAX));
        let v: Vec<i32> = (1..=1000).collect();
        assert_eq!(sum(&v), 500_500);
    }

    #[test]
    fn palindromes() {
        assert!(is_palindrome(&[]));
        assert!(is_palindrome(&[4]));
        assert!(is_palindrome(&[1, 2, 1]));
        assert!(is_palindrome(&[1, 2, 2, 1]));
        assert!(!is_palindrome(&[1, 2]));
        assert!(!is_palindrome(&[1, 2, 3, 1]));
    }

    #[test]
    fn long_palindrome() {
        let mut v: Vec<i32> = (0..500).collect();
        v.extend((0..500).rev());
        assert!(is_palindrome(&v));
        v[700] = -1;
        assert!(!is_palindrome(&v));
    }

    #[test]
    fn zero_ends() {
        assert!(starts_and_ends_with_zero(&[0, 5, 0]));
        assert!(starts_and_ends_with_zero(&[0, 0]));
        assert!(!starts_and_ends_with_zero(&[0]));
        assert!(!starts_and_ends_with_zero(&[0, 1]));
        assert!(!starts_and_ends_with_zero(&[]));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each function is a `match` on the slice whose arms go from the shortest shapes to the
general one. The recursive functions peel off elements with rest patterns.

**Rust concepts:**
- Slice patterns: `[]` (exactly empty), `[x]` (exactly one), `[a, b]` (exactly two),
  `[first, .., last]` (two or more), `[head, tail @ ..]` (one or more, with the tail bound).
- Matching on `&[i32]` binds elements as `&i32` (default binding modes); use `*x` or
  `i64::from(*x)` to get the value.
- The compiler checks slice patterns for exhaustiveness: `[]`, `[_]` and `[first, middle @ .., last]`
  together cover every length.
- A pattern only names elements, not the length: the item count in `describe` still comes from
  `v.len()` inside the arm.
- `matches!(v, [0, .., 0])` combines slice patterns with literals in a one-line test.

```rust
/// A short description of the slice's shape.
pub fn describe(v: &[i32]) -> String {
    match v {
        [] => String::from("empty"),
        [x] => format!("one: {x}"),
        [a, b] => format!("pair: {a}, {b}"),
        [first, .., last] => format!("from {first} to {last} ({} items)", v.len()),
    }
}

/// The sum of the slice, computed recursively.
pub fn sum(v: &[i32]) -> i64 {
    match v {
        [] => 0,
        [head, tail @ ..] => i64::from(*head) + sum(tail),
    }
}

/// Whether the slice reads the same backwards, checked recursively.
pub fn is_palindrome(v: &[i32]) -> bool {
    match v {
        [] | [_] => true,
        [first, middle @ .., last] => first == last && is_palindrome(middle),
    }
}

/// Whether the slice has at least two elements and starts and ends with 0.
pub fn starts_and_ends_with_zero(v: &[i32]) -> bool {
    matches!(v, [0, .., 0])
}
```

Full tested solution: [`src/solutions/p0251_slice_patterns_describe.rs`](../src/solutions/p0251_slice_patterns_describe.rs)

</details>
