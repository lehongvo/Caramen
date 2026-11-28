# Problem 0584: A Text Pipeline

- **Difficulty:** Medium
- **Topics:** `Vec<Box<dyn Transform>>`, a blanket impl `impl<F: Fn(&str) -> String> Transform for F`, default trait methods, builder methods taking `self`
- **Source:** Original · [The Rust Book 18.2](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)

## Problem

A text pipeline applies a list of transformations in order. A transformation is anything that
implements

```text
pub trait Transform {
    fn apply(&self, input: &str) -> String;
    fn name(&self) -> String { "custom".to_string() }    // default method
}
```

1. Implement it for two structs: `Trim` (removes surrounding whitespace; name `"trim"`) and
   `Replace { from: String, to: String }` (replaces every occurrence; name `"replace(from->to)"`).
2. Add a **blanket impl** so that every closure or function `F: Fn(&str) -> String` is a
   `Transform` too (with the default name).
3. `Pipeline` stores `Vec<Box<dyn Transform>>` and offers:
   - `new()` and `Default`;
   - `then<T: Transform + 'static>(self, step: T) -> Self`: a builder method that appends a step and
     returns the pipeline, so calls can be chained;
   - `len(&self)`, `is_empty(&self)`;
   - `run(&self, input: &str) -> String`: applies every step in order (with no steps, returns the
     input unchanged);
   - `describe(&self) -> String`: the step names joined with `" -> "`.

## Examples

```text
let p = Pipeline::new()
    .then(Trim)
    .then(|s: &str| s.to_uppercase())
    .then(Replace { from: " ".into(), to: "_".into() });
p.run("  hello big world ")   => "HELLO_BIG_WORLD"
p.describe()                  => "trim -> custom -> replace( ->_)"
```

## Constraints

- No special constraints.

## Hints

1. `impl<F> Transform for F where F: Fn(&str) -> String { fn apply(&self, input: &str) -> String {
   self(input) } }`.
2. Closures passed to a generic `T: Transform` need their parameter type written out
   (`|s: &str| ...`): the compiler cannot infer it through the blanket impl.
3. In `run`, fold over the steps starting with `input.to_string()`.

## Starter code

Write your code in [`src/practice/p0584_text_pipeline_closures.rs`](../src/practice/p0584_text_pipeline_closures.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0584
```

```rust
/// One step of a text pipeline.
pub trait Transform {
    fn apply(&self, input: &str) -> String;

    fn name(&self) -> String {
        todo!("Write your solution here")
    }
}

/// Removes surrounding whitespace.
pub struct Trim;

/// Replaces every occurrence of `from` with `to`.
pub struct Replace {
    pub from: String,
    pub to: String,
}

impl Transform for Trim {
    fn apply(&self, input: &str) -> String {
        todo!("Write your solution here")
    }

    fn name(&self) -> String {
        todo!("Write your solution here")
    }
}

impl Transform for Replace {
    fn apply(&self, input: &str) -> String {
        todo!("Write your solution here")
    }

    fn name(&self) -> String {
        todo!("Write your solution here")
    }
}

impl<F> Transform for F
where
    F: Fn(&str) -> String,
{
    fn apply(&self, input: &str) -> String {
        todo!("Write your solution here")
    }
}

/// An ordered list of transformations.
pub struct Pipeline {}

impl Pipeline {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn then<T: Transform + 'static>(self, step: T) -> Self {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn run(&self, input: &str) -> String {
        todo!("Write your solution here")
    }

    pub fn describe(&self) -> String {
        todo!("Write your solution here")
    }
}

impl Default for Pipeline {
    fn default() -> Self {
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

    fn example() -> Pipeline {
        Pipeline::new()
            .then(Trim)
            .then(|s: &str| s.to_uppercase())
            .then(Replace {
                from: " ".into(),
                to: "_".into(),
            })
    }

    #[test]
    fn example_pipeline() {
        let p = example();
        assert_eq!(p.len(), 3);
        assert_eq!(p.run("  hello big world "), "HELLO_BIG_WORLD");
        assert_eq!(p.describe(), "trim -> custom -> replace( ->_)");
    }

    #[test]
    fn empty_pipeline() {
        let p = Pipeline::default();
        assert!(p.is_empty());
        assert_eq!(p.run(" unchanged "), " unchanged ");
        assert_eq!(p.describe(), "");
    }

    #[test]
    fn functions_and_capturing_closures() {
        fn exclaim(s: &str) -> String {
            format!("{s}!")
        }
        let suffix = String::from(" (edited)");
        let p = Pipeline::new()
            .then(exclaim)
            .then(move |s: &str| format!("{s}{suffix}"));
        assert_eq!(p.run("done"), "done! (edited)");
        assert_eq!(exclaim.name(), "custom");
    }

    #[test]
    fn long_pipeline() {
        let mut p = Pipeline::new();
        for _ in 0..100 {
            p = p.then(|s: &str| format!("{s}a"));
        }
        assert_eq!(p.run("").len(), 100);
        assert_eq!(
            Trim.apply("  x  ")
                + &Replace {
                    from: "a".into(),
                    to: "b".into()
                }
                .apply("banana"),
            "xbbnbnb"
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every step is boxed as a `dyn Transform`, whatever its concrete type (struct, closure or
plain function). `run` threads a `String` through all steps; `describe` collects the names.

**Rust concepts:**
- A blanket impl over `F: Fn(&str) -> String` lets closures act as trait objects of your own trait,
  which is how many libraries accept "either a struct or a closure".
- It does not conflict with `impl Transform for Trim` because the `Fn` traits are `#[fundamental]`:
  the compiler may assume a local type like `Trim` does not implement `Fn` unless you say so.
- Builder methods that take and return `self` (`then(mut self, ..) -> Self`) allow chaining without
  `let mut`.

```rust
/// One step of a text pipeline.
pub trait Transform {
    fn apply(&self, input: &str) -> String;

    fn name(&self) -> String {
        "custom".to_string()
    }
}

/// Removes surrounding whitespace.
pub struct Trim;

/// Replaces every occurrence of `from` with `to`.
pub struct Replace {
    pub from: String,
    pub to: String,
}

impl Transform for Trim {
    fn apply(&self, input: &str) -> String {
        input.trim().to_string()
    }

    fn name(&self) -> String {
        "trim".to_string()
    }
}

impl Transform for Replace {
    fn apply(&self, input: &str) -> String {
        input.replace(&self.from, &self.to)
    }

    fn name(&self) -> String {
        format!("replace({}->{})", self.from, self.to)
    }
}

impl<F> Transform for F
where
    F: Fn(&str) -> String,
{
    fn apply(&self, input: &str) -> String {
        self(input)
    }
}

/// An ordered list of transformations.
pub struct Pipeline {
    steps: Vec<Box<dyn Transform>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline { steps: Vec::new() }
    }

    pub fn then<T: Transform + 'static>(self, step: T) -> Self {
        let mut steps = self.steps;
        steps.push(Box::new(step));
        Pipeline { steps }
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn run(&self, input: &str) -> String {
        self.steps
            .iter()
            .fold(input.to_string(), |text, step| step.apply(&text))
    }

    pub fn describe(&self) -> String {
        self.steps
            .iter()
            .map(|s| s.name())
            .collect::<Vec<_>>()
            .join(" -> ")
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
```

Full tested solution: [`src/solutions/p0584_text_pipeline_closures.rs`](../src/solutions/p0584_text_pipeline_closures.rs)

</details>
