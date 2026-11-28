//! # Problem 0236: Parking Slots with take and replace
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Option::take`, `Option::replace`, `get_or_insert_with`, `as_mut`, `as_deref`, `Vec<Option<T>>`
//! - **Source:** Original · [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html) · [`Option::take`](https://doc.rust-lang.org/std/option/enum.Option.html#method.take)
//!
//! ## Problem
//!
//! A garage has `n` numbered slots (`0` to `n - 1`). Each slot is either empty or holds one car,
//! identified by its plate. Store the slots as `Vec<Option<String>>` and implement on `Garage`:
//!
//! - `new(n: usize) -> Garage`: all slots empty;
//! - `park(&mut self, plate: &str) -> Option<usize>`: put the car in the **lowest-numbered** empty
//!   slot and return that slot, or `None` if the garage is full;
//! - `leave(&mut self, slot: usize) -> Option<String>`: empty the slot and return the plate that was
//!   there (`None` if it was already empty or the slot does not exist);
//! - `swap_car(&mut self, slot: usize, plate: &str) -> Option<String>`: put `plate` in the slot
//!   (empty or not) and return the previous plate. If the slot does not exist, do nothing and return
//!   `None`;
//! - `occupant(&self, slot: usize) -> Option<&str>`: the plate in the slot, if any;
//! - `add_suffix(&mut self, slot: usize, suffix: &str) -> bool`: append `suffix` to the plate in the
//!   slot (a car gets a trailer, say). `false` if there is no car there;
//! - `reserve(&mut self, slot: usize) -> Option<&str>`: if the slot is empty, put the placeholder
//!   plate `"RESERVED"` in it. Return the plate now in the slot (`None` only for a missing slot).
//!
//! ## Examples
//!
//! ```text
//! let mut g = Garage::new(3);
//! g.park("AB-123")          => Some(0)
//! g.park("CD-456")          => Some(1)
//! g.leave(0)                => Some("AB-123")
//! g.park("EF-789")          => Some(0)       (lowest empty slot again)
//! g.swap_car(1, "GH-000")   => Some("CD-456")
//! g.swap_car(2, "IJ-111")   => None          (slot 2 was empty; now holds IJ-111)
//! g.park("KL-222")          => None          (full)
//! g.add_suffix(0, "+T")     => true          (slot 0 now holds "EF-789+T")
//! g.reserve(0)              => Some("EF-789+T")
//! g.reserve(9)              => None
//! ```
//!
//! ## Constraints
//!
//! - `n <= 10^4`.
//!
//! ## Hints
//!
//! 1. `slot.take()` moves the value out of an `Option` and leaves `None` behind.
//! 2. `slot.replace(value)` stores `Some(value)` and returns the old content.
//! 3. `self.slots.get_mut(i)` is `Option<&mut Option<String>>`: use `?` to handle a missing slot.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every method first looks up the slot with `get` / `get_mut` (so a bad index is just
//! `None`), then uses the `Option` method that does exactly the requested job.
//!
//! **Rust concepts:**
//! - `Option::take(&mut self) -> Option<T>` is the standard way to move a value out of a field you
//!   only have `&mut` access to: plain `let x = *slot;` would not compile for a `String`.
//! - `Option::replace(&mut self, value)` writes `Some(value)` in place and hands back the old
//!   content, so nothing is lost and nothing is cloned.
//! - `get_or_insert_with(|| ...)` fills an empty option lazily and returns a `&mut` to the content.
//! - `as_mut()` turns `&mut Option<String>` into `Option<&mut String>`, ready for `push_str`.
//! - `as_deref()` turns `&Option<String>` into `Option<&str>`.
//! - `iter().position(Option::is_none)` finds the first empty slot.
// @solution-end

/// A garage with numbered slots.
#[derive(Debug, Clone, PartialEq)]
pub struct Garage {
    // @solution-start remove
    slots: Vec<Option<String>>,
    // @solution-end
}

impl Garage {
    /// A garage with `n` empty slots.
    pub fn new(n: usize) -> Garage {
        // @solution-start
        Garage {
            slots: vec![None; n],
        }
        // @solution-end
    }

