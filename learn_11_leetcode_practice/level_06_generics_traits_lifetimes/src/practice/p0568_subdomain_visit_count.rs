// ==================================================================
// PRACTICE 0568: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0568_subdomain_visit_count.md
// Run the tests:          cargo test practice::p0568
// ==================================================================

//! # Problem 0568: Subdomain Visit Count
//!
//! - **Difficulty:** Medium
//! - **Topics:** `HashMap<&str, i32>` keyed by sub-slices of the input, `split_once`, `match_indices`, borrowing for the whole function
//! - **Source:** [LeetCode 811. Subdomain Visit Count](https://leetcode.com/problems/subdomain-visit-count/)
//!
//! ## Problem
//!
//! A visit to `discuss.leetcode.com` also counts as a visit to `leetcode.com` and to `com`. Each input
//! string has the form `"<count> <domain>"`, e.g. `"9001 discuss.leetcode.com"`, meaning the domain was
//! visited `count` times.
//!
//! Return, in any order, one string `"<total> <domain>"` for every domain and parent domain that was
//! visited, with its total number of visits.
//!
//! Implement `Solution::subdomain_visits(cpdomains: Vec<String>) -> Vec<String>`.
//!
//! Every parent domain is a **suffix** of the full domain (`"leetcode.com"` is a suffix of
//! `"discuss.leetcode.com"`), so the map can use `&str` keys that point into the input strings.
//!
//! ## Examples
//!
//! ```text
//! Input:  ["9001 discuss.leetcode.com"]
//! Output: ["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"]
//!
//! Input:  ["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"]
//! Output: ["901 mail.com", "50 yahoo.com", "900 google.mail.com", "5 wiki.org", "5 org",
//!          "1 intel.mail.com", "951 com"]                                   (any order)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= cpdomains.len() <= 100`, `1 <= count <= 10^4`
//! - Domains have 1 to 100 characters: lowercase letters and dots, with 1 or 2 dots, no leading or
//!   trailing dot.
//!
//! ## Hints
//!
//! 1. `entry.split_once(' ')` separates the count from the domain.
//! 2. The parent domains start right after each `'.'`: `&domain[i + 1..]` for every dot index `i`.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(input: &[&str]) -> Vec<String> {
        let mut out = Solution::subdomain_visits(input.iter().map(|s| s.to_string()).collect());
        out.sort();
        out
    }

    #[test]
    fn example_single_entry() {
        assert_eq!(
            sorted(&["9001 discuss.leetcode.com"]),
            vec!["9001 com", "9001 discuss.leetcode.com", "9001 leetcode.com"]
        );
    }

    #[test]
    fn example_many_entries() {
        let mut expected = vec![
            "901 mail.com",
            "50 yahoo.com",
            "900 google.mail.com",
            "5 wiki.org",
            "5 org",
            "1 intel.mail.com",
            "951 com",
        ];
        expected.sort();
        assert_eq!(
            sorted(&[
                "900 google.mail.com",
                "50 yahoo.com",
                "1 intel.mail.com",
                "5 wiki.org"
            ]),
            expected
        );
    }

    #[test]
    fn repeated_domains_add_up() {
        assert_eq!(
            sorted(&["3 a.b", "4 a.b", "1 c.b"]),
            vec!["1 c.b", "7 a.b", "8 b"]
        );
    }

    #[test]
    fn many_entries() {
        let input: Vec<String> = (0..100)
            .map(|i| format!("{} s{}.site.com", 100, i % 10))
            .collect();
        let refs: Vec<&str> = input.iter().map(String::as_str).collect();
        let out = sorted(&refs);
        assert_eq!(out.len(), 12);
        assert!(out.contains(&"10000 com".to_string()));
        assert!(out.contains(&"10000 site.com".to_string()));
        assert!(out.contains(&"1000 s7.site.com".to_string()));
    }
}
