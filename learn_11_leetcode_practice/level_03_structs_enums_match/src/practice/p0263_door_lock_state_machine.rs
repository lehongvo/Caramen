// ==================================================================
// PRACTICE 0263: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0263_door_lock_state_machine.md
// Run the tests:          cargo test practice::p0263
// ==================================================================

//! # Problem 0263: Door Lock State Machine
//!
//! - **Difficulty:** Medium
//! - **Topics:** state machine with `match (state, event)`, enum states carrying data, guards in transitions, `Copy` enums
//! - **Source:** Original · [The Rust Book 6.2](https://doc.rust-lang.org/book/ch06-02-match.html) · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)
//!
//! ## Problem
//!
//! A smart door has a secret numeric `code` and four states:
//!
//! ```text
//! enum DoorState { Opened, Closed, Locked { failed: u8 }, Alarm }
//! enum Event     { Open, Close, Lock, Unlock(u32), Reset(u32) }
//! ```
//!
//! Implement `next_state(state: DoorState, event: Event, code: u32) -> DoorState` with these
//! transitions (any pair not listed leaves the state **unchanged**):
//!
//! | From | Event | To |
//! | --- | --- | --- |
//! | `Opened` | `Close` | `Closed` |
//! | `Closed` | `Open` | `Opened` |
//! | `Closed` | `Lock` | `Locked { failed: 0 }` |
//! | `Locked { .. }` | `Unlock(c)` with `c == code` | `Closed` |
//! | `Locked { failed }` | `Unlock(c)` with a wrong `c`, if this is the 3rd wrong try (`failed == 2`) | `Alarm` |
//! | `Locked { failed }` | `Unlock(c)` with a wrong `c`, otherwise | `Locked { failed: failed + 1 }` |
//! | `Alarm` | `Reset(c)` with `c == code` | `Locked { failed: 0 }` |
//!
//! Then write `run(code: u32, events: &[Event]) -> (DoorState, usize)`: start `Closed`, apply all
//! events, and return the final state and the number of events that actually **changed** the state.
//!
//! ## Examples
//!
//! ```text
//! next_state(Closed, Lock, 1234)                        => Locked { failed: 0 }
//! next_state(Locked { failed: 0 }, Unlock(1234), 1234)  => Closed
//! next_state(Locked { failed: 1 }, Unlock(1), 1234)     => Locked { failed: 2 }
//! next_state(Locked { failed: 2 }, Unlock(1), 1234)     => Alarm
//! next_state(Alarm, Unlock(1234), 1234)                 => Alarm   (only Reset leaves Alarm)
//! next_state(Opened, Lock, 1234)                        => Opened  (cannot lock an open door)
//! run(7, &[Open, Close, Lock, Unlock(1), Unlock(7)])    => (Closed, 5)
//! run(7, &[Lock, Unlock(1), Unlock(2), Unlock(3), Open, Reset(7)]) => (Locked { failed: 0 }, 5)
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` events.
//!
//! ## Hints
//!
//! 1. `match (state, event)` with arms such as `(DoorState::Closed, Event::Lock) => ...`.
//! 2. Guards express the code checks: `(DoorState::Locked { .. }, Event::Unlock(c)) if c == code`.
//! 3. The arm for the 3rd wrong try must come before the general "wrong code" arm.

/// The state of the door.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorState {
    Opened,
    Closed,
    Locked { failed: u8 },
    Alarm,
}

/// Something that happens to the door.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Open,
    Close,
    Lock,
    Unlock(u32),
    Reset(u32),
}

/// The state after `event`, for a door with secret `code`.
pub fn next_state(state: DoorState, event: Event, code: u32) -> DoorState {
    todo!("Write your solution here")
}

/// Runs the events from `Closed`; returns the final state and the number of real changes.
pub fn run(code: u32, events: &[Event]) -> (DoorState, usize) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::DoorState::*;
    use super::Event::*;
    use super::*;

    #[test]
    fn basic_transitions() {
        assert_eq!(next_state(Opened, Close, 1234), Closed);
        assert_eq!(next_state(Closed, Open, 1234), Opened);
        assert_eq!(next_state(Closed, Lock, 1234), Locked { failed: 0 });
        assert_eq!(next_state(Opened, Lock, 1234), Opened);
        assert_eq!(next_state(Closed, Close, 1234), Closed);
    }

    #[test]
    fn unlocking() {
        assert_eq!(next_state(Locked { failed: 0 }, Unlock(1234), 1234), Closed);
        assert_eq!(next_state(Locked { failed: 2 }, Unlock(1234), 1234), Closed);
        assert_eq!(
            next_state(Locked { failed: 1 }, Unlock(1), 1234),
            Locked { failed: 2 }
        );
        assert_eq!(next_state(Locked { failed: 2 }, Unlock(1), 1234), Alarm);
        assert_eq!(
            next_state(Locked { failed: 0 }, Open, 1234),
            Locked { failed: 0 }
        );
    }

    #[test]
    fn alarm_needs_reset_with_code() {
        assert_eq!(next_state(Alarm, Unlock(1234), 1234), Alarm);
        assert_eq!(next_state(Alarm, Reset(1), 1234), Alarm);
        assert_eq!(next_state(Alarm, Reset(1234), 1234), Locked { failed: 0 });
        assert_eq!(next_state(Closed, Reset(1234), 1234), Closed);
    }

    #[test]
    fn run_examples() {
        assert_eq!(
            run(7, &[Open, Close, Lock, Unlock(1), Unlock(7)]),
            (Closed, 5)
        );
        assert_eq!(
            run(7, &[Lock, Unlock(1), Unlock(2), Unlock(3), Open, Reset(7)]),
            (Locked { failed: 0 }, 5)
        );
        assert_eq!(run(7, &[]), (Closed, 0));
    }

    #[test]
    fn long_run_counts_only_changes() {
        let mut events = Vec::new();
        for _ in 0..2_500 {
            events.extend([Open, Open, Close, Close]);
        }
        assert_eq!(run(0, &events), (Closed, 5_000));
    }
}
