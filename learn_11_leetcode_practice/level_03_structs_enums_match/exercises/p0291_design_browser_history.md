# Problem 0291: Design Browser History

- **Difficulty:** Medium
- **Topics:** design struct with a `Vec` and a cursor, `Vec::truncate`, clamping with `min` / `saturating_sub`, `&mut self` methods
- **Source:** [LeetCode 1472. Design Browser History](https://leetcode.com/problems/design-browser-history/)

## Problem

A browser tab starts on a `homepage`. Implement its navigation history as `BrowserHistory`:

- `BrowserHistory::new(homepage: String) -> Self`;
- `visit(&mut self, url: String)`: go to `url` from the current page. This **erases all forward
  history** (the pages you could have reached with `forward`);
- `back(&mut self, steps: i32) -> String`: move back up to `steps` pages (stop at the first page
  if there are fewer) and return the current URL;
- `forward(&mut self, steps: i32) -> String`: move forward up to `steps` pages (stop at the last
  page) and return the current URL.

This is a LeetCode *design* problem, so there is no `impl Solution`. LeetCode's template
declares `&self` on every method and suggests `&mut self` where needed, as done here.

## Examples

```text
let mut h = BrowserHistory::new("leetcode.com".into());
h.visit("google.com".into());
h.visit("facebook.com".into());
h.visit("youtube.com".into());
h.back(1)        => "facebook.com"
h.back(1)        => "google.com"
h.forward(1)     => "facebook.com"
h.visit("linkedin.com".into());     // "youtube.com" is no longer reachable
h.forward(2)     => "linkedin.com"
h.back(2)        => "google.com"
h.back(7)        => "leetcode.com"
```

## Constraints

- `1 <= steps <= 100`; at most `5000` calls in total.

## Hints

1. Store the pages in a `Vec<String>` and the current position in a `usize`.
2. `visit` = `truncate(current + 1)`, then `push`.

## Starter code

Write your code in [`src/practice/p0291_design_browser_history.rs`](../src/practice/p0291_design_browser_history.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0291
```

```rust
/// The navigation history of one browser tab.
pub struct BrowserHistory {}

impl BrowserHistory {
    /// A history containing only `homepage`.
    pub fn new(homepage: String) -> Self {
        todo!("Write your solution here")
    }

    /// Goes to `url`, erasing the forward history.
    pub fn visit(&mut self, url: String) {
        todo!("Write your solution here")
    }

    /// Moves back up to `steps` pages.
    pub fn back(&mut self, steps: i32) -> String {
        todo!("Write your solution here")
    }

    /// Moves forward up to `steps` pages.
    pub fn forward(&mut self, steps: i32) -> String {
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
    fn example_session() {
        let mut h = BrowserHistory::new("leetcode.com".into());
        h.visit("google.com".into());
        h.visit("facebook.com".into());
        h.visit("youtube.com".into());
        assert_eq!(h.back(1), "facebook.com");
        assert_eq!(h.back(1), "google.com");
        assert_eq!(h.forward(1), "facebook.com");
        h.visit("linkedin.com".into());
        assert_eq!(h.forward(2), "linkedin.com");
        assert_eq!(h.back(2), "google.com");
        assert_eq!(h.back(7), "leetcode.com");
    }

    #[test]
    fn only_homepage() {
        let mut h = BrowserHistory::new("home".into());
        assert_eq!(h.back(5), "home");
        assert_eq!(h.forward(5), "home");
    }

    #[test]
    fn visit_after_back_erases_forward_pages() {
        let mut h = BrowserHistory::new("a".into());
        h.visit("b".into());
        h.visit("c".into());
        assert_eq!(h.back(2), "a");
        h.visit("d".into());
        assert_eq!(h.forward(1), "d");
        assert_eq!(h.back(1), "a");
        assert_eq!(h.forward(10), "d");
    }

    #[test]
    fn many_visits() {
        let mut h = BrowserHistory::new("p0".into());
        for i in 1..4_990 {
            h.visit(format!("p{i}"));
        }
        assert_eq!(h.back(100), "p4889");
        assert_eq!(h.forward(50), "p4939");
        assert_eq!(h.forward(100), "p4989");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the history is a vector of pages and a cursor. Moving back or forward only moves the
cursor, clamped to the valid range; visiting drops everything after the cursor before pushing.

**Complexity:** `O(1)` amortised for `visit` (plus the truncated pages), `O(1)` for `back` and
`forward`; `O(n)` space.

**Rust concepts:**
- `Vec::truncate(len)` drops the forward pages in place; their `String`s are freed.
- `saturating_sub` stops the cursor at `0`, and `min(len - 1)` stops it at the last page, so no
  bounds check can fail.
- Returning `self.pages[self.current].clone()` is required because LeetCode wants an owned
  `String`, while the history keeps its own copy.

```rust
/// The navigation history of one browser tab.
pub struct BrowserHistory {
    pages: Vec<String>,
    current: usize,
}

impl BrowserHistory {
    /// A history containing only `homepage`.
    pub fn new(homepage: String) -> Self {
        Self {
            pages: vec![homepage],
            current: 0,
        }
    }

    /// Goes to `url`, erasing the forward history.
    pub fn visit(&mut self, url: String) {
        self.pages.truncate(self.current + 1);
        self.pages.push(url);
        self.current += 1;
    }

    /// Moves back up to `steps` pages.
    pub fn back(&mut self, steps: i32) -> String {
        self.current = self.current.saturating_sub(steps.max(0) as usize);
        self.pages[self.current].clone()
    }

    /// Moves forward up to `steps` pages.
    pub fn forward(&mut self, steps: i32) -> String {
        let last = self.pages.len() - 1;
        self.current = (self.current + steps.max(0) as usize).min(last);
        self.pages[self.current].clone()
    }
}
```

Full tested solution: [`src/solutions/p0291_design_browser_history.rs`](../src/solutions/p0291_design_browser_history.rs)

</details>
