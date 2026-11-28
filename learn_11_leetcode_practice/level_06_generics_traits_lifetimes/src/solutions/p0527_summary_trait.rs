//! # Problem 0527: The Summary Trait
//!
//! - **Difficulty:** Easy
//! - **Topics:** defining a trait, required vs default methods, overriding a default, `impl Trait for Type`
//! - **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations)
//!
//! ## Problem
//!
//! Define a trait `Summary` with one **required** method and two **default** methods:
//!
//! - `summarize_author(&self) -> String`: required.
//! - `summarize(&self) -> String`: default body returns `"(Read more from {author}...)"`, where
//!   `{author}` is `self.summarize_author()`.
//! - `preview(&self, max_chars: usize) -> String`: default body returns `self.summarize()` if it has at
//!   most `max_chars` characters; otherwise its first `max_chars` characters followed by `"..."`.
//!
//! Then implement it for two types:
//!
//! - `NewsArticle { headline, location, author, content }` (all `String`, all `pub`):
//!   `summarize_author` returns the `author`, and it **overrides** `summarize` to return
//!   `"{headline}, by {author} ({location})"`.
//! - `SocialPost { username, content }` (both `String`, both `pub`): `summarize_author` returns
//!   `"@{username}"` and it keeps the default `summarize`.
//!
//! Neither type overrides `preview`. Notice that `preview` on a `NewsArticle` must use the
//! **overridden** `summarize`.
//!
//! ## Examples
//!
//! ```text
//! post    = SocialPost { username: "horse_ebooks", content: "of course, as you probably already know" }
//! post.summarize()      => "(Read more from @horse_ebooks...)"
//! post.preview(10)      => "(Read more..."
//!
//! article = NewsArticle { headline: "Penguins win the Stanley Cup Championship!",
//!                         location: "Pittsburgh, PA, USA", author: "Iceburgh", content: "..." }
//! article.summarize()   => "Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)"
//! article.preview(8)    => "Penguins..."
//! ```
//!
//! ## Constraints
//!
//! - `max_chars` counts Unicode scalar values (`char`s), not bytes.
//!
//! ## Hints
//!
//! 1. A default method is written inside the `trait` block, with a body.
//! 2. Default methods may call other trait methods, including required ones that the trait itself
//!    does not implement.
//! 3. `s.chars().take(n).collect::<String>()` keeps the first `n` characters.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the trait supplies `summarize` and `preview` in terms of `summarize_author`, so a type
//! only has to provide the author. `NewsArticle` replaces `summarize` with its own version, and the
//! default `preview` automatically picks it up because it calls `self.summarize()`.
//!
//! **Rust concepts:**
//! - Required methods have only a signature (`fn summarize_author(&self) -> String;`); default
//!   methods have a body that every implementor inherits unless it overrides it.
//! - Calls inside a default body are dispatched to the implementor's methods, so overriding one
//!   method changes the behaviour of the defaults that call it.
//! - You cannot call the default version from inside an override (there is no `super`), which is why
//!   the Book recommends keeping defaults small and composable.
// @solution-end

/// Something that can be summarised.
pub trait Summary {
    /// The author shown in summaries.
    fn summarize_author(&self) -> String;

    /// A one-line summary.
    fn summarize(&self) -> String {
        // @solution-start
        format!("(Read more from {}...)", self.summarize_author())
        // @solution-end
    }

    /// The summary, cut to `max_chars` characters (plus `"..."` when it was cut).
    fn preview(&self, max_chars: usize) -> String {
        // @solution-start
        let summary = self.summarize();
        if summary.chars().count() <= max_chars {
            summary
        } else {
            let mut cut: String = summary.chars().take(max_chars).collect();
            cut.push_str("...");
            cut
        }
        // @solution-end
    }
}

/// A news article.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        // @solution-start
        self.author.clone()
        // @solution-end
    }

    fn summarize(&self) -> String {
        // @solution-start
        format!("{}, by {} ({})", self.headline, self.author, self.location)
        // @solution-end
    }
}

/// A short social media post.
pub struct SocialPost {
    pub username: String,
    pub content: String,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        // @solution-start
        format!("@{}", self.username)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn post() -> SocialPost {
        SocialPost {
            username: "horse_ebooks".to_string(),
            content: "of course, as you probably already know".to_string(),
        }
    }

    fn article() -> NewsArticle {
        NewsArticle {
            headline: "Penguins win the Stanley Cup Championship!".to_string(),
            location: "Pittsburgh, PA, USA".to_string(),
            author: "Iceburgh".to_string(),
            content: "The Pittsburgh Penguins once again are the best hockey team in the NHL."
                .to_string(),
        }
    }

    #[test]
    fn social_post_uses_the_default_summary() {
        assert_eq!(post().summarize_author(), "@horse_ebooks");
        assert_eq!(post().summarize(), "(Read more from @horse_ebooks...)");
    }

    #[test]
    fn article_overrides_summary() {
        assert_eq!(article().summarize_author(), "Iceburgh");
        assert_eq!(
            article().summarize(),
            "Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)"
        );
    }

    #[test]
    fn preview_uses_the_right_summarize() {
        assert_eq!(post().preview(10), "(Read more...");
        assert_eq!(article().preview(8), "Penguins...");
        let full = post().summarize();
        assert_eq!(post().preview(full.len()), full);
        assert_eq!(post().preview(0), "...");
    }

    #[test]
    fn preview_counts_characters() {
        let p = SocialPost {
            username: "caf\u{e9}".to_string(),
            content: String::new(),
        };
        // "(Read more from @caf\u{e9}...)" has 25 characters but 26 bytes.
        assert_eq!(p.preview(25), p.summarize());
        assert_eq!(p.preview(20), "(Read more from @caf...");
    }
}
