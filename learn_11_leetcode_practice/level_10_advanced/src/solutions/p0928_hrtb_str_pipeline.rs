//! # Problem 0928: Higher-Ranked Closures
//!
//! - **Difficulty:** Hard
//! - **Topics:** higher-ranked trait bounds `for<'a> Fn(&'a str) -> &'a str`, boxed HRTB closures, lifetime elision in `Fn` bounds, borrowing from locals
//! - **Source:** Original · [The Rustonomicon: Higher-Rank Trait Bounds](https://doc.rust-lang.org/nomicon/hrtb.html) · [Reference: Higher-ranked trait bounds](https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds)
//!
//! ## Problem
//!
//! A *slicing step* takes a string slice and returns a sub-slice **of the same string**, such as
//! `str::trim`. Its type is `for<'a> Fn(&'a str) -> &'a str`: for **every** lifetime `'a`, give it a
//! `&'a str` and you get back a `&'a str`. Such a step can be applied to strings whose lifetime is
//! only known inside your function, which a step typed with one fixed lifetime could not.
//!
//! ```text
//! pub type Step = Box<dyn for<'a> Fn(&'a str) -> &'a str>;
//! ```
//!
//! Implement:
//!
//! - `apply_steps<'a>(input: &'a str, steps: &[Step]) -> &'a str`: apply the steps left to right.
//!   The result borrows from `input`: no allocation.
//! - `config_steps() -> Vec<Step>`: three steps, in this order: drop everything from the first `'#'`
//!   (a comment) to the end; trim whitespace; strip one leading `"export "` if present.
//! - `clean_lines(text: &str, steps: &[Step]) -> Vec<String>`: apply the steps to every line of
//!   `text` and keep the non-empty results, as owned `String`s.
//! - `apply_to_temporary<F>(parts: &[&str], f: F) -> String where F: for<'a> Fn(&'a str) -> &'a str`:
//!   join `parts` with `"/"` into a **local** `String`, apply `f` to it, and return the result as a
//!   new `String`. (Try writing this with a fixed lifetime parameter instead of `for<'a>`: it does
//!   not compile, because no lifetime chosen by the caller can name the local variable.)
//!
//! ## Examples
//!
//! ```text
//! let steps: Vec<Step> = vec![Box::new(str::trim), Box::new(|s| s.trim_start_matches('-'))];
//! apply_steps("  --flag ", &steps)                   => "flag"
//! apply_steps("export PATH=/bin  # system", &config_steps()) => "PATH=/bin"
//! clean_lines("# only a comment\n A=1 \n\nexport B=2", &config_steps()) => ["A=1", "B=2"]
//! apply_to_temporary(&["usr", "local", "bin"], |s| s.rsplit('/').next().unwrap_or(s)) => "bin"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `steps.iter().fold(input, |s, step| step(s))` threads the slice through the steps.
//! 2. `Box::new(|s: &str| ...)` builds a `Step` when the closure is checked against the expected
//!    `dyn for<'a> Fn(&'a str) -> &'a str` type; `str::trim` itself also fits.
//! 3. `F: Fn(&str) -> &str` is sugar for exactly the same higher-ranked bound (lifetime elision).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every step returns a slice of its argument, so folding the steps over `input` keeps
//! returning slices of `input`. The config steps use `split('#').next()`, `trim` and
//! `strip_prefix(...).unwrap_or(s)`. `apply_to_temporary` builds a local string and calls `f` with a
//! borrow that only lives inside the function; the higher-ranked bound is what allows that call.
//!
//! **Complexity:** linear in the input length times the number of steps.
//!
//! **Rust concepts:**
//! - `for<'a>` introduces a lifetime chosen at **each call** of the closure, not once by the caller
//!   of the function that receives it.
//! - In `Fn(&str) -> &str` the elided lifetimes are already higher-ranked; `for<'a>` spells it out
//!   and is required in places where elision does not apply, such as `dyn` types in aliases.
//! - A closure only gets a higher-ranked signature when its expected type is known (here: the boxed
//!   `dyn` type or the `F` bound); written as a bare `let` without annotations it might not.
// @solution-end

pub type Step = Box<dyn for<'a> Fn(&'a str) -> &'a str>;

pub fn apply_steps<'a>(input: &'a str, steps: &[Step]) -> &'a str {
    // @solution-start
    steps.iter().fold(input, |s, step| step(s))
    // @solution-end
}

pub fn config_steps() -> Vec<Step> {
    // @solution-start
    vec![
        Box::new(|s| s.split('#').next().unwrap_or(s)),
        Box::new(str::trim),
        Box::new(|s| s.strip_prefix("export ").unwrap_or(s)),
    ]
    // @solution-end
}

pub fn clean_lines(text: &str, steps: &[Step]) -> Vec<String> {
    // @solution-start
    text.lines()
        .map(|line| apply_steps(line, steps))
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
    // @solution-end
}

pub fn apply_to_temporary<F>(parts: &[&str], f: F) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    // @solution-start
    let joined = parts.join("/");
    f(&joined).to_string()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_apply_left_to_right_without_copying() {
        let steps: Vec<Step> = vec![Box::new(str::trim), Box::new(|s| s.trim_start_matches('-'))];
        let input = "  --flag ";
        let out = apply_steps(input, &steps);
        assert_eq!(out, "flag");
        // `out` points into `input`.
        let offset = out.as_ptr() as usize - input.as_ptr() as usize;
        assert_eq!(&input[offset..offset + out.len()], "flag");
        assert_eq!(apply_steps("unchanged", &[]), "unchanged");
    }

    #[test]
    fn config_steps_clean_one_line() {
        let steps = config_steps();
        assert_eq!(steps.len(), 3);
        assert_eq!(
            apply_steps("export PATH=/bin  # system", &steps),
            "PATH=/bin"
        );
        assert_eq!(apply_steps("   # comment only", &steps), "");
        assert_eq!(apply_steps("exported=1", &steps), "exported=1");
        assert_eq!(apply_steps("  export  X=1", &steps), " X=1");
    }

    #[test]
    fn clean_lines_keeps_non_empty_results() {
        let text = "# only a comment\n A=1 \n\nexport B=2\n  # indented comment\nC=3#trailing";
        assert_eq!(
            clean_lines(text, &config_steps()),
            vec!["A=1", "B=2", "C=3"]
        );
        assert!(clean_lines("", &config_steps()).is_empty());
    }

    #[test]
    fn higher_ranked_bound_allows_borrowing_locals() {
        let last = apply_to_temporary(&["usr", "local", "bin"], |s| {
            s.rsplit('/').next().unwrap_or(s)
        });
        assert_eq!(last, "bin");
        let whole = apply_to_temporary(&["a", "b"], |s| s);
        assert_eq!(whole, "a/b");
        let first_two = apply_to_temporary(&["x", "y", "z"], |s| &s[..3]);
        assert_eq!(first_two, "x/y");
    }

    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }

    #[test]
    fn plain_functions_are_steps_too() {
        let steps: Vec<Step> = vec![Box::new(first_word), Box::new(str::trim_end)];
        assert_eq!(apply_steps("  hello big world", &steps), "hello");
        assert_eq!(apply_to_temporary(&["one two", "three"], first_word), "one");
    }
}
