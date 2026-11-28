// ==================================================================
// PRACTICE 0312: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0312_stable_sort_by_department.md
// Run the tests:          cargo test practice::p0312
// ==================================================================

//! # Problem 0312: Stable Sort by Department
//!
//! - **Difficulty:** Easy
//! - **Topics:** stable `sort_by` vs `sort_unstable_by`, `String::cmp`, sorting structs
//! - **Source:** Original · [`slice::sort_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by) · [`slice::sort_unstable_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by)
//!
//! ## Problem
//!
//! An HR tool stores employees in this struct:
//!
//! ```text
//! pub struct Employee { pub name: String, pub department: String, pub salary: u32 }
//! ```
//!
//! A sort is **stable** when elements that compare equal keep their original relative order.
//! `sort`, `sort_by` and `sort_by_key` are stable; the `sort_unstable*` variants are not (they may
//! reorder equal elements, in exchange for being faster and allocation-free).
//!
//! Implement two functions that sort a slice in place:
//!
//! 1. `group_by_department(staff: &mut [Employee])` orders employees by department name
//!    (alphabetically). Employees of the **same** department must stay in the order they had
//!    before the call.
//! 2. `rank_by_salary(staff: &mut [Employee])` orders employees by salary, highest first; equal
//!    salaries are ordered by name, alphabetically. Names are unique, so this order is completely
//!    determined and an unstable sort is allowed.
//!
//! ## Examples
//!
//! ```text
//! staff (name, department, salary):
//!   (Ann, Sales, 50), (Bob, Dev, 70), (Cid, Sales, 70), (Dan, Dev, 60)
//!
//! group_by_department => (Bob, Dev), (Dan, Dev), (Ann, Sales), (Cid, Sales)
//!     (Bob stays before Dan and Ann before Cid, as in the input)
//!
//! rank_by_salary => (Bob, 70), (Cid, 70), (Dan, 60), (Ann, 50)
//!     (Bob and Cid both earn 70: alphabetical order decides)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= staff.len() <= 10^4`
//! - Names are unique; departments and salaries may repeat.
//!
//! ## Hints
//!
//! 1. `sort_by(|a, b| ...)` takes a closure returning a `std::cmp::Ordering`; `a.department.cmp(&b.department)` is one.
//! 2. `sort_by_key(|e| &e.department)` does **not** compile: the key may not borrow from the
//!    element. Use `sort_by` instead (or clone the key, which allocates).
//! 3. For "highest first", compare `b` with `a` instead of `a` with `b`.

/// One employee record.
#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    pub name: String,
    pub department: String,
    pub salary: u32,
}

/// Sorts by department; employees of the same department keep their relative order.
pub fn group_by_department(staff: &mut [Employee]) {
    todo!("Write your solution here")
}

/// Sorts by salary (highest first), then by name.
pub fn rank_by_salary(staff: &mut [Employee]) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn emp(name: &str, department: &str, salary: u32) -> Employee {
        Employee {
            name: name.to_string(),
            department: department.to_string(),
            salary,
        }
    }

    fn names(staff: &[Employee]) -> Vec<&str> {
        staff.iter().map(|e| e.name.as_str()).collect()
    }

    fn example() -> Vec<Employee> {
        vec![
            emp("Ann", "Sales", 50),
            emp("Bob", "Dev", 70),
            emp("Cid", "Sales", 70),
            emp("Dan", "Dev", 60),
        ]
    }

    #[test]
    fn group_by_department_example() {
        let mut staff = example();
        group_by_department(&mut staff);
        assert_eq!(names(&staff), vec!["Bob", "Dan", "Ann", "Cid"]);
    }

    #[test]
    fn rank_by_salary_example() {
        let mut staff = example();
        rank_by_salary(&mut staff);
        assert_eq!(names(&staff), vec!["Bob", "Cid", "Dan", "Ann"]);
    }

    #[test]
    fn empty_and_single() {
        let mut none: Vec<Employee> = vec![];
        group_by_department(&mut none);
        rank_by_salary(&mut none);
        assert!(none.is_empty());
        let mut one = vec![emp("Zoe", "Ops", 1)];
        group_by_department(&mut one);
        assert_eq!(names(&one), vec!["Zoe"]);
    }

    #[test]
    fn stability_on_many_equal_departments() {
        // 1000 employees in 3 departments; the name encodes the original position.
        let depts = ["C", "A", "B"];
        let mut staff: Vec<Employee> = (0..1000)
            .map(|i| emp(&format!("{i:04}"), depts[i % 3], 10))
            .collect();
        group_by_department(&mut staff);
        assert!(staff.windows(2).all(|w| w[0].department <= w[1].department));
        // Inside each department, the original order (increasing names) is kept.
        for d in depts {
            let in_dept: Vec<&str> = staff
                .iter()
                .filter(|e| e.department == d)
                .map(|e| e.name.as_str())
                .collect();
            assert!(in_dept.windows(2).all(|w| w[0] < w[1]));
        }
    }
}