    /// Parks in the lowest empty slot.
    pub fn park(&mut self, plate: &str) -> Option<usize> {
        // @solution-start
        let index = self.slots.iter().position(Option::is_none)?;
        self.slots[index] = Some(plate.to_string());
        Some(index)
        // @solution-end
    }

    /// Empties a slot and returns its plate.
    pub fn leave(&mut self, slot: usize) -> Option<String> {
        // @solution-start
        self.slots.get_mut(slot)?.take()
        // @solution-end
    }

    /// Puts `plate` in the slot and returns the previous plate.
    pub fn swap_car(&mut self, slot: usize, plate: &str) -> Option<String> {
        // @solution-start
        self.slots.get_mut(slot)?.replace(plate.to_string())
        // @solution-end
    }

    /// The plate in the slot.
    pub fn occupant(&self, slot: usize) -> Option<&str> {
        // @solution-start
        self.slots.get(slot)?.as_deref()
        // @solution-end
    }

    /// Appends `suffix` to the plate in the slot.
    pub fn add_suffix(&mut self, slot: usize, suffix: &str) -> bool {
        // @solution-start
        match self.slots.get_mut(slot).and_then(Option::as_mut) {
            Some(plate) => {
                plate.push_str(suffix);
                true
            }
            None => false,
        }
        // @solution-end
    }

    /// Reserves an empty slot; returns the plate now in the slot.
    pub fn reserve(&mut self, slot: usize) -> Option<&str> {
        // @solution-start
        let plate = self
            .slots
            .get_mut(slot)?
            .get_or_insert_with(|| String::from("RESERVED"));
        Some(plate.as_str())
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walkthrough_example() {
        let mut g = Garage::new(3);
        assert_eq!(g.park("AB-123"), Some(0));
        assert_eq!(g.park("CD-456"), Some(1));
        assert_eq!(g.leave(0), Some("AB-123".to_string()));
        assert_eq!(g.park("EF-789"), Some(0));
        assert_eq!(g.swap_car(1, "GH-000"), Some("CD-456".to_string()));
        assert_eq!(g.swap_car(2, "IJ-111"), None);
        assert_eq!(g.occupant(2), Some("IJ-111"));
        assert_eq!(g.park("KL-222"), None);
        assert!(g.add_suffix(0, "+T"));
        assert_eq!(g.reserve(0), Some("EF-789+T"));
        assert_eq!(g.reserve(9), None);
    }

    #[test]
    fn leave_twice_and_missing_slots() {
        let mut g = Garage::new(2);
        g.park("X");
        assert_eq!(g.leave(0), Some("X".to_string()));
        assert_eq!(g.leave(0), None);
        assert_eq!(g.leave(5), None);
        assert_eq!(g.occupant(5), None);
        assert_eq!(g.swap_car(5, "Y"), None);
        assert_eq!(g.occupant(0), None);
    }

    #[test]
    fn suffix_needs_a_car() {
        let mut g = Garage::new(1);
        assert!(!g.add_suffix(0, "!"));
        assert!(!g.add_suffix(3, "!"));
        g.park("CAR");
        assert!(g.add_suffix(0, "-1"));
        assert!(g.add_suffix(0, "-2"));
        assert_eq!(g.occupant(0), Some("CAR-1-2"));
    }

    #[test]
    fn reserve_fills_only_empty_slots() {
        let mut g = Garage::new(2);
        assert_eq!(g.reserve(1), Some("RESERVED"));
        assert_eq!(g.park("A"), Some(0));
        assert_eq!(g.park("B"), None);
        assert_eq!(g.leave(1), Some("RESERVED".to_string()));
    }

    #[test]
    fn empty_garage_and_large_garage() {
        let mut empty = Garage::new(0);
        assert_eq!(empty.park("A"), None);
        let mut big = Garage::new(10_000);
        for i in 0..10_000 {
            assert_eq!(big.park(&format!("P{i}")), Some(i));
        }
        assert_eq!(big.park("late"), None);
        assert_eq!(big.leave(4_321), Some("P4321".to_string()));
        assert_eq!(big.park("again"), Some(4_321));
    }
}
