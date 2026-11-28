//! # Problem 0068: Maximum Height of a Triangle
//!
//! - **Difficulty:** Easy
//! - **Topics:** helper functions, simulation with `loop`, tuples for alternating state, `max`
//! - **Source:** [LeetCode 3200. Maximum Height of a Triangle](https://leetcode.com/problems/maximum-height-of-a-triangle/)
//!
//! ## Problem
//!
//! You have `red` red balls and `blue` blue balls. You want to stack them into a triangle where
//! row 1 has 1 ball, row 2 has 2 balls, row 3 has 3 balls, and so on, following two rules:
//!
//! - all balls in one row have the **same** color;
//! - two adjacent rows have **different** colors.
//!
//! Return the largest number of rows (the height) you can build.
//!
//! Implement `Solution::max_height_of_triangle(red: i32, blue: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  red = 2, blue = 4
//! Output: 3
//! Explanation: row 1 red (1), row 2 blue (2), row 3 red would need 3 red balls (only 1 left),
//!              so start with blue instead: blue 1, red 2, blue 3 -> height 3.
//!
//! Input:  red = 2, blue = 1
//! Output: 2
//!
//! Input:  red = 1, blue = 1
//! Output: 1
//!
//! Input:  red = 10, blue = 1
//! Output: 2
//! ```
//!
//! ## Constraints
//!
//! - `1 <= red, blue <= 100`
//!
//! ## Hints
//!
//! 1. Once you pick the color of row 1, every other row's color is forced.
//! 2. Write a helper that builds rows greedily for a given starting color, and try both colors.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `build(first, second)` places rows `1, 2, 3, ...` alternately from the `first` and
//! `second` piles until the current pile cannot supply the row. The answer is the better of
//! `build(red, blue)` and `build(blue, red)`.
//!
//! **Complexity:** `O(sqrt(red + blue))` time (the row sizes grow), `O(1)` space.
//!
//! **Rust concepts:**
//! - A private associated function (`fn build`, without `pub`) is a helper inside `impl Solution`;
//!   it is called as `Self::build(...)`.
//! - `(current, other) = (other, current);` swaps two variables with a tuple assignment
//!   (destructuring assignment, Rust 1.59+). Here it switches the color for the next row.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        // @solution-start
        Self::build(red, blue).max(Self::build(blue, red))
        // @solution-end
    }

    // @solution-start remove
    fn build(first: i32, second: i32) -> i32 {
        let (mut current, mut other) = (first, second);
        let mut row = 1;
        loop {
            if current < row {
                break row - 1;
            }
            current -= row;
            (current, other) = (other, current);
            row += 1;
        }
    }
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_four() {
        assert_eq!(Solution::max_height_of_triangle(2, 4), 3);
    }

    #[test]
    fn example_two_one() {
        assert_eq!(Solution::max_height_of_triangle(2, 1), 2);
    }

    #[test]
    fn example_one_one() {
        assert_eq!(Solution::max_height_of_triangle(1, 1), 1);
    }

    #[test]
    fn example_unbalanced() {
        assert_eq!(Solution::max_height_of_triangle(10, 1), 2);
        assert_eq!(Solution::max_height_of_triangle(1, 10), 2);
    }

    #[test]
    fn largest_inputs() {
        // Odd rows 1+3+...+19 = 100 and even rows 2+4+...+18 = 90, so 19 rows fit.
        assert_eq!(Solution::max_height_of_triangle(100, 100), 19);
        assert_eq!(Solution::max_height_of_triangle(100, 90), 19);
        assert_eq!(Solution::max_height_of_triangle(100, 89), 18);
    }
}
