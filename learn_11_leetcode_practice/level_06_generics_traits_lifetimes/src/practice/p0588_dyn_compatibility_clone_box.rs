// ==================================================================
// PRACTICE 0588: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0588_dyn_compatibility_clone_box.md
// Run the tests:          cargo test practice::p0588
// ==================================================================

//! # Problem 0588: Dyn Compatibility and Cloning Boxes
//!
//! - **Difficulty:** Medium
//! - **Topics:** dyn-compatible traits (formerly "object safety"), `where Self: Sized` methods, cloning `Box<dyn Trait>` via a helper supertrait and a blanket impl
//! - **Source:** Original · [The Rust Reference: dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)
//!
//! ## Problem
//!
//! A trait can be used as `dyn Trait` only if it is **dyn compatible**. Among other rules, its methods
//! must not return `Self` or have generic type parameters, unless they are marked `where Self: Sized`
//! (such methods are simply not available on trait objects). `Clone` itself is not dyn compatible
//! (`fn clone(&self) -> Self`), so `Vec<Box<dyn Shape>>` cannot be cloned directly.
//!
//! Build a shape trait that is dyn compatible **and** cloneable:
//!
//! ```text
//! pub trait ShapeClone { fn clone_box(&self) -> Box<dyn Shape>; }
//! impl<T: Shape + Clone + 'static> ShapeClone for T { ... }          // blanket impl
//!
//! pub trait Shape: ShapeClone {
//!     fn name(&self) -> &'static str;
//!     fn area(&self) -> f64;
//!     fn scaled(&self, k: f64) -> Self where Self: Sized;             // not callable on dyn Shape
//!     fn same_kind<S: Shape>(&self, other: &S) -> bool where Self: Sized { ... }  // generic method
//! }
//! impl Clone for Box<dyn Shape> { ... }                               // uses clone_box
//! ```
//!
//! Implement all of the above, `Shape` for `Square { side }` (name `"square"`) and `Circle { radius }`
//! (name `"circle"`), both deriving `Clone`, and:
//!
//! - `duplicate(shapes: &[Box<dyn Shape>]) -> Vec<Box<dyn Shape>>`: a deep copy (use `to_vec()`,
//!   which needs `Box<dyn Shape>: Clone`).
//! - `scale_all<S: Shape>(shapes: &[S], k: f64) -> Vec<S>`: static dispatch, so `scaled` is allowed.
//! - `total_area(shapes: &[Box<dyn Shape>]) -> f64`.
//!
//! `scaled(k)` multiplies the lengths by `k` (so areas grow by `k^2`). `same_kind` compares names.
//!
//! ## Examples
//!
//! ```text
//! let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Square { side: 2.0 }), Box::new(Circle { radius: 1.0 })];
//! let copy = duplicate(&shapes);              // independent boxes
//! total_area(&copy)                           => 4 + PI
//! Square { side: 1.0 }.scaled(3.0).side       => 3.0
//! Square { side: 1.0 }.same_kind(&Circle { radius: 1.0 }) => false
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `clone_box` in the blanket impl is `Box::new(self.clone())`.
//! 2. In `impl Clone for Box<dyn Shape>`, call `(**self).clone_box()`: `self` is `&Box<dyn Shape>`, and
//!    you want the method of the shape inside the box.
//! 3. Remove `where Self: Sized` from `scaled` and the line `let v: Vec<Box<dyn Shape>>` stops compiling
//!    (error E0038: the trait is not dyn compatible).

use std::f64::consts::PI;

/// Dyn-compatible cloning for shapes.
pub trait ShapeClone {
    fn clone_box(&self) -> Box<dyn Shape>;
}

impl<T: Shape + Clone + 'static> ShapeClone for T {
    fn clone_box(&self) -> Box<dyn Shape> {
        todo!("Write your solution here")
    }
}

/// A dyn-compatible shape trait.
pub trait Shape: ShapeClone {
    fn name(&self) -> &'static str;
    fn area(&self) -> f64;

    fn scaled(&self, k: f64) -> Self
    where
        Self: Sized;

    fn same_kind<S: Shape>(&self, other: &S) -> bool
    where
        Self: Sized,
    {
        todo!("Write your solution here")
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        todo!("Write your solution here")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Square {
    pub side: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

impl Shape for Square {
    fn name(&self) -> &'static str {
        todo!("Write your solution here")
    }

    fn area(&self) -> f64 {
        todo!("Write your solution here")
    }

    fn scaled(&self, k: f64) -> Self {
        todo!("Write your solution here")
    }
}

impl Shape for Circle {
    fn name(&self) -> &'static str {
        todo!("Write your solution here")
    }

    fn area(&self) -> f64 {
        todo!("Write your solution here")
    }

    fn scaled(&self, k: f64) -> Self {
        todo!("Write your solution here")
    }
}

/// A deep copy of a list of boxed shapes.
pub fn duplicate(shapes: &[Box<dyn Shape>]) -> Vec<Box<dyn Shape>> {
    todo!("Write your solution here")
}

/// Scales every shape (static dispatch).
pub fn scale_all<S: Shape>(shapes: &[S], k: f64) -> Vec<S> {
    todo!("Write your solution here")
}

/// The sum of all areas.
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn sample() -> Vec<Box<dyn Shape>> {
        vec![
            Box::new(Square { side: 2.0 }),
            Box::new(Circle { radius: 1.0 }),
        ]
    }

    #[test]
    fn boxes_can_be_cloned() {
        let shapes = sample();
        let copy = duplicate(&shapes);
        assert_eq!(copy.len(), 2);
        assert!(close(total_area(&copy), 4.0 + PI));
        let single: Box<dyn Shape> = shapes[0].clone();
        assert_eq!(single.name(), "square");
        // The copy is independent: it points to a different allocation.
        assert!(!std::ptr::eq(
            shapes[1].as_ref() as *const dyn Shape as *const u8,
            copy[1].as_ref() as *const dyn Shape as *const u8
        ));
    }

    #[test]
    fn sized_only_methods_on_concrete_types() {
        let s = Square { side: 1.0 };
        assert_eq!(s.scaled(3.0), Square { side: 3.0 });
        assert!(s.same_kind(&Square { side: 9.0 }));
        assert!(!s.same_kind(&Circle { radius: 1.0 }));
        let circles = scale_all(&[Circle { radius: 1.0 }, Circle { radius: 2.0 }], 0.5);
        assert_eq!(
            circles,
            vec![Circle { radius: 0.5 }, Circle { radius: 1.0 }]
        );
    }

    #[test]
    fn clone_box_works_for_user_shapes() {
        #[derive(Clone)]
        struct Strip(f64);
        impl Shape for Strip {
            fn name(&self) -> &'static str {
                "strip"
            }
            fn area(&self) -> f64 {
                self.0
            }
            fn scaled(&self, k: f64) -> Self {
                Strip(self.0 * k)
            }
        }
        let mut shapes = sample();
        shapes.push(Box::new(Strip(1.5)));
        let names: Vec<&str> = shapes.clone().iter().map(|s| s.name()).collect();
        assert_eq!(names, vec!["square", "circle", "strip"]);
        assert!(close(total_area(&duplicate(&shapes)), 4.0 + PI + 1.5));
    }
}
