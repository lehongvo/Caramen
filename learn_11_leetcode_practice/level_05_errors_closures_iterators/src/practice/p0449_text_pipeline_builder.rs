// ==================================================================
// PRACTICE 0449: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0449_text_pipeline_builder.md
// Run the tests:          cargo test practice::p0449
// ==================================================================

//! # Problem 0449: A Text Transformation Pipeline
//!
//! - **Difficulty:** Medium
//! - **Topics:** builder pattern taking `self` by value, `Vec<(String, Box<dyn Fn(String) -> String>)>`, closures owning their configuration, `fold` to run the steps
//! - **Source:** Original · [std `Iterator::fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) · [Rust By Example: Closures as input parameters](https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html)
//!
//! ## Problem
//!
//! Build a reusable text pipeline: a list of named steps, each a closure `String -> String`,
//! applied in order. The pipeline is created with a *builder*: every method takes `self` by value,
//! adds a step, and returns the pipeline, so calls can be chained:
//!
//! ```text
//! let p = Pipeline::new().trim().replace("  ", " ").uppercase().then("exclaim", |s| s + "!");
//! p.run("  hello  world ") -> "HELLO WORLD!"
//! p.names()                -> ["trim", "replace", "uppercase", "exclaim"]
//! ```
//!
//! Implement:
//!
//! - `Pipeline::new() -> Pipeline`: no steps (running it returns the input unchanged).
//! - `then(self, name: &str, step: impl Fn(String) -> String + 'static) -> Pipeline`: add a custom
//!   step.
//! - `trim(self) -> Pipeline`: add a step named `"trim"` that removes leading and trailing
//!   whitespace.
//! - `uppercase(self) -> Pipeline`: add a step named `"uppercase"`.
//! - `replace(self, from: &str, to: &str) -> Pipeline`: add a step named `"replace"` that replaces
//!   every occurrence of `from` with `to`. The step must own copies of both strings.
//! - `run(&self, input: &str) -> String`: apply all steps in order. A pipeline can run many times.
//! - `names(&self) -> Vec<&str>`: the step names in order.
//!
//! ## Examples
//!
//! ```text
//! Pipeline::new().run("as is")                                  -> "as is"
//! Pipeline::new().uppercase().trim().run("  a b ")              -> "A B"
//! Pipeline::new().replace("cat", "dog").run("cat catalog")      -> "dog dogalog"
//! ```
//!
//! ## Constraints
//!
//! - At most `100` steps.
//!
//! ## Hints
//!
//! 1. `then` is the general method; `trim`, `uppercase` and `replace` can all call it.
//! 2. In `replace`, turn `from` and `to` into `String`s **before** creating a `move` closure.
//! 3. `run` is `self.steps.iter().fold(input.to_string(), |text, (_, step)| step(text))`.

type Step = Box<dyn Fn(String) -> String>;

#[derive(Default)]
pub struct Pipeline {}

impl Pipeline {
    pub fn new() -> Pipeline {
        todo!("Write your solution here")
    }

    pub fn then(mut self, name: &str, step: impl Fn(String) -> String + 'static) -> Pipeline {
        todo!("Write your solution here")
    }

    pub fn trim(self) -> Pipeline {
        todo!("Write your solution here")
    }

    pub fn uppercase(self) -> Pipeline {
        todo!("Write your solution here")
    }

    pub fn replace(self, from: &str, to: &str) -> Pipeline {
        todo!("Write your solution here")
    }

    pub fn run(&self, input: &str) -> String {
        todo!("Write your solution here")
    }

    pub fn names(&self) -> Vec<&str> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pipeline_is_the_identity() {
        let p = Pipeline::new();
        assert_eq!(p.run("as is"), "as is");
        assert_eq!(p.names(), Vec::<&str>::new());
    }

    #[test]
    fn full_example() {
        let p = Pipeline::new()
            .trim()
            .replace("  ", " ")
            .uppercase()
            .then("exclaim", |s| s + "!");
        assert_eq!(p.run("  hello  world "), "HELLO WORLD!");
        assert_eq!(p.names(), vec!["trim", "replace", "uppercase", "exclaim"]);
        assert_eq!(p.run("again"), "AGAIN!");
    }

    #[test]
    fn steps_run_in_order() {
        assert_eq!(Pipeline::new().uppercase().trim().run("  a b "), "A B");
        let shout_then_tag = Pipeline::new()
            .uppercase()
            .then("tag", |s| format!("<{s}>"));
        let tag_then_shout = Pipeline::new()
            .then("tag", |s| format!("<{s}>"))
            .uppercase();
        assert_eq!(shout_then_tag.run("x"), "<X>");
        assert_eq!(tag_then_shout.run("x"), "<X>");
        let p = Pipeline::new().replace("a", "b").replace("b", "c");
        assert_eq!(p.run("ab"), "cc");
    }

    #[test]
    fn replace_owns_its_strings() {
        let p = {
            let from = String::from("cat");
            let to = String::from("dog");
            Pipeline::new().replace(&from, &to)
        };
        assert_eq!(p.run("cat catalog"), "dog dogalog");
    }

    #[test]
    fn custom_steps_can_capture_values() {
        let width = 6;
        let p = Pipeline::new().then("pad", move |s| format!("{s:>width$}"));
        assert_eq!(p.run("ab"), "    ab");
    }
}
