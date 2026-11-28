# Problem 0283: Semantic Versions

- **Difficulty:** Medium
- **Topics:** parsing into a struct, derived `PartialOrd`/`Ord` on a struct, an enum selecting what to change, struct update syntax, `filter_map` + `max`
- **Source:** Original · [The Rust Book, Appendix C](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) · [The Rust Book 5.1](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

## Problem

A version number `MAJOR.MINOR.PATCH` (like `1.4.2`) is stored as
`Version { major, minor, patch }` (all `u64`). Versions compare field by field: first `major`,
then `minor`, then `patch`. Implement:

- `Version::parse(s: &str) -> Option<Version>`: accepts an optional leading `'v'` and exactly
  three dot-separated numbers. Each number is made of ASCII digits only, with **no leading zero**
  unless it is exactly `"0"`. Anything else gives `None`;
- `text(&self) -> String`: `"MAJOR.MINOR.PATCH"` (without `v`);
- `bump(self, part: Part) -> Version` with `enum Part { Major, Minor, Patch }`: increment that
  part and reset the parts after it to `0` (`1.4.2` bumped at `Minor` is `1.5.0`);
- `is_compatible(self, other: Version) -> bool`: "caret" compatibility. If `self.major > 0`,
  `other` is compatible when it has the same `major` and is `>= self`. If `self.major == 0`, it
  must have the same `major` **and** `minor` and be `>= self`;
- `latest(candidates: &[&str]) -> Option<Version>`: the greatest valid version among the
  strings (invalid ones are ignored).

## Examples

```text
Version::parse("1.4.2")     => Some(Version { major: 1, minor: 4, patch: 2 })
Version::parse("v0.10.0")   => Some(Version { major: 0, minor: 10, patch: 0 })
Version::parse("1.04.2")    => None     (leading zero)
Version::parse("1.4")       => None
Version::parse("1.4.2.0")   => None
1.4.2.bump(Minor)           => 1.5.0
1.4.2.bump(Major)           => 2.0.0
1.4.2.is_compatible(1.9.0)  => true
1.4.2.is_compatible(2.0.0)  => false
0.3.1.is_compatible(0.4.0)  => false
latest(&["1.2.10", "1.10.0", "oops", "1.9.9"]) => Some(1.10.0)
```

## Constraints

- Numbers fit in a `u64`; `candidates.len() <= 10^4`.

## Hints

1. Declare the fields in the order `major, minor, patch` and derive `PartialOrd, Ord`: the
   derived comparison is exactly version ordering (unlike comparing the strings: `"1.10.0" < "1.9.0"`).
2. `let [major, minor, patch] = parts[..] else { return None; };` checks there are three parts.
3. `Version { minor: self.minor + 1, patch: 0, ..self }` uses struct update syntax.

## Starter code

Write your code in [`src/practice/p0283_semantic_version.rs`](../src/practice/p0283_semantic_version.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0283
```

```rust
/// A semantic version `major.minor.patch`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

/// Which part of a version to bump.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Major,
    Minor,
    Patch,
}

impl Version {
    /// Parses `"1.2.3"` or `"v1.2.3"`.
    pub fn parse(s: &str) -> Option<Version> {
        todo!("Write your solution here")
    }

    /// `"MAJOR.MINOR.PATCH"`.
    pub fn text(&self) -> String {
        todo!("Write your solution here")
    }

    /// Increments one part and resets the later parts.
    pub fn bump(self, part: Part) -> Version {
        todo!("Write your solution here")
    }

    /// Caret compatibility: can `other` replace `self`?
    pub fn is_compatible(self, other: Version) -> bool {
        todo!("Write your solution here")
    }
}

/// The greatest valid version among the candidates.
pub fn latest(candidates: &[&str]) -> Option<Version> {
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

    fn v(major: u64, minor: u64, patch: u64) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    #[test]
    fn parsing() {
        assert_eq!(Version::parse("1.4.2"), Some(v(1, 4, 2)));
        assert_eq!(Version::parse("v0.10.0"), Some(v(0, 10, 0)));
        assert_eq!(Version::parse("0.0.0"), Some(v(0, 0, 0)));
        assert_eq!(Version::parse("1.04.2"), None);
        assert_eq!(Version::parse("1.4"), None);
        assert_eq!(Version::parse("1.4.2.0"), None);
        assert_eq!(Version::parse("1..2"), None);
        assert_eq!(Version::parse("1.4.+2"), None);
        assert_eq!(Version::parse("V1.4.2"), None);
        assert_eq!(Version::parse("vv1.4.2"), None);
    }

    #[test]
    fn derived_order_is_numeric() {
        assert!(v(1, 10, 0) > v(1, 9, 9));
        assert!(v(2, 0, 0) > v(1, 99, 99));
        assert!(v(1, 2, 3) == v(1, 2, 3));
        assert_eq!(v(1, 10, 0).text(), "1.10.0");
    }

    #[test]
    fn bumping() {
        assert_eq!(v(1, 4, 2).bump(Part::Minor), v(1, 5, 0));
        assert_eq!(v(1, 4, 2).bump(Part::Major), v(2, 0, 0));
        assert_eq!(v(1, 4, 2).bump(Part::Patch), v(1, 4, 3));
    }

    #[test]
    fn caret_compatibility() {
        assert!(v(1, 4, 2).is_compatible(v(1, 9, 0)));
        assert!(v(1, 4, 2).is_compatible(v(1, 4, 2)));
        assert!(!v(1, 4, 2).is_compatible(v(1, 4, 1)));
        assert!(!v(1, 4, 2).is_compatible(v(2, 0, 0)));
        assert!(v(0, 3, 1).is_compatible(v(0, 3, 7)));
        assert!(!v(0, 3, 1).is_compatible(v(0, 4, 0)));
    }

    #[test]
    fn latest_ignores_invalid() {
        assert_eq!(
            latest(&["1.2.10", "1.10.0", "oops", "1.9.9"]),
            Some(v(1, 10, 0))
        );
        assert_eq!(latest(&["x", "1.2"]), None);
        assert_eq!(latest(&[]), None);
    }

    #[test]
    fn many_candidates() {
        let owned: Vec<String> = (0..10_000)
            .map(|i| format!("{}.{}.{}", i % 7, i % 13, i))
            .collect();
        let refs: Vec<&str> = owned.iter().map(String::as_str).collect();
        let best = latest(&refs).expect("all valid");
        assert_eq!(best.major, 6);
        assert_eq!(
            best,
            (0..10_000u64)
                .map(|i| v(i % 7, i % 13, i))
                .max()
                .expect("non-empty")
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** parse each part with a helper that enforces the digit and leading-zero rules, rely on
the derived `Ord` for comparisons and `max`, and express `bump` as a `match` on `Part` that
builds the new version with struct update syntax.

**Rust concepts:**
- Deriving `PartialOrd`/`Ord` on a struct compares fields lexicographically **in declaration
  order**, so the field order is part of the meaning of the type.
- `s.strip_prefix('v').unwrap_or(s)` removes an optional prefix.
- `Version { patch: self.patch + 1, ..self }` copies the remaining fields from `self`
  (possible because `Version: Copy`).
- `candidates.iter().filter_map(|s| Version::parse(s)).max()` combines parsing, filtering and the
  derived order in one chain.

```rust
/// A semantic version `major.minor.patch`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

/// Which part of a version to bump.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Major,
    Minor,
    Patch,
}

/// Digits only, no leading zero unless the number is exactly "0".
fn parse_number(s: &str) -> Option<u64> {
    let valid = match s.as_bytes() {
        [] => false,
        [b'0'] => true,
        [b'0', ..] => false,
        bytes => bytes.iter().all(u8::is_ascii_digit),
    };
    if valid { s.parse().ok() } else { None }
}

impl Version {
    /// Parses `"1.2.3"` or `"v1.2.3"`.
    pub fn parse(s: &str) -> Option<Version> {
        let s = s.strip_prefix('v').unwrap_or(s);
        let parts: Vec<&str> = s.split('.').collect();
        let [major, minor, patch] = parts[..] else {
            return None;
        };
        Some(Version {
            major: parse_number(major)?,
            minor: parse_number(minor)?,
            patch: parse_number(patch)?,
        })
    }

    /// `"MAJOR.MINOR.PATCH"`.
    pub fn text(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    /// Increments one part and resets the later parts.
    pub fn bump(self, part: Part) -> Version {
        match part {
            Part::Major => Version {
                major: self.major + 1,
                minor: 0,
                patch: 0,
            },
            Part::Minor => Version {
                minor: self.minor + 1,
                patch: 0,
                ..self
            },
            Part::Patch => Version {
                patch: self.patch + 1,
                ..self
            },
        }
    }

    /// Caret compatibility: can `other` replace `self`?
    pub fn is_compatible(self, other: Version) -> bool {
        let same_line = match self.major {
            0 => other.major == 0 && other.minor == self.minor,
            major => other.major == major,
        };
        same_line && other >= self
    }
}

/// The greatest valid version among the candidates.
pub fn latest(candidates: &[&str]) -> Option<Version> {
    candidates.iter().filter_map(|s| Version::parse(s)).max()
}
```

Full tested solution: [`src/solutions/p0283_semantic_version.rs`](../src/solutions/p0283_semantic_version.rs)

</details>
