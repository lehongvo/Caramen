// ==================================================================
// PRACTICE 0399: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0399_design_a_food_rating_system.md
// Run the tests:          cargo test practice::p0399
// ==================================================================

//! # Problem 0399: Design a Food Rating System
//!
//! - **Difficulty:** Medium
//! - **Topics:** two indexes kept in sync: `HashMap<String, (String, i32)>` + `HashMap<String, BTreeSet<(Reverse<i32>, String)>>`, ordered-set updates
//! - **Source:** [LeetCode 2353. Design a Food Rating System](https://leetcode.com/problems/design-a-food-rating-system/)
//!
//! ## Problem
//!
//! A food rating system stores, for every food, its cuisine and its rating. Implement the
//! `FoodRatings` struct:
//!
//! - `FoodRatings::new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self`:
//!   food `foods[i]` belongs to cuisine `cuisines[i]` and has rating `ratings[i]`;
//! - `change_rating(&mut self, food: String, new_rating: i32)`: set the rating of `food`;
//! - `highest_rated(&self, cuisine: String) -> String`: the food with the **highest** rating in that
//!   cuisine; among equally rated foods, the **lexicographically smallest** name.
//!
//! Food names are unique, every `food` passed to `change_rating` exists, and every `cuisine` passed
//! to `highest_rated` has at least one food. `change_rating` takes `&mut self` instead of the
//! template's `&self`, as LeetCode's comment allows.
//!
//! ## Examples
//!
//! ```text
//! let mut fr = FoodRatings::new(
//!     ["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"],
//!     ["korean", "japanese", "japanese", "greek", "japanese", "korean"],
//!     [9, 12, 8, 15, 14, 7]);
//! fr.highest_rated("korean")   => "kimchi"   (9 beats 7)
//! fr.highest_rated("japanese") => "ramen"    (14)
//! fr.change_rating("sushi", 16);
//! fr.highest_rated("japanese") => "sushi"
//! fr.change_rating("ramen", 16);
//! fr.highest_rated("japanese") => "ramen"    (tie at 16: "ramen" < "sushi")
//! ```
//!
//! ## Constraints
//!
//! - `1 <= foods.len() <= 2 * 10^4`, `1 <= ratings[i], new_rating <= 10^8`
//! - Names have 1 to 10 lowercase letters.
//! - At most `2 * 10^4` calls to `change_rating` and `highest_rated` in total.
//!
//! ## Hints
//!
//! 1. Per cuisine, keep the foods in an ordered set whose **first** element is the answer:
//!    `BTreeSet<(Reverse<i32>, String)>` sorts by rating descending, then name ascending.
//! 2. To change a rating you must remove the **old** entry from the set, so also remember each
//!    food's cuisine and current rating in a `HashMap`.

use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap};

/// Ratings of foods, queryable per cuisine.
pub struct FoodRatings {}

impl FoodRatings {
    pub fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        todo!("Write your solution here")
    }

    pub fn change_rating(&mut self, food: String, new_rating: i32) {
        todo!("Write your solution here")
    }

    pub fn highest_rated(&self, cuisine: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    fn s(x: &str) -> String {
        x.to_string()
    }

    fn example() -> FoodRatings {
        FoodRatings::new(
            strings(&["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"]),
            strings(&[
                "korean", "japanese", "japanese", "greek", "japanese", "korean",
            ]),
            vec![9, 12, 8, 15, 14, 7],
        )
    }

    #[test]
    fn example_sequence() {
        let mut fr = example();
        assert_eq!(fr.highest_rated(s("korean")), "kimchi");
        assert_eq!(fr.highest_rated(s("japanese")), "ramen");
        fr.change_rating(s("sushi"), 16);
        assert_eq!(fr.highest_rated(s("japanese")), "sushi");
        fr.change_rating(s("ramen"), 16);
        assert_eq!(fr.highest_rated(s("japanese")), "ramen");
    }

    #[test]
    fn lowering_the_top_rating() {
        let mut fr = example();
        assert_eq!(fr.highest_rated(s("greek")), "moussaka");
        fr.change_rating(s("kimchi"), 1);
        assert_eq!(fr.highest_rated(s("korean")), "bulgogi");
        fr.change_rating(s("bulgogi"), 1);
        // Tie at 1: "bulgogi" < "kimchi".
        assert_eq!(fr.highest_rated(s("korean")), "bulgogi");
    }

    #[test]
    fn cuisines_are_independent() {
        let mut fr = example();
        fr.change_rating(s("moussaka"), 100_000_000);
        assert_eq!(fr.highest_rated(s("japanese")), "ramen");
        assert_eq!(fr.highest_rated(s("greek")), "moussaka");
    }

    #[test]
    fn many_foods_and_updates() {
        let foods: Vec<String> = (0..20_000).map(|i| format!("f{i:05}")).collect();
        let cuisines: Vec<String> = (0..20_000).map(|i| format!("c{}", i % 10)).collect();
        let ratings: Vec<i32> = (0..20_000).map(|i| i % 1000 + 1).collect();
        let mut fr = FoodRatings::new(foods, cuisines, ratings);
        // In cuisine c0 the best rating is 991 (i = 990, 1990, ...); the smallest name wins.
        assert_eq!(fr.highest_rated(s("c0")), "f00990");
        for i in (0..20_000).step_by(10) {
            fr.change_rating(format!("f{i:05}"), 5);
        }
        // Every food of c0 now has rating 5.
        assert_eq!(fr.highest_rated(s("c0")), "f00000");
        assert_eq!(fr.highest_rated(s("c9")), "f00999");
    }
}
