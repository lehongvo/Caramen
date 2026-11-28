# Problem 0490: Number of Laser Beams in a Bank

- **Difficulty:** Medium
- **Topics:** `map` rows to counts, `filter` empty rows, `fold` with `(previous, total)` state
- **Source:** [LeetCode 2125. Number of Laser Beams in a Bank](https://leetcode.com/problems/number-of-laser-beams-in-a-bank/)

## Problem

The floor plan of a bank is a list of strings `bank`; `bank[i]` is row `i` and contains `'0'`
(empty cell) or `'1'` (a security device). A laser beam connects two devices when:

- they are on two different rows `r1 < r2`, and
- **every** row strictly between `r1` and `r2` has no device at all.

Each such pair of devices is one beam. Return the total number of beams.

Implement `Solution::number_of_beams(bank: Vec<String>) -> i32`.

## Examples

```text
Input:  bank = ["011001", "000000", "010100", "001000"]
Output: 8
Explanation: rows 0, 2 and 3 have 3, 2 and 1 devices; row 1 is empty.
             Beams: 3 * 2 between rows 0 and 2, plus 2 * 1 between rows 2 and 3.

Input:  bank = ["000", "111", "000"]
Output: 0
Explanation: only one row has devices.
```

## Constraints

- `1 <= bank.len(), bank[i].len() <= 500`
- `bank[i]` contains only `'0'` and `'1'`.

## Hints

1. Only the **number** of devices per row matters, and empty rows can simply be removed.
2. After that, the answer is the sum of `count[i] * count[i + 1]` over neighbouring rows. A `fold`
   that remembers the previous count computes it without building a vector.

## Starter code

Write your code in [`src/practice/p0490_number_of_laser_beams_in_a_bank.rs`](../src/practice/p0490_number_of_laser_beams_in_a_bank.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0490
```

```rust
pub struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
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

    fn run(rows: &[&str]) -> i32 {
        Solution::number_of_beams(rows.iter().map(|r| r.to_string()).collect())
    }

    #[test]
    fn example_with_empty_row() {
        assert_eq!(run(&["011001", "000000", "010100", "001000"]), 8);
    }

    #[test]
    fn example_single_device_row() {
        assert_eq!(run(&["000", "111", "000"]), 0);
    }

    #[test]
    fn several_empty_rows_between() {
        assert_eq!(run(&["11", "00", "00", "11"]), 4);
        assert_eq!(run(&["1"]), 0);
        assert_eq!(run(&["0", "0"]), 0);
    }

    #[test]
    fn largest_input() {
        let full = "1".repeat(500);
        let rows: Vec<&str> = vec![full.as_str(); 500];
        assert_eq!(run(&rows), 499 * 500 * 500);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** map each row to its number of devices, drop the zeros, and add the products of
consecutive counts with a `fold` whose state is `(previous count, total)`.

**Complexity:** `O(rows * cols)` time, `O(1)` extra space.

**Rust concepts:**
- `bytes().filter(|&b| b == b'1').count()` counts devices without allocating.
- Filtering out empty rows turns the "rows in between must be empty" rule into "neighbours in
  the filtered sequence".
- Starting the fold with `previous = 0` makes the first row contribute `0 * count = 0`.
- The largest possible total, `499 * 500 * 500`, fits in an `i32`.

```rust
pub struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let (_, total) = bank
            .iter()
            .map(|row| row.bytes().filter(|&b| b == b'1').count() as i32)
            .filter(|&devices| devices > 0)
            .fold((0, 0), |(previous, total), devices| {
                (devices, total + previous * devices)
            });
        total
    }
}
```

Full tested solution: [`src/solutions/p0490_number_of_laser_beams_in_a_bank.rs`](../src/solutions/p0490_number_of_laser_beams_in_a_bank.rs)

</details>
