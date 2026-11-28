# Problem 0387: My Calendar I

- **Difficulty:** Medium
- **Topics:** `BTreeMap<i32, i32>` of intervals, `range(..end).next_back()` to find the neighbour, half-open intervals
- **Source:** [LeetCode 729. My Calendar I](https://leetcode.com/problems/my-calendar-i/)

## Problem

Implement a calendar that accepts an event only if it does not overlap any event already
booked. An event is a **half-open** interval `[start, end)`: it includes `start` and excludes
`end`, so `[10, 20)` and `[20, 30)` do not overlap.

Implement the `MyCalendar` struct:

- `MyCalendar::new() -> Self`: an empty calendar;
- `book(&mut self, start_time: i32, end_time: i32) -> bool`: if `[start_time, end_time)` does
  not overlap any booked event, book it and return `true`; otherwise return `false` and book
  nothing.

`book` takes `&mut self` instead of the template's `&self`, as LeetCode's comment allows.

## Examples

```text
let mut cal = MyCalendar::new();
cal.book(10, 20) => true
cal.book(15, 25) => false    (overlaps [10, 20) on [15, 20))
cal.book(20, 30) => true     (starts exactly when [10, 20) ends)
cal.book(5, 10)  => true
cal.book(5, 11)  => false
```

## Constraints

- `0 <= start_time < end_time <= 10^9`
- At most `1000` calls to `book`.

## Hints

1. Two half-open intervals `[s1, e1)` and `[s2, e2)` overlap exactly when `s1 < e2 && s2 < e1`.
2. Store the booked events in a `BTreeMap` from start to end; they never overlap, so they are
   sorted by start **and** by end.
3. Only one booked event can matter: the one with the largest start that is `< end_time`.

## Starter code

Write your code in [`src/practice/p0387_my_calendar_i.rs`](../src/practice/p0387_my_calendar_i.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0387
```

```rust
use std::collections::BTreeMap;

/// A calendar of non-overlapping half-open events.
pub struct MyCalendar {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl MyCalendar {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn book(&mut self, start_time: i32, end_time: i32) -> bool {
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
    fn example_sequence() {
        let mut cal = MyCalendar::new();
        assert!(cal.book(10, 20));
        assert!(!cal.book(15, 25));
        assert!(cal.book(20, 30));
        assert!(cal.book(5, 10));
        assert!(!cal.book(5, 11));
    }

    #[test]
    fn containment_both_ways() {
        let mut cal = MyCalendar::new();
        assert!(cal.book(10, 20));
        assert!(!cal.book(12, 18));
        assert!(!cal.book(0, 100));
        assert!(!cal.book(10, 20));
    }

    #[test]
    fn gap_between_bookings() {
        let mut cal = MyCalendar::new();
        assert!(cal.book(0, 5));
        assert!(cal.book(10, 15));
        assert!(cal.book(5, 10));
        assert!(!cal.book(4, 6));
        assert!(cal.book(15, 1_000_000_000));
        assert!(!cal.book(999_999_999, 1_000_000_000));
    }

    #[test]
    fn thousand_bookings() {
        let mut cal = MyCalendar::new();
        // Even slots [2i, 2i + 1) first, in reverse order, then odd slots.
        for i in (0..500).rev() {
            assert!(cal.book(2 * i, 2 * i + 1));
        }
        for i in 0..499 {
            assert!(cal.book(2 * i + 1, 2 * i + 2));
        }
        assert!(!cal.book(0, 1_000));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** keep the bookings in a `BTreeMap<start, end>`. For a new event `[s, e)`, look at the
booking with the largest start `< e` (found with `range(..e).next_back()`). If that booking
ends after `s`, the events overlap. If it ends at or before `s`, every earlier booking ends even
sooner, and every later booking starts at or after `e`, so there is no conflict.

**Complexity:** `O(log n)` per `book`, `O(n)` space.

**Rust concepts:**
- `range(..end)` yields the entries with keys `< end` in order; `next_back()` takes the last one
  without walking the others (the range is a `DoubleEndedIterator`).
- The `match` reads "no neighbour, or the neighbour ends in time". The pattern
  `Some((_, &prev_end))` destructures the `Option<(&i32, &i32)>` and copies the end out.
- The map is only modified after the immutable `range` borrow has ended.

```rust
use std::collections::BTreeMap;

/// A calendar of non-overlapping half-open events.
pub struct MyCalendar {
    bookings: BTreeMap<i32, i32>,
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl MyCalendar {
    pub fn new() -> Self {
        Self {
            bookings: BTreeMap::new(),
        }
    }

    pub fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        let free = match self.bookings.range(..end_time).next_back() {
            // The booking starting last before `end_time` must end by `start_time`.
            Some((_, &prev_end)) => prev_end <= start_time,
            None => true,
        };
        if free {
            self.bookings.insert(start_time, end_time);
        }
        free
    }
}
```

Full tested solution: [`src/solutions/p0387_my_calendar_i.rs`](../src/solutions/p0387_my_calendar_i.rs)

</details>
