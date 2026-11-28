//! # Problem 0903: A Facade with `pub use`
//!
//! - **Difficulty:** Medium
//! - **Topics:** private modules, `pub use` re-exports, `use ... as`, a `prelude` module, `dyn Trait`
//! - **Source:** Original · [The Rust Book 7.4](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use) · [Reference: use declarations](https://doc.rust-lang.org/reference/items/use-declarations.html)
//!
//! ## Problem
//!
//! Libraries often keep their internal module layout **private** and publish a flat, stable API with
//! `pub use`. The `geometry` module below does exactly that:
//!
//! ```text
//! geometry
//! ├── shapes            (private module: users cannot write geometry::shapes::...)
//! │   ├── circle        Circle
//! │   ├── rect          Rect                 re-exported as geometry::Rectangle
//! │   └── polygon       Triangle
//! ├── Shape             the trait
//! ├── total_area, describe, largest
//! └── prelude           re-exports Circle, Rectangle, Triangle, Shape
//! ```
//!
//! The layout and the re-exports are given. Implement the bodies:
//!
//! - `Shape for Circle`: area `PI * radius * radius`, name `"circle"`.
//! - `Shape for Rect`: area `width * height`, name `"rectangle"`.
//! - `Triangle::new(a, b, c) -> Option<Triangle>`: `None` unless all sides are positive and each side
//!   is **strictly** smaller than the sum of the other two. `Shape for Triangle`: Heron's formula
//!   `sqrt(s(s-a)(s-b)(s-c))` with `s = (a+b+c)/2`, name `"triangle"`.
//! - `total_area(shapes: &[&dyn Shape]) -> f64`: the sum of the areas (`0.0` for no shapes).
//! - `describe(shape: &dyn Shape) -> String`: `"<name> with area <area with 2 decimals>"`.
//! - `largest(shapes) -> Option<&dyn Shape>`: the shape with the largest area; the **first** one
//!   on ties; `None` for an empty slice.
//!
//! ## Examples
//!
//! ```text
//! use geometry::prelude::*;
//! describe(&Circle { radius: 2.0 })              => "circle with area 12.57"
//! describe(&Rectangle { width: 2.0, height: 3.5 }) => "rectangle with area 7.00"
//! Triangle::new(3.0, 4.0, 5.0)                   => Some(..), area 6.0
//! Triangle::new(1.0, 2.0, 3.0)                   => None  (degenerate)
//! total_area(&[&Rectangle { width: 1.0, height: 1.0 }, &Rectangle { width: 2.0, height: 2.0 }]) => 5.0
//! ```
//!
//! ## Constraints
//!
//! - All lengths are finite.
//!
//! ## Hints
//!
//! 1. Inside `geometry::shapes::circle`, the trait is `super::super::Shape`.
//! 2. `format!("{:.2}", x)` prints two decimals.
//! 3. For "first on ties", keep the current best and only replace it when a shape is strictly larger.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each shape implements `Shape` in its own private module; the facade re-exports the
//! types so users only ever see `geometry::Circle`, `geometry::Rectangle` and `geometry::Triangle`.
//! `largest` folds over the slice with `reduce`, replacing the best shape only on a strictly larger
//! area.
//!
//! **Rust concepts:**
//! - A `pub` item inside a **private** module is still unreachable by its full path from outside;
//!   `pub use self::shapes::circle::Circle;` publishes it at a new, shorter path.
//! - `pub use ... as Rectangle` renames on re-export: the internal name `Rect` never leaks.
//! - A `prelude` module of re-exports lets users write `use geometry::prelude::*;`.
//! - `pub(in path)` (not needed here) restricts an item to an ancestor module, e.g.
//!   `pub(in super::super)`; `pub(super)` and `pub(crate)` are common special cases.
// @solution-end

pub mod geometry {
    mod shapes {
        pub mod circle {
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub struct Circle {
                pub radius: f64,
            }

            impl super::super::Shape for Circle {
                fn area(&self) -> f64 {
                    // @solution-start
                    std::f64::consts::PI * self.radius * self.radius
                    // @solution-end
                }

                fn name(&self) -> &'static str {
                    // @solution-start
                    "circle"
                    // @solution-end
                }
            }
        }

        pub mod rect {
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub struct Rect {
                pub width: f64,
                pub height: f64,
            }

            impl super::super::Shape for Rect {
                fn area(&self) -> f64 {
                    // @solution-start
                    self.width * self.height
                    // @solution-end
                }

