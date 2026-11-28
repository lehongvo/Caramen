# Problem 0195: Compare Version Numbers

- **Difficulty:** Medium
- **Topics:** two `split('.')` iterators, `str::parse`, `Ordering`, treating a missing part as `0`
- **Source:** [LeetCode 165. Compare Version Numbers](https://leetcode.com/problems/compare-version-numbers/)

## Problem

A version number is a list of *revisions* separated by dots, such as `"1.0.12"`. Each revision is
a non-negative integer that may have leading zeros (`"01"` and `"1"` are the same revision).

Compare two versions revision by revision, from left to right. If one version has fewer
revisions, its missing revisions count as `0` (so `"1.0"` equals `"1"`). Return `-1` if
`version1 < version2`, `1` if `version1 > version2`, and `0` if they are equal.

Implement `Solution::compare_version(version1: String, version2: String) -> i32`.

## Examples

```text
Input:  version1 = "1.2", version2 = "1.10"
Output: -1               (2 < 10)

Input:  version1 = "1.01", version2 = "1.001"
Output: 0                (both are 1.1)

Input:  version1 = "1.0", version2 = "1.0.0.0"
Output: 0
```

## Constraints

- `1 <= version1.len(), version2.len() <= 500`
- Versions contain only digits and `'.'`, are valid, and every revision fits in a 32-bit integer.

## Hints

1. Create `version1.split('.')` and `version2.split('.')` and advance both together.
2. Parse each revision with `part.parse::<u32>()`; leading zeros are accepted. Use `0` when one
   iterator is exhausted.
3. Stop at the first difference; if both run out, the versions are equal.

## Starter code

Write your code in [`src/practice/p0195_compare_version_numbers.rs`](../src/practice/p0195_compare_version_numbers.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0195
```

```rust
pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
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

    fn cmp(a: &str, b: &str) -> i32 {
        Solution::compare_version(a.to_string(), b.to_string())
    }

    #[test]
    fn example_numeric_not_lexicographic() {
        assert_eq!(cmp("1.2", "1.10"), -1);
    }

    #[test]
    fn example_leading_zeros() {
        assert_eq!(cmp("1.01", "1.001"), 0);
    }

    #[test]
    fn example_missing_revisions_are_zero() {
        assert_eq!(cmp("1.0", "1.0.0.0"), 0);
    }

    #[test]
    fn greater_and_extra_revision() {
        assert_eq!(cmp("2.0", "1.9.9"), 1);
        assert_eq!(cmp("1.0.1", "1"), 1);
        assert_eq!(cmp("1", "1.0.1"), -1);
    }

    #[test]
    fn large_revisions() {
        assert_eq!(cmp("4294967295", "4294967294"), 1);
        assert_eq!(cmp("0001.0000000000002", "1.2"), 0);
    }

    #[test]
    fn long_versions() {
        let a = vec!["0"; 250].join(".");
        let b = format!("{}.1", vec!["0"; 249].join("."));
        assert_eq!(cmp(&a, &b), -1);
        assert_eq!(cmp(&a, "0"), 0);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** walk both lists of revisions in lock step. A finished list contributes zeros. The first
pair of different numbers decides the answer; reaching the end of both lists means equality.

**Complexity:** `O(n + m)` time, `O(1)` extra space.

**Rust concepts:**
- `split('.')` yields borrowed `&str` revisions; `parse::<u32>()` returns a `Result`, and the
  constraints guarantee it is `Ok` (hence `expect` with a reason).
- `Ord::cmp` returns `std::cmp::Ordering`; `Ordering::Equal` lets the loop continue.
- Comparing numbers instead of strings makes `"10" > "9"` and `"01" == "1"` work.

```rust
use std::cmp::Ordering;

/// Parses one revision, or returns 0 for a missing one.
fn revision(part: Option<&str>) -> u32 {
    part.map_or(0, |p| p.parse().expect("every revision is a 32-bit number"))
}

pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let (mut a, mut b) = (version1.split('.'), version2.split('.'));
        loop {
            match (a.next(), b.next()) {
                (None, None) => return 0,
                (x, y) => match revision(x).cmp(&revision(y)) {
                    Ordering::Less => return -1,
                    Ordering::Greater => return 1,
                    Ordering::Equal => {}
                },
            }
        }
    }
}
```

Full tested solution: [`src/solutions/p0195_compare_version_numbers.rs`](../src/solutions/p0195_compare_version_numbers.rs)

</details>
