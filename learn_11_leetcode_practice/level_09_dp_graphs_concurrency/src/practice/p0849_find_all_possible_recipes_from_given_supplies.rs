// ==================================================================
// PRACTICE 0849: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0849_find_all_possible_recipes_from_given_supplies.md
// Run the tests:          cargo test practice::p0849
// ==================================================================

//! # Problem 0849: Find All Possible Recipes from Given Supplies
//!
//! - **Difficulty:** Medium
//! - **Topics:** topological sort over `String` names, `HashMap<&str, Vec<usize>>`, borrowing keys
//!   from the input
//! - **Source:** [LeetCode 2115. Find All Possible Recipes from Given Supplies](https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/)
//!
//! ## Problem
//!
//! You know `n` recipes. Recipe `recipes[i]` can be cooked if you have every item in
//! `ingredients[i]`. An ingredient is either one of your `supplies` (you have an unlimited amount
//! of each) or another recipe, which you must be able to cook first. Recipes can depend on each
//! other, possibly in a cycle (then none of the recipes on the cycle can be cooked). Return every
//! recipe that can be cooked, in any order.
//!
//! Implement `Solution::find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  recipes = ["bread"], ingredients = [["yeast", "flour"]],
//!         supplies = ["yeast", "flour", "corn"]
//! Output: ["bread"]
//!
//! Input:  recipes = ["bread", "sandwich"],
//!         ingredients = [["yeast", "flour"], ["bread", "meat"]],
//!         supplies = ["yeast", "flour", "meat"]
//! Output: ["bread", "sandwich"]
//! Explanation: bread first, then the sandwich that needs it.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= recipes.len() == ingredients.len() <= 100`, `1 <= ingredients[i].len() <= 100`
//! - `1 <= supplies.len() <= 100`; names are 1 to 10 lowercase letters
//! - Recipe names and supply names are all distinct; `ingredients[i]` has no duplicates.
//!
//! ## Hints
//!
//! 1. Build a graph whose edges go from an ingredient to every recipe that uses it. A recipe's
//!    in-degree is its number of ingredients.
//! 2. Start Kahn's algorithm from the supplies. Each time an item becomes available, decrement the
//!    in-degree of the recipes that use it; a recipe reaching `0` is cookable and becomes available.
//! 3. Ingredients that are neither supplies nor cookable recipes simply never become available.

use std::collections::{HashMap, VecDeque};

pub struct Solution;

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    fn run(recipes: &[&str], ingredients: &[&[&str]], supplies: &[&str]) -> Vec<String> {
        let ing = ingredients.iter().map(|l| strings(l)).collect();
        let mut got = Solution::find_all_recipes(strings(recipes), ing, strings(supplies));
        got.sort();
        got
    }

    #[test]
    fn example_bread() {
        let got = run(
            &["bread"],
            &[&["yeast", "flour"]],
            &["yeast", "flour", "corn"],
        );
        assert_eq!(got, vec!["bread"]);
    }

    #[test]
    fn example_sandwich_needs_bread() {
        let got = run(
            &["bread", "sandwich"],
            &[&["yeast", "flour"], &["bread", "meat"]],
            &["yeast", "flour", "meat"],
        );
        assert_eq!(got, vec!["bread", "sandwich"]);
    }

    #[test]
    fn three_levels() {
        let got = run(
            &["bread", "sandwich", "burger"],
            &[
                &["yeast", "flour"],
                &["bread", "meat"],
                &["sandwich", "meat", "bread"],
            ],
            &["yeast", "flour", "meat"],
        );
        assert_eq!(got, vec!["bread", "burger", "sandwich"]);
    }

    #[test]
    fn missing_ingredient() {
        let got = run(
            &["cake", "pie"],
            &[&["egg", "sugar"], &["flour"]],
            &["egg", "flour"],
        );
        assert_eq!(got, vec!["pie"]);
    }

    #[test]
    fn cycle_blocks_both_recipes() {
        let got = run(&["a", "b", "c"], &[&["b"], &["a"], &["x"]], &["x"]);
        assert_eq!(got, vec!["c"]);
    }
}
