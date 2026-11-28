// ==================================================================
// PRACTICE 0555: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0555_solve_the_equation.md
// Run the tests:          cargo test practice::p0555
// ==================================================================

//! # Problem 0555: Solve the Equation
//!
//! - **Difficulty:** Medium
//! - **Topics:** a small value type `Linear` with `Add`/`Sub`/`Neg`/`Sum`, parsing terms with signs, `FromStr`
//! - **Source:** [LeetCode 640. Solve the Equation](https://leetcode.com/problems/solve-the-equation/)
//!
//! ## Problem
//!
//! You are given a linear equation in one variable `x`, such as `"x+5-3+x=6+x-2"`. It contains only
//! `+`, `-`, the variable `x` (optionally preceded by an integer coefficient, like `2x` or `0x`) and
//! non-negative integer constants, with exactly one `=` and no spaces.
//!
//! Return:
//!
//! - `"x=#value"` when there is exactly one solution (it is guaranteed to be an integer),
//! - `"Infinite solutions"` when every `x` works,
//! - `"No solution"` when no `x` works.
//!
//! Implement `Solution::solve_equation(equation: String) -> String`.
//!
//! Represent each side as a value `Linear { coef: i64, constant: i64 }` meaning `coef * x + constant`.
//! Implement `Add`, `Sub` and `Neg` for `Linear`, and `FromStr for Linear` (with `type Err = String`)
//! that parses one side such as `"-2x+3-x"`. Then the equation `left = right` becomes
//! `left - right = 0`.
//!
//! ## Examples
//!
//! ```text
//! Input:  "x+5-3+x=6+x-2"     Output: "x=2"
//! Input:  "x=x"               Output: "Infinite solutions"
//! Input:  "2x=x"              Output: "x=0"
//! Input:  "2x+3x-6x=x+2"      Output: "x=-1"
//! Input:  "x=x+2"             Output: "No solution"
//! "-2x+3-x".parse::<Linear>() => Linear { coef: -3, constant: 3 }
//! ```
//!
//! ## Constraints
//!
//! - `3 <= equation.len() <= 1000`
//! - `equation` has exactly one `=`, and both sides are non-empty valid expressions.
//! - Coefficients and constants are in `0..=1000`.
//!
//! ## Hints
//!
//! 1. Split each side into signed terms: start a new term at every `+` or `-` (except at position `0`).
//! 2. A term ending with `x` is a variable term: `"x"` means `1`, `"-x"` means `-1`, `"7x"` means `7`.
//! 3. After moving everything to the left (`left - right`), the equation is `coef * x + constant = 0`.

use std::iter::Sum;
use std::ops::{Add, Neg, Sub};
use std::str::FromStr;

pub struct Solution;

/// `coef * x + constant`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Linear {
    pub coef: i64,
    pub constant: i64,
}

impl Add for Linear {
    type Output = Linear;

    fn add(self, rhs: Linear) -> Linear {
        todo!("Write your solution here")
    }
}

impl Neg for Linear {
    type Output = Linear;

    fn neg(self) -> Linear {
        todo!("Write your solution here")
    }
}

impl Sub for Linear {
    type Output = Linear;

    fn sub(self, rhs: Linear) -> Linear {
        todo!("Write your solution here")
    }
}

impl Sum for Linear {
    fn sum<I: Iterator<Item = Linear>>(iter: I) -> Linear {
        todo!("Write your solution here")
    }
}

impl FromStr for Linear {
    type Err = String;

    fn from_str(side: &str) -> Result<Self, Self::Err> {
        todo!("Write your solution here")
    }
}

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve(e: &str) -> String {
        Solution::solve_equation(e.to_string())
    }

    #[test]
    fn example_equations() {
        assert_eq!(solve("x+5-3+x=6+x-2"), "x=2");
        assert_eq!(solve("x=x"), "Infinite solutions");
        assert_eq!(solve("2x=x"), "x=0");
        assert_eq!(solve("2x+3x-6x=x+2"), "x=-1");
        assert_eq!(solve("x=x+2"), "No solution");
    }

    #[test]
    fn parsing_sides() {
        assert_eq!(
            "-2x+3-x".parse::<Linear>(),
            Ok(Linear {
                coef: -3,
                constant: 3
            })
        );
        assert_eq!(
            "0x".parse::<Linear>(),
            Ok(Linear {
                coef: 0,
                constant: 0
            })
        );
        assert_eq!(
            "10".parse::<Linear>(),
            Ok(Linear {
                coef: 0,
                constant: 10
            })
        );
        assert!("2y".parse::<Linear>().is_err());
    }

    #[test]
    fn operators() {
        let a = Linear {
            coef: 2,
            constant: -1,
        };
        let b = Linear {
            coef: 5,
            constant: 4,
        };
        assert_eq!(
            a + b,
            Linear {
                coef: 7,
                constant: 3
            }
        );
        assert_eq!(
            a - b,
            Linear {
                coef: -3,
                constant: -5
            }
        );
        assert_eq!(
            -a,
            Linear {
                coef: -2,
                constant: 1
            }
        );
        assert_eq!(
            vec![a, b, a].into_iter().sum::<Linear>(),
            Linear {
                coef: 9,
                constant: 2
            }
        );
    }

    #[test]
    fn zero_coefficients_and_negative_answers() {
        assert_eq!(solve("0x=0"), "Infinite solutions");
        assert_eq!(solve("0x=5"), "No solution");
        assert_eq!(solve("-x=-1"), "x=1");
        assert_eq!(solve("3x+9=0"), "x=-3");
        assert_eq!(solve("1000x-1000=0"), "x=1");
    }

    #[test]
    fn long_equation() {
        // 500 copies of "+x" on the left and 499 on the right: x = 7.
        let left = "+x".repeat(500);
        let right = format!("{}+7", "+x".repeat(499));
        assert_eq!(solve(&format!("{}={}", &left[1..], &right[1..])), "x=7");
    }
}
