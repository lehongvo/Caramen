# Problem 0394: Seat Reservation Manager

- **Difficulty:** Medium
- **Topics:** `BinaryHeap<Reverse<i32>>` as a min-heap, collecting a range into a heap, design struct
- **Source:** [LeetCode 1845. Seat Reservation Manager](https://leetcode.com/problems/seat-reservation-manager/)

## Problem

A theatre has `n` seats numbered `1` to `n`, all free at the start. Implement the `SeatManager`
struct:

- `SeatManager::new(n: i32) -> Self`: `n` free seats;
- `reserve(&mut self) -> i32`: reserve the **smallest-numbered** free seat and return its number;
- `unreserve(&mut self, seat_number: i32)`: free the seat `seat_number` again.

`reserve` is only called when a free seat exists, and `unreserve` only for a reserved seat. The
mutating methods take `&mut self` instead of the template's `&self`, as LeetCode's comment
allows.

## Examples

```text
let mut m = SeatManager::new(5);
m.reserve()     => 1
m.reserve()     => 2
m.unreserve(2);
m.reserve()     => 2
m.reserve()     => 3
m.reserve()     => 4
m.reserve()     => 5
m.unreserve(5);
```

## Constraints

- `1 <= n <= 10^5`, `1 <= seat_number <= n`
- At most `10^5` calls in total; every call is valid.

## Hints

1. You need "remove the smallest" and "insert any" quickly: a min-heap.
2. `BinaryHeap` is a max-heap; store `Reverse(seat)` to flip the order.

## Starter code

Write your code in [`src/practice/p0394_seat_reservation_manager.rs`](../src/practice/p0394_seat_reservation_manager.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0394
```

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Hands out the smallest free seat number.
pub struct SeatManager {}

impl SeatManager {
    pub fn new(n: i32) -> Self {
        todo!("Write your solution here")
    }

    pub fn reserve(&mut self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn unreserve(&mut self, seat_number: i32) {
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
        let mut m = SeatManager::new(5);
        assert_eq!(m.reserve(), 1);
        assert_eq!(m.reserve(), 2);
        m.unreserve(2);
        assert_eq!(m.reserve(), 2);
        assert_eq!(m.reserve(), 3);
        assert_eq!(m.reserve(), 4);
        assert_eq!(m.reserve(), 5);
        m.unreserve(5);
        assert_eq!(m.reserve(), 5);
    }

    #[test]
    fn single_seat() {
        let mut m = SeatManager::new(1);
        assert_eq!(m.reserve(), 1);
        m.unreserve(1);
        assert_eq!(m.reserve(), 1);
    }

    #[test]
    fn unreserved_seats_come_back_smallest_first() {
        let mut m = SeatManager::new(10);
        for _ in 0..10 {
            m.reserve();
        }
        m.unreserve(7);
        m.unreserve(3);
        m.unreserve(9);
        assert_eq!(m.reserve(), 3);
        assert_eq!(m.reserve(), 7);
        assert_eq!(m.reserve(), 9);
    }

    #[test]
    fn large_theatre() {
        let mut m = SeatManager::new(100_000);
        for expected in 1..=50_000 {
            assert_eq!(m.reserve(), expected);
        }
        m.unreserve(12_345);
        assert_eq!(m.reserve(), 12_345);
        assert_eq!(m.reserve(), 50_001);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** keep every free seat in a min-heap. `reserve` pops the smallest; `unreserve` pushes the
seat back.

**Complexity:** `O(n)` to build the heap, `O(log n)` per call, `O(n)` space.

A lazier variant stores only a counter `next` ("every seat `>= next` is free") plus a heap of
seats freed below `next`, which makes `new` `O(1)`; see "Smallest Number in Infinite Set".

**Rust concepts:**
- `(1..=n).map(Reverse).collect::<BinaryHeap<_>>()` builds the heap in `O(n)`; `Reverse` is a
  tuple struct, so its name works as a function.
- `pop()` returns `Option<Reverse<i32>>`; the pattern `Some(Reverse(seat))` unwraps both layers.
- Every call is valid by the constraints, so the `None` branch only returns a sentinel.

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Hands out the smallest free seat number.
pub struct SeatManager {
    free: BinaryHeap<Reverse<i32>>,
}

impl SeatManager {
    pub fn new(n: i32) -> Self {
        Self {
            free: (1..=n).map(Reverse).collect(),
        }
    }

    pub fn reserve(&mut self) -> i32 {
        match self.free.pop() {
            Some(Reverse(seat)) => seat,
            // Unreachable: the constraints guarantee a free seat.
            None => -1,
        }
    }

    pub fn unreserve(&mut self, seat_number: i32) {
        self.free.push(Reverse(seat_number));
    }
}
```

Full tested solution: [`src/solutions/p0394_seat_reservation_manager.rs`](../src/solutions/p0394_seat_reservation_manager.rs)

</details>
