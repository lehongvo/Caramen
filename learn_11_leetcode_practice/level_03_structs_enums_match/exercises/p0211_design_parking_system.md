# Problem 0211: Design Parking System

- **Difficulty:** Easy
- **Topics:** LeetCode "design" problems, `new` constructor, `&mut self`, arrays as fields
- **Source:** [LeetCode 1603. Design Parking System](https://leetcode.com/problems/design-parking-system/)

## Problem

A parking lot has three kinds of spaces: **big**, **medium** and **small**, with a fixed number of
spaces of each kind. Cars arrive one after another. A car of type `1` needs a big space, type `2`
a medium space, and type `3` a small space; a car can only park in a space of its own kind.

Implement the `ParkingSystem` struct:

- `ParkingSystem::new(big: i32, medium: i32, small: i32) -> Self` creates a lot with that many
  free spaces of each kind.
- `add_car(&mut self, car_type: i32) -> bool` parks the car if a free space of its kind exists
  (the space is then taken for good) and returns `true`; otherwise returns `false`.

This is a LeetCode *design* problem, so there is no `impl Solution`: LeetCode creates the object
with `new` and then calls its methods. LeetCode's template writes the struct and methods without `pub` and every method with
`&self`; here they are `pub` (still accepted by LeetCode), and `add_car` takes `&mut self`
because it changes the lot, as the template's comment allows.

## Examples

```text
let mut lot = ParkingSystem::new(1, 1, 0);
lot.add_car(1) => true    (the only big space is taken)
lot.add_car(2) => true    (the only medium space is taken)
lot.add_car(3) => false   (there are no small spaces)
lot.add_car(1) => false   (no big space left)
```

## Constraints

- `0 <= big, medium, small <= 1000`
- `car_type` is `1`, `2` or `3`.
- At most `1000` calls to `add_car`.

## Hints

1. Store the free counts in a field such as `free: [i32; 3]` and index it with `car_type - 1`.
2. Check the count before decrementing it.

## Starter code

Write your code in [`src/practice/p0211_design_parking_system.rs`](../src/practice/p0211_design_parking_system.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0211
```

```rust
/// A parking lot with big, medium and small spaces.
pub struct ParkingSystem {}

impl ParkingSystem {
    /// Creates a lot with the given number of free spaces of each kind.
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        todo!("Write your solution here")
    }

    /// Parks a car of type 1 (big), 2 (medium) or 3 (small) if a space is free.
    pub fn add_car(&mut self, car_type: i32) -> bool {
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
    fn example_one_big_one_medium() {
        let mut lot = ParkingSystem::new(1, 1, 0);
        assert!(lot.add_car(1));
        assert!(lot.add_car(2));
        assert!(!lot.add_car(3));
        assert!(!lot.add_car(1));
    }

    #[test]
    fn empty_lot_rejects_everyone() {
        let mut lot = ParkingSystem::new(0, 0, 0);
        assert!(!lot.add_car(1));
        assert!(!lot.add_car(2));
        assert!(!lot.add_car(3));
    }

    #[test]
    fn kinds_are_independent() {
        let mut lot = ParkingSystem::new(0, 0, 2);
        assert!(lot.add_car(3));
        assert!(!lot.add_car(2));
        assert!(lot.add_car(3));
        assert!(!lot.add_car(3));
    }

    #[test]
    fn large_lot_fills_exactly() {
        let mut lot = ParkingSystem::new(1000, 1000, 1000);
        let parked = (0..1000).filter(|_| lot.add_car(2)).count();
        assert_eq!(parked, 1000);
        assert!(!lot.add_car(2));
        assert!(lot.add_car(1));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** keep one counter of free spaces per kind. Parking a car checks its counter; if it is
positive, decrement it and report success.

**Complexity:** `O(1)` time per call, `O(1)` space.

**Rust concepts:**
- A design problem is just a struct with private state plus an `impl` block with `new` and
  methods.
- `[i32; 3]` is a fixed-size array stored inline in the struct (no heap allocation).
- Converting the `i32` car type to an index needs a cast: `(car_type - 1) as usize`. The cast is
  safe because the constraints guarantee `1..=3`.
- `&mut self` lets `add_car` update the counters; callers need `let mut lot = ...`.

```rust
/// A parking lot with big, medium and small spaces.
pub struct ParkingSystem {
    free: [i32; 3],
}

impl ParkingSystem {
    /// Creates a lot with the given number of free spaces of each kind.
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            free: [big, medium, small],
        }
    }

    /// Parks a car of type 1 (big), 2 (medium) or 3 (small) if a space is free.
    pub fn add_car(&mut self, car_type: i32) -> bool {
        // The constraints guarantee 1 <= car_type <= 3.
        let slot = &mut self.free[(car_type - 1) as usize];
        if *slot > 0 {
            *slot -= 1;
            true
        } else {
            false
        }
    }
}
```

Full tested solution: [`src/solutions/p0211_design_parking_system.rs`](../src/solutions/p0211_design_parking_system.rs)

</details>
