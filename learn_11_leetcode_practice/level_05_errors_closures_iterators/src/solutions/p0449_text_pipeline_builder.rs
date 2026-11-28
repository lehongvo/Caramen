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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the pipeline is just a `Vec` of named boxed closures. Builder methods push one more
//! entry and hand the pipeline back; `run` threads the text through every closure with `fold`.
//!
//! **Rust concepts:**
//! - Taking `mut self` and returning `Self` is the idiomatic consuming builder; it allows
//!   `Pipeline::new().trim().uppercase()` without any temporary variables.
//! - Each convenience method creates a different closure type; boxing them as
//!   `Box<dyn Fn(String) -> String>` lets them share one `Vec`.
//! - The `replace` closure owns its configuration (`move` of two `String`s), so it satisfies the
//!   `'static` bound and keeps working after the caller's strings are gone.
//! - The steps are `Fn` (they only read their captures), so `run` can take `&self` and be called
//!   any number of times.
// @solution-end

type Step = Box<dyn Fn(String) -> String>;

#[derive(Default)]
pub struct Pipeline {
    // @solution-start remove
    steps: Vec<(String, Step)>,
    // @solution-end
}

impl Pipeline {
    pub fn new() -> Pipeline {
        // @solution-start
        Self::default()
        // @solution-end
    }

    pub fn then(mut self, name: &str, step: impl Fn(String) -> String + 'static) -> Pipeline {
        // @solution-start
        self.steps.push((name.to_string(), Box::new(step)));
        self
        // @solution-end
    }

    pub fn trim(self) -> Pipeline {
        // @solution-start
        self.then("trim", |s| s.trim().to_string())
        // @solution-end
    }

    pub fn uppercase(self) -> Pipeline {
        // @solution-start
        self.then("uppercase", |s| s.to_uppercase())
        // @solution-end
    }

    pub fn replace(self, from: &str, to: &str) -> Pipeline {
        // @solution-start
        let from = from.to_string();
        let to = to.to_string();
        self.then("replace", move |s| s.replace(&from, &to))
        // @solution-end
    }

    pub fn run(&self, input: &str) -> String {
        // @solution-start
        self.steps
            .iter()
            .fold(input.to_string(), |text, (_, step)| step(text))
        // @solution-end
    }

    pub fn names(&self) -> Vec<&str> {
        // @solution-start
        self.steps.iter().map(|(name, _)| name.as_str()).collect()
        // @solution-end
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
