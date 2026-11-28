//! # Problem 0595: Generic Traits vs Associated Types
//!
//! - **Difficulty:** Medium
//! - **Topics:** a generic trait `ConvertTo<T>` implemented several times for one type, a trait with `type Output` implemented once, type annotations and fully qualified calls to pick an impl
//! - **Source:** Original · [The Rust Book 20.2](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types)
//!
//! ## Problem
//!
//! Both of these look similar, but they model different things:
//!
//! ```text
//! pub trait ConvertTo<T> { fn convert(&self) -> T; }                          // generic trait
//! pub trait Decoder { type Output; fn decode(&self, input: &str) -> Option<Self::Output>; }
//! ```
//!
//! - A type can implement `ConvertTo<T>` **many times**, once per `T`. The caller chooses which one.
//! - A type implements `Decoder` **once**, and the implementor chooses `Output`.
//!
//! Implement:
//!
//! 1. For `Celsius(pub f64)`: `ConvertTo<f64>` (the raw value), `ConvertTo<Fahrenheit>`
//!    (`F = C * 9 / 5 + 32`, with `Fahrenheit(pub f64)`) and `ConvertTo<String>` (`"{value:.1} C"`).
//! 2. `convert_all<T, S: ConvertTo<T>>(items: &[S]) -> Vec<T>`.
//! 3. Decoders: `IntDecoder` (`Output = i64`, parses the trimmed input), `BoolDecoder` (`Output =
//!    bool`; accepts `"yes"`/`"true"`/`"1"` and `"no"`/`"false"`/`"0"`, ignoring ASCII case) and
//!    `ListDecoder` (`Output = Vec<String>`; splits on `,`, trims every item and drops empty ones;
//!    never fails).
//! 4. `decode_all<D: Decoder>(decoder: &D, inputs: &[&str]) -> Vec<Option<D::Output>>`.
//!
//! ## Examples
//!
//! ```text
//! let c = Celsius(21.5);
//! let raw: f64 = c.convert();                       => 21.5
//! let text: String = c.convert();                   => "21.5 C"
//! <Celsius as ConvertTo<Fahrenheit>>::convert(&c)   => Fahrenheit(70.7)
//! convert_all::<f64, _>(&[Celsius(1.0), Celsius(2.0)]) => [1.0, 2.0]
//! decode_all(&BoolDecoder, &["YES", "0", "maybe"])  => [Some(true), Some(false), None]
//! ListDecoder.decode(" a, ,b ")                     => Some(["a", "b"])
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `c.convert()` alone is ambiguous: the compiler needs the target type from an annotation, a
//!    turbofish on a generic function, or `<Celsius as ConvertTo<String>>::convert(&c)`.
//! 2. With `Decoder`, `decoder.decode(..)` is never ambiguous: the decoder type determines `Output`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** three small `ConvertTo` impls, one per target type, and three `Decoder` impls, one per
//! decoder. The generic helpers just map.
//!
//! **Rust concepts:**
//! - Choose a **generic trait** when one type should support several variants of the behaviour
//!   (`From<T>`, `Add<Rhs>`, `ConvertTo<T>`); choose an **associated type** when there is exactly one
//!   natural answer per implementor (`Iterator::Item`, `Deref::Target`, `Decoder::Output`).
//! - `convert_all::<f64, _>` uses `_` to let the compiler infer the second type parameter.
//! - Fully qualified syntax `<Type as Trait<Arg>>::method(&value)` selects one impl explicitly.
// @solution-end

/// Conversion into a target type chosen by the caller.
pub trait ConvertTo<T> {
    fn convert(&self) -> T;
}

