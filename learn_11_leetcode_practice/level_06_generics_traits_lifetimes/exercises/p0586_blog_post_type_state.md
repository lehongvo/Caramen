# Problem 0586: Blog Post as Types

- **Difficulty:** Medium
- **Topics:** encoding states as distinct types, consuming transitions `fn request_review(self) -> PendingReviewPost`, enums for branching results, compile-time guarantees
- **Source:** Original · [The Rust Book 18.3](https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html#encoding-states-and-behavior-as-types)

## Problem

Problem 0585 ignores invalid operations at run time. Here, invalid operations **do not compile**:
each state is its own type and only has the methods that make sense for it.

- `Post::new() -> DraftPost` (the Book's API: `new` returns the draft type).
- `DraftPost`: `add_text(&mut self, text: &str)`, `request_review(self) -> PendingReviewPost`.
- `PendingReviewPost`: `approvals(&self) -> u8`, `reject(self) -> DraftPost` (keeps the text, resets
  approvals), and `approve(self) -> Approval`, where

  ```text
  pub enum Approval { Pending(PendingReviewPost), Published(Post) }
  ```

  The first approval returns `Approval::Pending` (with one approval recorded); the second returns
  `Approval::Published`.
- `Post` (published): `content(&self) -> &str`.

There is no `content` method on drafts and no `add_text` on published posts: calling them is a
compile error, not a silent no-op. Every transition takes `self` by value, so the old state can no
longer be used.

## Examples

```text
let mut draft = Post::new();
draft.add_text("I ate a salad");
let pending = draft.request_review();
let post = match pending.approve() {
    Approval::Pending(p) => match p.approve() {
        Approval::Published(post) => post,
        Approval::Pending(_) => unreachable!(),
    },
    Approval::Published(_) => unreachable!(),
};
post.content()  => "I ate a salad"
```

## Constraints

- No special constraints.

## Hints

1. Each type only needs the fields it uses: the text, and for the pending post the approval count.
2. Clippy warns when `new` does not return `Self` (`new_ret_no_self`); here it is intentional, so
   allow the lint on that method.

## Starter code

Write your code in [`src/practice/p0586_blog_post_type_state.rs`](../src/practice/p0586_blog_post_type_state.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0586
```

```rust
/// A published post.
#[derive(Debug)]
pub struct Post {}

/// A post that is still being written.
#[derive(Debug)]
pub struct DraftPost {}

/// A post waiting for approvals.
#[derive(Debug)]
pub struct PendingReviewPost {}

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
        todo!("Write your solution here")
    }

    pub fn content(&self) -> &str {
        todo!("Write your solution here")
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        todo!("Write your solution here")
    }

    pub fn request_review(self) -> PendingReviewPost {
        todo!("Write your solution here")
    }
}

impl PendingReviewPost {
    pub fn approvals(&self) -> u8 {
        todo!("Write your solution here")
    }

    pub fn approve(self) -> Approval {
        todo!("Write your solution here")
    }

    pub fn reject(self) -> DraftPost {
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
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every state is a struct and every transition is a method that consumes one struct and
returns another. The branch "one more approval needed vs published" becomes an enum, so callers
must handle both outcomes.

**Rust concepts:**
- Moving `self` into a transition makes stale states impossible: after `draft.request_review()`,
  `draft` is moved and cannot be edited any more.
- The compiler, not run-time checks, enforces the workflow; this is often called the *type-state*
  pattern.
- Trade-off: the caller juggles several types (and `match`es on `Approval`), whereas the trait-object
  version hides the state behind one `Post` type.

```rust
/// A published post.
#[derive(Debug)]
pub struct Post {
    content: String,
}

/// A post that is still being written.
#[derive(Debug)]
pub struct DraftPost {
    content: String,
}

/// A post waiting for approvals.
#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
    approvals: u8,
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
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
        }
    }
}

impl PendingReviewPost {
    pub fn approvals(&self) -> u8 {
        self.approvals
    }

    pub fn approve(self) -> Approval {
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
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
```

Full tested solution: [`src/solutions/p0586_blog_post_type_state.rs`](../src/solutions/p0586_blog_post_type_state.rs)

</details>
