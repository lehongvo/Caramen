# Problem 0337: Unique Email Addresses

- **Difficulty:** Easy
- **Topics:** `HashSet<String>`, `split_once`, normalising strings before inserting them
- **Source:** [LeetCode 929. Unique Email Addresses](https://leetcode.com/problems/unique-email-addresses/)

## Problem

Every email address has a **local name** and a **domain name** separated by a single `'@'`.
A mail server applies two rules to the local name only (never to the domain):

- every `'.'` in the local name is ignored: `"a.l.ice@x.com"` reaches `"alice@x.com"`;
- a `'+'` in the local name and everything after it (up to the `'@'`) is ignored:
  `"alice+news@x.com"` reaches `"alice@x.com"`.

Both rules can apply to the same address. Given a list of addresses, return how many
**different** addresses actually receive mail.

Implement `Solution::num_unique_emails(emails: Vec<String>) -> i32`.

## Examples

```text
Input:  emails = ["test.email+alex@leetcode.com",
                  "test.e.mail+bob.cathy@leetcode.com",
                  "testemail+david@lee.tcode.com"]
Output: 2   ("testemail@leetcode.com" and "testemail@lee.tcode.com")

Input:  emails = ["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"]
Output: 3
```

## Constraints

- `1 <= emails.len() <= 100`, `1 <= emails[i].len() <= 100`
- Addresses contain lowercase letters, `'+'`, `'.'` and exactly one `'@'`.
- Local and domain names are non-empty; local names do not start with `'+'`.

## Hints

1. Normalise each address to the form the server delivers to, then count distinct values with
   a `HashSet<String>`.
2. `s.split_once('@')` returns `Some((local, domain))`.

## Starter code

Write your code in [`src/practice/p0337_unique_email_addresses.rs`](../src/practice/p0337_unique_email_addresses.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0337
```

```rust
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
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

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_one() {
        let emails = strings(&[
            "test.email+alex@leetcode.com",
            "test.e.mail+bob.cathy@leetcode.com",
            "testemail+david@lee.tcode.com",
        ]);
        assert_eq!(Solution::num_unique_emails(emails), 2);
    }

    #[test]
    fn example_two() {
        let emails = strings(&["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"]);
        assert_eq!(Solution::num_unique_emails(emails), 3);
    }

    #[test]
    fn rules_apply_to_the_local_name_only() {
        let emails = strings(&["a@b.c", "a@bc", "a+x@b.c", "a.@b.c"]);
        assert_eq!(Solution::num_unique_emails(emails), 2);
    }

    #[test]
    fn many_variants_of_one_address() {
        let emails: Vec<String> = (0..100)
            .map(|i| format!("{}j.o+{i}@mail.com", ".".repeat(i % 5)))
            .collect();
        assert_eq!(Solution::num_unique_emails(emails), 1);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** rewrite every address into its canonical form: keep the local name up to the first
`'+'`, drop the dots, and append `'@'` and the unchanged domain. Insert the canonical strings
into a set and return its size.

**Complexity:** `O(total length)` time and space.

**Rust concepts:**
- `split_once('@')` splits at the first `'@'` without allocating; `split('+').next()` is the
  part before the first `'+'` (or the whole string if there is none).
- `filter(|&c| c != '.')` plus `collect::<String>()` builds the cleaned local name.
- `filter_map` skips addresses without `'@'` instead of panicking (the constraints exclude
  them anyway).

```rust
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let unique: HashSet<String> = emails
            .iter()
            .filter_map(|email| {
                let (local, domain) = email.split_once('@')?;
                let local = local.split('+').next().unwrap_or("");
                let cleaned: String = local.chars().filter(|&c| c != '.').collect();
                Some(format!("{cleaned}@{domain}"))
            })
            .collect();
        unique.len() as i32
    }
}
```

Full tested solution: [`src/solutions/p0337_unique_email_addresses.rs`](../src/solutions/p0337_unique_email_addresses.rs)

</details>
