# Problem 0282: INI Line Parser

- **Difficulty:** Medium
- **Topics:** an enum of line kinds with data, `strip_prefix` / `strip_suffix` in `if let`, `split_once`, tracking state while scanning
- **Source:** Original · [The Rust Book 6.3](https://doc.rust-lang.org/book/ch06-03-if-let.html) · [`str::strip_prefix`](https://doc.rust-lang.org/std/primitive.str.html#method.strip_prefix)

## Problem

INI files are simple configuration files made of lines like these:

```text
; a comment
name = demo
[server]
host = "example.org"
port=8080
```

Classify one line with `parse_line(line: &str) -> Line`, after trimming surrounding whitespace:

- empty: `Line::Blank`;
- starting with `;` or `#`: `Line::Comment(text)`, the text after the marker, trimmed;
- `[name]` where `name` (trimmed) is not empty: `Line::Section(name)`;
- `key = value` (split at the **first** `=`), where the trimmed key is not empty:
  `Line::Pair { key, value }` with both trimmed. If the value is wrapped in double quotes
  (`"..."`, at least two characters), remove them;
- anything else: `Line::Invalid`.

Then write `lookup(text: &str, section: &str, key: &str) -> Option<String>`: the value of `key`
inside `section`. Pairs before the first section header belong to the section `""`. If a key
appears several times in the same section, the **last** one wins.

## Examples

```text
parse_line("  ; hello ")           => Comment("hello")
parse_line("[ server ]")           => Section("server")
parse_line("[]")                   => Invalid
parse_line("host = \"a b\"")       => Pair { key: "host", value: "a b" }
parse_line("url = x=1")            => Pair { key: "url", value: "x=1" }
parse_line("= 5")                  => Invalid   (empty key)
parse_line("just words")           => Invalid

text = "name = demo\n[server]\nport = 80\nport = 8080\n[client]\nport = 1"
lookup(text, "server", "port")     => Some("8080")
lookup(text, "", "name")           => Some("demo")
lookup(text, "client", "name")     => None
```

## Constraints

- At most `10^4` lines.

## Hints

1. `if let Some(rest) = line.strip_prefix(';') { ... }` both tests and removes the marker.
2. For sections, chain `strip_prefix('[')` and `strip_suffix(']')` with `and_then`.
3. In `lookup`, keep `current_section: String` up to date while scanning, and remember the last
   matching value.

## Starter code

Write your code in [`src/practice/p0282_ini_line_parser.rs`](../src/practice/p0282_ini_line_parser.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0282
```

```rust
/// One classified INI line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Blank,
    Comment(String),
    Section(String),
    Pair { key: String, value: String },
    Invalid,
}

/// Classifies one line.
pub fn parse_line(line: &str) -> Line {
    todo!("Write your solution here")
}

/// The last value of `key` in `section` (`""` for pairs before any header).
pub fn lookup(text: &str, section: &str, key: &str) -> Option<String> {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn pair(k: &str, v: &str) -> Line {
        Line::Pair {
            key: k.to_string(),
            value: v.to_string(),
        }
    }

    #[test]
    fn blanks_and_comments() {
        assert_eq!(parse_line(""), Line::Blank);
        assert_eq!(parse_line("   \t"), Line::Blank);
        assert_eq!(parse_line("  ; hello "), Line::Comment("hello".to_string()));
        assert_eq!(parse_line("#x = 1"), Line::Comment("x = 1".to_string()));
    }

    #[test]
    fn sections() {
        assert_eq!(
            parse_line("[ server ]"),
            Line::Section("server".to_string())
        );
        assert_eq!(parse_line("[a.b]"), Line::Section("a.b".to_string()));
        assert_eq!(parse_line("[]"), Line::Invalid);
        assert_eq!(parse_line("[  ]"), Line::Invalid);
        assert_eq!(parse_line("[open"), Line::Invalid);
    }

    #[test]
    fn pairs() {
        assert_eq!(parse_line("host = \"a b\""), pair("host", "a b"));
        assert_eq!(parse_line("port=8080"), pair("port", "8080"));
        assert_eq!(parse_line("url = x=1"), pair("url", "x=1"));
        assert_eq!(parse_line("empty ="), pair("empty", ""));
        assert_eq!(parse_line("q = \""), pair("q", "\""));
        assert_eq!(parse_line("q = \"\""), pair("q", ""));
        assert_eq!(parse_line("= 5"), Line::Invalid);
        assert_eq!(parse_line("just words"), Line::Invalid);
    }

    #[test]
    fn lookups() {
        let text = "name = demo\n[server]\nport = 80\nport = 8080\n[client]\nport = 1";
        assert_eq!(lookup(text, "server", "port"), Some("8080".to_string()));
        assert_eq!(lookup(text, "", "name"), Some("demo".to_string()));
        assert_eq!(lookup(text, "client", "port"), Some("1".to_string()));
        assert_eq!(lookup(text, "client", "name"), None);
        assert_eq!(lookup(text, "missing", "port"), None);
    }

    #[test]
    fn sections_can_reopen() {
        let text = "[a]\nk = 1\n[b]\nk = 2\n[a]\nk = 3\n; [b]\nbroken line";
        assert_eq!(lookup(text, "a", "k"), Some("3".to_string()));
        assert_eq!(lookup(text, "b", "k"), Some("2".to_string()));
    }

    #[test]
    fn many_lines() {
        let text: String = (0..10_000)
            .map(|i| format!("key{} = {i}\n", i % 10))
            .collect();
        assert_eq!(lookup(&text, "", "key7"), Some("9997".to_string()));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `parse_line` tries the line kinds in the order of the statement, each with an `if let`
on a `strip_*` or `split_once` call. `lookup` scans the lines with a `match` on the parsed
`Line`, tracking the current section.

**Rust concepts:**
- `strip_prefix` / `strip_suffix` return `Option<&str>`, so they combine naturally with
  `if let` and `and_then`.
- `line.strip_prefix([';', '#'])` accepts either marker: an array of `char`s is a valid string
  pattern meaning "any one of these characters".
- The `Line` enum separates parsing from interpretation: `lookup` never looks at raw text.
- Matching a `Line` by value moves the `String`s out of it, avoiding clones.

```rust
/// One classified INI line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Blank,
    Comment(String),
    Section(String),
    Pair { key: String, value: String },
    Invalid,
}

/// Classifies one line.
pub fn parse_line(line: &str) -> Line {
    let line = line.trim();
    if line.is_empty() {
        return Line::Blank;
    }
    if let Some(rest) = line.strip_prefix([';', '#']) {
        return Line::Comment(rest.trim().to_string());
    }
    if let Some(name) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        let name = name.trim();
        return if name.is_empty() {
            Line::Invalid
        } else {
            Line::Section(name.to_string())
        };
    }
    if let Some((key, value)) = line.split_once('=') {
        let key = key.trim();
        if key.is_empty() {
            return Line::Invalid;
        }
        let value = value.trim();
        let value = match value.strip_prefix('"').and_then(|v| v.strip_suffix('"')) {
            Some(inner) => inner,
            None => value,
        };
        return Line::Pair {
            key: key.to_string(),
            value: value.to_string(),
        };
    }
    Line::Invalid
}

/// The last value of `key` in `section` (`""` for pairs before any header).
pub fn lookup(text: &str, section: &str, key: &str) -> Option<String> {
    let mut current = String::new();
    let mut found = None;
    for raw in text.lines() {
        match parse_line(raw) {
            Line::Section(name) => current = name,
            Line::Pair { key: k, value } if current == section && k == key => found = Some(value),
            _ => {}
        }
    }
    found
}
```

Full tested solution: [`src/solutions/p0282_ini_line_parser.rs`](../src/solutions/p0282_ini_line_parser.rs)

</details>
