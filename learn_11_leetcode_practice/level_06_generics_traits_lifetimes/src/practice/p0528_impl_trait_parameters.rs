// ==================================================================
// PRACTICE 0528: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0528_impl_trait_parameters.md
// Run the tests:          cargo test practice::p0528
// ==================================================================

//! # Problem 0528: `impl Trait` Parameters
//!
//! - **Difficulty:** Easy
//! - **Topics:** `&impl Trait` parameters, trait bound syntax `<T: Trait>`, multiple bounds `impl A + B`, same type vs different types
//! - **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters)
//!
//! ## Problem
//!
//! Two small traits are given:
//!
//! - `Named` with `fn name(&self) -> String`
//! - `Aged` with `fn age(&self) -> u32`
//!
//! and three types: `Person { name, age }` (implements both), `Pet { name, species, age }` (implements
//! both; its name is `"{name} the {species}"`) and `Company { name }` (only `Named`).
//!
//! First implement the traits for the three types, then write four functions whose **signatures**
//! are the point of the exercise:
//!
//! - `greet(item: &impl Named) -> String`: returns `"Hello, {name}!"`.
//! - `introduce(a: &impl Named, b: &impl Named) -> String`: returns `"{a} meets {b}"`. `a` and `b` may
//!   have **different** types (a `Person` can meet a `Company`).
//! - `older<T: Named + Aged>(a: &T, b: &T) -> String`: returns the name of the older one (`a` on a
//!   tie). Both arguments must have the **same** type `T`.
//! - `describe(item: &(impl Named + Aged)) -> String`: returns `"{name} ({age})"`.
//!
//! ## Examples
//!
//! ```text
//! ada  = Person { name: "Ada", age: 36 }
//! rex  = Pet { name: "Rex", species: "dog", age: 4 }
//! acme = Company { name: "Acme" }
//! greet(&rex)                          => "Hello, Rex the dog!"
//! introduce(&ada, &acme)               => "Ada meets Acme"
//! older(&ada, &Person { name: "Bo", age: 40 }) => "Bo"
//! describe(&rex)                       => "Rex the dog (4)"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `&impl Named` is shorthand for a hidden type parameter: `fn greet<T: Named>(item: &T)`.
//! 2. Two `impl Named` parameters are two independent hidden parameters, so their types may differ.
//!    With one explicit `T`, both arguments must be the same type.
//! 3. Several bounds in `impl` position need parentheses when behind `&`: `&(impl Named + Aged)`.

/// Something with a name.
pub trait Named {
    fn name(&self) -> String;
}

/// Something with an age in years.
pub trait Aged {
    fn age(&self) -> u32;
}

pub struct Person {
    pub name: String,
    pub age: u32,
}

pub struct Pet {
    pub name: String,
    pub species: String,
    pub age: u32,
}

pub struct Company {
    pub name: String,
}

impl Named for Person {
    fn name(&self) -> String {
        todo!("Write your solution here")
    }
}

impl Aged for Person {
    fn age(&self) -> u32 {
        todo!("Write your solution here")
    }
}

impl Named for Pet {
    fn name(&self) -> String {
        todo!("Write your solution here")
    }
}

impl Aged for Pet {
    fn age(&self) -> u32 {
        todo!("Write your solution here")
    }
}

impl Named for Company {
    fn name(&self) -> String {
        todo!("Write your solution here")
    }
}

/// `"Hello, {name}!"`
pub fn greet(item: &impl Named) -> String {
    todo!("Write your solution here")
}

/// `"{a} meets {b}"`; `a` and `b` may have different types.
pub fn introduce(a: &impl Named, b: &impl Named) -> String {
    todo!("Write your solution here")
}

/// The name of the older of two values of the same type (`a` on a tie).
pub fn older<T: Named + Aged>(a: &T, b: &T) -> String {
    todo!("Write your solution here")
}

/// `"{name} ({age})"`
pub fn describe(item: &(impl Named + Aged)) -> String {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ada() -> Person {
        Person {
            name: "Ada".to_string(),
            age: 36,
        }
    }

    fn rex() -> Pet {
        Pet {
            name: "Rex".to_string(),
            species: "dog".to_string(),
            age: 4,
        }
    }

    fn acme() -> Company {
        Company {
            name: "Acme".to_string(),
        }
    }

    #[test]
    fn greet_any_named_type() {
        assert_eq!(greet(&ada()), "Hello, Ada!");
        assert_eq!(greet(&rex()), "Hello, Rex the dog!");
        assert_eq!(greet(&acme()), "Hello, Acme!");
    }

    #[test]
    fn introduce_mixes_types() {
        assert_eq!(introduce(&ada(), &acme()), "Ada meets Acme");
        assert_eq!(introduce(&rex(), &ada()), "Rex the dog meets Ada");
    }

    #[test]
    fn older_needs_the_same_type() {
        let bo = Person {
            name: "Bo".to_string(),
            age: 40,
        };
        assert_eq!(older(&ada(), &bo), "Bo");
        assert_eq!(older(&bo, &ada()), "Bo");
        let twin = Person {
            name: "Twin".to_string(),
            age: 36,
        };
        assert_eq!(older(&ada(), &twin), "Ada");
        let kit = Pet {
            name: "Kit".to_string(),
            species: "cat".to_string(),
            age: 9,
        };
        assert_eq!(older(&rex(), &kit), "Kit the cat");
    }

    #[test]
    fn describe_needs_both_traits() {
        assert_eq!(describe(&rex()), "Rex the dog (4)");
        assert_eq!(describe(&ada()), "Ada (36)");
    }
}
