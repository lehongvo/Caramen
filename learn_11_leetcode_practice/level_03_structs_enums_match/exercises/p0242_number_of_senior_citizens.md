# Problem 0242: Number of Senior Citizens

- **Difficulty:** Easy
- **Topics:** parsing fixed-width records into a struct, an `enum` field, string slicing with `get`, `filter_map`
- **Source:** [LeetCode 2678. Number of Senior Citizens](https://leetcode.com/problems/number-of-senior-citizens/)

## Problem

Each passenger is described by a string of exactly 15 characters:

- characters `0..10`: the phone number (10 digits);
- character `10`: the gender, `'M'`, `'F'` or `'O'`;
- characters `11..13`: the age (2 digits);
- characters `13..15`: the seat.

Return how many passengers are **strictly older than 60**.

Implement `Solution::count_seniors(details: Vec<String>) -> i32`.

For this level, parse each string into a `struct Passenger { phone, gender, age, seat }` where
`gender` is an `enum Gender { Male, Female, Other }`, then count with the struct.

## Examples

```text
Input:  details = ["7868190130M7522", "5303914400F9211", "9273338290F4010"]
Output: 2        (ages 75, 92 and 40)

Input:  details = ["1313579440F2036", "2921522980M5644"]
Output: 0        (ages 20 and 56)
```

## Constraints

- `1 <= details.len() <= 100`
- Every string has length 15, digits where digits are expected, and a gender of `'M'`, `'F'` or `'O'`.

## Hints

1. `s.get(11..13)` returns `Option<&str>`; parse it with `.parse::<u8>().ok()`.
2. `Passenger::parse(s: &str) -> Option<Passenger>` can use `?` on every step.

## Starter code

Write your code in [`src/practice/p0242_number_of_senior_citizens.rs`](../src/practice/p0242_number_of_senior_citizens.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0242
```

```rust
pub struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
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

    fn list(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_two_seniors() {
        assert_eq!(
            Solution::count_seniors(list(&[
                "7868190130M7522",
                "5303914400F9211",
                "9273338290F4010"
            ])),
            2
        );
    }

    #[test]
    fn example_no_seniors() {
        assert_eq!(
            Solution::count_seniors(list(&["1313579440F2036", "2921522980M5644"])),
            0
        );
    }

    #[test]
    fn sixty_is_not_senior() {
        assert_eq!(Solution::count_seniors(list(&["1111111111O6001"])), 0);
        assert_eq!(Solution::count_seniors(list(&["1111111111O6101"])), 1);
    }

    #[test]
    fn hundred_passengers() {
        let all: Vec<String> = (0..100).map(|i| format!("9999999999F{:02}AB", i)).collect();
        // Ages 61..=99 are seniors.
        assert_eq!(Solution::count_seniors(all), 39);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** parse every record into a `Passenger` (skipping malformed ones), then count the
passengers with `age > 60`.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- Fixed-width formats map naturally to byte ranges; `str::get(range)` returns `None` instead of
  panicking when the range is out of bounds.
- `?` inside a function returning `Option` makes the parser a straight list of steps.
- The struct borrows the phone and seat as `&'a str` slices of the input, so parsing allocates
  nothing. The lifetime `'a` ties the `Passenger` to the string it came from.
- The fields `phone`, `gender` and `seat` are never read here, which would trigger the
  `dead_code` lint. `#[allow(dead_code)]` on the struct, with a comment, documents that the
  whole record is modelled on purpose.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Gender {
    Male,
    Female,
    Other,
}

/// One passenger record, borrowing from the input string.
// The full record is modelled, but this problem only reads `age`.
#[allow(dead_code)]
#[derive(Debug)]
struct Passenger<'a> {
    phone: &'a str,
    gender: Gender,
    age: u8,
    seat: &'a str,
}

impl<'a> Passenger<'a> {
    fn parse(s: &'a str) -> Option<Passenger<'a>> {
        if s.len() != 15 {
            return None;
        }
        let phone = s.get(0..10)?;
        let gender = match s.get(10..11)? {
            "M" => Gender::Male,
            "F" => Gender::Female,
            "O" => Gender::Other,
            _ => return None,
        };
        let age = s.get(11..13)?.parse().ok()?;
        let seat = s.get(13..15)?;
        Some(Passenger {
            phone,
            gender,
            age,
            seat,
        })
    }

    fn is_senior(&self) -> bool {
        self.age > 60
    }
}

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let seniors = details
            .iter()
            .filter_map(|s| Passenger::parse(s))
            .filter(Passenger::is_senior)
            .count();
        // At most 100 passengers.
        seniors as i32
    }
}
```

Full tested solution: [`src/solutions/p0242_number_of_senior_citizens.rs`](../src/solutions/p0242_number_of_senior_citizens.rs)

</details>
