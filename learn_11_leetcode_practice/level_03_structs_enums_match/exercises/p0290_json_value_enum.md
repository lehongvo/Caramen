# Problem 0290: A JSON-like Value Enum

- **Difficulty:** Medium
- **Topics:** a recursive `enum` (through `Vec`), recursion with `match`, returning `Option<&T>` into a tree, escaping strings
- **Source:** Original · [The Rust Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)

## Problem

Model JSON data with one enum whose variants can contain more values of the same enum:

```text
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),   // keys in insertion order
}
```

An enum can refer to itself through a `Vec` (which stores its elements on the heap). Implement:

- `to_text(&self) -> String`: compact JSON with no spaces: `null`, `true`, numbers printed with
  `{}` (so `1.0` prints as `1` and `-0.5` as `-0.5`), strings in double quotes with `"` and `\`
  escaped as `\"` and `\\`, arrays as `[a,b]`, objects as `{"k":v,...}`;
- `depth(&self) -> usize`: `1` for a scalar or an empty container, otherwise `1` + the largest
  depth among the children;
- `get(&self, key: &str) -> Option<&Json>`: for an `Object`, the value of the first entry with
  that key; `None` for anything else;
- `at_path(&self, path: &[&str]) -> Option<&Json>`: follow the path step by step: in an
  object, a step is a key; in an array, a step is an index (`"0"`, `"1"`, ...). An empty path
  returns `self`;
- `sum_numbers(&self) -> f64`: the sum of every `Number` anywhere inside the value.

## Examples

```text
v = Object([("name", Str("Ana")), ("tags", Array([Str("a"), Number(2.0)])), ("ok", Bool(true))])
v.to_text()                  => {"name":"Ana","tags":["a",2],"ok":true}
v.depth()                    => 3
v.get("ok")                  => Some(Bool(true))
v.at_path(&["tags", "1"])    => Some(Number(2.0))
v.at_path(&["tags", "9"])    => None
v.sum_numbers()              => 2.0
Str("say \"hi\"").to_text()  => "say \"hi\""     (with the quotes escaped)
```

## Constraints

- Values are at most 100 levels deep.

## Hints

1. Every function is a `match self` whose container arms call the same function on the children.
2. `items.iter().map(Json::to_text).collect::<Vec<_>>().join(",")` builds the inside of an array.
3. In `at_path`, `match (self, path) { (_, []) => Some(self), (Json::Object(_), [key, rest @ ..]) => ..., ... }`.

## Starter code

