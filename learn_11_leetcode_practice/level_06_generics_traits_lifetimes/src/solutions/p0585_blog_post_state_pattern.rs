//! # Problem 0585: Blog Post State Pattern
//!
//! - **Difficulty:** Medium
//! - **Topics:** the state pattern with `Option<Box<dyn State>>`, methods taking `self: Box<Self>`, `Option::take`, default trait methods returning references
//! - **Source:** Original · [The Rust Book 18.3](https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html)
//!
//! ## Problem
//!
//! Implement the blog post workflow of The Rust Book **including its suggested extensions**. A `Post`
//! moves through states:
//!
//! ```text
//! Draft --request_review--> PendingReview --approve (twice)--> Published
//!                                 |
//!                                 +--reject--> Draft
//! ```
//!
//! Rules:
//!
//! - `Post::new()` starts in `Draft` with empty content. `Default` is implemented too.
//! - `add_text(&mut self, text: &str)` appends text **only in `Draft`**; in other states it does
//!   nothing.
//! - `content(&self) -> &str` returns the text only when `Published`, otherwise `""`.
//! - `request_review` moves `Draft` to `PendingReview` (other states are unchanged).
//! - `approve` in `PendingReview` counts approvals; the **second** approval publishes the post.
//! - `reject` in `PendingReview` goes back to `Draft` and resets the approvals.
//! - `status(&self) -> &'static str` is `"draft"`, `"pending review"` or `"published"`.
//!
//! The `Post` holds `state: Option<Box<dyn State>>` and `content: String`. The private trait `State`
//! has transition methods that consume the boxed state (`fn approve(self: Box<Self>) -> Box<dyn
//! State>`), so a state can turn itself into the next one. Write the state structs `Draft`,
//! `PendingReview { approvals: u8 }` and `Published`.
//!
//! ## Examples
//!
//! ```text
//! let mut post = Post::new();
//! post.add_text("I ate a salad");
//! post.content()            => ""
//! post.request_review();
//! post.add_text(" today");  // ignored: not a draft any more
//! post.approve();           post.status() => "pending review"
//! post.approve();           post.status() => "published"
//! post.content()            => "I ate a salad"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. A transition in `Post` looks like `if let Some(s) = self.state.take() { self.state =
//!    Some(s.approve()) }`: `take` moves the box out so the state can be consumed.
//! 2. A default body such as `fn reject(self: Box<Self>) -> Box<dyn State> { self }` does **not**
//!    compile: turning `Box<Self>` into `Box<dyn State>` needs `Self: Sized`, which a default method
//!    cannot assume. So every state implements every transition; those that change nothing return
//!    `self`. Default methods still work for non-transition methods such as `can_edit` and `content`.
//! 3. `content` in the trait can take the post: `fn content<'a>(&self, post: &'a Post) -> &'a str`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each state struct implements the three transitions (returning `self` when nothing
//! changes) and overrides the default `can_edit`/`content` only where needed. `Post` delegates every
//! operation to the current state and swaps in whatever state the transition returns.
//!
//! **Rust concepts:**
//! - `self: Box<Self>` methods take ownership of the boxed state, so the old state cannot be used after
//!   a transition.
//! - `Option<Box<dyn State>>` plus `take()` works around "cannot move out of borrowed content": the
//!   `Option` is briefly `None` while the transition runs.
//! - Inside a trait, `Self` may be unsized (it could be `dyn State` itself), so a default method cannot
//!   coerce `Box<Self>` to `Box<dyn State>`; only impls for concrete (sized) types can.
//! - The pattern makes adding a state easy, but the states are coupled to each other (each one names its
//!   successors) and invalid operations are silently ignored. Problem 0586 shows the type-based
//!   alternative.
// @solution-end

/// A blog post whose behaviour depends on its current state.
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn name(&self) -> &'static str;

    fn can_edit(&self) -> bool {
        // @solution-start
        false
        // @solution-end
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        // @solution-start
        ""
        // @solution-end
    }
}

// @solution-start remove
struct Draft;

struct PendingReview {
    approvals: u8,
}

struct Published;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approvals: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn name(&self) -> &'static str {
        "draft"
    }

    fn can_edit(&self) -> bool {
        true
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approvals + 1 >= 2 {
            Box::new(Published)
        } else {
            Box::new(PendingReview {
                approvals: self.approvals + 1,
            })
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft)
    }

    fn name(&self) -> &'static str {
        "pending review"
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn name(&self) -> &'static str {
        "published"
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
// @solution-end

impl Post {
    pub fn new() -> Post {
        // @solution-start
        Post {
            state: Some(Box::new(Draft)),
            content: String::new(),
        }
        // @solution-end
    }

    pub fn add_text(&mut self, text: &str) {
        // @solution-start
        if self.state.as_ref().is_some_and(|s| s.can_edit()) {
            self.content.push_str(text);
        }
        // @solution-end
    }

    pub fn content(&self) -> &str {
        // @solution-start
        self.state.as_ref().map_or("", |s| s.content(self))
        // @solution-end
    }

    pub fn request_review(&mut self) {
        // @solution-start
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
        // @solution-end
    }

    pub fn approve(&mut self) {
        // @solution-start
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
        // @solution-end
    }

    pub fn reject(&mut self) {
        // @solution-start
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
        // @solution-end
    }

    pub fn status(&self) -> &'static str {
        // @solution-start
        self.state.as_ref().map_or("unknown", |s| s.name())
        // @solution-end
    }
}

impl Default for Post {
    fn default() -> Self {
        // @solution-start
        Self::new()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_workflow() {
        let mut post = Post::new();
        post.add_text("I ate a salad");
        assert_eq!(post.content(), "");
        assert_eq!(post.status(), "draft");
        post.request_review();
        post.add_text(" today");
        assert_eq!(post.content(), "");
        post.approve();
        assert_eq!(post.status(), "pending review");
        post.approve();
        assert_eq!(post.status(), "published");
        assert_eq!(post.content(), "I ate a salad");
    }

    #[test]
    fn reject_returns_to_draft_and_resets_approvals() {
        let mut post = Post::default();
        post.add_text("v1");
        post.request_review();
        post.approve();
        post.reject();
        assert_eq!(post.status(), "draft");
        post.add_text(" v2");
        post.request_review();
        post.approve();
        assert_eq!(post.status(), "pending review"); // the earlier approval was forgotten
        post.approve();
        assert_eq!(post.content(), "v1 v2");
    }

    #[test]
    fn invalid_operations_are_ignored() {
        let mut post = Post::new();
        post.approve();
        post.reject();
        assert_eq!(post.status(), "draft");
        post.request_review();
        post.request_review();
        assert_eq!(post.status(), "pending review");
        post.approve();
        post.approve();
        post.reject();
        post.request_review();
        assert_eq!(post.status(), "published");
        post.add_text("late");
        assert_eq!(post.content(), "");
    }
}
