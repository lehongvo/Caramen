//! # Problem 0593: Associated Constants
//!
//! - **Difficulty:** Easy
//! - **Topics:** `const` items in traits, default constant values, associated functions using `Self::CONST`, generic functions reading `U::SYMBOL`
//! - **Source:** Original · [The Rust Reference: associated constants](https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants)
//!
//! ## Problem
//!
//! Traits can declare **constants** that every implementor provides, and generic code can read them
//! through the type parameter (no value needed).
//!
//! 1. Length units:
//!
//!    ```text
//!    pub trait Unit { const SYMBOL: &'static str; const METERS: f64; }   // how many meters one unit is
//!    ```
//!
//!    The marker types `Meter` (`"m"`, `1.0`), `Kilometer` (`"km"`, `1000.0`), `Foot` (`"ft"`,
//!    `0.3048`) and `Inch` (`"in"`, `0.0254`) implement it (these impls are provided). Write
//!    `convert<Src: Unit, Dst: Unit>(value: f64) -> f64` and `format_length<U: Unit>(value: f64) ->
//!    String` returning `"{value:.2} {SYMBOL}"`.
//! 2. Polygons:
//!
//!    ```text
//!    pub trait Polygon {
//!        const SIDES: u32;
//!        const NAME: &'static str = "polygon";       // default value
//!        fn interior_angle_sum() -> u32 { ... }      // (SIDES - 2) * 180
//!        fn describe() -> String { ... }             // "{NAME} with {SIDES} sides"
//!    }
//!    ```
//!
//!    Write the two default functions. The impls for `Triangle` (3 sides, name `"triangle"`), `Square`
//!    (4, `"square"`) and `Heptagon` (7 sides, keeping the **default** name) are provided.
//!
//! ## Examples
//!
//! ```text
//! convert::<Kilometer, Meter>(2.5)       => 2500.0
//! convert::<Foot, Inch>(1.0)             => 12.0     (within 1e-9)
//! format_length::<Foot>(3.0)             => "3.00 ft"
//! Square::interior_angle_sum()           => 360
//! Heptagon::describe()                   => "polygon with 7 sides"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Inside a trait's default function, constants are read as `Self::SIDES`.
//! 2. In generic code they are read through the parameter: `Src::METERS`, `U::SYMBOL`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** conversion goes through the base unit: `value * Src::METERS / Dst::METERS`. The polygon
//! functions are defaults computed from the constants, so implementors only state facts.
//!
//! **Rust concepts:**
//! - Associated constants are resolved at compile time per implementing type; the marker types carry
//!   no data at all.
//! - A trait constant may have a default (`const NAME: &'static str = "polygon";`) that implementors
//!   can override, just like default methods.
//! - `convert::<Foot, Inch>(1.0)` uses the turbofish because the types cannot be inferred from a
//!   plain `f64` argument.
// @solution-end

/// A unit of length.
pub trait Unit {
    const SYMBOL: &'static str;
    const METERS: f64;
}

pub struct Meter;
pub struct Kilometer;
pub struct Foot;
pub struct Inch;

impl Unit for Meter {
    const SYMBOL: &'static str = "m";
    const METERS: f64 = 1.0;
}

impl Unit for Kilometer {
    const SYMBOL: &'static str = "km";
    const METERS: f64 = 1000.0;
}

impl Unit for Foot {
    const SYMBOL: &'static str = "ft";
    const METERS: f64 = 0.3048;
}

impl Unit for Inch {
    const SYMBOL: &'static str = "in";
    const METERS: f64 = 0.0254;
}

/// Converts a length from one unit to another.
pub fn convert<Src: Unit, Dst: Unit>(value: f64) -> f64 {
    // @solution-start
    value * Src::METERS / Dst::METERS
    // @solution-end
}

/// `"{value:.2} {symbol}"`
pub fn format_length<U: Unit>(value: f64) -> String {
    // @solution-start
    format!("{value:.2} {}", U::SYMBOL)
    // @solution-end
}

/// A regular polygon described by constants.
pub trait Polygon {
    const SIDES: u32;
    const NAME: &'static str = "polygon";

    fn interior_angle_sum() -> u32 {
        // @solution-start
        (Self::SIDES - 2) * 180
        // @solution-end
    }

    fn describe() -> String {
        // @solution-start
        format!("{} with {} sides", Self::NAME, Self::SIDES)
        // @solution-end
    }
}

pub struct Triangle;
pub struct Square;
pub struct Heptagon;

impl Polygon for Triangle {
    const SIDES: u32 = 3;
    const NAME: &'static str = "triangle";
}

impl Polygon for Square {
    const SIDES: u32 = 4;
    const NAME: &'static str = "square";
}

impl Polygon for Heptagon {
    const SIDES: u32 = 7;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn unit_conversions() {
        assert_eq!(convert::<Kilometer, Meter>(2.5), 2500.0);
        assert!(close(convert::<Foot, Inch>(1.0), 12.0));
        assert!(close(convert::<Meter, Foot>(0.3048), 1.0));
        assert_eq!(convert::<Inch, Inch>(7.0), 7.0);
    }

    #[test]
    fn formatting() {
        assert_eq!(format_length::<Foot>(3.0), "3.00 ft");
        assert_eq!(
            format_length::<Kilometer>(convert::<Meter, Kilometer>(1234.5)),
            "1.23 km"
        );
        assert_eq!(Inch::SYMBOL, "in");
    }

    #[test]
    fn polygons() {
        assert_eq!(Triangle::interior_angle_sum(), 180);
        assert_eq!(Square::interior_angle_sum(), 360);
        assert_eq!(Heptagon::interior_angle_sum(), 900);
        assert_eq!(Triangle::describe(), "triangle with 3 sides");
        assert_eq!(Heptagon::describe(), "polygon with 7 sides");
        assert_eq!(<Square as Polygon>::NAME, "square");
    }
}
