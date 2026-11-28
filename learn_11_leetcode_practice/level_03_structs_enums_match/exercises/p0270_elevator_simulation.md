# Problem 0270: Elevator Simulation

- **Difficulty:** Medium
- **Topics:** struct + direction `enum` state, tick-based simulation, `match` with guards to choose a direction, `Vec<bool>` request table
- **Source:** Original · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) · [The Rust Book 6.2](https://doc.rust-lang.org/book/ch06-02-match.html)

## Problem

Simulate one elevator in a building with floors `0..floors`. The elevator has a current floor, a
`Direction` (`Up`, `Down` or `Idle`) and a set of pending requested floors. Time advances in
**ticks**; on each tick exactly one of these happens, checked in this order:

1. no request is pending: the direction becomes `Idle` and nothing else happens;
2. the current floor is requested: the elevator stops, the request is served (removed), and the
   direction is unchanged;
3. otherwise the elevator moves one floor. It keeps going `Up` if it is moving up and some
   request is above it, and keeps going `Down` likewise. In every other case (idle, or nothing
   left ahead) it heads towards the **nearest** pending floor, choosing `Up` on a tie.

Implement `Elevator`:

- `new(floors: usize, start: usize) -> Elevator`: idle at floor `start`, no requests;
- `floor(&self) -> usize` and `direction(&self) -> Direction`;
- `request(&mut self, floor: usize) -> bool`: add a request; `false` (and nothing changes) if
  the floor does not exist;
- `tick(&mut self) -> Option<usize>`: run one tick; return `Some(floor)` when a request was served;
- `run_until_idle(&mut self) -> (Vec<usize>, usize)`: tick until no request is pending and
  return the served floors in order and the number of ticks used (not counting a final idle tick).

## Examples

```text
let mut e = Elevator::new(10, 0);
e.request(5); e.request(2); e.request(8);
e.run_until_idle()  => ([2, 5, 8], 11)   (2 moves + stop, 3 moves + stop, 3 moves + stop)

let mut e = Elevator::new(10, 5);
e.request(7); e.request(3);
e.run_until_idle()  => ([7, 3], 8)       (tie between 7 and 3: up first)

let mut e = Elevator::new(10, 4);
e.request(4);
e.tick()            => Some(4)
e.tick()            => None              (and the direction is now Idle)
```

## Constraints

- `1 <= floors <= 1000`, `start < floors`.

## Hints

1. Store requests as `Vec<bool>` indexed by floor; helpers `any_above()` and `any_below()` keep
   the direction logic readable.
2. Pick the direction with a `match self.direction { Direction::Up if self.any_above() => ..., ... }`.

## Starter code

Write your code in [`src/practice/p0270_elevator_simulation.rs`](../src/practice/p0270_elevator_simulation.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0270
```

```rust
/// Where the elevator is heading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Idle,
}

/// A single elevator.
#[derive(Debug, Clone)]
pub struct Elevator {}

impl Elevator {
    /// An idle elevator at `start` in a building with `floors` floors.
    pub fn new(floors: usize, start: usize) -> Elevator {
        todo!("Write your solution here")
    }

    /// The current floor.
    pub fn floor(&self) -> usize {
        todo!("Write your solution here")
    }

    /// The current direction.
    pub fn direction(&self) -> Direction {
        todo!("Write your solution here")
    }

    /// Adds a request; `false` for a floor that does not exist.
    pub fn request(&mut self, floor: usize) -> bool {
        todo!("Write your solution here")
    }

    /// Runs one tick; returns the floor served, if any.
    pub fn tick(&mut self) -> Option<usize> {
        todo!("Write your solution here")
    }

    /// Ticks until no request is pending; returns the served floors and the ticks used.
    pub fn run_until_idle(&mut self) -> (Vec<usize>, usize) {
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
    fn example_upward_trip() {
        let mut e = Elevator::new(10, 0);
        assert!(e.request(5));
        assert!(e.request(2));
        assert!(e.request(8));
        assert_eq!(e.run_until_idle(), (vec![2, 5, 8], 11));
        assert_eq!(e.floor(), 8);
        assert_eq!(e.direction(), Direction::Idle);
    }

    #[test]
    fn example_tie_goes_up() {
        let mut e = Elevator::new(10, 5);
        e.request(7);
        e.request(3);
        assert_eq!(e.run_until_idle(), (vec![7, 3], 8));
        assert_eq!(e.floor(), 3);
    }

    #[test]
    fn serving_the_current_floor() {
        let mut e = Elevator::new(10, 4);
        e.request(4);
        assert_eq!(e.tick(), Some(4));
        assert_eq!(e.tick(), None);
        assert_eq!(e.direction(), Direction::Idle);
        assert!(!e.request(10));
    }

    #[test]
    fn keeps_direction_while_requests_ahead() {
        let mut e = Elevator::new(10, 4);
        e.request(6);
        e.request(3);
        // Nearest is 3 (distance 1), so it goes down first, then up to 6.
        assert_eq!(e.tick(), None);
        assert_eq!(e.direction(), Direction::Down);
        assert_eq!(e.run_until_idle(), (vec![3, 6], 1 + 3 + 1));
    }

    #[test]
    fn new_request_behind_does_not_reverse() {
        let mut e = Elevator::new(20, 0);
        e.request(10);
        for _ in 0..5 {
            e.tick();
        }
        assert_eq!(e.floor(), 5);
        e.request(4);
        e.request(9);
        // Still moving up with requests above: serves 9 and 10 before going back to 4.
        assert_eq!(e.run_until_idle().0, vec![9, 10, 4]);
    }

    #[test]
    fn tall_building() {
        let mut e = Elevator::new(1000, 0);
        for f in (0..1000).step_by(100) {
            e.request(f);
        }
        let (served, ticks) = e.run_until_idle();
        assert_eq!(served, (0..1000).step_by(100).collect::<Vec<_>>());
        assert_eq!(ticks, 900 + 10);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `tick` follows the three rules literally. Choosing the direction is a `match` on the
current direction with guards ("still something ahead?"), falling back to a nearest-request
search. `run_until_idle` repeats `tick` while requests remain.

**Rust concepts:**
- The `Direction` enum makes the elevator's mode explicit; `Idle` is a real state, not a magic
  number.
- Guards on enum arms (`Direction::Up if self.any_above()`) combine the state and a condition.
- `iter().position` / `rposition` find the nearest request below or above; `Option` handles
  "none in that direction".
- Private helper methods (`any_above`, `nearest_direction`) live in the same `impl` block but
  are not `pub`.

```rust
/// Where the elevator is heading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Idle,
}

/// A single elevator.
#[derive(Debug, Clone)]
pub struct Elevator {
    floor: usize,
    direction: Direction,
    requested: Vec<bool>,
}

impl Elevator {
    /// An idle elevator at `start` in a building with `floors` floors.
    pub fn new(floors: usize, start: usize) -> Elevator {
        Elevator {
            floor: start,
            direction: Direction::Idle,
            requested: vec![false; floors],
        }
    }

    /// The current floor.
    pub fn floor(&self) -> usize {
        self.floor
    }

    /// The current direction.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Adds a request; `false` for a floor that does not exist.
    pub fn request(&mut self, floor: usize) -> bool {
        match self.requested.get_mut(floor) {
            Some(slot) => {
                *slot = true;
                true
            }
            None => false,
        }
    }

    /// Runs one tick; returns the floor served, if any.
    pub fn tick(&mut self) -> Option<usize> {
        if !self.requested.contains(&true) {
            self.direction = Direction::Idle;
            return None;
        }
        if self.requested[self.floor] {
            self.requested[self.floor] = false;
            return Some(self.floor);
        }
        self.direction = match self.direction {
            Direction::Up if self.any_above() => Direction::Up,
            Direction::Down if self.any_below() => Direction::Down,
            _ => self.nearest_direction(),
        };
        match self.direction {
            Direction::Up => self.floor += 1,
            Direction::Down => self.floor -= 1,
            Direction::Idle => {}
        }
        None
    }

    /// Ticks until no request is pending; returns the served floors and the ticks used.
    pub fn run_until_idle(&mut self) -> (Vec<usize>, usize) {
        let mut served = Vec::new();
        let mut ticks = 0;
        while self.requested.contains(&true) {
            ticks += 1;
            if let Some(floor) = self.tick() {
                served.push(floor);
            }
        }
        self.tick();
        (served, ticks)
    }

    fn any_above(&self) -> bool {
        self.requested[self.floor + 1..].contains(&true)
    }

    fn any_below(&self) -> bool {
        self.requested[..self.floor].contains(&true)
    }

    /// The direction of the nearest pending request (ties go up).
    fn nearest_direction(&self) -> Direction {
        let above = self.requested[self.floor + 1..]
            .iter()
            .position(|&r| r)
            .map(|i| i + 1);
        let below = self.requested[..self.floor]
            .iter()
            .rposition(|&r| r)
            .map(|i| self.floor - i);
        match (above, below) {
            (Some(up), Some(down)) if up <= down => Direction::Up,
            (Some(_), None) => Direction::Up,
            (_, Some(_)) => Direction::Down,
            (None, None) => Direction::Idle,
        }
    }
}
```

Full tested solution: [`src/solutions/p0270_elevator_simulation.rs`](../src/solutions/p0270_elevator_simulation.rs)

</details>
