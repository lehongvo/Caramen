# Problem 0722: Corporate Flight Bookings

- **Difficulty:** Medium
- **Topics:** difference array, prefix sums with `scan`, slice patterns `[a, b, c]`
- **Source:** [LeetCode 1109. Corporate Flight Bookings](https://leetcode.com/problems/corporate-flight-bookings/)

## Problem

There are `n` flights numbered `1` to `n`. Each booking `[first, last, seats]` reserves `seats`
seats on **every** flight from `first` to `last` (inclusive). Return a vector `answer` of length
`n` where `answer[i]` is the total number of seats reserved on flight `i + 1`.

Implement `Solution::corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32>`.

## Examples

```text
Input:  bookings = [[1, 2, 10], [2, 3, 20], [2, 5, 25]], n = 5
Output: [10, 55, 45, 25, 25]

  flight:     1   2   3   4   5
  booking 1: 10  10
  booking 2:     20  20
  booking 3:     25  25  25  25

Input:  bookings = [[1, 2, 10], [2, 2, 15]], n = 2
Output: [10, 25]
```

## Constraints

- `1 <= n <= 2 * 10^4`
- `1 <= bookings.len() <= 2 * 10^4`
- `bookings[i].len() == 3` and `1 <= first <= last <= n`
- `1 <= seats <= 10^4`

## Hints

1. Adding `seats` to every flight of every booking is `O(n)` per booking: too slow in the worst
   case.
2. Record only where each booking **starts** and where it **stops** contributing, then compute
   running totals.

## Starter code

Write your code in [`src/practice/p0722_corporate_flight_bookings.rs`](../src/practice/p0722_corporate_flight_bookings.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0722
```

```rust
pub struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
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

    #[test]
    fn example_three_bookings() {
        let bookings = vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]];
        assert_eq!(
            Solution::corp_flight_bookings(bookings, 5),
            vec![10, 55, 45, 25, 25]
        );
    }

    #[test]
    fn example_two_flights() {
        let bookings = vec![vec![1, 2, 10], vec![2, 2, 15]];
        assert_eq!(Solution::corp_flight_bookings(bookings, 2), vec![10, 25]);
    }

    #[test]
    fn single_flight() {
        assert_eq!(
            Solution::corp_flight_bookings(vec![vec![1, 1, 7]], 1),
            vec![7]
        );
    }

    #[test]
    fn flights_without_bookings_stay_zero() {
        let bookings = vec![vec![2, 3, 5]];
        assert_eq!(
            Solution::corp_flight_bookings(bookings, 5),
            vec![0, 5, 5, 0, 0]
        );
    }

    #[test]
    fn largest_totals() {
        let n = 20_000;
        let bookings = vec![vec![1, n, 10_000]; 20_000];
        let answer = Solution::corp_flight_bookings(bookings, n);
        assert_eq!(answer.len(), 20_000);
        assert!(answer.iter().all(|&seats| seats == 200_000_000));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** difference array: let `diff[i]` be "how much the seat count changes when going from
flight `i` to flight `i + 1`" (0-based). A booking adds `seats` at index `first - 1` and removes
them again at index `last`. After all bookings, the running sum (prefix sum) of `diff` is the
seat count of each flight. The extra slot `diff[n]` absorbs bookings that end at the last flight.

**Complexity:** `O(n + b)` time for `b` bookings, `O(n)` space.

**Rust concepts:**
- `if let [first, last, seats] = booking[..]` destructures a slice of exactly three elements
  (a [slice pattern](https://doc.rust-lang.org/reference/patterns.html#slice-patterns)).
- `scan` is a `map` with state: it carries the running total and yields it after each step.
- The largest total is `2 * 10^4 * 10^4 = 2 * 10^8`, which fits in `i32`.

```rust
pub struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut diff = vec![0; n + 1];
        for booking in &bookings {
            if let [first, last, seats] = booking[..] {
                diff[first as usize - 1] += seats;
                diff[last as usize] -= seats;
            }
        }
        diff.iter()
            .take(n)
            .scan(0, |running, &change| {
                *running += change;
                Some(*running)
            })
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0722_corporate_flight_bookings.rs`](../src/solutions/p0722_corporate_flight_bookings.rs)

</details>
