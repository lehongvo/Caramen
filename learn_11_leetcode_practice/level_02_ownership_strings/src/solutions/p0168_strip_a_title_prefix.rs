//! # Problem 0168: Strip a Title Prefix
//!
//! - **Difficulty:** Easy
//! - **Topics:** choosing `&str` or `String` as a return type, `strip_prefix`, `Option::filter`, `format!`
//! - **Source:** Original · [`str::strip_prefix`](https://doc.rust-lang.org/std/primitive.str.html#method.strip_prefix) · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html)
//!
//! ## Problem
//!
//! A contact list stores names such as `"Dr. Ada Lovelace"`. Write two functions and notice why
//! their return types differ.
//!
//! 1. `strip_title(name: &str) -> &str` trims whitespace around `name`, then removes **one** leading
//!    title if present. The titles are `"Dr."`, `"Mr."`, `"Mrs."`, `"Ms."` and `"Prof."`, and a title
//!    only counts when it is followed by at least one space; the title and the spaces after it are
//!    removed. The result is always a part of the input, so it is returned as a borrowed `&str`.
//! 2. `with_title(name: &str, title: &str) -> String` returns `title`, one space and the name
//!    without its old title (as given by `strip_title`). This creates text that does not exist in
//!    the input, so it must return an owned `String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  strip_title("  Dr. Ada Lovelace ")
//! Output: "Ada Lovelace"
//!
//! Input:  strip_title("Mrs.   Robinson")
//! Output: "Robinson"
//!
//! Input:  strip_title("Dr.Who")
//! Output: "Dr.Who"           (no space after the title)
//!
//! Input:  strip_title("Drake")
//! Output: "Drake"
//!
//! Input:  with_title("Mr. Grace Hopper", "Rear Admiral")
//! Output: "Rear Admiral Grace Hopper"
//! ```
//!
//! ## Constraints
//!
//! - `0 <= name.len(), title.len() <= 1000`
//!
//! ## Hints
//!
//! 1. `s.strip_prefix("Dr.")` returns `Some(rest)` if `s` starts with `"Dr."`, else `None`.
//! 2. Keep the match only if `rest` starts with `' '`, then `trim_start()` it.
//! 3. `with_title` can reuse `strip_title` and build the result with `format!`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** trim the name, then try each title with `strip_prefix`; the first one followed by a
//! space wins, and the remainder (trimmed at the start) is the answer. Every step returns a slice of
//! the input, so the function never allocates. `with_title` must create new text and therefore
//! returns a `String`.
//!
//! **Complexity:** `O(n)` time for both functions; `strip_title` uses `O(1)` extra space.
//!
//! **Rust concepts:**
//! - Rule of thumb for signatures: take `&str` when you only read text; return `&str` when the
//!   result is a piece of an input; return `String` when you build new text.
//! - `strip_prefix` returns `Option<&str>`; `Option::filter` keeps the `Some` only when the predicate
//!   holds.
//! - A `const` array of `&'static str` is a simple lookup table.
// @solution-end

// @solution-start remove
const TITLES: [&str; 5] = ["Dr.", "Mr.", "Mrs.", "Ms.", "Prof."];
// @solution-end

pub fn strip_title(name: &str) -> &str {
    // @solution-start
    let name = name.trim();
    for title in TITLES {
        if let Some(rest) = name.strip_prefix(title).filter(|rest| rest.starts_with(' ')) {
            return rest.trim_start();
        }
    }
    name
    // @solution-end
}

pub fn with_title(name: &str, title: &str) -> String {
    // @solution-start
    format!("{title} {}", strip_title(name))
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_doctor_with_spaces() {
        assert_eq!(strip_title("  Dr. Ada Lovelace "), "Ada Lovelace");
    }

    #[test]
    fn example_several_spaces_after_title() {
        assert_eq!(strip_title("Mrs.   Robinson"), "Robinson");
    }

    #[test]
    fn example_title_without_space_is_kept() {
        assert_eq!(strip_title("Dr.Who"), "Dr.Who");
    }

    #[test]
    fn example_name_that_only_looks_like_a_title() {
        assert_eq!(strip_title("Drake"), "Drake");
        assert_eq!(strip_title("Ms"), "Ms");
    }

    #[test]
    fn example_with_title() {
        assert_eq!(with_title("Mr. Grace Hopper", "Rear Admiral"), "Rear Admiral Grace Hopper");
    }

    #[test]
    fn only_one_title_is_removed() {
        assert_eq!(strip_title("Prof. Dr. Smith"), "Dr. Smith");
        assert_eq!(strip_title("Ms. Mr. X"), "Mr. X");
    }

    #[test]
    fn empty_and_title_only() {
        assert_eq!(strip_title(""), "");
        assert_eq!(strip_title(" Dr. "), "Dr.");
        assert_eq!(with_title("", "Dr."), "Dr. ");
    }

    #[test]
    fn strip_title_returns_a_slice_of_the_input() {
        let name = String::from("Prof. Turing");
        let bare = strip_title(&name);
        assert_eq!(bare.as_ptr(), name[6..].as_ptr());
    }
}
