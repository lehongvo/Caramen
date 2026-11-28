//! # Problem 0390: Dota2 Senate
//!
//! - **Difficulty:** Medium
//! - **Topics:** two `VecDeque` queues of turn numbers, greedy simulation, re-queueing with an offset
//! - **Source:** [LeetCode 649. Dota2 Senate](https://leetcode.com/problems/dota2-senate/)
//!
//! ## Problem
//!
//! A senate has senators from two parties, Radiant (`'R'`) and Dire (`'D'`), given as a string
//! `senate` in the order in which they act. The vote proceeds in rounds; in each round every senator
//! who still has their rights acts once, in order, and does **one** of the following:
//!
//! - **ban** one senator of the other party, who loses all rights in this and all later rounds;
//! - **announce victory**, if every senator who still has rights belongs to their own party.
//!
//! Every senator plays optimally for their party. Return `"Radiant"` or `"Dire"`, the party that
//! eventually announces victory.
//!
//! Implement `Solution::predict_party_victory(senate: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  senate = "RD"
//! Output: "Radiant"   (R bans D, then R is alone)
//!
//! Input:  senate = "RDD"
//! Output: "Dire"      (R bans the first D; the second D bans R)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= senate.len() <= 10^4`
//! - `senate` contains only `'R'` and `'D'`.
//!
//! ## Hints
//!
//! 1. The best move is always to ban the **next** opponent who would act, since that one would
//!    otherwise ban a member of your party first.
//! 2. Keep one queue per party with the positions of its active senators.
//! 3. Compare the two front positions: the earlier senator bans the other and comes back for the
//!    next round with position `+ n`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** greedy simulation with two queues of turn numbers. Repeatedly take the front senator of
//! each party; the one who acts earlier bans the other (who is simply dropped) and re-enters the
//! queue at `position + n`, which places them after everybody else in the current round. The party
//! whose queue is non-empty at the end wins.
//!
//! **Complexity:** `O(n)` time (every comparison bans one senator), `O(n)` space.
//!
//! **Rust concepts:**
//! - Two `VecDeque<usize>`s built with `push_back` in one pass over `senate.bytes()`.
//! - `while let (Some(&r), Some(&d)) = (radiant.front(), dire.front())` loops while both parties
//!   have senators, peeking at both fronts at once.
//! - The `+ n` trick avoids simulating rounds explicitly: positions keep increasing, so their
//!   order is the acting order across rounds.
// @solution-end

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        // @solution-start
        let n = senate.len();
        let mut radiant: VecDeque<usize> = VecDeque::new();
        let mut dire: VecDeque<usize> = VecDeque::new();
        for (i, b) in senate.bytes().enumerate() {
            if b == b'R' {
                radiant.push_back(i);
            } else {
                dire.push_back(i);
            }
        }
        while let (Some(&r), Some(&d)) = (radiant.front(), dire.front()) {
            radiant.pop_front();
            dire.pop_front();
            if r < d {
                radiant.push_back(r + n);
            } else {
                dire.push_back(d + n);
            }
        }
        if radiant.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn winner(senate: &str) -> String {
        Solution::predict_party_victory(senate.to_string())
    }

    #[test]
    fn example_one() {
        assert_eq!(winner("RD"), "Radiant");
    }

    #[test]
    fn example_two() {
        assert_eq!(winner("RDD"), "Dire");
    }

    #[test]
    fn single_party_and_single_senator() {
        assert_eq!(winner("D"), "Dire");
        assert_eq!(winner("RRR"), "Radiant");
    }

    #[test]
    fn order_matters_more_than_numbers() {
        // Two D's act first and ban two R's; the remaining R bans one D, the last D wins.
        assert_eq!(winner("DDRRR"), "Dire");
        assert_eq!(winner("DRRDRDRDRDDRDRDR"), "Radiant");
    }

    #[test]
    fn long_senate() {
        let senate = format!("{}{}", "D".repeat(5_000), "R".repeat(5_000));
        assert_eq!(winner(&senate), "Dire");
        let alternating: String = (0..10_000)
            .map(|i| if i % 2 == 0 { 'R' } else { 'D' })
            .collect();
        assert_eq!(winner(&alternating), "Radiant");
    }
}
