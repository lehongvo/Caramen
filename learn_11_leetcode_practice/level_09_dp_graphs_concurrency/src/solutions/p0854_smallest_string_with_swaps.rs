//! # Problem 0854: Smallest String With Swaps
//!
//! - **Difficulty:** Medium
//! - **Topics:** union-find over indices, grouping with `HashMap<usize, Vec<usize>>`, sorting each
//!   group, rebuilding a `String` from bytes
//! - **Source:** [LeetCode 1202. Smallest String With Swaps](https://leetcode.com/problems/smallest-string-with-swaps/)
//!
//! ## Problem
//!
//! You are given a string `s` and a list of index pairs; `pairs[i] = [a, b]` means you may swap
//! the characters at positions `a` and `b`. You may use each pair any number of times, in any
//! order. Return the lexicographically smallest string you can obtain.
//!
//! Implement `Solution::smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "dcab", pairs = [[0, 3], [1, 2]]
//! Output: "bacd"
//! Explanation: swap s[0] and s[3] -> "bcad"; swap s[1] and s[2] -> "bacd".
//!
//! Input:  s = "dcab", pairs = [[0, 3], [1, 2], [0, 2]]
//! Output: "abcd"
//!
//! Input:  s = "cba", pairs = [[0, 1], [1, 2]]
//! Output: "abc"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 10^5`; `s` contains only lowercase English letters
//! - `0 <= pairs.len() <= 10^5`; `0 <= a, b < s.len()`
//!
//! ## Hints
//!
//! 1. Swaps chain together: if you can swap `a-b` and `b-c`, you can arrange the characters at
//!    `a`, `b`, `c` in **any** order. Positions connected through pairs form groups.
//! 2. Within a group, the best arrangement puts the group's characters in sorted order across the
//!    group's positions, also sorted.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** union the two indices of every pair. Group the indices by their root (indices come out
//! in increasing order because we scan `0..n`). For each group, collect its bytes, sort them, and
//! write them back to the group's positions in order.
//!
//! **Complexity:** `O(n log n + p * alpha(n))` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - `s.into_bytes()` takes the `String`'s buffer as a `Vec<u8>` without copying, and
//!   `String::from_utf8(bytes)` turns it back; the bytes are still ASCII letters, so the conversion
//!   cannot fail (`unwrap_or_default` keeps it panic-free).
//! - `HashMap::entry(root).or_default().push(i)` groups indices by root.
//! - `iter().zip(...)` writes the sorted characters into the sorted positions.
// @solution-end

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        // @solution-start
        fn find(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        let mut bytes = s.into_bytes();
        let n = bytes.len();
        let mut parent: Vec<usize> = (0..n).collect();
        for p in &pairs {
            let (ra, rb) = (
                find(&mut parent, p[0] as usize),
                find(&mut parent, p[1] as usize),
            );
            parent[ra] = rb;
        }
        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            let root = find(&mut parent, i);
            groups.entry(root).or_default().push(i);
        }
        for positions in groups.values() {
            let mut chars: Vec<u8> = positions.iter().map(|&i| bytes[i]).collect();
            chars.sort_unstable();
            for (&i, c) in positions.iter().zip(chars) {
                bytes[i] = c;
            }
        }
        String::from_utf8(bytes).unwrap_or_default()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str, pairs: &[[i32; 2]]) -> String {
        Solution::smallest_string_with_swaps(
            s.to_string(),
            pairs.iter().map(|p| p.to_vec()).collect(),
        )
    }

    #[test]
    fn example_two_groups() {
        assert_eq!(run("dcab", &[[0, 3], [1, 2]]), "bacd");
    }

    #[test]
    fn example_one_group() {
        assert_eq!(run("dcab", &[[0, 3], [1, 2], [0, 2]]), "abcd");
    }

    #[test]
    fn example_chain() {
        assert_eq!(run("cba", &[[0, 1], [1, 2]]), "abc");
    }

    #[test]
    fn no_pairs_and_repeated_pairs() {
        assert_eq!(run("zyx", &[]), "zyx");
        assert_eq!(run("ba", &[[0, 1], [1, 0], [0, 1]]), "ab");
    }

    #[test]
    fn long_string_with_even_and_odd_groups() {
        // Pairs (i, i + 2) connect all even positions and all odd positions separately.
        let s: String = (0..100_000)
            .map(|i| (b'z' - (i % 26) as u8) as char)
            .collect();
        let pairs: Vec<[i32; 2]> = (0..99_998).map(|i| [i, i + 2]).collect();
        let got = run(&s, &pairs);
        let bytes = got.as_bytes();
        assert!(bytes.iter().step_by(2).is_sorted());
        assert!(bytes.iter().skip(1).step_by(2).is_sorted());
        let mut a: Vec<u8> = s.into_bytes();
        let mut b = got.into_bytes();
        a.sort_unstable();
        b.sort_unstable();
        assert_eq!(a, b, "the result must be a rearrangement of the input");
    }
}
