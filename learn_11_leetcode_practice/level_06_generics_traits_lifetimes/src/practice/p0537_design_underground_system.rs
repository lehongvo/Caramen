// ==================================================================
// PRACTICE 0537: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0537_design_underground_system.md
// Run the tests:          cargo test practice::p0537
// ==================================================================

//! # Problem 0537: Design Underground System
//!
//! - **Difficulty:** Medium
//! - **Topics:** `#[derive(Hash, PartialEq, Eq)]` on a struct used as a `HashMap` key, design struct, running averages, `Default`
//! - **Source:** [LeetCode 1396. Design Underground System](https://leetcode.com/problems/design-underground-system/)
//!
//! ## Problem
//!
//! A metro system records when riders enter and leave stations and reports the **average travel
//! time** between two stations.
//!
//! - `UndergroundSystem::new() -> Self`
//! - `check_in(&mut self, id: i32, station_name: String, t: i32)`: rider `id` enters `station_name`
//!   at time `t`. A rider is checked in at most at one place at a time.
//! - `check_out(&mut self, id: i32, station_name: String, t: i32)`: rider `id`, who is currently
//!   checked in, leaves at `station_name` at time `t`. This completes one trip.
//! - `get_average_time(&self, start_station: String, end_station: String) -> f64`: the average
//!   duration of all completed trips that went **directly** from `start_station` to `end_station`.
//!   At least one such trip exists when this is called. The direction matters: `A -> B` and `B -> A`
//!   are different routes.
//!
//! Store the totals per route in a `HashMap` keyed by a struct `Route { from: String, to: String }`
//! that **derives** `Hash`, `PartialEq` and `Eq`. Also implement `Default`. This is a LeetCode
//! *design* problem, so there is no `impl Solution`: paste the structs and their impl blocks into
//! LeetCode.
//!
//! ## Examples
//!
//! ```text
//! let mut u = UndergroundSystem::new();
//! u.check_in(45, "Leyton", 3);
//! u.check_in(32, "Paradise", 8);
//! u.check_out(45, "Waterloo", 15);                  // Leyton -> Waterloo took 12
//! u.check_out(32, "Cambridge", 22);                 // Paradise -> Cambridge took 14
//! u.get_average_time("Paradise", "Cambridge") => 14.0
//! u.check_in(10, "Leyton", 24);
//! u.check_out(10, "Waterloo", 38);                  // Leyton -> Waterloo took 14
//! u.get_average_time("Leyton", "Waterloo")    => 13.0  ((12 + 14) / 2)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= id, t <= 10^6`
//! - Station names have 1 to 10 English letters.
//! - At most `2 * 10^4` calls in total.
//! - Times of one rider are strictly increasing; answers within `10^-5` are accepted.
//!
//! ## Hints
//!
//! 1. Keep the open trips in `HashMap<i32, (String, i32)>`: rider id to (station, time).
//! 2. Keep, per route, the total duration and the number of trips: `HashMap<Route, (i64, i64)>`.

use std::collections::HashMap;

/// A directed route between two stations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Route {
    pub from: String,
    pub to: String,
}

/// Tracks trips and average travel times.
pub struct UndergroundSystem {}

impl UndergroundSystem {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        todo!("Write your solution here")
    }

    pub fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        todo!("Write your solution here")
    }

    pub fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        todo!("Write your solution here")
    }
}

impl Default for UndergroundSystem {
    fn default() -> Self {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(text: &str) -> String {
        text.to_string()
    }

    #[test]
    fn example_sequence() {
        let mut u = UndergroundSystem::new();
        u.check_in(45, s("Leyton"), 3);
        u.check_in(32, s("Paradise"), 8);
        u.check_out(45, s("Waterloo"), 15);
        u.check_out(32, s("Cambridge"), 22);
        assert_eq!(u.get_average_time(s("Paradise"), s("Cambridge")), 14.0);
        u.check_in(10, s("Leyton"), 24);
        u.check_out(10, s("Waterloo"), 38);
        assert_eq!(u.get_average_time(s("Leyton"), s("Waterloo")), 13.0);
    }

    #[test]
    fn direction_matters() {
        let mut u = UndergroundSystem::default();
        u.check_in(1, s("A"), 0);
        u.check_out(1, s("B"), 10);
        u.check_in(1, s("B"), 20);
        u.check_out(1, s("A"), 50);
        assert_eq!(u.get_average_time(s("A"), s("B")), 10.0);
        assert_eq!(u.get_average_time(s("B"), s("A")), 30.0);
    }

    #[test]
    fn fractional_average() {
        let mut u = UndergroundSystem::new();
        for (id, (start, end)) in [(1, 3), (2, 7), (5, 6)].into_iter().enumerate() {
            u.check_in(id as i32, s("X"), start);
            u.check_out(id as i32, s("Y"), end);
        }
        // durations 2, 5, 1 -> 8 / 3
        assert!((u.get_average_time(s("X"), s("Y")) - 8.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn route_is_a_hash_key() {
        use std::collections::HashSet;
        let routes: HashSet<Route> = [("a", "b"), ("b", "a"), ("a", "b")]
            .iter()
            .map(|&(f, t)| Route {
                from: s(f),
                to: s(t),
            })
            .collect();
        assert_eq!(routes.len(), 2);
        let mut u = UndergroundSystem::new();
        for id in 0..10_000 {
            u.check_in(id, s("Start"), id + 1);
            u.check_out(id, s("End"), 1_000_000);
        }
        // durations are 999_999 - id, whose average over 0..10_000 is 999_999 - 4_999.5
        assert_eq!(u.get_average_time(s("Start"), s("End")), 994_999.5);
    }
}
