//! # Problem 0910: A `json!` Macro
//!
//! - **Difficulty:** Hard
//! - **Topics:** recursive `macro_rules!` over token trees, `$key:literal`, `From` conversions, `Display`
//! - **Source:** Original · [Reference: Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html#metavariables) · [Rust by Example: DSL](https://doc.rust-lang.org/rust-by-example/macros/dsl.html)
//!
//! ## Problem
//!
//! Build a tiny JSON value type and a macro that lets you write JSON almost literally in Rust:
//!
//! ```text
//! pub enum Json { Null, Bool(bool), Number(f64), Str(String), Array(Vec<Json>), Object(Vec<(String, Json)>) }
//! ```
//!
//! (`Object` keeps its keys in the order written.)
//!
//! 1. `From<bool>`, `From<i32>`, `From<i64>`, `From<f64>`, `From<&str>` and `From<String>` for
//!    `Json` (numbers become `Number`).
//! 2. `json!(...)` accepts:
//!    - `null` gives `Json::Null`;
//!    - `[v1, v2, ...]` gives an `Array`; `{"key": v, ...}` gives an `Object` (keys are string
//!      literals); trailing commas are allowed in both;
//!    - anything else is an expression converted with `Json::from(...)`.
//!
//!    Inside arrays and objects every value is a **single token tree**: a literal, `null`, `true`,
//!    a variable name, a nested `[...]` or `{...}`, or any expression in parentheses such as `(-5)`
//!    or `(n * 2)`. Nested values are handled by calling `json!` recursively.
//! 3. `Display` prints compact JSON: `null`, `true`, numbers with Rust's `{}` formatting for `f64`
//!    (`36.0` prints as `36`), strings in double quotes with `"` and `\` escaped as `\"` and `\\` and
//!    a newline as `\n`, arrays as `[a,b]`, objects as `{"k":v,"k2":v2}`, no spaces.
//! 4. `get(&self, key: &str) -> Option<&Json>`: the value of the first matching key of an `Object`,
//!    `None` otherwise (also `None` for non-objects).
//!
//! ## Examples
//!
//! ```text
//! json!(null)                  => Json::Null
//! json!([1, "a", [true, null]]) => Array([Number(1.0), Str("a"), Array([Bool(true), Null])])
//! let age = 36;
//! json!({"name": "Ada", "age": age, "langs": ["en", "fr"], "delta": (-1.5)}).to_string()
//!     => {"name":"Ada","age":36,"langs":["en","fr"],"delta":-1.5}
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Order the arms: `(null)` first, then `([ ... ])`, then `({ ... })`, then `($other:expr)`.
//! 2. Array arm: `([ $($elem:tt),* $(,)? ]) => { Json::Array(vec![ $( json!($elem) ),* ]) }`.
//! 3. Object arm: `({ $($key:literal : $value:tt),* $(,)? })`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the macro mirrors the JSON grammar. `null` and the two bracketed forms get their own
//! arms; each element of an array or object is a single token tree that is fed back into `json!`,
//! so nesting is handled by recursion. Every other input is an ordinary Rust expression converted
//! with `From`. `Display` walks the tree recursively, writing separators between items.
//!
//! **Rust concepts:**
//! - A `tt` (token tree) matches one token or one whole bracketed group, which is what makes the
//!   recursive descent possible: `[true, null]` is a single `tt`.
//! - `$key:literal` matches a literal token; it may be followed by `:` (only `expr`, `stmt`, `pat`,
//!   `path`, `ty` and `vis` fragments have follow-set restrictions).
//! - `json!(1)` picks `From<i32>`: an integer literal with several candidate impls falls back to
//!   `i32`.
//! - `Display` for a recursive enum calls `write!(f, "{item}")` on the children, which recurses
//!   through the same `fmt` method.
// @solution-end

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl From<bool> for Json {
    fn from(value: bool) -> Self {
        // @solution-start
        Json::Bool(value)
        // @solution-end
    }
}

impl From<i32> for Json {
    fn from(value: i32) -> Self {
        // @solution-start
        Json::Number(f64::from(value))
        // @solution-end
    }
}

impl From<i64> for Json {
    fn from(value: i64) -> Self {
        // @solution-start
        Json::Number(value as f64)
        // @solution-end
    }
}

impl From<f64> for Json {
    fn from(value: f64) -> Self {
        // @solution-start
        Json::Number(value)
        // @solution-end
    }
}

impl From<&str> for Json {
    fn from(value: &str) -> Self {
        // @solution-start
        Json::Str(value.to_string())
        // @solution-end
    }
}

