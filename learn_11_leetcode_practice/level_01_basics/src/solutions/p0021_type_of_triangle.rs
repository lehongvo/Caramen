//! # Problem 0021: Type of Triangle
//!
//! - **Difficulty:** Easy
//! - **Topics:** comparison and logical operators, indexing a `Vec`, `String::from`
//! - **Source:** [LeetCode 3024. Type of Triangle](https://leetcode.com/problems/type-of-triangle/)
//!
//! ## Problem
//!
//! You are given a vector `nums` with exactly three positive integers: candidate side lengths of
//! a triangle. Three lengths form a real triangle only if **every** pair of sides is strictly
//! longer, together, than the remaining side. Return:
//!
//! - `"none"` if the three lengths cannot form a triangle;
//! - `"equilateral"` if all three sides are equal;
//! - `"isosceles"` if exactly two sides are equal;
//! - `"scalene"` if all three sides are different.
//!
//! Implement `Solution::triangle_type(nums: Vec<i32>) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [3, 3, 3]
//! Output: "equilateral"
//!
//! Input:  nums = [3, 4, 5]
//! Output: "scalene"
//!
//! Input:  nums = [5, 5, 3]
//! Output: "isosceles"
//!
//! Input:  nums = [1, 2, 3]
//! Output: "none"
//! Explanation: 1 + 2 is not strictly greater than 3 (the "triangle" is flat).
//! ```
//!
//! ## Constraints
//!
//! - `nums.len() == 3`
//! - `1 <= nums[i] <= 100`
//!
//! ## Hints
//!
//! 1. Copy the three values into variables first: `let (a, b, c) = (nums[0], nums[1], nums[2]);`.
//! 2. Check the "not a triangle" case before anything else.
//! 3. Equilateral must be tested before isosceles, because an equilateral triangle also has two equal sides.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** first reject impossible triangles with `a + b <= c || a + c <= b || b + c <= a`. Then
//! count equalities: all equal is equilateral, any pair equal is isosceles, otherwise scalene.
//!
//! **Complexity:** `O(1)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `nums[0]` reads an element of a `Vec` by index. `i32` is `Copy`, so the value is copied out
//!   and `nums` is still usable.
//! - A tuple on the right-hand side and a tuple pattern on the left (`let (a, b, c) = ...`) bind
//!   several variables at once.
//! - The `if`/`else if` chain evaluates to a `&str`, and `String::from` turns it into the owned
//!   `String` that LeetCode's signature requires.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        // @solution-start
        let (a, b, c) = (nums[0], nums[1], nums[2]);
        let kind = if a + b <= c || a + c <= b || b + c <= a {
            "none"
        } else if a == b && b == c {
            "equilateral"
        } else if a == b || b == c || a == c {
            "isosceles"
        } else {
            "scalene"
        };
        String::from(kind)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_equilateral() {
        assert_eq!(Solution::triangle_type(vec![3, 3, 3]), "equilateral");
    }

    #[test]
    fn example_scalene() {
        assert_eq!(Solution::triangle_type(vec![3, 4, 5]), "scalene");
    }

    #[test]
    fn example_isosceles_any_position() {
        assert_eq!(Solution::triangle_type(vec![5, 5, 3]), "isosceles");
        assert_eq!(Solution::triangle_type(vec![5, 3, 5]), "isosceles");
        assert_eq!(Solution::triangle_type(vec![3, 5, 5]), "isosceles");
    }

    #[test]
    fn example_flat_triangle_is_none() {
        assert_eq!(Solution::triangle_type(vec![1, 2, 3]), "none");
        assert_eq!(Solution::triangle_type(vec![3, 1, 2]), "none");
    }

    #[test]
    fn two_equal_sides_but_too_short() {
        // 1 + 1 < 100, so this is not a triangle even though two sides are equal.
        assert_eq!(Solution::triangle_type(vec![1, 1, 100]), "none");
        assert_eq!(Solution::triangle_type(vec![100, 100, 1]), "isosceles");
    }
}
