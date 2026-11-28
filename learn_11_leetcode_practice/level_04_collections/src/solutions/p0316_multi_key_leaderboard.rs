//! # Problem 0316: A Multi-Key Leaderboard
//!
//! - **Difficulty:** Easy
//! - **Topics:** `std::cmp::Ordering`, `Ord::cmp`, `Ordering::then_with`, `Ordering::reverse`, comparator functions
//! - **Source:** Original · [`Ordering`](https://doc.rust-lang.org/std/cmp/enum.Ordering.html)
//!
//! ## Problem
//!
//! A puzzle game ranks players with this struct:
//!
//! ```text
//! pub struct Player { pub name: String, pub score: u32, pub time_secs: u32 }
//! ```
//!
//! The ranking rules are, in order of importance:
//!
//! 1. a **higher** `score` ranks first;
//! 2. on equal scores, a **lower** `time_secs` ranks first;
//! 3. on equal score and time, names in alphabetical order.
//!
//! Implement:
//!
//! - `compare_players(a: &Player, b: &Player) -> Ordering` returning `Ordering::Less` when `a`
//!   ranks before `b`, `Ordering::Greater` when `a` ranks after `b`, and `Ordering::Equal` when
//!   all three fields tie;
//! - `leaderboard(players: &mut [Player])` sorting the slice by that ranking;
//! - `top_names(players: &[Player], k: usize) -> Vec<String>` returning the names of the best `k`
//!   players in rank order (all of them if there are fewer than `k`), **without** modifying
//!   `players`.
//!
//! ## Examples
//!
//! ```text
//! players = [("Ivy", 90, 300), ("Max", 95, 400), ("Ada", 90, 250), ("Bo", 90, 250)]
//!
//! compare_players(Max, Ivy) => Less      (higher score)
//! compare_players(Ivy, Ada) => Greater   (same score, Ivy is slower)
//! compare_players(Ada, Bo)  => Less      (same score and time, "Ada" < "Bo")
//! leaderboard => Max, Ada, Bo, Ivy
//! top_names(players, 2) => ["Max", "Ada"]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= players.len() <= 10^4`
//!
//! ## Hints
//!
//! 1. `x.cmp(&y)` compares two values and returns an `Ordering`.
//! 2. `first.then_with(|| second)` uses `second` only when `first` is `Equal`.
//! 3. For "higher first" either swap the operands (`b.score.cmp(&a.score)`) or call `.reverse()`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** encode the three rules as a chain of comparisons. The score comparison is reversed
//! because a higher score must come first; the other two are natural ascending orders.
//! `leaderboard` passes the comparator to `sort_by`, and `top_names` sorts a vector of references
//! so the caller's slice stays untouched.
//!
//! **Rust concepts:**
//! - `Ordering` is a plain enum with three variants; `then_with` takes a closure so the later keys
//!   are computed lazily, while `then` takes an already computed `Ordering`.
//! - A function item like `compare_players` can be passed where a closure is expected:
//!   `players.sort_by(compare_players)`.
//! - `Vec<&Player>` sorts pointers instead of moving whole structs; the comparator is adapted with
//!   `|a, b| compare_players(a, b)` (auto-deref turns `&&Player` into `&Player`).
//! - `take(k)` stops after `k` items, and does nothing special when there are fewer.
// @solution-end

use std::cmp::Ordering;

/// One player on the leaderboard.
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub time_secs: u32,
}

/// Higher score first, then lower time, then name.
pub fn compare_players(a: &Player, b: &Player) -> Ordering {
    // @solution-start
    a.score
        .cmp(&b.score)
        .reverse()
        .then_with(|| a.time_secs.cmp(&b.time_secs))
        .then_with(|| a.name.cmp(&b.name))
    // @solution-end
}

/// Sorts the players by rank.
pub fn leaderboard(players: &mut [Player]) {
    // @solution-start
    players.sort_by(compare_players);
    // @solution-end
}

/// Returns the names of the best `k` players, best first.
pub fn top_names(players: &[Player], k: usize) -> Vec<String> {
    // @solution-start
    let mut ranked: Vec<&Player> = players.iter().collect();
    ranked.sort_by(|a, b| compare_players(a, b));
    ranked.into_iter().take(k).map(|p| p.name.clone()).collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn player(name: &str, score: u32, time_secs: u32) -> Player {
        Player {
            name: name.to_string(),
            score,
            time_secs,
        }
    }

    fn example() -> Vec<Player> {
        vec![
            player("Ivy", 90, 300),
            player("Max", 95, 400),
            player("Ada", 90, 250),
            player("Bo", 90, 250),
        ]
    }

    #[test]
    fn compare_players_examples() {
        let p = example();
        assert_eq!(compare_players(&p[1], &p[0]), Ordering::Less);
        assert_eq!(compare_players(&p[0], &p[2]), Ordering::Greater);
        assert_eq!(compare_players(&p[2], &p[3]), Ordering::Less);
        assert_eq!(compare_players(&p[3], &p[3].clone()), Ordering::Equal);
    }

    #[test]
    fn leaderboard_example() {
        let mut p = example();
        leaderboard(&mut p);
        let names: Vec<&str> = p.iter().map(|x| x.name.as_str()).collect();
        assert_eq!(names, vec!["Max", "Ada", "Bo", "Ivy"]);
    }

    #[test]
    fn top_names_does_not_modify_input() {
        let p = example();
        assert_eq!(top_names(&p, 2), vec!["Max".to_string(), "Ada".to_string()]);
        assert_eq!(top_names(&p, 10).len(), 4);
        assert_eq!(top_names(&p, 0), Vec::<String>::new());
        assert_eq!(p, example());
    }

    #[test]
    fn many_players() {
        let mut p: Vec<Player> = (0..10_000u32)
            .map(|i| player(&format!("{:05}", 9_999 - i), i % 100, i % 7))
            .collect();
        leaderboard(&mut p);
        assert!(
            p.windows(2)
                .all(|w| compare_players(&w[0], &w[1]) != Ordering::Greater)
        );
        assert_eq!(p[0].score, 99);
        assert_eq!(p[0].time_secs, 0);
    }
}