                fn name(&self) -> &'static str {
                    // @solution-start
                    "rectangle"
                    // @solution-end
                }
            }
        }

        pub mod polygon {
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub struct Triangle {
                a: f64,
                b: f64,
                c: f64,
            }

            impl Triangle {
                pub fn new(a: f64, b: f64, c: f64) -> Option<Triangle> {
                    // @solution-start
                    let positive = a > 0.0 && b > 0.0 && c > 0.0;
                    let strict = a < b + c && b < a + c && c < a + b;
                    (positive && strict).then_some(Triangle { a, b, c })
                    // @solution-end
                }
            }

            impl super::super::Shape for Triangle {
                fn area(&self) -> f64 {
                    // @solution-start
                    let s = (self.a + self.b + self.c) / 2.0;
                    (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
                    // @solution-end
                }

                fn name(&self) -> &'static str {
                    // @solution-start
                    "triangle"
                    // @solution-end
                }
            }
        }
    }

    pub use self::shapes::circle::Circle;
    pub use self::shapes::polygon::Triangle;
    pub use self::shapes::rect::Rect as Rectangle;

    pub trait Shape {
        fn area(&self) -> f64;
        fn name(&self) -> &'static str;
    }

    pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
        // @solution-start
        shapes.iter().map(|shape| shape.area()).sum()
        // @solution-end
    }

    pub fn describe(shape: &dyn Shape) -> String {
        // @solution-start
        format!("{} with area {:.2}", shape.name(), shape.area())
        // @solution-end
    }

    pub fn largest<'a>(shapes: &[&'a dyn Shape]) -> Option<&'a dyn Shape> {
        // @solution-start
        shapes.iter().copied().reduce(|best, shape| {
            if shape.area() > best.area() {
                shape
            } else {
                best
            }
        })
        // @solution-end
    }

    pub mod prelude {
        pub use super::{Circle, Rectangle, Shape, Triangle};
    }
}

#[cfg(test)]
mod tests {
    use super::geometry::prelude::*;
    use super::geometry::{describe, largest, total_area};

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn describe_each_shape() {
        assert_eq!(describe(&Circle { radius: 2.0 }), "circle with area 12.57");
        assert_eq!(
            describe(&Rectangle {
                width: 2.0,
                height: 3.5
            }),
            "rectangle with area 7.00"
        );
        let triangle = Triangle::new(3.0, 4.0, 5.0).unwrap();
        assert_eq!(describe(&triangle), "triangle with area 6.00");
    }

    #[test]
    fn triangle_validation() {
        assert!(Triangle::new(1.0, 2.0, 3.0).is_none());
        assert!(Triangle::new(0.0, 1.0, 1.0).is_none());
        assert!(Triangle::new(-3.0, 4.0, 5.0).is_none());
        let equilateral = Triangle::new(2.0, 2.0, 2.0).unwrap();
        assert!(close(equilateral.area(), 3.0_f64.sqrt()));
    }

    #[test]
    fn total_area_of_mixed_shapes() {
        let small = Rectangle {
            width: 1.0,
            height: 1.0,
        };
        let big = Rectangle {
            width: 2.0,
            height: 2.0,
        };
        let circle = Circle { radius: 1.0 };
        assert!(close(total_area(&[&small, &big]), 5.0));
        assert!(close(
            total_area(&[&small, &circle]),
            1.0 + std::f64::consts::PI
        ));
        assert_eq!(total_area(&[]), 0.0);
    }

    #[test]
    fn largest_keeps_the_first_on_ties() {
        let square = Rectangle {
            width: 2.0,
            height: 2.0,
        };
        let same = Rectangle {
            width: 4.0,
            height: 1.0,
        };
        let circle = Circle { radius: 1.0 };
        let best = largest(&[&circle, &square, &same]).unwrap();
        assert_eq!(best.name(), "rectangle");
        assert!(close(best.area(), 4.0));
        // A 3-4-5 triangle and a 2 x 3 rectangle both have area 6: the first one wins.
        let triangle = Triangle::new(3.0, 4.0, 5.0).unwrap();
        let rect = Rectangle {
            width: 2.0,
            height: 3.0,
        };
        assert_eq!(largest(&[&triangle, &rect]).unwrap().name(), "triangle");
        assert_eq!(largest(&[&rect, &triangle]).unwrap().name(), "rectangle");
        assert!(largest(&[]).is_none());
    }
}
