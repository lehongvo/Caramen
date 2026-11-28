// ==================================================================
// PRACTICE 0201: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0201_rectangle_area.md
// Run the tests:          cargo test practice::p0201
// ==================================================================

//! # Problem 0201: Rectangle Area with a Struct
//!
//! - **Difficulty:** Easy
//! - **Topics:** defining a `struct`, named fields, field access, borrowing a struct with `&`
//! - **Source:** Original · [The Rust Book 5.2](https://doc.rust-lang.org/book/ch05-02-example-structs.html)
//!
//! ## Problem
//!
//! A *struct* groups several related values under one name. Each value is a *field* with its own
//! name and type. The struct below describes a rectangle:
//!
//! ```text
//! pub struct Rectangle {
//!     pub width: u32,
//!     pub height: u32,
//! }
//! ```
//!
//! You create a value with a *struct literal*, `Rectangle { width: 30, height: 50 }`, and read a
//! field with a dot, `rect.width`.
//!
//! Implement two free functions:
//!
//! - `area(rect: &Rectangle) -> u32` returns `width * height`.
//! - `total_area(rects: &[Rectangle]) -> u32` returns the sum of the areas of every rectangle in
//!   the slice, or `0` when the slice is empty.
//!
//! Both functions only **borrow** their input, so the caller still owns the rectangles afterwards.
//!
//! ## Examples
//!
//! ```text
//! Input:  Rectangle { width: 30, height: 50 }
//! Output: area = 1500
//!
//! Input:  [Rectangle { width: 2, height: 3 }, Rectangle { width: 4, height: 5 }]
//! Output: total_area = 6 + 20 = 26
//!
//! Input:  []
//! Output: total_area = 0
//! ```
//!
//! ## Constraints
//!
//! - `0 <= width, height <= 10^4`
//! - `0 <= rects.len() <= 10^3`, and the total area fits in a `u32`.
//!
//! ## Hints
//!
//! 1. Inside `area`, `rect` is a `&Rectangle`, but `rect.width` still works: the dot operator
//!    follows the reference for you.
//! 2. `total_area` can call `area` for every element: `rects.iter().map(area).sum()`.

/// A rectangle described by its width and height.
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

/// Returns the area of `rect`.
pub fn area(rect: &Rectangle) -> u32 {
    todo!("Write your solution here")
}

/// Returns the sum of the areas of all rectangles.
pub fn total_area(rects: &[Rectangle]) -> u32 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_book_rectangle() {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };
        assert_eq!(area(&rect), 1500);
    }

    #[test]
    fn example_total_of_two() {
        let rects = [
            Rectangle {
                width: 2,
                height: 3,
            },
            Rectangle {
                width: 4,
                height: 5,
            },
        ];
        assert_eq!(total_area(&rects), 26);
    }

    #[test]
    fn example_empty_slice() {
        assert_eq!(total_area(&[]), 0);
    }

    #[test]
    fn zero_width_has_zero_area() {
        let flat = Rectangle {
            width: 0,
            height: 99,
        };
        assert_eq!(area(&flat), 0);
    }

    #[test]
    fn caller_keeps_ownership_after_borrowing() {
        let rect = Rectangle {
            width: 7,
            height: 8,
        };
        assert_eq!(area(&rect), 56);
        // `rect` was only borrowed, so its fields are still readable here.
        assert_eq!(rect.width + rect.height, 15);
        assert_eq!(area(&rect), 56);
    }

    #[test]
    fn largest_values() {
        let big = Rectangle {
            width: 10_000,
            height: 10_000,
        };
        assert_eq!(area(&big), 100_000_000);
        let many: Vec<Rectangle> = (0..1000)
            .map(|i| Rectangle {
                width: i,
                height: 2,
            })
            .collect();
        // 2 * (0 + 1 + ... + 999) = 999 * 1000
        assert_eq!(total_area(&many), 999_000);
    }
}
