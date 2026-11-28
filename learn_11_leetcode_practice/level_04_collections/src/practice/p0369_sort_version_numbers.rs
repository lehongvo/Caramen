// ==================================================================
// PRACTICE 0369: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0369_sort_version_numbers.md
// Run the tests:          cargo test practice::p0369
// ==================================================================

//! # Problem 0369: Sort Version Numbers
//!
//! - **Difficulty:** Medium
//! - **Topics:** implementing `Ord` and `PartialOrd` by hand, `Ordering::then_with`, why `#[derive(Ord)]` is sometimes wrong, `max` on `Ord` items
//! - **Source:** Original · [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html) · [`Ordering`](https://doc.rust-lang.org/std/cmp/enum.Ordering.html)
//!
//! ## Problem
//!
//! Software versions look like `"1.4.2"` or, for pre-releases, `"1.4.2-beta"`. Represent them as:
//!
//! ```text
//! pub struct Version { pub major: u32, pub minor: u32, pub patch: u32, pub pre: Option<String> }
//! ```
//!
//! Versions are ordered like this (a simplified form of Semantic Versioning):
//!
//! 1. compare `major`, then `minor`, then `patch` **as numbers** (so `1.10.0 > 1.9.0`);
//! 2. if all three are equal, a pre-release is **older** than the release: `1.0.0-alpha < 1.0.0`;
//! 3. two pre-releases of the same numbers compare their labels as strings: `alpha < beta`.
//!
//! Implement:
//!
//! - `parse_version(s: &str) -> Option<Version>`: parse `"MAJOR.MINOR.PATCH"` or
//!   `"MAJOR.MINOR.PATCH-LABEL"` (exactly three numeric parts; a non-empty label after the first
//!   `'-'`); return `None` for anything else;
//! - `impl Ord for Version` (and `PartialOrd`) following the rules above, so that `<`, `sort`,
//!   `max` and `BinaryHeap` all use them;
//! - `sort_versions(list: &mut [Version])`: sort from oldest to newest;
//! - `latest_release(list: &[Version]) -> Option<&Version>`: the newest version that is **not** a
//!   pre-release.
//!
//! ## Examples
//!
//! ```text
//! parse_version("1.4.2")      => Some(Version { 1, 4, 2, None })
//! parse_version("2.0.0-rc1")  => Some(Version { 2, 0, 0, Some("rc1") })
//! parse_version("1.4")        => None
//! parse_version("1.x.0")      => None
//!
//! sorting ["1.0.0", "1.0.0-beta", "0.9.12", "1.10.0", "1.0.0-alpha", "1.2.0"]
//!   => ["0.9.12", "1.0.0-alpha", "1.0.0-beta", "1.0.0", "1.2.0", "1.10.0"]
//!
//! latest_release of ["1.2.0", "2.0.0-rc1", "1.10.0"] => "1.10.0"
//! ```
//!
//! ## Constraints
//!
//! - Every number fits in `u32`; at most `10^4` versions.
//!
//! ## Hints
//!
//! 1. `#[derive(PartialOrd, Ord)]` compares fields in declaration order, and for `Option` it puts
//!    `None` **before** `Some(_)`. That is the opposite of rule 2, so implement `Ord` yourself.
//! 2. `self.major.cmp(&other.major).then_with(|| ...)` chains the comparisons.
//! 3. Write `partial_cmp` as `Some(self.cmp(other))` so the two traits always agree.

use std::cmp::Ordering;

/// A version `major.minor.patch` with an optional pre-release label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!("Write your solution here")
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Parses `"M.m.p"` or `"M.m.p-label"`.
pub fn parse_version(s: &str) -> Option<Version> {
    todo!("Write your solution here")
}

/// Sorts from the oldest to the newest version.
pub fn sort_versions(list: &mut [Version]) {
    todo!("Write your solution here")
}

/// The newest version that is not a pre-release.
pub fn latest_release(list: &[Version]) -> Option<&Version> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Version {
        // Test helper: build the expected value directly, without calling `parse_version`.
        let (numbers, pre) = match s.split_once('-') {
            Some((n, p)) => (n, Some(p.to_string())),
            None => (s, None),
        };
        let n: Vec<u32> = numbers.split('.').map(|x| x.parse().unwrap()).collect();
        Version {
            major: n[0],
            minor: n[1],
            patch: n[2],
            pre,
        }
    }

    #[test]
    fn parse_examples() {
        assert_eq!(parse_version("1.4.2"), Some(v("1.4.2")));
        assert_eq!(parse_version("2.0.0-rc1"), Some(v("2.0.0-rc1")));
        assert_eq!(parse_version("1.4"), None);
        assert_eq!(parse_version("1.x.0"), None);
        assert_eq!(parse_version("1.2.3.4"), None);
        assert_eq!(parse_version("1.2.3-"), None);
        assert_eq!(parse_version(""), None);
    }

    #[test]
    fn ordering_rules() {
        assert!(v("1.10.0") > v("1.9.0"));
        assert!(v("1.0.0-alpha") < v("1.0.0"));
        assert!(v("1.0.0-alpha") < v("1.0.0-beta"));
        assert!(v("0.9.9") < v("1.0.0-alpha"));
        assert_eq!(v("3.1.4").cmp(&v("3.1.4")), Ordering::Equal);
    }

    #[test]
    fn sort_example() {
        let mut list: Vec<Version> = [
            "1.0.0",
            "1.0.0-beta",
            "0.9.12",
            "1.10.0",
            "1.0.0-alpha",
            "1.2.0",
        ]
        .iter()
        .map(|s| v(s))
        .collect();
        sort_versions(&mut list);
        let expected: Vec<Version> = [
            "0.9.12",
            "1.0.0-alpha",
            "1.0.0-beta",
            "1.0.0",
            "1.2.0",
            "1.10.0",
        ]
        .iter()
        .map(|s| v(s))
        .collect();
        assert_eq!(list, expected);
    }

    #[test]
    fn latest_release_examples() {
        let list = vec![v("1.2.0"), v("2.0.0-rc1"), v("1.10.0")];
        assert_eq!(latest_release(&list), Some(&v("1.10.0")));
        assert_eq!(latest_release(&[v("1.0.0-alpha")]), None);
        assert_eq!(latest_release(&[]), None);
    }

    #[test]
    fn many_versions() {
        let mut list: Vec<Version> = (0..10_000u32)
            .map(|i| Version {
                major: i % 7,
                minor: (i * 13) % 11,
                patch: i % 5,
                pre: if i % 3 == 0 {
                    Some(format!("rc{}", i % 4))
                } else {
                    None
                },
            })
            .collect();
        sort_versions(&mut list);
        assert!(list.windows(2).all(|w| w[0] <= w[1]));
        let latest = latest_release(&list).cloned();
        assert_eq!(
            latest.map(|l| (l.major, l.minor, l.pre)),
            Some((6, 10, None))
        );
    }
}
