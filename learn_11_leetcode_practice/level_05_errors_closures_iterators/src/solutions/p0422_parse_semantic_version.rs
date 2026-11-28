//! # Problem 0422: Parse and Compare Semantic Versions
//!
//! - **Difficulty:** Medium
//! - **Topics:** `split`, `let ... else`, `?`, collecting into `Result<Vec<_>, _>`, derived `Ord`, `Iterator::max`
//! - **Source:** Original · [std `Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) · [Semantic Versioning](https://semver.org/)
//!
//! ## Problem
//!
//! A (simplified) semantic version has the form `MAJOR.MINOR.PATCH`, for example `1.4.2`, with an
//! optional leading `v` (`v1.4.2`). Versions are compared part by part: first `MAJOR`, then
//! `MINOR`, then `PATCH`.
//!
//! `Version` derives `PartialOrd` and `Ord`; derived ordering compares the fields **in declaration
//! order**, which is exactly the semantic-version order.
//!
//! Implement:
//!
//! - `parse_version(s: &str) -> Result<Version, String>`: remove one optional leading `'v'`, split
//!   on `'.'`. If there are not exactly 3 parts, return `"expected 3 parts, found N"`. Each part must
//!   be a non-empty string of ASCII digits, without leading zeros (`"0"` is fine, `"01"` is not),
//!   that fits in a `u64`; otherwise return ``"invalid part `PART`"`` for the first bad part.
//! - `compare(a: &str, b: &str) -> Result<Ordering, String>`: parse both (first `a`, then `b`) and
//!   compare them.
//! - `latest(versions: &[&str]) -> Result<Option<Version>, String>`: the greatest version, `None`
//!   for an empty list, or the error of the first invalid string.
//!
//! ## Examples
//!
//! ```text
//! parse_version("1.4.2")         -> Ok(Version { major: 1, minor: 4, patch: 2 })
//! parse_version("v0.10.0")       -> Ok(Version { major: 0, minor: 10, patch: 0 })
//! parse_version("1.4")           -> Err("expected 3 parts, found 2")
//! parse_version("1.04.2")        -> Err("invalid part `04`")
//! parse_version("1.x.2")         -> Err("invalid part `x`")
//! compare("1.10.0", "1.9.9")     -> Ok(Greater)      (10 > 9 as numbers, not as text)
//! latest(&["1.2.3", "1.10.0", "0.9.9"]) -> Ok(Some(Version { major: 1, minor: 10, patch: 0 }))
//! latest(&["1.2.3", "oops"])     -> Err("expected 3 parts, found 1")
//! ```
//!
//! ## Constraints
//!
//! - Inputs have at most `100` characters; lists have at most `10^3` versions.
//!
//! ## Hints
//!
//! 1. `let [a, b, c] = parts.as_slice() else { return Err(...) };` destructures a slice of exactly
//!    three elements (a `let ... else` statement).
//! 2. Write a helper that parses one part; check `p.bytes().all(|b| b.is_ascii_digit())` because
//!    `parse::<u64>()` would also accept a leading `'+'`.
//! 3. `versions.iter().map(|s| parse_version(s)).collect::<Result<Vec<_>, _>>()` gives all versions
//!    or the first error; then `max()`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** validate the shape first (three parts), then each part, so that every error message
//! is precise. Once parsed, a `Version` can be compared with the derived `Ord`, so `compare` is one
//! `cmp` call and `latest` is `collect` + `max`.
//!
//! **Rust concepts:**
//! - `let PATTERN = expr else { ... };` binds the pattern or runs the `else` block, which must
//!   leave the function (`return`, `break`, `panic!`, ...).
//! - Derived `PartialOrd`/`Ord` on a struct compare fields lexicographically in declaration order.
//! - `Result<Vec<T>, E>: FromIterator<Result<T, E>>` stops at the first error.
//! - `Result::map(|v| v.into_iter().max())` turns `Result<Vec<Version>, _>` into
//!   `Result<Option<Version>, _>`.
// @solution-end

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

// @solution-start remove
fn parse_part(part: &str) -> Result<u64, String> {
    let invalid = || format!("invalid part `{part}`");
    let digits_only = !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit());
    let leading_zero = part.len() > 1 && part.starts_with('0');
    if !digits_only || leading_zero {
        return Err(invalid());
    }
    part.parse::<u64>().map_err(|_| invalid())
}
// @solution-end

pub fn parse_version(s: &str) -> Result<Version, String> {
    // @solution-start
    let s = s.strip_prefix('v').unwrap_or(s);
    let parts: Vec<&str> = s.split('.').collect();
    let [major, minor, patch] = parts.as_slice() else {
        return Err(format!("expected 3 parts, found {}", parts.len()));
    };
    Ok(Version {
        major: parse_part(major)?,
        minor: parse_part(minor)?,
        patch: parse_part(patch)?,
    })
    // @solution-end
}

pub fn compare(a: &str, b: &str) -> Result<Ordering, String> {
    // @solution-start
    let a = parse_version(a)?;
    let b = parse_version(b)?;
    Ok(a.cmp(&b))
    // @solution-end
}

pub fn latest(versions: &[&str]) -> Result<Option<Version>, String> {
    // @solution-start
    versions
        .iter()
        .map(|s| parse_version(s))
        .collect::<Result<Vec<_>, _>>()
        .map(|all| all.into_iter().max())
    // @solution-end
}

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
    fn parse_valid_versions() {
        assert_eq!(parse_version("1.4.2"), Ok(v(1, 4, 2)));
        assert_eq!(parse_version("v0.10.0"), Ok(v(0, 10, 0)));
        assert_eq!(
            parse_version("18446744073709551615.0.0"),
            Ok(v(u64::MAX, 0, 0))
        );
    }

    #[test]
    fn wrong_number_of_parts() {
        assert_eq!(
            parse_version("1.4"),
            Err("expected 3 parts, found 2".to_string())
        );
        assert_eq!(
            parse_version("1.2.3.4"),
            Err("expected 3 parts, found 4".to_string())
        );
        assert_eq!(
            parse_version(""),
            Err("expected 3 parts, found 1".to_string())
        );
    }

    #[test]
    fn invalid_parts() {
        assert_eq!(
            parse_version("1.04.2"),
            Err("invalid part `04`".to_string())
        );
        assert_eq!(parse_version("1.x.2"), Err("invalid part `x`".to_string()));
        assert_eq!(parse_version("1..2"), Err("invalid part ``".to_string()));
        assert_eq!(
            parse_version("+1.0.0"),
            Err("invalid part `+1`".to_string())
        );
        assert_eq!(
            parse_version("vv1.0.0"),
            Err("invalid part `v1`".to_string())
        );
        assert_eq!(
            parse_version("1.0.18446744073709551616"),
            Err("invalid part `18446744073709551616`".to_string())
        );
    }

    #[test]
    fn compare_numerically() {
        assert_eq!(compare("1.10.0", "1.9.9"), Ok(Ordering::Greater));
        assert_eq!(compare("v2.0.0", "2.0.0"), Ok(Ordering::Equal));
        assert_eq!(compare("0.0.1", "0.1.0"), Ok(Ordering::Less));
        assert_eq!(
            compare("1.0", "x"),
            Err("expected 3 parts, found 2".to_string())
        );
    }

    #[test]
    fn latest_version() {
        assert_eq!(latest(&["1.2.3", "1.10.0", "0.9.9"]), Ok(Some(v(1, 10, 0))));
        assert_eq!(latest(&[]), Ok(None));
        assert_eq!(
            latest(&["1.2.3", "oops"]),
            Err("expected 3 parts, found 1".to_string())
        );
    }
}
