// ==================================================================
// PRACTICE 0547: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0547_semantic_version.md
// Run the tests:          cargo test practice::p0547
// ==================================================================

//! # Problem 0547: Semantic Versions
//!
//! - **Difficulty:** Medium
//! - **Topics:** `FromStr` + `Display` round trip, derived `PartialOrd`/`Ord` (field order matters), `Hash`, `filter_map` over parse results
//! - **Source:** Original · [std::cmp::Ord (derivable)](https://doc.rust-lang.org/std/cmp/trait.Ord.html#derivable)
//!
//! ## Problem
//!
//! A version like `1.4.2` has a `major`, a `minor` and a `patch` number. Model it as
//! `Version { major: u32, minor: u32, patch: u32 }` and:
//!
//! 1. **Derive** `PartialEq, Eq, PartialOrd, Ord, Hash` (plus `Debug, Clone, Copy`). Derived ordering
//!    compares fields **in declaration order**, which gives exactly "major first, then minor, then patch",
//!    so `1.10.0 > 1.9.7`.
//! 2. Implement `FromStr` with `type Err = VersionError`:
//!    - an optional leading `v` is allowed (`"v2.0.1"`);
//!    - then exactly three parts separated by `.`, otherwise `VersionError::WrongPartCount(n)`;
//!    - every part must be a non-empty string of ASCII digits **without leading zeros** (`"0"` is fine,
//!      `"01"` is not) that fits in `u32`, otherwise `VersionError::InvalidNumber(part)`.
//! 3. Implement `Display` as `"major.minor.patch"` (no `v`).
//! 4. `latest(versions: &[&str]) -> Option<Version>`: the greatest **valid** version; invalid strings
//!    are ignored.
//! 5. `sort_versions(versions: &[&str]) -> Result<Vec<String>, VersionError>`: all versions sorted
//!    ascending and displayed back, or the first parse error.
//! 6. `is_compatible(required: Version, candidate: Version) -> bool`: simplified Cargo caret rule:
//!    `candidate >= required`, same `major`, and when `major == 0` also the same `minor`.
//!
//! ## Examples
//!
//! ```text
//! "1.4.2".parse::<Version>()          => Ok(Version { major: 1, minor: 4, patch: 2 })
//! "v2.0.1".parse::<Version>()         => Ok(2.0.1)
//! "1.2".parse::<Version>()            => Err(WrongPartCount(2))
//! "1.02.3".parse::<Version>()         => Err(InvalidNumber("02"))
//! latest(&["1.9.7", "1.10.0", "oops", "0.99.99"])   => Some(1.10.0)
//! sort_versions(&["1.10.0", "1.9.7", "v1.2.3"])      => Ok(["1.2.3", "1.9.7", "1.10.0"])
//! is_compatible(1.2.0, 1.9.3) => true;   is_compatible(0.2.0, 0.3.0) => false
//! ```
//!
//! ## Constraints
//!
//! - `0 <= versions.len() <= 10^4`
//!
//! ## Hints
//!
//! 1. `s.strip_prefix('v').unwrap_or(s)` removes an optional prefix.
//! 2. String comparison would put `"1.10.0"` before `"1.9.7"`; comparing parsed numbers does not.
//! 3. `versions.iter().filter_map(|s| s.parse().ok()).max()` ignores the invalid ones.

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionError {
    WrongPartCount(usize),
    InvalidNumber(String),
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("Write your solution here")
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

/// The greatest valid version.
pub fn latest(versions: &[&str]) -> Option<Version> {
    todo!("Write your solution here")
}

/// All versions sorted ascending, displayed in canonical form.
pub fn sort_versions(versions: &[&str]) -> Result<Vec<String>, VersionError> {
    todo!("Write your solution here")
}

/// Simplified caret compatibility.
pub fn is_compatible(required: Version, candidate: Version) -> bool {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(major: u32, minor: u32, patch: u32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    #[test]
    fn parsing() {
        assert_eq!("1.4.2".parse::<Version>(), Ok(v(1, 4, 2)));
        assert_eq!("v2.0.1".parse::<Version>(), Ok(v(2, 0, 1)));
        assert_eq!("0.0.0".parse::<Version>(), Ok(v(0, 0, 0)));
        assert_eq!(
            "1.2".parse::<Version>(),
            Err(VersionError::WrongPartCount(2))
        );
        assert_eq!(
            "1.2.3.4".parse::<Version>(),
            Err(VersionError::WrongPartCount(4))
        );
        assert_eq!(
            "1.02.3".parse::<Version>(),
            Err(VersionError::InvalidNumber("02".to_string()))
        );
        assert_eq!(
            "1..3".parse::<Version>(),
            Err(VersionError::InvalidNumber(String::new()))
        );
        assert_eq!(
            "1.2.99999999999".parse::<Version>(),
            Err(VersionError::InvalidNumber("99999999999".to_string()))
        );
        assert_eq!(
            "1.+2.3".parse::<Version>(),
            Err(VersionError::InvalidNumber("+2".to_string()))
        );
    }

    #[test]
    fn display_and_order() {
        assert_eq!(v(1, 10, 0).to_string(), "1.10.0");
        assert!(v(1, 10, 0) > v(1, 9, 7));
        assert!(v(2, 0, 0) > v(1, 99, 99));
        let p: Version = "v3.1.4".parse().unwrap();
        assert_eq!(p.to_string().parse::<Version>(), Ok(p));
    }

    #[test]
    fn latest_and_sorting() {
        assert_eq!(
            latest(&["1.9.7", "1.10.0", "oops", "0.99.99"]),
            Some(v(1, 10, 0))
        );
        assert_eq!(latest(&["bad", ""]), None);
        assert_eq!(
            sort_versions(&["1.10.0", "1.9.7", "v1.2.3"]),
            Ok(vec![
                "1.2.3".to_string(),
                "1.9.7".to_string(),
                "1.10.0".to_string()
            ])
        );
        assert_eq!(
            sort_versions(&["1.0.0", "x"]),
            Err(VersionError::WrongPartCount(1))
        );
    }

    #[test]
    fn compatibility() {
        assert!(is_compatible(v(1, 2, 0), v(1, 9, 3)));
        assert!(!is_compatible(v(1, 2, 0), v(2, 0, 0)));
        assert!(!is_compatible(v(1, 2, 0), v(1, 1, 9)));
        assert!(is_compatible(v(0, 2, 0), v(0, 2, 5)));
        assert!(!is_compatible(v(0, 2, 0), v(0, 3, 0)));
    }

    #[test]
    fn versions_as_hash_keys() {
        use std::collections::HashSet;
        let set: HashSet<Version> = ["1.0.0", "v1.0.0", "1.0.1"]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(set.len(), 2);
    }
}
