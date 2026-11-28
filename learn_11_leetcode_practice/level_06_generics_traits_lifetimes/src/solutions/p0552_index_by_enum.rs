//! # Problem 0552: Indexing a Schedule by Weekday
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Index` and `IndexMut` with a non-integer index, generic struct `WeeklySchedule<T>`, `std::array::from_fn`, `impl<T: Default> Default`
//! - **Source:** Original · [std::ops::Index](https://doc.rust-lang.org/std/ops/trait.Index.html)
//!
//! ## Problem
//!
//! `schedule[Weekday::Mon]` reads better than `schedule[0]`, and it can never be out of bounds. Build a
//! generic `WeeklySchedule<T>` that stores one `T` per day in a field `days: [T; 7]` (Monday first).
//!
//! Implement:
//!
//! - `impl<T> Index<Weekday> for WeeklySchedule<T>` with `type Output = T`, and `IndexMut<Weekday>`.
//! - `impl<T: Default> Default for WeeklySchedule<T>`: every day gets `T::default()`.
//! - `WeeklySchedule::from_fn(f: impl FnMut(Weekday) -> T) -> Self`: calls `f` for each day from Monday
//!   to Sunday.
//! - `total(&self) -> u32` in a separate `impl WeeklySchedule<u32>` block (only schedules of `u32`
//!   have it).
//! - `busiest(&self) -> Weekday` in `impl<T: PartialOrd> WeeklySchedule<T>`: the day with the greatest
//!   value, the earliest day on ties.
//!
//! `Weekday` is an enum `Mon, Tue, Wed, Thu, Fri, Sat, Sun` with an associated constant
//! `Weekday::ALL: [Weekday; 7]` and a method `index(self) -> usize` (`Mon` is `0`).
//!
//! ## Examples
//!
//! ```text
//! let mut hours: WeeklySchedule<u32> = WeeklySchedule::default();
//! hours[Weekday::Mon] = 8;
//! hours[Weekday::Fri] += 6;
//! hours[Weekday::Mon]       => 8
//! hours.total()             => 14
//! hours.busiest()           => Weekday::Mon
//! WeeklySchedule::from_fn(|d| d.index() * 10)[Weekday::Sun] => 60
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `fn index(&self, day: Weekday) -> &T { &self.days[day.index()] }`; `index_mut` is the same with
//!    `&mut`.
//! 2. `std::array::from_fn(|i| ...)` builds a `[T; 7]` from a closure over the positions `0..7`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** map each weekday to its position in the array; `Index`/`IndexMut` forward to the array.
//! Construction goes through `std::array::from_fn`, which works for any `T` (no `Copy` or `Default`
//! needed for `from_fn`).
//!
//! **Rust concepts:**
//! - `Index<Idx>` is generic over the index type, so any type (here an enum) can be an index. The
//!   associated type `Output` is what `schedule[day]` evaluates to.
//! - `IndexMut` is used automatically when the indexed place is assigned to or mutated
//!   (`hours[day] += 6`).
//! - Methods in `impl WeeklySchedule<u32>` and `impl<T: PartialOrd> WeeklySchedule<T>` exist only for
//!   the matching `T`.
// @solution-end

use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl Weekday {
    pub const ALL: [Weekday; 7] = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];

    /// `Mon` is 0, ..., `Sun` is 6.
    pub fn index(self) -> usize {
        self as usize
    }
}

/// One value per weekday.
#[derive(Debug, Clone, PartialEq)]
pub struct WeeklySchedule<T> {
    days: [T; 7],
}

impl<T> WeeklySchedule<T> {
    pub fn from_fn(mut f: impl FnMut(Weekday) -> T) -> Self {
        // @solution-start
        WeeklySchedule {
            days: std::array::from_fn(|i| f(Weekday::ALL[i])),
        }
        // @solution-end
    }
}

impl<T: Default> Default for WeeklySchedule<T> {
    fn default() -> Self {
        // @solution-start
        Self::from_fn(|_| T::default())
        // @solution-end
    }
}

impl<T> Index<Weekday> for WeeklySchedule<T> {
    type Output = T;

    fn index(&self, day: Weekday) -> &T {
        // @solution-start
        &self.days[day.index()]
        // @solution-end
    }
}

impl<T> IndexMut<Weekday> for WeeklySchedule<T> {
    fn index_mut(&mut self, day: Weekday) -> &mut T {
        // @solution-start
        &mut self.days[day.index()]
        // @solution-end
    }
}

impl WeeklySchedule<u32> {
    pub fn total(&self) -> u32 {
        // @solution-start
        self.days.iter().sum()
        // @solution-end
    }
}

impl<T: PartialOrd> WeeklySchedule<T> {
    pub fn busiest(&self) -> Weekday {
        // @solution-start
        let mut best = Weekday::Mon;
        for day in Weekday::ALL {
            if self[day] > self[best] {
                best = day;
            }
        }
        best
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_hours() {
        let mut hours: WeeklySchedule<u32> = WeeklySchedule::default();
        hours[Weekday::Mon] = 8;
        hours[Weekday::Fri] += 6;
        assert_eq!(hours[Weekday::Mon], 8);
        assert_eq!(hours[Weekday::Tue], 0);
        assert_eq!(hours.total(), 14);
        assert_eq!(hours.busiest(), Weekday::Mon);
    }

    #[test]
    fn from_fn_visits_days_in_order() {
        let s = WeeklySchedule::from_fn(|d| d.index() * 10);
        assert_eq!(s[Weekday::Sun], 60);
        assert_eq!(s[Weekday::Wed], 20);
        let mut order = Vec::new();
        let _ = WeeklySchedule::from_fn(|d| order.push(d));
        assert_eq!(order, Weekday::ALL.to_vec());
    }

    #[test]
    fn works_with_non_copy_values() {
        let mut notes: WeeklySchedule<Vec<String>> = WeeklySchedule::default();
        notes[Weekday::Sat].push("hike".to_string());
        notes[Weekday::Sat].push("cook".to_string());
        notes[Weekday::Thu].push("gym".to_string());
        assert_eq!(notes[Weekday::Sat].len(), 2);
        // Vectors compare lexicographically: ["hike", "cook"] > ["gym"] > [].
        assert_eq!(notes.busiest(), Weekday::Sat);
    }

    #[test]
    fn busiest_ties_and_floats() {
        let temps = WeeklySchedule::from_fn(|d| {
            if d == Weekday::Tue || d == Weekday::Sat {
                30.5
            } else {
                20.0
            }
        });
        assert_eq!(temps.busiest(), Weekday::Tue);
        let flat: WeeklySchedule<u32> = WeeklySchedule::from_fn(|_| 5);
        assert_eq!(flat.busiest(), Weekday::Mon);
        assert_eq!(flat.total(), 35);
    }
}