Write your code in [`src/practice/p0290_json_value_enum.rs`](../src/practice/p0290_json_value_enum.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0290
```

```rust
/// A JSON-like value.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    /// Compact JSON text.
    pub fn to_text(&self) -> String {
        todo!("Write your solution here")
    }

    /// Nesting depth: 1 for scalars and empty containers.
    pub fn depth(&self) -> usize {
        todo!("Write your solution here")
    }

    /// The value of the first entry with `key`, for objects.
    pub fn get(&self, key: &str) -> Option<&Json> {
        todo!("Write your solution here")
    }

    /// Follows object keys and array indexes.
    pub fn at_path(&self, path: &[&str]) -> Option<&Json> {
        todo!("Write your solution here")
    }

    /// The sum of all numbers inside the value.
    pub fn sum_numbers(&self) -> f64 {
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
    use super::Json::*;
    use super::*;

    fn obj(entries: Vec<(&str, Json)>) -> Json {
        Object(
            entries
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        )
    }

    fn sample() -> Json {
        obj(vec![
            ("name", Str("Ana".into())),
            ("tags", Array(vec![Str("a".into()), Number(2.0)])),
            ("ok", Bool(true)),
        ])
    }

    #[test]
    fn serializing() {
        assert_eq!(
            sample().to_text(),
            r#"{"name":"Ana","tags":["a",2],"ok":true}"#
        );
        assert_eq!(Null.to_text(), "null");
        assert_eq!(Number(-0.5).to_text(), "-0.5");
        assert_eq!(Array(vec![]).to_text(), "[]");
        assert_eq!(obj(vec![]).to_text(), "{}");
        assert_eq!(
            Str(r#"say "hi" \o/"#.into()).to_text(),
            r#""say \"hi\" \\o/""#
        );
    }

    #[test]
    fn depths() {
        assert_eq!(sample().depth(), 3);
        assert_eq!(Null.depth(), 1);
        assert_eq!(Array(vec![]).depth(), 1);
        assert_eq!(Array(vec![Array(vec![Array(vec![])])]).depth(), 3);
    }

    #[test]
    fn lookups() {
        let v = sample();
        assert_eq!(v.get("ok"), Some(&Bool(true)));
        assert_eq!(v.get("missing"), None);
        assert_eq!(Number(1.0).get("x"), None);
        assert_eq!(v.at_path(&["tags", "1"]), Some(&Number(2.0)));
        assert_eq!(v.at_path(&["tags", "9"]), None);
        assert_eq!(v.at_path(&["tags", "x"]), None);
        assert_eq!(v.at_path(&["name", "first"]), None);
        assert_eq!(v.at_path(&[]), Some(&v));
    }

    #[test]
    fn summing_numbers() {
        assert_eq!(sample().sum_numbers(), 2.0);
        let nested = Array(vec![
            Number(1.5),
            obj(vec![
                ("a", Number(2.0)),
                ("b", Array(vec![Number(-0.5), Null])),
            ]),
        ]);
        assert_eq!(nested.sum_numbers(), 3.0);
        assert_eq!(Str("5".into()).sum_numbers(), 0.0);
    }

    #[test]
    fn deep_nesting() {
        let mut v = Number(1.0);
        for _ in 0..99 {
            v = Array(vec![v]);
        }
        assert_eq!(v.depth(), 100);
        assert_eq!(v.sum_numbers(), 1.0);
        let path = vec!["0"; 99];
        assert_eq!(v.at_path(&path), Some(&Number(1.0)));
        assert_eq!(v.to_text().len(), 1 + 2 * 99);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** structural recursion: each function handles the scalar variants directly and, for
containers, combines the results of recursive calls on the elements.

**Rust concepts:**
- A recursive enum needs indirection so that its size is finite; `Vec<Json>` provides it
  (`Box<Json>` would be the other common choice).
- Matching on a tuple `(self, path)` with slice patterns (`[]`, `[step, rest @ ..]`) expresses
  "walk the path" without manual indexing.
- Returned references (`Option<&Json>`) borrow from `self`, so lookups never clone the tree.
- `fold(0.0, |acc, x| acc + x.sum_numbers())` and `.map(..).max()` aggregate over children.

```rust
/// A JSON-like value.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

fn quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

impl Json {
    /// Compact JSON text.
    pub fn to_text(&self) -> String {
        match self {
            Json::Null => String::from("null"),
            Json::Bool(b) => b.to_string(),
            Json::Number(n) => n.to_string(),
            Json::Str(s) => quote(s),
            Json::Array(items) => {
                let inner: Vec<String> = items.iter().map(Json::to_text).collect();
                format!("[{}]", inner.join(","))
            }
            Json::Object(entries) => {
                let inner: Vec<String> = entries
                    .iter()
                    .map(|(k, v)| format!("{}:{}", quote(k), v.to_text()))
                    .collect();
                format!("{{{}}}", inner.join(","))
            }
        }
    }

    /// Nesting depth: 1 for scalars and empty containers.
    pub fn depth(&self) -> usize {
        let children_depth = match self {
            Json::Array(items) => items.iter().map(Json::depth).max(),
            Json::Object(entries) => entries.iter().map(|(_, v)| v.depth()).max(),
            _ => None,
        };
        1 + children_depth.unwrap_or(0)
    }

    /// The value of the first entry with `key`, for objects.
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Follows object keys and array indexes.
    pub fn at_path(&self, path: &[&str]) -> Option<&Json> {
        match (self, path) {
            (_, []) => Some(self),
            (Json::Object(_), [key, rest @ ..]) => self.get(key)?.at_path(rest),
            (Json::Array(items), [index, rest @ ..]) => {
                let i: usize = index.parse().ok()?;
                items.get(i)?.at_path(rest)
            }
            _ => None,
        }
    }

    /// The sum of all numbers inside the value.
    pub fn sum_numbers(&self) -> f64 {
        match self {
            Json::Number(n) => *n,
            Json::Array(items) => items.iter().fold(0.0, |acc, x| acc + x.sum_numbers()),
            Json::Object(entries) => entries
                .iter()
                .fold(0.0, |acc, (_, v)| acc + v.sum_numbers()),
            Json::Null | Json::Bool(_) | Json::Str(_) => 0.0,
        }
    }
}
```

Full tested solution: [`src/solutions/p0290_json_value_enum.rs`](../src/solutions/p0290_json_value_enum.rs)

</details>
