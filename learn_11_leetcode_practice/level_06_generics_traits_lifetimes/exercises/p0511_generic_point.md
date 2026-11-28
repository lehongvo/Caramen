# Problem 0511: Generic Point with Concrete Impls

- **Difficulty:** Easy
- **Topics:** generic structs, `impl<T> Point<T>`, `impl Point<f64>`, `impl Point<i32>`
- **Source:** Original · [The Rust Book 10.1](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions)

## Problem

Define a generic 2D point `Point<T>` with two public fields `x: T` and `y: T` (both of the same
type). Then write three impl blocks:

1. `impl<T> Point<T>`: methods available for **every** `T`:
   - `new(x: T, y: T) -> Self`
   - `x(&self) -> &T` and `y(&self) -> &T` (getters that borrow the coordinates)
   - `into_tuple(self) -> (T, T)`
2. `impl Point<f64>`: methods that only exist for floating-point points:
   - `distance_from_origin(&self) -> f64`: the Euclidean distance `sqrt(x^2 + y^2)`.
   - `distance_to(&self, other: &Point<f64>) -> f64`: the Euclidean distance between two points.
3. `impl Point<i32>`: methods that only exist for integer points:
   - `manhattan_from_origin(&self) -> i32`: `|x| + |y|`.
   - `quadrant(&self) -> u8`: `1`, `2`, `3` or `4` (counter-clockwise from the top-right
     quadrant), or `0` if the point lies on an axis.

Calling `distance_from_origin` on a `Point<i32>` must not compile, and calling `quadrant` on a
`Point<f64>` must not compile either. This is what concrete impl blocks give you.

Derive `Debug`, `Clone`, `Copy` and `PartialEq` on the struct.

## Examples

```text
Point::new(3.0, 4.0).distance_from_origin()           => 5.0
Point::new(1.0, 1.0).distance_to(&Point::new(4.0, 5.0)) => 5.0
Point::new(-3, 4).manhattan_from_origin()             => 7
Point::new(-3, 4).quadrant()                          => 2
Point::new(0, 9).quadrant()                           => 0
Point::new('a', 'b').into_tuple()                     => ('a', 'b')
```

## Constraints

- Integer coordinates satisfy `|x|, |y| <= 10^9`, so `|x| + |y|` fits in `i32` only when it is at
  most `i32::MAX`; the tests stay within that range.

## Hints

1. A generic method block starts with `impl<T> Point<T>`: the first `<T>` declares the parameter,
   the second uses it.
2. A concrete block has no parameter list: `impl Point<f64> { ... }`.
3. `f64::hypot(a, b)` computes `sqrt(a^2 + b^2)` accurately.

## Starter code

Write your code in [`src/practice/p0511_generic_point.rs`](../src/practice/p0511_generic_point.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0511
```

```rust
/// A point whose two coordinates have the same type `T`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        todo!("Write your solution here")
    }

    pub fn x(&self) -> &T {
        todo!("Write your solution here")
    }

    pub fn y(&self) -> &T {
        todo!("Write your solution here")
    }

    pub fn into_tuple(self) -> (T, T) {
        todo!("Write your solution here")
    }
}

impl Point<f64> {
    /// Euclidean distance to `(0, 0)`.
    pub fn distance_from_origin(&self) -> f64 {
        todo!("Write your solution here")
    }

    /// Euclidean distance to `other`.
    pub fn distance_to(&self, other: &Point<f64>) -> f64 {
        todo!("Write your solution here")
    }
}

impl Point<i32> {
    /// `|x| + |y|`.
    pub fn manhattan_from_origin(&self) -> i32 {
        todo!("Write your solution here")
    }

    /// Quadrant `1..=4`, or `0` on an axis.
    pub fn quadrant(&self) -> u8 {
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
    fn generic_methods_for_any_type() {
        let p = Point::new('a', 'b');
        assert_eq!(*p.x(), 'a');
        assert_eq!(*p.y(), 'b');
        assert_eq!(p.into_tuple(), ('a', 'b'));
        let s = Point::new(String::from("left"), String::from("right"));
        assert_eq!(s.x(), "left");
        assert_eq!(s.into_tuple().1, "right");
    }

    #[test]
    fn float_distances() {
        assert_eq!(Point::new(3.0, 4.0).distance_from_origin(), 5.0);
        assert_eq!(Point::new(1.0, 1.0).distance_to(&Point::new(4.0, 5.0)), 5.0);
        let p = Point::new(-2.5, 0.0);
        assert_eq!(p.distance_from_origin(), 2.5);
        assert_eq!(p.distance_to(&p), 0.0);
    }

    #[test]
    fn integer_methods() {
        assert_eq!(Point::new(-3, 4).manhattan_from_origin(), 7);
        assert_eq!(Point::new(0, 0).manhattan_from_origin(), 0);
        assert_eq!(Point::new(5, 6).quadrant(), 1);
        assert_eq!(Point::new(-3, 4).quadrant(), 2);
        assert_eq!(Point::new(-1, -1).quadrant(), 3);
        assert_eq!(Point::new(7, -2).quadrant(), 4);
        assert_eq!(Point::new(0, 9).quadrant(), 0);
        assert_eq!(Point::new(-4, 0).quadrant(), 0);
    }

    #[test]
    fn derived_traits_and_large_values() {
        let p = Point::new(1_000_000_000, -1_000_000_000);
        let q = p; // Copy
        assert_eq!(p, q);
        assert_eq!(q.manhattan_from_origin(), 2_000_000_000);
        assert_eq!(format!("{:?}", Point::new(1, 2)), "Point { x: 1, y: 2 }");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the struct is generic, the getters live in the generic block, and the numeric methods
live in concrete blocks so they can use `f64` and `i32` operations directly.

**Rust concepts:**
- `impl<T> Point<T>` defines methods for every instantiation; inside it you can only do what works
  for an arbitrary `T` (move it, borrow it), because there are no bounds.
- `impl Point<f64>` adds methods to one instantiation only. The compiler rejects
  `Point::new(1, 2).distance_from_origin()` with "method not found in `Point<i32>`".
- `#[derive(Clone, Copy, PartialEq)]` on a generic struct adds the bound automatically: the
  derived `impl<T: Copy> Copy for Point<T>` only applies when `T` is `Copy`.

```rust
/// A point whose two coordinates have the same type `T`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl Point<f64> {
    /// Euclidean distance to `(0, 0)`.
    pub fn distance_from_origin(&self) -> f64 {
        self.x.hypot(self.y)
    }

    /// Euclidean distance to `other`.
    pub fn distance_to(&self, other: &Point<f64>) -> f64 {
        (self.x - other.x).hypot(self.y - other.y)
    }
}

impl Point<i32> {
    /// `|x| + |y|`.
    pub fn manhattan_from_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    /// Quadrant `1..=4`, or `0` on an axis.
    pub fn quadrant(&self) -> u8 {
        match (self.x.signum(), self.y.signum()) {
            (1, 1) => 1,
            (-1, 1) => 2,
            (-1, -1) => 3,
            (1, -1) => 4,
            _ => 0,
        }
    }
}
```

Full tested solution: [`src/solutions/p0511_generic_point.rs`](../src/solutions/p0511_generic_point.rs)

</details>
