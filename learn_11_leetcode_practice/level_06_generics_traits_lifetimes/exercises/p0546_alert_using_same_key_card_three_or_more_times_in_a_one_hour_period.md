# Problem 0546: Alert Using Same Key-Card Three or More Times in a One Hour Period

- **Difficulty:** Medium
- **Topics:** `HashMap<&str, Vec<u16>>` borrowing names from the input, parsing `"HH:MM"`, sorting, `windows(3)`
- **Source:** [LeetCode 1604. Alert Using Same Key-Card Three or More Times in a One Hour Period](https://leetcode.com/problems/alert-using-same-key-card-three-or-more-times-in-a-one-hour-period/)

## Problem

Workers open doors with key-cards. Entry `i` says that the worker `key_name[i]` used their card at
time `key_time[i]` (`"HH:MM"`, 24-hour format, all on the same day). The security system raises an
**alert** for a worker who used their card **three or more times within one hour**: three uses whose
times differ by at most 60 minutes (for example `"10:00"`, `"10:40"`, `"11:00"`). The window does
not wrap around midnight (`"23:51"` and `"00:10"` are almost 24 hours apart).

Return the names of the alerted workers, sorted alphabetically, each once.

Implement `Solution::alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String>`.

## Examples

```text
names = ["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"]
times = ["10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"]
Output: ["daniel"]

names = ["alice", "alice", "alice", "bob", "bob", "bob", "bob"]
times = ["12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"]
Output: ["bob"]      (alice's times are not in order; 12:00, 12:01, 18:00 is not one hour)
```

## Constraints

- `1 <= key_name.len() == key_time.len() <= 10^5`
- Names are 1 to 10 lowercase letters; times are valid `"HH:MM"` strings.
- Every `(name, time)` pair is unique.

## Hints

1. Group the times per worker in a `HashMap<&str, Vec<u16>>` (minutes since midnight). The keys can
   borrow from `key_name`: no `String` needs to be cloned while grouping.
2. Sort each worker's times; an alert exists when some window of three consecutive times satisfies
   `t[i + 2] - t[i] <= 60`.

## Starter code

Write your code in [`src/practice/p0546_alert_using_same_key_card_three_or_more_times_in_a_one_hour_period.rs`](../src/practice/p0546_alert_using_same_key_card_three_or_more_times_in_a_one_hour_period.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0546
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
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

    fn strings(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_daniel() {
        let names = strings(&["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"]);
        let times = strings(&[
            "10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00",
        ]);
        assert_eq!(Solution::alert_names(names, times), vec!["daniel"]);
    }

    #[test]
    fn example_bob() {
        let names = strings(&["alice", "alice", "alice", "bob", "bob", "bob", "bob"]);
        let times = strings(&[
            "12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00",
        ]);
        assert_eq!(Solution::alert_names(names, times), vec!["bob"]);
    }

    #[test]
    fn no_wrap_around_midnight_and_sorted_output() {
        let names = strings(&["zed", "zed", "zed", "amy", "amy", "amy", "kim", "kim"]);
        let times = strings(&[
            "23:30", "23:59", "00:10", "08:00", "08:30", "09:00", "01:00", "01:01",
        ]);
        assert_eq!(Solution::alert_names(names, times), vec!["amy"]);
        let names = strings(&["b", "b", "b", "a", "a", "a"]);
        let times = strings(&["00:00", "00:30", "01:00", "23:00", "23:01", "23:02"]);
        assert_eq!(Solution::alert_names(names, times), vec!["a", "b"]);
    }

    #[test]
    fn many_entries() {
        let mut names = Vec::new();
        let mut times = Vec::new();
        for worker in 0..1000 {
            for k in 0..3 {
                names.push(format!("w{worker:04}"));
                // Even workers use their card every 30 minutes, odd ones every 31 minutes.
                let step = if worker % 2 == 0 { 30 } else { 31 };
                let m = k * step;
                times.push(format!("{:02}:{:02}", m / 60, m % 60));
            }
        }
        let alerted = Solution::alert_names(names, times);
        assert_eq!(alerted.len(), 500);
        assert_eq!(alerted[0], "w0000");
        assert_eq!(alerted[499], "w0998");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** convert every time to minutes, group by worker, sort each group, and check all windows of
three consecutive uses. Collect alerted names, convert them to `String` only at the end, and sort.

**Complexity:** `O(n log n)` time, `O(n)` space.

**Rust concepts:**
- `HashMap<&str, Vec<u16>>` borrows its keys from `key_name`, which must stay alive while the map
  is used; the borrow checker enforces it.
- `windows(3)` yields overlapping sub-slices `&[u16]` of length 3 without copying.
- A tiny helper `fn minutes(t: &str) -> u16` keeps the parsing in one place; the constraints
  guarantee the format, so it uses byte arithmetic on the four digits.

```rust
use std::collections::HashMap;

pub struct Solution;

/// Minutes since midnight of a valid `"HH:MM"` string.
fn minutes(t: &str) -> u16 {
    let b = t.as_bytes();
    let digit = |i: usize| u16::from(b[i] - b'0');
    (digit(0) * 10 + digit(1)) * 60 + digit(3) * 10 + digit(4)
}

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut uses: HashMap<&str, Vec<u16>> = HashMap::new();
        for (name, time) in key_name.iter().zip(&key_time) {
            uses.entry(name.as_str()).or_default().push(minutes(time));
        }
        let mut alerted: Vec<String> = uses
            .into_iter()
            .filter_map(|(name, mut times)| {
                times.sort_unstable();
                times
                    .windows(3)
                    .any(|w| w[2] - w[0] <= 60)
                    .then(|| name.to_string())
            })
            .collect();
        alerted.sort();
        alerted
    }
}
```

Full tested solution: [`src/solutions/p0546_alert_using_same_key_card_three_or_more_times_in_a_one_hour_period.rs`](../src/solutions/p0546_alert_using_same_key_card_three_or_more_times_in_a_one_hour_period.rs)

</details>
