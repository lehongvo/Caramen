//! # Problem 0219: Shapes: Area and Perimeter
//!
//! - **Difficulty:** Easy
//! - **Topics:** enum variants with named fields, methods on enums, `match` with destructuring, `Option<usize>`
//! - **Source:** Original · [The Rust Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) · [The Rust Book 6.2](https://doc.rust-lang.org/book/ch06-02-match.html)
//!
//! ## Problem
//!
//! A `Shape` is one of:
//!
//! - `Circle { radius }`,
//! - `Rectangle { width, height }`,
//! - `Triangle { a, b, c }` (the three side lengths; they always form a valid triangle).
//!
//! All fields are `f64`. Implement:
//!
//! - `area(&self) -> f64` (use `std::f64::consts::PI`; for a triangle use Heron's formula
//!   `sqrt(s (s - a) (s - b) (s - c))` with `s = (a + b + c) / 2`);
//! - `perimeter(&self) -> f64` (`2 π r` for a circle);
//! - `name(&self) -> &'static str`: `"circle"`, `"rectangle"` or `"triangle"`;
//! - the free function `largest(shapes: &[Shape]) -> Option<usize>`: the index of the shape with the
//!   largest area (the **first** one if several tie), or `None` if `shapes` is empty.
//!
//! ## Examples
//!
//! ```text
//! Shape::Rectangle { width: 3.0, height: 4.0 }.area()       => 12.0
//! Shape::Rectangle { width: 3.0, height: 4.0 }.perimeter()  => 14.0
//! Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 }.area()         => 6.0
//! Shape::Circle { radius: 1.0 }.perimeter()                 => 6.283185307179586   (2π)
//! largest(&[Rectangle { width: 1.0, height: 1.0 }, Circle { radius: 1.0 }, Triangle { a: 3.0, b: 4.0, c: 5.0 }])
//!                                                           => Some(2)   (areas 1, π, 6)
//! largest(&[])                                              => None
//! ```
//!
//! ## Constraints
//!
//! - All lengths are positive and finite, at most `10^4`. Tests compare with a tolerance of `1e-9`.
//!
//! ## Hints
//!
//! 1. Destructure the fields directly in the pattern: `Shape::Circle { radius } => PI * radius * radius`.
//! 2. When matching on `&self`, the bound fields are references (`&f64`); arithmetic on `&f64` works,
//!    or write `*radius`.
//! 3. For `largest`, keep the best `(index, area)` seen so far and replace it only on a strictly larger area.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each method is one `match` with one arm per variant, destructuring the fields it needs.
//! `largest` scans once and keeps the first maximum.
//!
//! **Rust concepts:**
//! - Struct-like variants are matched with `Variant { field1, field2 }`; the names must match the
//!   declaration (or be renamed with `field: name`).
//! - `match self` on `&Shape` binds `&f64` references; `f64` implements arithmetic for `&f64`, and
//!   `*x` dereferences explicitly where needed.
//! - `f64` is not `Ord` (because of `NaN`), so `Iterator::max_by_key` does not work directly; a
//!   simple loop (or `max_by` with `partial_cmp`) is the usual answer.
// @solution-end

use std::f64::consts::PI;

/// A 2D shape.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { a: f64, b: f64, c: f64 },
}

impl Shape {
    /// The area of the shape.
    pub fn area(&self) -> f64 {
        // @solution-start
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { a, b, c } => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
        // @solution-end
    }

    /// The perimeter of the shape.
    pub fn perimeter(&self) -> f64 {
        // @solution-start
        match self {
            Shape::Circle { radius } => 2.0 * PI * radius,
            Shape::Rectangle { width, height } => 2.0 * (width + height),
            Shape::Triangle { a, b, c } => a + b + c,
        }
        // @solution-end
    }

    /// `"circle"`, `"rectangle"` or `"triangle"`.
    pub fn name(&self) -> &'static str {
        // @solution-start
        match self {
            Shape::Circle { .. } => "circle",
            Shape::Rectangle { .. } => "rectangle",
            Shape::Triangle { .. } => "triangle",
        }
        // @solution-end
    }
}

/// The index of the first shape with the largest area, or `None` if empty.
pub fn largest(shapes: &[Shape]) -> Option<usize> {
    // @solution-start
    let mut best: Option<(usize, f64)> = None;
    for (i, shape) in shapes.iter().enumerate() {
        let area = shape.area();
        match best {
            Some((_, best_area)) if area <= best_area => {}
            _ => best = Some((i, area)),
        }
    }
    best.map(|(i, _)| i)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::Shape::*;
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn rectangle() {
        let r = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert!(close(r.area(), 12.0));
        assert!(close(r.perimeter(), 14.0));
        assert_eq!(r.name(), "rectangle");
    }

    #[test]
    fn circle() {
        let c = Circle { radius: 1.0 };
        assert!(close(c.area(), PI));
        assert!(close(c.perimeter(), std::f64::consts::TAU));
        assert_eq!(c.name(), "circle");
        assert!(close(Circle { radius: 2.0 }.area(), 4.0 * PI));
    }

    #[test]
    fn triangle_heron() {
        let t = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };
        assert!(close(t.area(), 6.0));
        assert!(close(t.perimeter(), 12.0));
        assert_eq!(t.name(), "triangle");
        let eq = Triangle {
            a: 2.0,
            b: 2.0,
            c: 2.0,
        };
        assert!(close(eq.area(), 3f64.sqrt()));
    }

    #[test]
    fn largest_example_and_empty() {
        let shapes = [
            Rectangle {
                width: 1.0,
                height: 1.0,
            },
            Circle { radius: 1.0 },
            Triangle {
                a: 3.0,
                b: 4.0,
                c: 5.0,
            },
        ];
        assert_eq!(largest(&shapes), Some(2));
        assert_eq!(largest(&[]), None);
    }

    #[test]
    fn largest_prefers_first_on_ties() {
        let shapes = [
            Rectangle {
                width: 2.0,
                height: 3.0,
            },
            Rectangle {
                width: 3.0,
                height: 2.0,
            },
            Rectangle {
                width: 1.0,
                height: 6.0,
            },
        ];
        assert_eq!(largest(&shapes), Some(0));
        assert_eq!(largest(&shapes[1..]), Some(0));
    }
}
