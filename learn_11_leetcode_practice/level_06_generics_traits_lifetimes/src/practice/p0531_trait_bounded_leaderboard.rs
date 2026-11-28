// ==================================================================
// PRACTICE 0531: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0531_trait_bounded_leaderboard.md
// Run the tests:          cargo test practice::p0531
// ==================================================================

//! # Problem 0531: A Trait-Bounded Leaderboard
//!
//! - **Difficulty:** Medium
//! - **Topics:** traits with `&str`-returning methods, generic functions over `&[T]` with `T: Scored`, `where` clauses, sorting references
//! - **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax)
//!
//! ## Problem
//!
//! Define a trait for anything that can appear on a leaderboard:
//!
//! ```text
//! pub trait Scored {
//!     fn label(&self) -> &str;
//!     fn score(&self) -> u32;
//! }
//! ```
//!
//! Implement it for:
//!
//! - `Player { name: String, points: u32 }`: label is `name`, score is `points`.
//! - `Team { name: String, members: Vec<Player> }`: label is `name`, score is the **sum** of its
//!   members' points.
//!
//! Then write three generic functions that work for any `T: Scored`:
//!
//! - `best<T: Scored>(items: &[T]) -> Option<&T>`: the item with the highest score (the first one on
//!   ties), `None` when empty.
//! - `total_score<T: Scored>(items: &[T]) -> u64`: the sum of all scores, as `u64` so it cannot
//!   overflow.
//! - `leaderboard<T>(items: &[T]) -> Vec<String> where T: Scored`: one line per item,
//!   `"{rank}. {label} ({score})"`, sorted by score **descending** and then by label **ascending**.
//!   Ranks start at `1` and simply count lines (ties still get different ranks).
//!
//! ## Examples
//!
//! ```text
//! players = [ann: 30, bob: 50, cid: 30]
//! best(&players)          => bob
//! total_score(&players)   => 110
//! leaderboard(&players)   => ["1. bob (50)", "2. ann (30)", "3. cid (30)"]
//!
//! teams = [red: [ann 30, bob 50], blue: [cid 90]]
//! leaderboard(&teams)     => ["1. blue (90)", "2. red (80)"]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= items.len() <= 10^4`
//!
//! ## Hints
//!
//! 1. `fn label(&self) -> &str` uses lifetime elision: the returned `&str` borrows from `self`.
//! 2. Sort a `Vec<&T>` of references instead of the items themselves: no `Clone` bound needed.
//! 3. `sort_by(|a, b| b.score().cmp(&a.score()).then_with(|| a.label().cmp(b.label())))` encodes
//!    "score descending, label ascending".

/// Something that can appear on a leaderboard.
pub trait Scored {
    fn label(&self) -> &str;
    fn score(&self) -> u32;
}

pub struct Player {
    pub name: String,
    pub points: u32,
}

pub struct Team {
    pub name: String,
    pub members: Vec<Player>,
}

impl Scored for Player {
    fn label(&self) -> &str {
        todo!("Write your solution here")
    }

    fn score(&self) -> u32 {
        todo!("Write your solution here")
    }
}

impl Scored for Team {
    fn label(&self) -> &str {
        todo!("Write your solution here")
    }

    fn score(&self) -> u32 {
        todo!("Write your solution here")
    }
}

/// The highest-scoring item (the first one on ties).
pub fn best<T: Scored>(items: &[T]) -> Option<&T> {
    todo!("Write your solution here")
}

/// The sum of all scores.
pub fn total_score<T: Scored>(items: &[T]) -> u64 {
    todo!("Write your solution here")
}

/// `"{rank}. {label} ({score})"`, by score descending then label ascending.
pub fn leaderboard<T>(items: &[T]) -> Vec<String>
where
    T: Scored,
{
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn player(name: &str, points: u32) -> Player {
        Player {
            name: name.to_string(),
            points,
        }
    }

    fn players() -> Vec<Player> {
        vec![player("ann", 30), player("bob", 50), player("cid", 30)]
    }

    #[test]
    fn best_and_total_for_players() {
        let ps = players();
        assert_eq!(best(&ps).map(Scored::label), Some("bob"));
        assert_eq!(total_score(&ps), 110);
        let tie = vec![player("x", 5), player("y", 5)];
        assert_eq!(best(&tie).map(Scored::label), Some("x"));
    }

    #[test]
    fn player_leaderboard() {
        assert_eq!(
            leaderboard(&players()),
            vec!["1. bob (50)", "2. ann (30)", "3. cid (30)"]
        );
    }

    #[test]
    fn teams_sum_their_members() {
        let teams = vec![
            Team {
                name: "red".to_string(),
                members: vec![player("ann", 30), player("bob", 50)],
            },
            Team {
                name: "blue".to_string(),
                members: vec![player("cid", 90)],
            },
        ];
        assert_eq!(teams[0].score(), 80);
        assert_eq!(best(&teams).map(Scored::label), Some("blue"));
        assert_eq!(leaderboard(&teams), vec!["1. blue (90)", "2. red (80)"]);
    }

    #[test]
    fn empty_and_large() {
        let none: Vec<Player> = Vec::new();
        assert!(best(&none).is_none());
        assert_eq!(total_score(&none), 0);
        assert!(leaderboard(&none).is_empty());
        let many: Vec<Player> = (0..10_000)
            .map(|i| player(&format!("p{i:05}"), u32::MAX - (i % 3)))
            .collect();
        assert_eq!(
            total_score(&many),
            many.iter().map(|p| p.points as u64).sum::<u64>()
        );
        let board = leaderboard(&many);
        assert_eq!(board[0], format!("1. p00000 ({})", u32::MAX));
        assert_eq!(board[1], format!("2. p00003 ({})", u32::MAX));
    }
}
