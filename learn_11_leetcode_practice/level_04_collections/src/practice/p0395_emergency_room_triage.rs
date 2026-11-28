// ==================================================================
// PRACTICE 0395: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0395_emergency_room_triage.md
// Run the tests:          cargo test practice::p0395
// ==================================================================

//! # Problem 0395: Emergency Room Triage
//!
//! - **Difficulty:** Medium
//! - **Topics:** `impl Ord` for a struct stored in a `BinaryHeap`, keeping `Ord` consistent with `Eq`, `peek`, `clone` + `into_sorted_vec`
//! - **Source:** Original · [`BinaryHeap`](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html) · [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
//!
//! ## Problem
//!
//! An emergency room treats patients by urgency. A patient is
//!
//! ```text
//! pub struct Patient { pub name: String, pub severity: u8, pub arrival: u32 }
//! ```
//!
//! and patient `a` must be treated **before** patient `b` when:
//!
//! 1. `a.severity > b.severity` (higher severity first), or
//! 2. severities are equal and `a.arrival < b.arrival` (earlier arrival first), or
//! 3. both are equal and `a.name < b.name` (alphabetical, so the order is total).
//!
//! Implement `Ord` (and `PartialOrd`) for `Patient` so that **"treated first" means "greater"**:
//! a `BinaryHeap<Patient>` then pops the next patient to treat. Then implement `Triage`, which
//! assigns arrival numbers `0, 1, 2, ...` in the order patients are admitted:
//!
//! - `Triage::new() -> Self` (also `Triage::default()`);
//! - `admit(&mut self, name: &str, severity: u8)`;
//! - `next_patient(&mut self) -> Option<String>`: remove and return the next patient's name;
//! - `peek_next(&self) -> Option<&str>`: the next patient's name, without removing them;
//! - `waiting(&self) -> usize`: the number of waiting patients;
//! - `queue_order(&self) -> Vec<String>`: the names of all waiting patients in treatment order,
//!   without changing the triage.
//!
//! ## Examples
//!
//! ```text
//! admit("Ann", 2); admit("Bob", 5); admit("Cy", 2); admit("Dee", 5)
//! queue_order()  => ["Bob", "Dee", "Ann", "Cy"]    (severity 5 first, then by arrival)
//! next_patient() => Some("Bob")
//! admit("Eve", 9)
//! peek_next()    => Some("Eve")
//! waiting()      => 4
//! ```
//!
//! ## Constraints
//!
//! - At most `10^5` operations.
//!
//! ## Hints
//!
//! 1. `BinaryHeap` pops the **greatest** element, so "earlier arrival" must compare as greater:
//!    compare `other.arrival` with `self.arrival` (swapped).
//! 2. `#[derive(PartialEq, Eq)]` compares all fields, so `cmp` must return `Equal` only when all
//!    fields are equal: include every field in the comparison.
//! 3. A heap does not iterate in priority order. For `queue_order`, clone it and use
//!    `into_sorted_vec()` (ascending), then reverse.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A waiting patient.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Patient {
    pub name: String,
    pub severity: u8,
    pub arrival: u32,
}

impl Ord for Patient {
    /// "Greater" means "treated first".
    fn cmp(&self, other: &Self) -> Ordering {
        todo!("Write your solution here")
    }
}

impl PartialOrd for Patient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The waiting room.
#[derive(Debug, Default)]
pub struct Triage {}

impl Triage {
    /// Creates an empty waiting room.
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    /// Admits a patient; arrival numbers increase with every admission.
    pub fn admit(&mut self, name: &str, severity: u8) {
        todo!("Write your solution here")
    }

    /// Removes and returns the next patient to treat.
    pub fn next_patient(&mut self) -> Option<String> {
        todo!("Write your solution here")
    }

    /// The next patient to treat, without removing them.
    pub fn peek_next(&self) -> Option<&str> {
        todo!("Write your solution here")
    }

    /// Number of waiting patients.
    pub fn waiting(&self) -> usize {
        todo!("Write your solution here")
    }

    /// All waiting patients in treatment order.
    pub fn queue_order(&self) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patient(name: &str, severity: u8, arrival: u32) -> Patient {
        Patient {
            name: name.to_string(),
            severity,
            arrival,
        }
    }

    #[test]
    fn ordering_rules() {
        assert!(patient("A", 5, 9) > patient("B", 4, 0));
        assert!(patient("A", 5, 1) > patient("B", 5, 2));
        assert!(patient("Al", 5, 1) > patient("Bo", 5, 1));
        assert_eq!(patient("A", 1, 1).cmp(&patient("A", 1, 1)), Ordering::Equal);
    }

    #[test]
    fn example_sequence() {
        let mut t = Triage::new();
        t.admit("Ann", 2);
        t.admit("Bob", 5);
        t.admit("Cy", 2);
        t.admit("Dee", 5);
        assert_eq!(t.queue_order(), vec!["Bob", "Dee", "Ann", "Cy"]);
        assert_eq!(t.next_patient(), Some("Bob".to_string()));
        t.admit("Eve", 9);
        assert_eq!(t.peek_next(), Some("Eve"));
        assert_eq!(t.waiting(), 4);
    }

    #[test]
    fn empty_room() {
        let mut t = Triage::default();
        assert_eq!(t.next_patient(), None);
        assert_eq!(t.peek_next(), None);
        assert_eq!(t.waiting(), 0);
        assert!(t.queue_order().is_empty());
    }

    #[test]
    fn queue_order_does_not_remove_anyone() {
        let mut t = Triage::new();
        t.admit("X", 1);
        t.admit("Y", 1);
        assert_eq!(t.queue_order(), vec!["X", "Y"]);
        assert_eq!(t.waiting(), 2);
        assert_eq!(t.next_patient(), Some("X".to_string()));
    }

    #[test]
    fn heap_of_patients_matches_sorting() {
        let mut t = Triage::new();
        let mut expected: Vec<Patient> = Vec::new();
        for i in 0..10_000u32 {
            let severity = (i * 7 % 10) as u8;
            let name = format!("p{i}");
            t.admit(&name, severity);
            expected.push(patient(&name, severity, i));
        }
        expected.sort_by(|a, b| b.cmp(a));
        let order: Vec<String> = std::iter::from_fn(|| t.next_patient()).collect();
        let expected: Vec<String> = expected.into_iter().map(|p| p.name).collect();
        assert_eq!(order, expected);
        // Severity 9 first (i = 7, 17, 27, ...), earliest arrival first.
        assert_eq!(order[0], "p7");
        assert_eq!(order[1], "p17");
    }
}