/// Parses text into the decoder's single output type.
pub trait Decoder {
    type Output;
    fn decode(&self, input: &str) -> Option<Self::Output>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

pub struct IntDecoder;
pub struct BoolDecoder;
pub struct ListDecoder;

impl ConvertTo<f64> for Celsius {
    fn convert(&self) -> f64 {
        // @solution-start
        self.0
        // @solution-end
    }
}

impl ConvertTo<Fahrenheit> for Celsius {
    fn convert(&self) -> Fahrenheit {
        // @solution-start
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
        // @solution-end
    }
}

impl ConvertTo<String> for Celsius {
    fn convert(&self) -> String {
        // @solution-start
        format!("{:.1} C", self.0)
        // @solution-end
    }
}

/// Converts every item into `T`.
pub fn convert_all<T, S: ConvertTo<T>>(items: &[S]) -> Vec<T> {
    // @solution-start
    items.iter().map(ConvertTo::convert).collect()
    // @solution-end
}

impl Decoder for IntDecoder {
    type Output = i64;

    fn decode(&self, input: &str) -> Option<i64> {
        // @solution-start
        input.trim().parse().ok()
        // @solution-end
    }
}

impl Decoder for BoolDecoder {
    type Output = bool;

    fn decode(&self, input: &str) -> Option<bool> {
        // @solution-start
        match input.trim().to_ascii_lowercase().as_str() {
            "yes" | "true" | "1" => Some(true),
            "no" | "false" | "0" => Some(false),
            _ => None,
        }
        // @solution-end
    }
}

impl Decoder for ListDecoder {
    type Output = Vec<String>;

    fn decode(&self, input: &str) -> Option<Vec<String>> {
        // @solution-start
        Some(
            input
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect(),
        )
        // @solution-end
    }
}

/// Decodes every input with the same decoder.
pub fn decode_all<D: Decoder>(decoder: &D, inputs: &[&str]) -> Vec<Option<D::Output>> {
    // @solution-start
    inputs.iter().map(|s| decoder.decode(s)).collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_type_many_conversions() {
        let c = Celsius(21.5);
        let raw: f64 = c.convert();
        assert_eq!(raw, 21.5);
        let text: String = c.convert();
        assert_eq!(text, "21.5 C");
        let f = <Celsius as ConvertTo<Fahrenheit>>::convert(&c);
        assert!((f.0 - 70.7).abs() < 1e-9);
    }

    #[test]
    fn caller_picks_the_target_type() {
        assert_eq!(
            convert_all::<f64, _>(&[Celsius(1.0), Celsius(2.0)]),
            vec![1.0, 2.0]
        );
        let labels: Vec<String> = convert_all(&[Celsius(-3.24), Celsius(100.0)]);
        assert_eq!(labels, vec!["-3.2 C", "100.0 C"]);
        let fs: Vec<Fahrenheit> = convert_all(&[Celsius(100.0)]);
        assert_eq!(fs, vec![Fahrenheit(212.0)]);
    }

    #[test]
    fn decoders_choose_their_output() {
        assert_eq!(
            decode_all(&IntDecoder, &[" 42", "x", "-7"]),
            vec![Some(42), None, Some(-7)]
        );
        assert_eq!(
            decode_all(&BoolDecoder, &["YES", "0", "maybe", " False "]),
            vec![Some(true), Some(false), None, Some(false)]
        );
        assert_eq!(
            ListDecoder.decode(" a, ,b "),
            Some(vec!["a".to_string(), "b".to_string()])
        );
        assert_eq!(ListDecoder.decode(""), Some(vec![]));
    }

    #[test]
    fn generic_code_names_the_output_type() {
        fn first_success<D: Decoder>(d: &D, inputs: &[&str]) -> Option<D::Output> {
            inputs.iter().find_map(|s| d.decode(s))
        }
        assert_eq!(
            first_success(&IntDecoder, &["a", "b", "12", "13"]),
            Some(12)
        );
        assert_eq!(first_success(&BoolDecoder, &["?"]), None);
        let many: Vec<String> = (0..1000).map(|i| i.to_string()).collect();
        let refs: Vec<&str> = many.iter().map(String::as_str).collect();
        let total: i64 = decode_all(&IntDecoder, &refs).into_iter().flatten().sum();
        assert_eq!(total, 499_500);
    }
}
