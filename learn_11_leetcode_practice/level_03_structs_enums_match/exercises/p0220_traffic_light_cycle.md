# Problem 0220: Traffic Light Cycle

- **Difficulty:** Easy
- **Topics:** enum as a state machine, `next(self) -> Self`, methods returning data per variant, simulation loop
- **Source:** Original · [The Rust Book 6.2](https://doc.rust-lang.org/book/ch06-02-match.html)

## Problem

A traffic light cycles `Red -> Green -> Yellow -> Red -> ...`. Each phase lasts a fixed number of
seconds: `Red` 30, `Green` 25, `Yellow` 5 (a full cycle is 60 seconds).

Implement on the enum `Light`:

- `next(self) -> Light`: the phase that follows;
- `duration(self) -> u32`: the length of the phase in seconds;
- `may_drive(self) -> bool`: `true` only for `Green`.

Then write `light_at(start: Light, seconds: u64) -> Light`: the light switched to `start` at
time `0` (the beginning of that phase). Return the phase shown at time `seconds`. A phase that
lasts `d` seconds and starts at time `t` is shown during `t, t + 1, ..., t + d - 1`.

## Examples

```text
Light::Red.next()          => Green
Light::Yellow.next()       => Red
Light::Green.duration()    => 25
light_at(Red, 0)           => Red
light_at(Red, 29)          => Red
light_at(Red, 30)          => Green
light_at(Red, 57)          => Yellow     (Red 0-29, Green 30-54, Yellow 55-59)
light_at(Green, 25)        => Yellow
light_at(Red, 10^12)       => Green      (10^12 % 60 == 40)
```

## Constraints

- `0 <= seconds <= 10^12`, so stepping one second at a time is too slow.

## Hints

1. The pattern repeats every 60 seconds, whatever the starting phase: reduce `seconds % 60` first.
2. Then walk forward phase by phase: while the remaining time is at least the current phase's
   duration, subtract it and move to `next()`.

## Starter code

Write your code in [`src/practice/p0220_traffic_light_cycle.rs`](../src/practice/p0220_traffic_light_cycle.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0220
```

```rust
/// A traffic light phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Red,
    Green,
    Yellow,
}

impl Light {
    /// The phase after this one.
    pub fn next(self) -> Light {
        todo!("Write your solution here")
    }

    /// The phase length in seconds.
    pub fn duration(self) -> u32 {
        todo!("Write your solution here")
    }

    /// Whether cars may drive.
    pub fn may_drive(self) -> bool {
        todo!("Write your solution here")
    }
}

/// The phase shown `seconds` after the light switched to `start`.
pub fn light_at(start: Light, seconds: u64) -> Light {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::Light::*;
    use super::*;

    #[test]
    fn transitions_and_durations() {
        assert_eq!(Red.next(), Green);
        assert_eq!(Green.next(), Yellow);
        assert_eq!(Yellow.next(), Red);
        assert_eq!(Red.duration() + Green.duration() + Yellow.duration(), 60);
        assert_eq!(Green.duration(), 25);
    }

    #[test]
    fn only_green_may_drive() {
        assert!(Green.may_drive());
        assert!(!Red.may_drive());
        assert!(!Yellow.may_drive());
    }

    #[test]
    fn phase_boundaries_from_red() {
        assert_eq!(light_at(Red, 0), Red);
        assert_eq!(light_at(Red, 29), Red);
        assert_eq!(light_at(Red, 30), Green);
        assert_eq!(light_at(Red, 54), Green);
        assert_eq!(light_at(Red, 55), Yellow);
        assert_eq!(light_at(Red, 57), Yellow);
        assert_eq!(light_at(Red, 59), Yellow);
        assert_eq!(light_at(Red, 60), Red);
    }

    #[test]
    fn other_starting_phases() {
        assert_eq!(light_at(Green, 25), Yellow);
        assert_eq!(light_at(Green, 30), Red);
        assert_eq!(light_at(Yellow, 4), Yellow);
        assert_eq!(light_at(Yellow, 5), Red);
        assert_eq!(light_at(Yellow, 35), Green);
    }

    #[test]
    fn huge_times() {
        // 10^12 % 60 == 40, inside the green phase (30-54).
        assert_eq!(light_at(Red, 1_000_000_000_000), Green);
        // 6 * 10^11 is a multiple of 60.
        assert_eq!(light_at(Red, 600_000_000_000), Red);
        assert_eq!(light_at(Red, 600_000_000_000 + 31), Green);
        assert_eq!(light_at(Green, 600_000_000_000 - 1), Red);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a full cycle is `Red + Green + Yellow = 60` seconds, so only `seconds % 60` matters.
From `start`, skip whole phases while the remaining time covers them; at most three steps.

**Complexity:** `O(1)` time and space.

**Rust concepts:**
- An enum is a natural state machine: `next(self) -> Self` is the transition function, and the
  exhaustive `match` guarantees that every state has a successor.
- The enum derives `Copy`, so `light = light.next()` replaces the value without any borrowing.
- `u64::from(light.duration())` widens a `u32` to `u64` losslessly; prefer `From` over `as` when
  the conversion cannot lose information.

```rust
/// A traffic light phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Red,
    Green,
    Yellow,
}

impl Light {
    /// The phase after this one.
    pub fn next(self) -> Light {
        match self {
            Light::Red => Light::Green,
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Red,
        }
    }

    /// The phase length in seconds.
    pub fn duration(self) -> u32 {
        match self {
            Light::Red => 30,
            Light::Green => 25,
            Light::Yellow => 5,
        }
    }

    /// Whether cars may drive.
    pub fn may_drive(self) -> bool {
        self == Light::Green
    }
}

/// The phase shown `seconds` after the light switched to `start`.
pub fn light_at(start: Light, seconds: u64) -> Light {
    const CYCLE: u64 = 60;
    let mut remaining = seconds % CYCLE;
    let mut light = start;
    while remaining >= u64::from(light.duration()) {
        remaining -= u64::from(light.duration());
        light = light.next();
    }
    light
}
```

Full tested solution: [`src/solutions/p0220_traffic_light_cycle.rs`](../src/solutions/p0220_traffic_light_cycle.rs)

</details>
