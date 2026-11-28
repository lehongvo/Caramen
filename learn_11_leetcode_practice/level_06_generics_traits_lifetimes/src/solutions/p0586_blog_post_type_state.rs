//! # Problem 0586: Blog Post as Types
//!
//! - **Difficulty:** Medium
//! - **Topics:** encoding states as distinct types, consuming transitions `fn request_review(self) -> PendingReviewPost`, enums for branching results, compile-time guarantees
//! - **Source:** Original · [The Rust Book 18.3](https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html#encoding-states-and-behavior-as-types)
//!
//! ## Problem
//!
//! Problem 0585 ignores invalid operations at run time. Here, invalid operations **do not compile**:
//! each state is its own type and only has the methods that make sense for it.
//!
//! - `Post::new() -> DraftPost` (the Book's API: `new` returns the draft type).
//! - `DraftPost`: `add_text(&mut self, text: &str)`, `request_review(self) -> PendingReviewPost`.
//! - `PendingReviewPost`: `approvals(&self) -> u8`, `reject(self) -> DraftPost` (keeps the text, resets
//!   approvals), and `approve(self) -> Approval`, where
//!
//!   ```text
//!   pub enum Approval { Pending(PendingReviewPost), Published(Post) }
//!   ```
//!
//!   The first approval returns `Approval::Pending` (with one approval recorded); the second returns
//!   `Approval::Published`.
//! - `Post` (published): `content(&self) -> &str`.
//!
//! There is no `content` method on drafts and no `add_text` on published posts: calling them is a
//! compile error, not a silent no-op. Every transition takes `self` by value, so the old state can no
//! longer be used.
//!
//! ## Examples
//!
//! ```text
//! let mut draft = Post::new();
//! draft.add_text("I ate a salad");
//! let pending = draft.request_review();
//! let post = match pending.approve() {
//!     Approval::Pending(p) => match p.approve() {
//!         Approval::Published(post) => post,
//!         Approval::Pending(_) => unreachable!(),
//!     },
//!     Approval::Published(_) => unreachable!(),
//! };
//! post.content()  => "I ate a salad"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Each type only needs the fields it uses: the text, and for the pending post the approval count.
//! 2. Clippy warns when `new` does not return `Self` (`new_ret_no_self`); here it is intentional, so
//!    allow the lint on that method.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every state is a struct and every transition is a method that consumes one struct and
//! returns another. The branch "one more approval needed vs published" becomes an enum, so callers
//! must handle both outcomes.
//!
//! **Rust concepts:**
//! - Moving `self` into a transition makes stale states impossible: after `draft.request_review()`,
//!   `draft` is moved and cannot be edited any more.
//! - The compiler, not run-time checks, enforces the workflow; this is often called the *type-state*
//!   pattern.
//! - Trade-off: the caller juggles several types (and `match`es on `Approval`), whereas the trait-object
//!   version hides the state behind one `Post` type.
// @solution-end

/// A published post.
#[derive(Debug)]
pub struct Post {
    // @solution-start remove
    content: String,
    // @solution-end
}

/// A post that is still being written.
#[derive(Debug)]
pub struct DraftPost {
    // @solution-start remove
    content: String,
    // @solution-end
}

/// A post waiting for approvals.
#[derive(Debug)]
pub struct PendingReviewPost {
    // @solution-start remove
    content: String,
    approvals: u8,
    // @solution-end
}

/// The result of approving a pending post.
#[derive(Debug)]
pub enum Approval {
    Pending(PendingReviewPost),
    Published(Post),
}

impl Post {
    // The Book's API: creating a post gives you a draft, not a published `Post`.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DraftPost {
        // @solution-start
        DraftPost {
            content: String::new(),
        }
        // @solution-end
    }

    pub fn content(&self) -> &str {
        // @solution-start
        &self.content
        // @solution-end
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        // @solution-start
        self.content.push_str(text);
        // @solution-end
    }

    pub fn request_review(self) -> PendingReviewPost {
        // @solution-start
        PendingReviewPost {
            content: self.content,
            approvals: 0,
        }
        // @solution-end
    }
}

impl PendingReviewPost {
    pub fn approvals(&self) -> u8 {
        // @solution-start
        self.approvals
        // @solution-end
    }

    pub fn approve(self) -> Approval {
        // @solution-start
        if self.approvals + 1 >= 2 {
            Approval::Published(Post {
                content: self.content,
            })
        } else {
            Approval::Pending(PendingReviewPost {
                content: self.content,
                approvals: self.approvals + 1,
            })
        }
        // @solution-end
    }

    pub fn reject(self) -> DraftPost {
        // @solution-start
        DraftPost {
            content: self.content,
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn publish(pending: PendingReviewPost) -> Post {
        let mut current = pending;
        loop {
            match current.approve() {
                Approval::Pending(p) => current = p,
                Approval::Published(post) => return post,
            }
        }
    }

    #[test]
    fn example_workflow() {
        let mut draft = Post::new();
        draft.add_text("I ate a salad");
        let pending = draft.request_review();
        let post = match pending.approve() {
            Approval::Pending(p) => match p.approve() {
                Approval::Published(post) => post,
                Approval::Pending(_) => panic!("two approvals must publish"),
            },
            Approval::Published(_) => panic!("one approval must not publish"),
        };
        assert_eq!(post.content(), "I ate a salad");
    }

    #[test]
    fn first_approval_is_recorded() {
        let mut draft = Post::new();
        draft.add_text("x");
        let pending = draft.request_review();
        assert_eq!(pending.approvals(), 0);
        match pending.approve() {
            Approval::Pending(p) => assert_eq!(p.approvals(), 1),
            Approval::Published(_) => panic!("needs two approvals"),
        }
    }

    #[test]
    fn reject_keeps_text_and_resets_approvals() {
        let mut draft = Post::new();
        draft.add_text("v1");
        let once = match draft.request_review().approve() {
            Approval::Pending(p) => p,
            Approval::Published(_) => panic!("needs two approvals"),
        };
        let mut draft = once.reject();
        draft.add_text(" v2");
        let pending = draft.request_review();
        assert_eq!(pending.approvals(), 0);
        assert_eq!(publish(pending).content(), "v1 v2");
    }
}
