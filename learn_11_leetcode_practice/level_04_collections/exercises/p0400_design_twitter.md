# Problem 0400: Design Twitter

- **Difficulty:** Medium
- **Topics:** `HashMap<i32, Vec<(u32, i32)>>` + `HashMap<i32, HashSet<i32>>`, k-way merge with a `BinaryHeap`, a global clock
- **Source:** [LeetCode 355. Design Twitter](https://leetcode.com/problems/design-twitter/)

## Problem

Design a tiny Twitter where users post tweets, follow each other, and read a news feed.
Implement the `Twitter` struct:

- `Twitter::new() -> Self`;
- `post_tweet(&mut self, user_id: i32, tweet_id: i32)`: the user posts a new tweet (every call
  uses a unique `tweet_id`);
- `get_news_feed(&self, user_id: i32) -> Vec<i32>`: the IDs of the **10 most recent** tweets
  posted by the user or by anyone the user follows, **most recent first** (fewer if there are
  fewer tweets);
- `follow(&mut self, follower_id: i32, followee_id: i32)`: the follower starts following the
  followee;
- `unfollow(&mut self, follower_id: i32, followee_id: i32)`: the follower stops following them.

"Most recent" means posted later, whatever the user. Following yourself has no extra effect
(your own tweets are always in your feed). The mutating methods take `&mut self` instead of the
template's `&self`, as LeetCode's comment allows.

## Examples

```text
let mut t = Twitter::new();
t.post_tweet(1, 5);
t.get_news_feed(1) => [5]
t.follow(1, 2);
t.post_tweet(2, 6);
t.get_news_feed(1) => [6, 5]     (tweet 6 is newer)
t.unfollow(1, 2);
t.get_news_feed(1) => [5]
```

## Constraints

- `1 <= user_id, follower_id, followee_id <= 500`, `0 <= tweet_id <= 10^4`
- At most `3 * 10^4` calls in total.

## Hints

1. Give every tweet a timestamp from a global counter; store each user's tweets in posting
   order (a `Vec`, newest last).
2. Store follow relations as `HashMap<i32, HashSet<i32>>` so follow/unfollow are `O(1)`.
3. The feed is a merge of several "newest first" lists: start a max-heap with each author's
   newest tweet and, after popping an author's tweet, push that author's next older one.

## Starter code

Write your code in [`src/practice/p0400_design_twitter.rs`](../src/practice/p0400_design_twitter.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0400
```

```rust
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
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
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
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `tweets[user]` is a `Vec<(time, tweet_id)>` in posting order and `following[user]` is a
`HashSet` of followees. For a feed, the authors are the user plus their followees. A max-heap
holds one candidate per author, `(time, author, index)`, starting at each author's last tweet.
Popping gives the newest remaining tweet; the author's previous tweet (index `- 1`) replaces it.
After 10 pops (or an empty heap) the feed is complete.

**Complexity:** `post_tweet`, `follow`, `unfollow` are `O(1)` on average; `get_news_feed` is
`O(f + 10 log f)` for `f` followees. Space is `O(tweets + follows)`.

**Rust concepts:**
- `entry(user).or_default().push(...)` and `.insert(...)` grow nested collections on demand.
- `self.following.get(&user_id).into_iter().flatten()` iterates the followees, or nothing when
  the user follows nobody, without a special case.
- Heap entries are plain tuples `(u32, i32, usize)`; the time component decides the order.
- `checked_sub(1)` on the index stops at the author's oldest tweet without underflowing.

```rust
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Users, tweets and follow relations.
pub struct Twitter {
    clock: u32,
    tweets: HashMap<i32, Vec<(u32, i32)>>,
    following: HashMap<i32, HashSet<i32>>,
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl Twitter {
    pub fn new() -> Self {
        Self {
            clock: 0,
            tweets: HashMap::new(),
            following: HashMap::new(),
        }
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.clock += 1;
        self.tweets
            .entry(user_id)
            .or_default()
            .push((self.clock, tweet_id));
    }

    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let followees = self.following.get(&user_id).into_iter().flatten();
        let authors = std::iter::once(&user_id).chain(followees.filter(|&&f| f != user_id));
        let mut heap: BinaryHeap<(u32, i32, usize)> = BinaryHeap::new();
        for &author in authors {
            if let Some(list) = self.tweets.get(&author) {
                if let Some(&(time, _)) = list.last() {
                    heap.push((time, author, list.len() - 1));
                }
            }
        }
        let mut feed = Vec::with_capacity(10);
        while feed.len() < 10 {
            let Some((_, author, index)) = heap.pop() else {
                break;
            };
            let list = &self.tweets[&author];
            feed.push(list[index].1);
            if let Some(prev) = index.checked_sub(1) {
                heap.push((list[prev].0, author, prev));
            }
        }
        feed
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.following
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(set) = self.following.get_mut(&follower_id) {
            set.remove(&followee_id);
        }
    }
}
```

Full tested solution: [`src/solutions/p0400_design_twitter.rs`](../src/solutions/p0400_design_twitter.rs)

</details>
