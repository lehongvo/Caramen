//! # Problem 0206: Rectangle Methods and a Square Constructor
//!
//! - **Difficulty:** Easy
//! - **Topics:** `impl` blocks, methods with `&self`, associated functions, `Self`
//! - **Source:** Original · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
//!
//! ## Problem
//!
//! Turn the free functions of problem 0201 into **methods**. A method is a function defined inside
//! `impl Rectangle { ... }` whose first parameter is `self` (here `&self`, a shared borrow of the
//! rectangle). It is called with dot syntax: `rect.area()`. A function inside the `impl` block
//! *without* `self` is an *associated function*, called with `::`, e.g. `Rectangle::square(5)`.
//!
//! Implement on `Rectangle { pub width: u32, pub height: u32 }`:
//!
//! - `Rectangle::new(width, height) -> Self` and `Rectangle::square(size) -> Self` (associated
//!   functions; a square has equal width and height).
//! - `area(&self) -> u32` and `perimeter(&self) -> u32`.
//! - `is_square(&self) -> bool`.
//! - `can_hold(&self, other: &Rectangle) -> bool`: `true` when `other` fits **strictly** inside
//!   `self` without rotating it, i.e. `self.width > other.width` **and** `self.height > other.height`.
//! - `rotated(&self) -> Self`: a new rectangle with width and height swapped.
//!
//! ## Examples
//!
//! ```text
//! let r1 = Rectangle::new(30, 50);
//! let r2 = Rectangle::new(10, 40);
//! let r3 = Rectangle::new(60, 45);
//! r1.area()        => 1500
//! r1.perimeter()   => 160
//! r1.can_hold(&r2) => true
//! r1.can_hold(&r3) => false
//! Rectangle::square(4).is_square() => true
//! r1.rotated()     => Rectangle { width: 50, height: 30 }
//! ```
//!
//! ## Constraints
//!
//! - `0 <= width, height <= 10^4`
//!
//! ## Hints
//!
//! 1. Inside the `impl` block, `Self` is another name for `Rectangle`.
//! 2. `square` can simply call `Self::new(size, size)`.
//! 3. `rect.area()` is sugar for `Rectangle::area(&rect)`: Rust borrows `rect` automatically.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every method reads the two fields through `&self`. `can_hold` compares both dimensions,
//! and `rotated` builds a new value with the fields swapped.
//!
//! **Rust concepts:**
//! - `&self` is short for `self: &Self`; the method only reads the rectangle.
//! - Associated functions without `self` are often constructors; `new` is a convention, not a
//!   keyword.
//! - *Automatic referencing*: `r1.can_hold(&r2)` passes `&r1` as `self` without writing `&r1`.
//! - Returning `Self` from `rotated` leaves the original rectangle unchanged.
// @solution-end

/// A rectangle with methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Builds a rectangle.
    pub fn new(width: u32, height: u32) -> Self {
        // @solution-start
        Self { width, height }
        // @solution-end
    }

    /// Builds a square.
    pub fn square(size: u32) -> Self {
        // @solution-start
        Self::new(size, size)
        // @solution-end
    }

    /// Width times height.
    pub fn area(&self) -> u32 {
        // @solution-start
        self.width * self.height
        // @solution-end
    }

    /// Sum of the four sides.
    pub fn perimeter(&self) -> u32 {
        // @solution-start
        2 * (self.width + self.height)
        // @solution-end
    }

    /// `true` when width equals height.
    pub fn is_square(&self) -> bool {
        // @solution-start
        self.width == self.height
        // @solution-end
    }

    /// `true` when `other` fits strictly inside `self` without rotation.
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        // @solution-start
        self.width > other.width && self.height > other.height
        // @solution-end
    }

    /// The same rectangle turned by 90 degrees.
    pub fn rotated(&self) -> Self {
        // @solution-start
        Self {
            width: self.height,
            height: self.width,
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_area_and_perimeter() {
        let r1 = Rectangle::new(30, 50);
        assert_eq!(r1.area(), 1500);
        assert_eq!(r1.perimeter(), 160);
    }

    #[test]
    fn example_can_hold() {
        let r1 = Rectangle::new(30, 50);
        assert!(r1.can_hold(&Rectangle::new(10, 40)));
        assert!(!r1.can_hold(&Rectangle::new(60, 45)));
    }

    #[test]
    fn example_square_and_rotation() {
        assert!(Rectangle::square(4).is_square());
        assert_eq!(Rectangle::square(4), Rectangle::new(4, 4));
        assert_eq!(
            Rectangle::new(30, 50).rotated(),
            Rectangle {
                width: 50,
                height: 30
            }
        );
    }

    #[test]
    fn equal_size_does_not_fit_strictly() {
        let r = Rectangle::new(5, 5);
        assert!(!r.can_hold(&r));
        assert!(!r.can_hold(&Rectangle::new(5, 1)));
    }

    #[test]
    fn rotation_can_make_it_fit() {
        let frame = Rectangle::new(10, 3);
        let card = Rectangle::new(2, 8);
        assert!(!frame.can_hold(&card));
        assert!(frame.can_hold(&card.rotated()));
        assert!(!Rectangle::new(10, 3).is_square());
    }

    #[test]
    fn degenerate_rectangles() {
        let line = Rectangle::new(0, 7);
        assert_eq!(line.area(), 0);
        assert_eq!(line.perimeter(), 14);
        assert!(Rectangle::square(0).is_square());
    }
}
