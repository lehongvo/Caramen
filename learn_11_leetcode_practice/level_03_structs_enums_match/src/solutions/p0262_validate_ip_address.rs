//! # Problem 0262: Validate IP Address
//!
//! - **Difficulty:** Medium
//! - **Topics:** an `enum` result with a `name()` method, slice patterns on bytes, `all`, `is_ascii_hexdigit`
//! - **Source:** [LeetCode 468. Validate IP Address](https://leetcode.com/problems/validate-ip-address/)
//!
//! ## Problem
//!
//! Decide whether the string `query_ip` is a valid IPv4 address, a valid IPv6 address, or neither,
//! and return `"IPv4"`, `"IPv6"` or `"Neither"`.
//!
//! - **IPv4**: exactly four parts separated by `'.'`. Each part is a decimal number from `0` to
//!   `255` written with 1 to 3 digits and **no leading zero** (`"0"` is fine, `"01"` is not).
//! - **IPv6**: exactly eight parts separated by `':'`. Each part has 1 to 4 hexadecimal digits
//!   (`0-9`, `a-f`, `A-F`); leading zeros are allowed (`"0db8"` is fine) and empty parts are not
//!   (so the `::` shorthand is invalid here).
//!
//! No other characters (spaces, signs, ...) are allowed.
//!
//! Implement `Solution::valid_ip_address(query_ip: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  query_ip = "172.16.254.1"
//! Output: "IPv4"
//!
//! Input:  query_ip = "2001:0db8:85a3:0:0:8A2E:0370:7334"
//! Output: "IPv6"
//!
//! Input:  query_ip = "256.256.256.256"
//! Output: "Neither"
//!
//! Input:  query_ip = "192.168.01.1"
//! Output: "Neither"    (leading zero)
//! ```
//!
//! ## Constraints
//!
//! - `query_ip` has at most 50 characters: English letters, digits, `'.'` and `':'`.
//!
//! ## Hints
//!
//! 1. Write `is_ipv4_part(&str) -> bool` and `is_ipv6_part(&str) -> bool`, then check every part.
//! 2. In `is_ipv4_part`, a `match part.as_bytes()` with slice patterns handles the length and
//!    leading-zero rules: `[b'0']` is fine, `[b'0', _, ..]` is not.
//! 3. `u8::is_ascii_hexdigit` tests `0-9`, `a-f` and `A-F` in one call.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** decide which family to try from the separators (a `'.'` means IPv4, a `':'` means
//! IPv6), split, check the part count, then validate every part with a small helper.
//!
//! **Complexity:** `O(n)` time, `O(n)` space for the split parts.
//!
//! **Rust concepts:**
//! - The result is an `enum IpKind { V4, V6, Neither }`; only `name()` deals with strings.
//! - Slice patterns on bytes express the IPv4 rules declaratively: `[b'0'] => true`,
//!   `[b'0', ..] => false`, and `[_] | [_, _] | [_, _, _]` for allowed lengths.
//! - `matches!(b, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')` would test a hex digit too, but the
//!   standard `b.is_ascii_hexdigit()` says it by name (clippy's `manual_is_ascii_check` suggests it).
//! - `part.parse::<u8>()` would accept `"+1"`, so checking that every byte is a digit first matters.
// @solution-end

pub struct Solution;

// @solution-start remove
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IpKind {
    V4,
    V6,
    Neither,
}

impl IpKind {
    fn name(self) -> &'static str {
        match self {
            IpKind::V4 => "IPv4",
            IpKind::V6 => "IPv6",
            IpKind::Neither => "Neither",
        }
    }
}

fn is_ipv4_part(part: &str) -> bool {
    let bytes = part.as_bytes();
    if !bytes.iter().all(u8::is_ascii_digit) {
        return false;
    }
    match bytes {
        [_] => true,
        [b'0', ..] => false,
        [_, _] => true,
        [_, _, _] => part.parse::<u16>().is_ok_and(|v| v <= 255),
        _ => false,
    }
}

fn is_ipv6_part(part: &str) -> bool {
    (1..=4).contains(&part.len()) && part.bytes().all(|b| b.is_ascii_hexdigit())
}

fn classify(ip: &str) -> IpKind {
    if ip.contains('.') {
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() == 4 && parts.iter().all(|p| is_ipv4_part(p)) {
            return IpKind::V4;
        }
    } else if ip.contains(':') {
        let parts: Vec<&str> = ip.split(':').collect();
        if parts.len() == 8 && parts.iter().all(|p| is_ipv6_part(p)) {
            return IpKind::V6;
        }
    }
    IpKind::Neither
}
// @solution-end

impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        // @solution-start
        classify(&query_ip).name().to_string()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kind(ip: &str) -> String {
        Solution::valid_ip_address(ip.to_string())
    }

    #[test]
    fn example_ipv4() {
        assert_eq!(kind("172.16.254.1"), "IPv4");
    }

    #[test]
    fn example_ipv6() {
        assert_eq!(kind("2001:0db8:85a3:0:0:8A2E:0370:7334"), "IPv6");
    }

    #[test]
    fn example_out_of_range_and_leading_zero() {
        assert_eq!(kind("256.256.256.256"), "Neither");
        assert_eq!(kind("192.168.01.1"), "Neither");
    }

    #[test]
    fn ipv4_edge_cases() {
        assert_eq!(kind("0.0.0.0"), "IPv4");
        assert_eq!(kind("255.255.255.255"), "IPv4");
        assert_eq!(kind("1.2.3"), "Neither");
        assert_eq!(kind("1.2.3.4."), "Neither");
        assert_eq!(kind("1..3.4"), "Neither");
        assert_eq!(kind("1.2.3.+4"), "Neither");
        assert_eq!(kind("1.2.3.1000"), "Neither");
        assert_eq!(kind("1e1.4.5.6"), "Neither");
    }

    #[test]
    fn ipv6_edge_cases() {
        assert_eq!(kind("2001:db8:85a3:0:0:8A2E:0370:7334"), "IPv6");
        assert_eq!(kind("2001:0db8:85a3::8A2E:037j:7334"), "Neither");
        assert_eq!(kind("02001:0db8:85a3:0000:0000:8a2e:0370:7334"), "Neither");
        assert_eq!(kind("2001:0db8:85a3:0:0:8A2E:0370:7334:"), "Neither");
        assert_eq!(kind("1:2:3:4:5:6:7:g"), "Neither");
        assert_eq!(kind("1.2.3.4:5"), "Neither");
        assert_eq!(kind(""), "Neither");
    }
}