impl From<String> for Json {
    fn from(value: String) -> Self {
        // @solution-start
        Json::Str(value)
        // @solution-end
    }
}

// Only invoked by the tests below (in a real crate it would be `#[macro_export]`ed).
#[allow(unused_macros)]
macro_rules! json {
    // @solution-start replace: ($($t:tt)*) => { todo!() };
    (null) => {
        Json::Null
    };
    ([ $($elem:tt),* $(,)? ]) => {
        Json::Array(vec![ $( json!($elem) ),* ])
    };
    ({ $($key:literal : $value:tt),* $(,)? }) => {
        Json::Object(vec![ $( (String::from($key), json!($value)) ),* ])
    };
    ($other:expr) => {
        Json::from($other)
    };
    // @solution-end
}

// @solution-start remove
fn write_escaped(f: &mut fmt::Formatter<'_>, s: &str) -> fmt::Result {
    f.write_str("\"")?;
    for c in s.chars() {
        match c {
            '"' => f.write_str("\\\"")?,
            '\\' => f.write_str("\\\\")?,
            '\n' => f.write_str("\\n")?,
            other => write!(f, "{other}")?,
        }
    }
    f.write_str("\"")
}
// @solution-end

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // @solution-start
        match self {
            Json::Null => f.write_str("null"),
            Json::Bool(b) => write!(f, "{b}"),
            Json::Number(n) => write!(f, "{n}"),
            Json::Str(s) => write_escaped(f, s),
            Json::Array(items) => {
                f.write_str("[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        f.write_str(",")?;
                    }
                    write!(f, "{item}")?;
                }
                f.write_str("]")
            }
            Json::Object(entries) => {
                f.write_str("{")?;
                for (i, (key, value)) in entries.iter().enumerate() {
                    if i > 0 {
                        f.write_str(",")?;
                    }
                    write_escaped(f, key)?;
                    write!(f, ":{value}")?;
                }
                f.write_str("}")
            }
        }
        // @solution-end
    }
}

impl Json {
    pub fn get(&self, key: &str) -> Option<&Json> {
        // @solution-start
        match self {
            Json::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalars() {
        let values: Vec<Json> = vec![json!(null), json!(true), json!(1), json!(2.5), json!("hi")];
        assert_eq!(
            values,
            vec![
                Json::Null,
                Json::Bool(true),
                Json::Number(1.0),
                Json::Number(2.5),
                Json::Str("hi".to_string())
            ]
        );
        let negative: Json = json!(-5);
        assert_eq!(negative, Json::Number(-5.0));
    }

    #[test]
    fn nested_arrays() {
        let value: Json = json!([1, "a", [true, null], []]);
        assert_eq!(
            value,
            Json::Array(vec![
                Json::Number(1.0),
                Json::Str("a".to_string()),
                Json::Array(vec![Json::Bool(true), Json::Null]),
                Json::Array(vec![]),
            ])
        );
        assert_eq!(value.to_string(), r#"[1,"a",[true,null],[]]"#);
    }

    #[test]
    fn objects_with_variables_and_expressions() {
        let age = 36;
        let name = String::from("Ada");
        let value: Json = json!({
            "name": name,
            "age": age,
            "langs": ["en", "fr"],
            "delta": (-1.5),
            "double": (age * 2),
            "extra": {},
        });
        assert_eq!(
            value.to_string(),
            r#"{"name":"Ada","age":36,"langs":["en","fr"],"delta":-1.5,"double":72,"extra":{}}"#
        );
        assert_eq!(value.get("age"), Some(&Json::Number(36.0)));
        assert_eq!(value.get("missing"), None);
        assert_eq!(value.get("langs").and_then(|l| l.get("en")), None);
    }

    #[test]
    fn strings_are_escaped() {
        let value: Json = json!({"quote": "say \"hi\"", "path": "C:\\tmp", "lines": "a\nb"});
        assert_eq!(
            value.to_string(),
            r#"{"quote":"say \"hi\"","path":"C:\\tmp","lines":"a\nb"}"#
        );
    }

    #[test]
    fn deep_nesting() {
        let big: i64 = 1 << 40;
        let value: Json = json!({"a": {"b": {"c": [[[big]]]}}});
        assert_eq!(
            value.to_string(),
            r#"{"a":{"b":{"c":[[[1099511627776]]]}}}"#
        );
        let inner = value.get("a").and_then(|a| a.get("b")).unwrap();
        assert!(matches!(inner.get("c"), Some(Json::Array(_))));
    }
}
