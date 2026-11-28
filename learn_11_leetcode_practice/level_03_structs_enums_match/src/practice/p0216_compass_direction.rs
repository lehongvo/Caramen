// ==================================================================
// PRACTICE 0216: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0216_compass_direction.md
// Run the tests:          cargo test practice::p0216
// ==================================================================

//! # Problem 0216: Compass Directions
//!
//! - **Difficulty:** Easy
//! - **Topics:** fieldless `enum`, exhaustive `match`, methods on enums, `Option<Self>` constructor
//! - **Source:** Original · [The Rust Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) · [The Rust Book 6.2](https://doc.rust-lang.org/book/ch06-02-match.html)
//!
//! ## Problem
//!
//! The enum `Direction` has four variants: `North`, `East`, `South` and `West`. Implement these
//! methods (all of them take `self` by value; the enum derives `Copy`):
//!
//! - `turn_right(self) -> Direction`: a quarter turn clockwise (`North -> East -> South -> West -> North`);
//! - `turn_left(self) -> Direction`: a quarter turn counter-clockwise;
//! - `opposite(self) -> Direction`: a half turn;
//! - `delta(self) -> (i32, i32)`: one step in that direction as `(dx, dy)`, where north is `+y`
//!   and east is `+x`;
//! - `Direction::from_letter(c: char) -> Option<Direction>`: `'N'`, `'E'`, `'S'`, `'W'` give the
//!   matching direction; any other character gives `None`.
//!
//! Then write `follow(start: Direction, turns: &str) -> Direction`, which applies a sequence of turn
//! letters: `'L'` (left), `'R'` (right) and `'U'` (U-turn). Any other character is ignored.
//!
//! Write every `match` on `Direction` **without** a `_` arm: listing all four variants lets the
//! compiler tell you if a variant is ever added and forgotten.
//!
//! ## Examples
//!
//! ```text
//! Direction::North.turn_right()   => East
//! Direction::North.turn_left()    => West
//! Direction::East.opposite()      => West
//! Direction::South.delta()        => (0, -1)
//! Direction::from_letter('W')     => Some(West)
//! Direction::from_letter('x')     => None
//! follow(North, "RRL")            => East      (N -R-> E -R-> S -L-> E)
//! follow(West, "UL")              => North     (W -U-> E -L-> N)
//! ```
//!
//! ## Constraints
//!
//! - `turns` has at most `10^5` characters.
//!
//! ## Hints
//!
//! 1. `opposite` can be written as two right turns, and `turn_left` as three.
//! 2. `follow` is a `fold` over `turns.chars()` starting from `start`.

/// A compass direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// A quarter turn clockwise.
    pub fn turn_right(self) -> Direction {
        todo!("Write your solution here")
    }

    /// A quarter turn counter-clockwise.
    pub fn turn_left(self) -> Direction {
        todo!("Write your solution here")
    }

    /// A half turn.
    pub fn opposite(self) -> Direction {
        todo!("Write your solution here")
    }

    /// One step as `(dx, dy)`; north is `+y`, east is `+x`.
    pub fn delta(self) -> (i32, i32) {
        todo!("Write your solution here")
    }

    /// `'N'`, `'E'`, `'S'` or `'W'` to a direction.
    pub fn from_letter(c: char) -> Option<Direction> {
        todo!("Write your solution here")
    }
}

/// Applies `'L'`, `'R'` and `'U'` turns to `start`; other characters are ignored.
pub fn follow(start: Direction, turns: &str) -> Direction {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::Direction::*;
    use super::*;

    #[test]
    fn turns_cycle_through_all_directions() {
        assert_eq!(North.turn_right(), East);
        assert_eq!(East.turn_right(), South);
        assert_eq!(South.turn_right(), West);
        assert_eq!(West.turn_right(), North);
        assert_eq!(North.turn_left(), West);
        assert_eq!(West.turn_left(), South);
    }

    #[test]
    fn opposites_and_deltas() {
        assert_eq!(East.opposite(), West);
        assert_eq!(North.opposite(), South);
        assert_eq!(North.delta(), (0, 1));
        assert_eq!(East.delta(), (1, 0));
        assert_eq!(South.delta(), (0, -1));
        assert_eq!(West.delta(), (-1, 0));
    }

    #[test]
    fn parse_letters() {
        assert_eq!(Direction::from_letter('W'), Some(West));
        assert_eq!(Direction::from_letter('N'), Some(North));
        assert_eq!(Direction::from_letter('x'), None);
        assert_eq!(Direction::from_letter('n'), None);
    }

    #[test]
    fn follow_examples() {
        assert_eq!(follow(North, "RRL"), East);
        assert_eq!(follow(West, "UL"), North);
        assert_eq!(follow(South, ""), South);
        assert_eq!(follow(South, "x?R"), West);
    }

    #[test]
    fn many_turns() {
        // 4 right turns are a full circle; 100_000 is a multiple of 4.
        let turns = "R".repeat(100_000);
        assert_eq!(follow(East, &turns), East);
        let turns = "L".repeat(100_001);
        assert_eq!(follow(East, &turns), North);
    }
}
