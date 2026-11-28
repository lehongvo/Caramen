# Problem 0215: 2D Vectors as Copy Structs

- **Difficulty:** Easy
- **Topics:** `#[derive(Clone, Copy)]`, methods taking `self` by value, methods returning `Self`, associated constants
- **Source:** Original · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) · [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)

## Problem

A `Vec2` is a point or a direction in the plane, with public fields `x` and `y` (`f64`).
Because it is small and made only of `f64`s, it derives `Copy`: methods can take `self` by value
and the caller can keep using the original afterwards.

Implement:

- the associated constant `Vec2::ZERO` (both coordinates `0.0`);
- `Vec2::new(x: f64, y: f64) -> Vec2`;
- `plus(self, other: Vec2) -> Vec2` and `minus(self, other: Vec2) -> Vec2` (component-wise);
- `scaled(self, factor: f64) -> Vec2` (both coordinates multiplied by `factor`);
- `dot(self, other: Vec2) -> f64` (`x1 * x2 + y1 * y2`);
- `length(self) -> f64` (the Euclidean length `sqrt(x² + y²)`);
- the free function `centroid(points: &[Vec2]) -> Option<Vec2>`: the average of all points, or
  `None` for an empty slice.

The methods are named `plus`/`minus` rather than `add`/`sub` on purpose: those names belong to
the operator traits `Add`/`Sub`, which a later level covers.

## Examples

```text
let a = Vec2::new(1.0, 2.0);
let b = Vec2::new(3.0, -1.0);
a.plus(b)          => Vec2 { x: 4.0, y: 1.0 }
a.minus(b)         => Vec2 { x: -2.0, y: 3.0 }
a.scaled(2.5)      => Vec2 { x: 2.5, y: 5.0 }
a.dot(b)           => 1.0      (3 - 2)
Vec2::new(3.0, 4.0).length() => 5.0
centroid(&[Vec2::new(0.0, 0.0), Vec2::new(4.0, 0.0), Vec2::new(2.0, 6.0)]) => Some(Vec2 { x: 2.0, y: 2.0 })
centroid(&[])      => None
```

## Constraints

- Coordinates are finite, with absolute value at most `10^6`. Tests compare with a tolerance of `1e-9`.

## Hints

1. An associated constant is declared inside `impl Vec2` as `pub const ZERO: Vec2 = ...;`.
2. `f64::hypot(x, y)` computes `sqrt(x² + y²)` accurately.
3. For `centroid`, fold all points with `plus`, then `scaled(1.0 / n)`.

## Starter code

Write your code in [`src/practice/p0215_vector2_copy_struct.rs`](../src/practice/p0215_vector2_copy_struct.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0215
```

```rust
/// A 2D vector with `f64` coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// The zero vector.
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    /// Creates a vector.
    pub fn new(x: f64, y: f64) -> Self {
        todo!("Write your solution here")
    }

    /// Component-wise sum.
    pub fn plus(self, other: Self) -> Self {
        todo!("Write your solution here")
    }

    /// Component-wise difference.
    pub fn minus(self, other: Self) -> Self {
        todo!("Write your solution here")
    }

    /// Both coordinates multiplied by `factor`.
    pub fn scaled(self, factor: f64) -> Self {
        todo!("Write your solution here")
    }

    /// Dot product.
    pub fn dot(self, other: Self) -> f64 {
        todo!("Write your solution here")
    }

    /// Euclidean length.
    pub fn length(self) -> f64 {
        todo!("Write your solution here")
    }
}

/// The average of `points`, or `None` if there are none.
pub fn centroid(points: &[Vec2]) -> Option<Vec2> {
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

    fn close(a: Vec2, b: Vec2) -> bool {
        (a.x - b.x).abs() < 1e-9 && (a.y - b.y).abs() < 1e-9
    }

    #[test]
    fn arithmetic_examples() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, -1.0);
        assert!(close(a.plus(b), Vec2 { x: 4.0, y: 1.0 }));
        assert!(close(a.minus(b), Vec2 { x: -2.0, y: 3.0 }));
        assert!(close(a.scaled(2.5), Vec2 { x: 2.5, y: 5.0 }));
        assert!((a.dot(b) - 1.0).abs() < 1e-9);
        // `a` and `b` are still usable: they were copied, not moved.
        assert_eq!(a, Vec2 { x: 1.0, y: 2.0 });
    }

    #[test]
    fn length_and_zero() {
        assert!((Vec2::new(3.0, 4.0).length() - 5.0).abs() < 1e-9);
        assert!((Vec2::new(-5.0, 12.0).length() - 13.0).abs() < 1e-9);
        assert_eq!(Vec2::new(0.0, 0.0), Vec2::ZERO);
        assert!(Vec2::ZERO.length().abs() < 1e-9);
    }

    #[test]
    fn perpendicular_vectors_have_zero_dot() {
        let a = Vec2::new(2.0, 3.0);
        let b = Vec2::new(-3.0, 2.0);
        assert!(a.dot(b).abs() < 1e-9);
        assert!((a.dot(a) - a.length() * a.length()).abs() < 1e-9);
    }

    #[test]
    fn centroid_of_triangle() {
        let pts = [
            Vec2::new(0.0, 0.0),
            Vec2::new(4.0, 0.0),
            Vec2::new(2.0, 6.0),
        ];
        let c = centroid(&pts).expect("non-empty");
        assert!(close(c, Vec2::new(2.0, 2.0)));
    }

    #[test]
    fn centroid_empty_and_single() {
        assert_eq!(centroid(&[]), None);
        let only = Vec2::new(-7.5, 1e6);
        assert!(close(centroid(&[only]).expect("one point"), only));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each method builds a new `Vec2` from the fields of `self` and `other`. `centroid`
returns early with `None` for an empty slice, otherwise sums with `fold(Vec2::ZERO, Vec2::plus)`
and divides by the count.

**Rust concepts:**
- `#[derive(Clone, Copy)]` is allowed because every field (`f64`) is `Copy`. Taking `self` by value
  then copies the struct instead of moving it.
- `Vec2::plus` used as a value is a function `fn(Vec2, Vec2) -> Vec2`, so it can be passed
  directly to `fold`.
- `Self` inside an `impl` is an alias for the type being implemented.
- `.iter().copied()` turns an iterator of `&Vec2` into an iterator of `Vec2` (possible because
  `Vec2: Copy`).

```rust
/// A 2D vector with `f64` coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// The zero vector.
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    /// Creates a vector.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Component-wise sum.
    pub fn plus(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    /// Component-wise difference.
    pub fn minus(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    /// Both coordinates multiplied by `factor`.
    pub fn scaled(self, factor: f64) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }

    /// Dot product.
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Euclidean length.
    pub fn length(self) -> f64 {
        self.x.hypot(self.y)
    }
}

/// The average of `points`, or `None` if there are none.
pub fn centroid(points: &[Vec2]) -> Option<Vec2> {
    if points.is_empty() {
        return None;
    }
    let sum = points.iter().copied().fold(Vec2::ZERO, Vec2::plus);
    Some(sum.scaled(1.0 / points.len() as f64))
}
```

Full tested solution: [`src/solutions/p0215_vector2_copy_struct.rs`](../src/solutions/p0215_vector2_copy_struct.rs)

</details>
