// ==================================================================
// PRACTICE 0400: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0400_design_twitter.md
// Run the tests:          cargo test practice::p0400
// ==================================================================

//! # Problem 0400: Design Twitter
//!
//! - **Difficulty:** Medium
//! - **Topics:** `HashMap<i32, Vec<(u32, i32)>>` + `HashMap<i32, HashSet<i32>>`, k-way merge with a `BinaryHeap`, a global clock
//! - **Source:** [LeetCode 355. Design Twitter](https://leetcode.com/problems/design-twitter/)
//!
//! ## Problem
//!
//! Design a tiny Twitter where users post tweets, follow each other, and read a news feed.
//! Implement the `Twitter` struct:
//!
//! - `Twitter::new() -> Self`;
//! - `post_tweet(&mut self, user_id: i32, tweet_id: i32)`: the user posts a new tweet (every call
//!   uses a unique `tweet_id`);
//! - `get_news_feed(&self, user_id: i32) -> Vec<i32>`: the IDs of the **10 most recent** tweets
//!   posted by the user or by anyone the user follows, **most recent first** (fewer if there are
//!   fewer tweets);
//! - `follow(&mut self, follower_id: i32, followee_id: i32)`: the follower starts following the
//!   followee;
//! - `unfollow(&mut self, follower_id: i32, followee_id: i32)`: the follower stops following them.
//!
//! "Most recent" means posted later, whatever the user. Following yourself has no extra effect
//! (your own tweets are always in your feed). The mutating methods take `&mut self` instead of the
//! template's `&self`, as LeetCode's comment allows.
//!
//! ## Examples
//!
//! ```text
//! let mut t = Twitter::new();
//! t.post_tweet(1, 5);
//! t.get_news_feed(1) => [5]
//! t.follow(1, 2);
//! t.post_tweet(2, 6);
//! t.get_news_feed(1) => [6, 5]     (tweet 6 is newer)
//! t.unfollow(1, 2);
//! t.get_news_feed(1) => [5]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= user_id, follower_id, followee_id <= 500`, `0 <= tweet_id <= 10^4`
//! - At most `3 * 10^4` calls in total.
//!
//! ## Hints
//!
//! 1. Give every tweet a timestamp from a global counter; store each user's tweets in posting
//!    order (a `Vec`, newest last).
//! 2. Store follow relations as `HashMap<i32, HashSet<i32>>` so follow/unfollow are `O(1)`.
//! 3. The feed is a merge of several "newest first" lists: start a max-heap with each author's
//!    newest tweet and, after popping an author's tweet, push that author's next older one.

use std::collections::{BinaryHeap, HashMap, HashSet};

/// Users, tweets and follow relations.
pub struct Twitter {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl Twitter {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        todo!("Write your solution here")
    }

    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        todo!("Write your solution here")
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        todo!("Write your solution here")
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut t = Twitter::new();
        t.post_tweet(1, 5);
        assert_eq!(t.get_news_feed(1), vec![5]);
        t.follow(1, 2);
        t.post_tweet(2, 6);
        assert_eq!(t.get_news_feed(1), vec![6, 5]);
        t.unfollow(1, 2);
        assert_eq!(t.get_news_feed(1), vec![5]);
    }

    #[test]
    fn empty_feeds_and_unknown_users() {
        let mut t = Twitter::new();
        assert_eq!(t.get_news_feed(7), Vec::<i32>::new());
        t.unfollow(7, 8);
        t.follow(7, 8);
        assert_eq!(t.get_news_feed(7), Vec::<i32>::new());
        t.post_tweet(8, 1);
        assert_eq!(t.get_news_feed(7), vec![1]);
        assert_eq!(t.get_news_feed(8), vec![1]);
    }

    #[test]
    fn following_yourself_does_not_duplicate() {
        let mut t = Twitter::new();
        t.post_tweet(1, 10);
        t.post_tweet(1, 11);
        t.follow(1, 1);
        assert_eq!(t.get_news_feed(1), vec![11, 10]);
    }

    #[test]
    fn feed_is_limited_to_ten_and_interleaves_users() {
        let mut t = Twitter::new();
        for i in 0..30 {
            // Users 1, 2 and 3 post in turn: tweet i by user i % 3 + 1.
            t.post_tweet(i % 3 + 1, i);
        }
        t.follow(1, 2);
        assert_eq!(
            t.get_news_feed(1),
            vec![28, 27, 25, 24, 22, 21, 19, 18, 16, 15]
        );
        t.follow(1, 3);
        assert_eq!(t.get_news_feed(1), (20..30).rev().collect::<Vec<i32>>());
        assert_eq!(
            t.get_news_feed(3),
            vec![29, 26, 23, 20, 17, 14, 11, 8, 5, 2]
        );
    }

    #[test]
    fn many_users() {
        let mut t = Twitter::new();
        for user in 1..=500 {
            t.post_tweet(user, user * 10);
            t.follow(1, user);
        }
        assert_eq!(
            t.get_news_feed(1),
            (491..=500).rev().map(|u| u * 10).collect::<Vec<i32>>()
        );
        assert_eq!(t.get_news_feed(250), vec![2_500]);
    }
}
