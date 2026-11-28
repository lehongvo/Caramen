# Problem 0255: Latest Time by Replacing Hidden Digits

- **Difficulty:** Easy
- **Topics:** `match` on tuples of `char`s, `char` range patterns, slice patterns on bytes, greedy choice
- **Source:** [LeetCode 1736. Latest Time by Replacing Hidden Digits](https://leetcode.com/problems/latest-time-by-replacing-hidden-digits/)

## Problem

You are given a time `"hh:mm"` on a 24-hour clock where some digits are hidden and shown as
`'?'`. Valid times go from `"00:00"` to `"23:59"`. Replace every `'?'` with a digit so that the
result is a **valid** time that is as **late** as possible, and return it.

Implement `Solution::maximum_time(time: String) -> String`.

## Examples

```text
Input:  time = "2?:?0"
Output: "23:50"

Input:  time = "0?:3?"
Output: "09:39"

Input:  time = "1?:22"
Output: "19:22"

Input:  time = "?4:5?"
Output: "14:59"   (the first digit cannot be 2, because 24 is not a valid hour)
```

## Constraints

- `time` has the format `"hh:mm"`; at least one valid time can be produced.

## Hints

1. The minutes are independent: a hidden tens digit becomes `'5'`, a hidden units digit `'9'`.
2. The two hour digits depend on each other: match on the pair `(h1, h2)`.

## Starter code

Write your code in [`src/practice/p0255_latest_time_by_replacing_hidden_digits.rs`](../src/practice/p0255_latest_time_by_replacing_hidden_digits.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0255
```

```rust
pub struct Solution;

impl Solution {
    pub fn maximum_time(time: String) -> String {
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

    fn latest(t: &str) -> String {
        Solution::maximum_time(t.to_string())
    }

    #[test]
    fn example_one() {
        assert_eq!(latest("2?:?0"), "23:50");
    }

    #[test]
    fn example_two() {
        assert_eq!(latest("0?:3?"), "09:39");
    }

    #[test]
    fn example_three() {
        assert_eq!(latest("1?:22"), "19:22");
    }

    #[test]
    fn first_digit_depends_on_second() {
        assert_eq!(latest("?4:5?"), "14:59");
        assert_eq!(latest("?3:00"), "23:00");
        assert_eq!(latest("?0:00"), "20:00");
    }

    #[test]
    fn all_hidden_and_none_hidden() {
        assert_eq!(latest("??:??"), "23:59");
        assert_eq!(latest("07:41"), "07:41");
        assert_eq!(latest("1?:?9"), "19:59");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** choose each hidden digit greedily from left to right. For the hour, the tens digit can
be `'2'` only if the units digit is hidden or at most `'3'`; the units digit can be `'9'`
unless the tens digit is `'2'`, in which case it is `'3'`.

**Complexity:** `O(1)` time and space.

**Rust concepts:**
- Matching a tuple `(h1, h2)` handles the four "hidden or not" combinations of the hour in one
  `match`, with range patterns such as `'0'..='3'` for the units digit.
- `let &[h1, h2, b':', m1, m2] = time.as_bytes() else { ... };` checks the format and names all
  five bytes in one slice pattern.
- Building the answer from bytes with `String::from_utf8` would need error handling; collecting
  `char`s into a `String` is simpler.

```rust
pub struct Solution;

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let &[h1, h2, b':', m1, m2] = time.as_bytes() else {
            // The format is guaranteed by the constraints.
            return time;
        };
        let (h1, h2) = match (h1 as char, h2 as char) {
            ('?', '?') => ('2', '3'),
            ('?', d @ '0'..='3') => ('2', d),
            ('?', d) => ('1', d),
            ('2', '?') => ('2', '3'),
            (t, '?') => (t, '9'),
            (t, d) => (t, d),
        };
        let m1 = if m1 == b'?' { '5' } else { m1 as char };
        let m2 = if m2 == b'?' { '9' } else { m2 as char };
        [h1, h2, ':', m1, m2].iter().collect()
    }
}
```

Full tested solution: [`src/solutions/p0255_latest_time_by_replacing_hidden_digits.rs`](../src/solutions/p0255_latest_time_by_replacing_hidden_digits.rs)

</details>
