# Problem 0007: Rectangle Area and Perimeter

- **Difficulty:** Easy
- **Topics:** tuples, returning several values, destructuring a tuple parameter
- **Source:** Original · [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)

## Problem

A rectangle is described by its `width` and `height` (non-negative whole numbers). Write two
functions:

1. `area_and_perimeter(width: u32, height: u32) -> (u32, u32)` returns the tuple
   `(area, perimeter)`, where `area = width * height` and `perimeter = 2 * (width + height)`.
2. `is_square(size: (u32, u32)) -> bool` receives the size as **one** tuple `(width, height)` and
   returns `true` if the rectangle is a square (both sides equal). A `0 x 0` rectangle counts as
   a square.

## Examples

```text
area_and_perimeter(3, 4)  -> (12, 14)
area_and_perimeter(5, 5)  -> (25, 20)
area_and_perimeter(0, 7)  -> (0, 14)

is_square((5, 5)) -> true
is_square((3, 4)) -> false
```

## Constraints

- `0 <= width, height <= 10_000` (so the area fits in a `u32`)

## Hints

1. A tuple value is written `(a, b)`; a tuple type is written `(u32, u32)`.
2. A pattern can unpack a tuple directly in the parameter list: `fn f((w, h): (u32, u32))`.

## Starter code

Write your code in [`src/practice/p0007_rectangle_area_and_perimeter.rs`](../src/practice/p0007_rectangle_area_and_perimeter.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0007
```

```rust
pub fn area_and_perimeter(width: u32, height: u32) -> (u32, u32) {
    todo!("Write your solution here")
}

pub fn is_square(size: (u32, u32)) -> bool {
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
    fn example_three_by_four() {
        assert_eq!(area_and_perimeter(3, 4), (12, 14));
    }

    #[test]
    fn example_square() {
        assert_eq!(area_and_perimeter(5, 5), (25, 20));
    }

    #[test]
    fn zero_width() {
        assert_eq!(area_and_perimeter(0, 7), (0, 14));
        assert_eq!(area_and_perimeter(0, 0), (0, 0));
    }

    #[test]
    fn largest_rectangle_fits() {
        let (area, perimeter) = area_and_perimeter(10_000, 10_000);
        assert_eq!(area, 100_000_000);
        assert_eq!(perimeter, 40_000);
    }

    #[test]
    fn square_detection() {
        assert!(is_square((5, 5)));
        assert!(is_square((0, 0)));
        assert!(!is_square((3, 4)));
        assert!(!is_square((4, 3)));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** compute both numbers and return them together as a tuple. For `is_square`, destructure
the tuple parameter into `w` and `h` and compare them.

**Rust concepts:**
- A tuple groups values of possibly different types into one value. The caller can unpack it
  again with `let (area, perimeter) = area_and_perimeter(3, 4);`.
- Function parameters are patterns, so `(w, h): (u32, u32)` destructures the tuple right away.
  The alternative is indexing with `size.0` and `size.1`.
- `w == h` is already a `bool`; there is no need for `if w == h { true } else { false }`.

```rust
pub fn area_and_perimeter(width: u32, height: u32) -> (u32, u32) {
    let area = width * height;
    let perimeter = 2 * (width + height);
    (area, perimeter)
}

pub fn is_square(size: (u32, u32)) -> bool {
    let (w, h) = size;
    w == h
}
```

Full tested solution: [`src/solutions/p0007_rectangle_area_and_perimeter.rs`](../src/solutions/p0007_rectangle_area_and_perimeter.rs)

</details>
