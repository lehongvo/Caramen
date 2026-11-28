// ==================================================================
// PRACTICE 0407: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0407_combine_optional_values.md
// Run the tests:          cargo test practice::p0407
// ==================================================================

//! # Problem 0407: Combine Optional Values with zip and or
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Option::zip`, `Option::or`, `Option::or_else`, `Option::xor`, `Option::map`
//! - **Source:** Original · [std `Option`: boolean operators](https://doc.rust-lang.org/std/option/index.html#boolean-operators)
//!
//! ## Problem
//!
//! Many values in a program are optional: a coordinate that was not entered, a command-line flag
//! that was not given... `Option` has methods that combine **two** options without any `match`:
//!
//! - `a.zip(b)`: `Some((x, y))` when **both** are `Some`, otherwise `None`;
//! - `a.or(b)`: `a` if it is `Some`, otherwise `b`;
//! - `a.or_else(|| ...)`: like `or`, but the fallback is computed only when needed;
//! - `a.xor(b)`: `Some` only when **exactly one** of them is `Some`.
//!
//! Implement:
//!
//! - `make_point(x: Option<i32>, y: Option<i32>) -> Option<(i32, i32)>`.
//! - `manhattan(a: Option<(i32, i32)>, b: Option<(i32, i32)>) -> Option<i32>`: the Manhattan
//!   distance `|x1 - x2| + |y1 - y2|` when both points are known.
//! - `pick_port(cli: Option<u16>, env: Option<u16>, default: u16) -> u16`: the command-line value
//!   if given, otherwise the environment value if given, otherwise `default`.
//! - `display_name(nickname: Option<&str>, full_name: &str) -> Option<String>`: the nickname if
//!   present, otherwise the first word of `full_name`; `None` if both are unavailable.
//! - `exactly_one(a: Option<i32>, b: Option<i32>) -> Option<i32>`: the value when exactly one
//!   option is `Some`.
//!
//! ## Examples
//!
//! ```text
//! make_point(Some(1), Some(2))                 -> Some((1, 2))
//! make_point(Some(1), None)                    -> None
//! manhattan(Some((1, 2)), Some((4, -2)))       -> Some(7)
//! pick_port(None, Some(8080), 80)              -> 8080
//! pick_port(Some(3000), Some(8080), 80)        -> 3000
//! display_name(None, "Grace Hopper")           -> Some("Grace")
//! display_name(Some("Amazing"), "Grace Hopper") -> Some("Amazing")
//! exactly_one(Some(4), None)                   -> Some(4)
//! exactly_one(Some(4), Some(5))                -> None
//! ```
//!
//! ## Constraints
//!
//! - Coordinates are in `-10^6..=10^6`, so the distance fits in an `i32`.
//!
//! ## Hints
//!
//! 1. `a.zip(b).map(|((x1, y1), (x2, y2))| ...)` destructures both points in the closure.
//! 2. `cli.or(env).unwrap_or(default)` reads exactly like the rule.
//! 3. For `display_name`, convert the nickname with `.map(String::from)` and use `or_else` for
//!    the fallback so the full name is only split when there is no nickname.

pub fn make_point(x: Option<i32>, y: Option<i32>) -> Option<(i32, i32)> {
    todo!("Write your solution here")
}

pub fn manhattan(a: Option<(i32, i32)>, b: Option<(i32, i32)>) -> Option<i32> {
    todo!("Write your solution here")
}

pub fn pick_port(cli: Option<u16>, env: Option<u16>, default: u16) -> u16 {
    todo!("Write your solution here")
}

pub fn display_name(nickname: Option<&str>, full_name: &str) -> Option<String> {
    todo!("Write your solution here")
}

pub fn exactly_one(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_point_needs_both() {
        assert_eq!(make_point(Some(1), Some(2)), Some((1, 2)));
        assert_eq!(make_point(Some(1), None), None);
        assert_eq!(make_point(None, Some(2)), None);
    }

    #[test]
    fn manhattan_distance() {
        assert_eq!(manhattan(Some((1, 2)), Some((4, -2))), Some(7));
        assert_eq!(manhattan(Some((0, 0)), Some((0, 0))), Some(0));
        assert_eq!(manhattan(None, Some((4, -2))), None);
        assert_eq!(
            manhattan(Some((-1_000_000, -1_000_000)), Some((1_000_000, 1_000_000))),
            Some(4_000_000)
        );
    }

    #[test]
    fn pick_port_priority() {
        assert_eq!(pick_port(None, Some(8080), 80), 8080);
        assert_eq!(pick_port(Some(3000), Some(8080), 80), 3000);
        assert_eq!(pick_port(None, None, 80), 80);
    }

    #[test]
    fn display_name_fallbacks() {
        assert_eq!(display_name(None, "Grace Hopper").as_deref(), Some("Grace"));
        assert_eq!(
            display_name(Some("Amazing"), "Grace Hopper").as_deref(),
            Some("Amazing")
        );
        assert_eq!(display_name(None, "   "), None);
    }

    #[test]
    fn exactly_one_is_xor() {
        assert_eq!(exactly_one(Some(4), None), Some(4));
        assert_eq!(exactly_one(None, Some(5)), Some(5));
        assert_eq!(exactly_one(Some(4), Some(5)), None);
        assert_eq!(exactly_one(None, None), None);
    }
}
