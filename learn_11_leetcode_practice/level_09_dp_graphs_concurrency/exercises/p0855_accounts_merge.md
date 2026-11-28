# Problem 0855: Accounts Merge

- **Difficulty:** Medium
- **Topics:** union-find keyed by strings via `HashMap<&str, usize>`, grouping into
  `BTreeSet`s, building `Vec<Vec<String>>`
- **Source:** [LeetCode 721. Accounts Merge](https://leetcode.com/problems/accounts-merge/)

## Problem

Each entry of `accounts` is a list whose first element is a person's name and whose other
elements are email addresses. Two accounts belong to the same person if they share **at least
one** email (directly or through a chain of accounts). People with the same name are not
necessarily the same person; all accounts of one person do have the same name.

Merge the accounts of each person. Return one list per person: the name followed by all of that
person's emails **sorted** (byte order), without duplicates. The lists may be returned in any
order.

Implement `Solution::accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>>`.

## Examples

```text
Input:  accounts = [["John", "johnsmith@mail.com", "john_newyork@mail.com"],
                    ["John", "johnsmith@mail.com", "john00@mail.com"],
                    ["Mary", "mary@mail.com"],
                    ["John", "johnnybravo@mail.com"]]
Output: [["John", "john00@mail.com", "john_newyork@mail.com", "johnsmith@mail.com"],
         ["Mary", "mary@mail.com"],
         ["John", "johnnybravo@mail.com"]]
Explanation: the first two accounts share "johnsmith@mail.com". The last John is a different
person.
```

## Constraints

- `1 <= accounts.len() <= 1000`, `2 <= accounts[i].len() <= 10`
- Names and emails have 1 to 30 characters; names are English letters.

## Hints

1. Union **accounts**, not emails: give each account an index, and remember for each email the
   first account that contained it. Seeing the email again means "union these two accounts".
2. Afterwards, collect each account's emails into a sorted set owned by its root account.

## Starter code

Write your code in [`src/practice/p0855_accounts_merge.rs`](../src/practice/p0855_accounts_merge.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0855
```

```rust
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub struct Solution;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
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

    fn to_input(rows: &[&[&str]]) -> Vec<Vec<String>> {
        rows.iter()
            .map(|r| r.iter().map(|s| s.to_string()).collect())
            .collect()
    }

    fn run(rows: &[&[&str]]) -> Vec<Vec<String>> {
        let mut got = Solution::accounts_merge(to_input(rows));
        got.sort();
        got
    }

    #[test]
    fn example_three_people() {
        let got = run(&[
            &["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            &["John", "johnsmith@mail.com", "john00@mail.com"],
            &["Mary", "mary@mail.com"],
            &["John", "johnnybravo@mail.com"],
        ]);
        let mut want = to_input(&[
            &[
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com",
            ],
            &["Mary", "mary@mail.com"],
            &["John", "johnnybravo@mail.com"],
        ]);
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn single_account_with_duplicate_email() {
        let got = run(&[&["Ann", "b@x", "a@x", "b@x"]]);
        assert_eq!(got, to_input(&[&["Ann", "a@x", "b@x"]]));
    }

    #[test]
    fn chain_of_accounts() {
        // 1 shares with 2, 2 shares with 3: all three merge, although 1 and 3 share nothing.
        let got = run(&[
            &["Bo", "a@x", "b@x"],
            &["Bo", "c@x", "d@x"],
            &["Bo", "b@x", "c@x"],
        ]);
        assert_eq!(got, to_input(&[&["Bo", "a@x", "b@x", "c@x", "d@x"]]));
    }

    #[test]
    fn same_name_different_people() {
        let got = run(&[&["Al", "1@x"], &["Al", "2@x"]]);
        assert_eq!(got, to_input(&[&["Al", "1@x"], &["Al", "2@x"]]));
    }

    #[test]
    fn many_accounts_in_one_chain() {
        let rows: Vec<Vec<String>> = (0..1000)
            .map(|i| {
                vec![
                    "Zed".to_string(),
                    format!("{i:04}@x"),
                    format!("{:04}@x", i + 1),
                ]
            })
            .collect();
        let got = Solution::accounts_merge(rows);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].len(), 1 + 1001);
        assert_eq!(got[0][1], "0000@x");
        assert_eq!(got[0][1001], "1000@x");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** union-find over account indices. `owner: HashMap<&str, usize>` maps an email to the
first account that listed it; every later account listing it is unioned with that one. Then,
for every account, add its emails to `merged[find(i)]`, a `BTreeSet<&str>` (sorted and
deduplicated). Finally emit `name + emails` for every root.

**Complexity:** `O(E log E)` time for `E` emails in total (the sorted sets dominate), `O(E)`
space.

**Rust concepts:**
- `HashMap<&str, usize>` and `BTreeSet<&str>` borrow the emails from `accounts`; strings are only
  cloned when the final answer is built.
- `entry(email).or_insert(i)` returns the stored owner, inserting `i` if the email is new.
- `std::iter::once(name).chain(set.into_iter()).map(str::to_string).collect()` builds one output
  row.

```rust
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub struct Solution;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        fn find(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        let mut parent: Vec<usize> = (0..accounts.len()).collect();
        let mut owner: HashMap<&str, usize> = HashMap::new();
        for (i, account) in accounts.iter().enumerate() {
            for email in &account[1..] {
                let first = *owner.entry(email.as_str()).or_insert(i);
                let (ra, rb) = (find(&mut parent, first), find(&mut parent, i));
                parent[ra] = rb;
            }
        }
        let mut merged: BTreeMap<usize, BTreeSet<&str>> = BTreeMap::new();
        for (i, account) in accounts.iter().enumerate() {
            let root = find(&mut parent, i);
            merged
                .entry(root)
                .or_default()
                .extend(account[1..].iter().map(String::as_str));
        }
        merged
            .into_iter()
            .map(|(root, emails)| {
                std::iter::once(accounts[root][0].as_str())
                    .chain(emails)
                    .map(str::to_string)
                    .collect()
            })
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0855_accounts_merge.rs`](../src/solutions/p0855_accounts_merge.rs)

</details>
