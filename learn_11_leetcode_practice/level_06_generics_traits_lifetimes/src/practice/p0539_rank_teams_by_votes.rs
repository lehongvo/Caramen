// ==================================================================
// PRACTICE 0539: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0539_rank_teams_by_votes.md
// Run the tests:          cargo test practice::p0539
// ==================================================================

//! # Problem 0539: Rank Teams by Votes
//!
//! - **Difficulty:** Medium
//! - **Topics:** manual `Ord` on a struct holding a `Vec`, lexicographic order of `Vec<u32>`, sorting with a custom order
//! - **Source:** [LeetCode 1366. Rank Teams by Votes](https://leetcode.com/problems/rank-teams-by-votes/)
//!
//! ## Problem
//!
//! Several voters each rank **all** teams (every team is an uppercase letter). Vote `"BCA"` means: `B`
//! first, `C` second, `A` third. Rank the teams like this:
//!
//! 1. The team with the most first-place votes is ranked higher.
//! 2. On a tie, compare second-place votes; if still tied, third-place votes, and so on.
//! 3. If two teams are tied on every position, the alphabetically smaller letter is ranked higher.
//!
//! Return the teams as one string, from the highest rank to the lowest.
//!
//! Implement `Solution::rank_teams(votes: Vec<String>) -> String`.
//!
//! Model each team as `TeamVotes { team: char, counts: Vec<u32> }` where `counts[p]` is the number of
//! votes placing the team at position `p`, and give it an `Ord` where "smaller" means "ranked higher".
//! Then a plain `sort()` produces the answer.
//!
//! ## Examples
//!
//! ```text
//! Input:  votes = ["ABC", "ACB", "ABC", "ACB", "ACB"]
//! Output: "ACB"      A has 5 first places; C beats B on second places (3 vs 2)
//!
//! Input:  votes = ["WXYZ", "XYZW"]
//! Output: "XWYZ"     W and X tie on first places; X has one second place, W none
//!
//! Input:  votes = ["BCA", "CAB", "CBA", "ABC", "ACB", "BAC"]
//! Output: "ABC"      complete tie, so alphabetical
//! ```
//!
//! ## Constraints
//!
//! - `1 <= votes.len() <= 1000`
//! - `1 <= votes[i].len() <= 26`, all votes have the same length.
//! - Each vote is a permutation of the same set of distinct uppercase letters.
//!
//! ## Hints
//!
//! 1. `Vec<u32>` already implements `Ord` lexicographically: it compares position 0, then 1, ...
//! 2. "More votes ranks higher" means comparing counts in **reverse**: `other.counts.cmp(&self.counts)`.
//! 3. Finish with `.then(self.team.cmp(&other.team))`.

use std::cmp::Ordering;

pub struct Solution;

/// Vote counts of one team; smaller means ranked higher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamVotes {
    pub team: char,
    pub counts: Vec<u32>,
}

impl Ord for TeamVotes {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!("Write your solution here")
    }
}

impl PartialOrd for TeamVotes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!("Write your solution here")
    }
}

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn votes(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_clear_winner() {
        assert_eq!(
            Solution::rank_teams(votes(&["ABC", "ACB", "ABC", "ACB", "ACB"])),
            "ACB"
        );
    }

    #[test]
    fn example_second_place_decides() {
        assert_eq!(Solution::rank_teams(votes(&["WXYZ", "XYZW"])), "XWYZ");
    }

    #[test]
    fn example_complete_tie() {
        assert_eq!(
            Solution::rank_teams(votes(&["BCA", "CAB", "CBA", "ABC", "ACB", "BAC"])),
            "ABC"
        );
    }

    #[test]
    fn single_vote_and_single_team() {
        assert_eq!(
            Solution::rank_teams(votes(&["ZMNAGUEDSJYLBOPHRQICWFXTVK"])),
            "ZMNAGUEDSJYLBOPHRQICWFXTVK"
        );
        assert_eq!(Solution::rank_teams(votes(&["M", "M"])), "M");
    }

    #[test]
    fn ordering_of_team_votes() {
        let a = TeamVotes {
            team: 'A',
            counts: vec![2, 0],
        };
        let b = TeamVotes {
            team: 'B',
            counts: vec![2, 1],
        };
        let c = TeamVotes {
            team: 'C',
            counts: vec![2, 1],
        };
        assert!(b < a); // more second places ranks higher
        assert!(b < c); // tie broken by letter
        let mut all = vec![a.clone(), c.clone(), b.clone()];
        all.sort();
        assert_eq!(all, vec![b, c, a]);
    }

    #[test]
    fn many_votes() {
        let mut list = vec!["ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(); 600];
        list.extend(vec!["ZYXWVUTSRQPONMLKJIHGFEDCBA".to_string(); 400]);
        // A has 600 first places and Z 400; B and Y have none, but B has 600 second places, ...
        assert_eq!(Solution::rank_teams(list), "AZBYCXDWEVFUGTHSIRJQKPLOMN");
    }
}
