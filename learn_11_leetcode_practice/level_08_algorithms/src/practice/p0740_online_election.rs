// ==================================================================
// PRACTICE 0740: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0740_online_election.md
// Run the tests:          cargo test practice::p0740
// ==================================================================

//! # Problem 0740: Online Election
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct, precomputation + binary search, `partition_point`, private fields
//! - **Source:** [LeetCode 911. Online Election](https://leetcode.com/problems/online-election/)
//!
//! ## Problem
//!
//! In an election, the `i`-th vote was cast for candidate `persons[i]` at time `times[i]`. Answer
//! queries of the form "who was leading at time `t`?". Votes cast exactly at time `t` count for
//! that query. When several candidates are tied for the most votes, the one among them who received
//! the **most recent** vote leads.
//!
//! Implement the struct `TopVotedCandidate`:
//!
//! - `TopVotedCandidate::new(persons: Vec<i32>, times: Vec<i32>) -> Self` stores the votes;
//! - `q(&self, t: i32) -> i32` returns the candidate leading at time `t`.
//!
//! ## Examples
//!
//! ```text
//! persons = [0, 1, 1, 0, 0, 1, 0], times = [0, 5, 10, 15, 20, 25, 30]
//!
//! q(3)  = 0     (only the vote at time 0 counts)
//! q(12) = 1     (votes 0:1, 1:2)
//! q(25) = 1     (0:3, 1:3, tied; 1 got the most recent vote, at time 25)
//! q(15) = 0     (0:2, 1:2, tied; 0 voted at time 15)
//! q(24) = 0     (0:3, 1:2)
//! q(8)  = 1     (0:1, 1:1, tied; 1 voted at time 5)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= persons.len() <= 5000` and `times.len() == persons.len()`
//! - `0 <= persons[i] < persons.len()`
//! - `0 <= times[i] <= 10^9`, `times` is strictly increasing
//! - `times[0] <= t <= 10^9` for every query; at most `10^4` queries.
//!
//! ## Hints
//!
//! 1. The leader only changes when a vote arrives. Compute the leader after every vote once, in
//!    `new`.
//! 2. A query then needs the last vote at or before `t`: a binary search over `times`.

pub struct TopVotedCandidate {}

impl TopVotedCandidate {
    pub fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        todo!("Write your solution here")
    }

    pub fn q(&self, t: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> TopVotedCandidate {
        TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30])
    }

    #[test]
    fn example_queries() {
        let election = sample();
        assert_eq!(election.q(3), 0);
        assert_eq!(election.q(12), 1);
        assert_eq!(election.q(25), 1);
        assert_eq!(election.q(15), 0);
        assert_eq!(election.q(24), 0);
        assert_eq!(election.q(8), 1);
    }

    #[test]
    fn query_exactly_at_vote_times_and_after_the_end() {
        let election = sample();
        assert_eq!(election.q(0), 0);
        assert_eq!(election.q(5), 1);
        assert_eq!(election.q(30), 0);
        assert_eq!(election.q(1_000_000_000), 0);
    }

    #[test]
    fn single_vote() {
        let election = TopVotedCandidate::new(vec![0], vec![7]);
        assert_eq!(election.q(7), 0);
        assert_eq!(election.q(100), 0);
    }

    #[test]
    fn clear_leader_is_not_replaced_by_newcomers() {
        let election = TopVotedCandidate::new(vec![2, 2, 2, 0, 1], vec![1, 2, 3, 4, 5]);
        assert_eq!(election.q(5), 2);
    }

    #[test]
    fn many_votes_and_queries() {
        // Candidates 0, 1, 2, 0, 1, 2, ...: after every vote the latest voter is tied for the lead.
        let n = 5000;
        let persons: Vec<i32> = (0..n).map(|i| i % 3).collect();
        let times: Vec<i32> = (0..n).map(|i| i * 10).collect();
        let election = TopVotedCandidate::new(persons, times);
        for i in (0..n).step_by(13) {
            assert_eq!(election.q(i * 10 + 9), i % 3);
        }
    }
}
