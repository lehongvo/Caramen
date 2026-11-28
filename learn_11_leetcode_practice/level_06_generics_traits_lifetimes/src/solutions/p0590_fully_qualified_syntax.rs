//! # Problem 0590: Fully Qualified Syntax
//!
//! - **Difficulty:** Easy
//! - **Topics:** methods with the same name in several traits, `Trait::method(&value)`, `<Type as Trait>::function()` for associated functions without `self`
//! - **Source:** Original · [The Rust Book 20.2](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name)
//!
//! ## Problem
//!
//! When a type has an inherent method and trait methods with the **same name**, `value.method()` calls
//! the inherent one. To call the others you must say which trait you mean.
//!
//! 1. `Human` has an inherent `fly(&self) -> String` returning `"*waving arms furiously*"`, and
//!    implements `Pilot::fly` (`"This is your captain speaking."`) and `Wizard::fly` (`"Up!"`).
//!    Write `all_flights(person: &Human) -> [String; 3]` returning the inherent, the pilot and the
//!    wizard version, in that order.
//! 2. `Dog` has an inherent associated function `baby_name() -> String` returning `"Spot"` and
//!    implements `Animal::baby_name()` returning `"puppy"`. These functions have **no `self`**, so
//!    `Animal::baby_name()` alone is ambiguous: use `<Dog as Animal>::baby_name()`. Write
//!    `dog_names() -> (String, String)` returning `(inherent, trait)`.
//! 3. `Cat` implements `Animal` with `"kitten"`. Write the generic
//!    `baby_name_of<A: Animal>() -> String`, called like `baby_name_of::<Cat>()`.
//!
//! ## Examples
//!
//! ```text
//! all_flights(&Human)        => ["*waving arms furiously*", "This is your captain speaking.", "Up!"]
//! dog_names()                => ("Spot", "puppy")
//! baby_name_of::<Cat>()      => "kitten"
//! baby_name_of::<Dog>()      => "puppy"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `Pilot::fly(person)` passes `person` as `self` explicitly.
//! 2. The general form is `<Type as Trait>::function(receiver_if_method, other_args...)`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** use the inherent call for the default and the fully qualified forms for the trait
//! versions. Inside a generic function, `A::baby_name()` is unambiguous because `A` is only known to
//! implement `Animal`.
//!
//! **Rust concepts:**
//! - Method call syntax prefers inherent methods; trait methods with the same name are still reachable
//!   with `Trait::method(&value)`.
//! - Associated functions without `self` give the compiler nothing to infer `Self` from, so
//!   `<Dog as Animal>::baby_name()` names both the type and the trait.
//! - In generic code (`A: Animal`), `A::baby_name()` always means the trait's function.
// @solution-end

pub trait Pilot {
    fn fly(&self) -> String;
}

pub trait Wizard {
    fn fly(&self) -> String;
}

pub trait Animal {
    fn baby_name() -> String;
}

pub struct Human;

pub struct Dog;

pub struct Cat;

impl Human {
    pub fn fly(&self) -> String {
        // @solution-start
        "*waving arms furiously*".to_string()
        // @solution-end
    }
}

impl Pilot for Human {
    fn fly(&self) -> String {
        // @solution-start
        "This is your captain speaking.".to_string()
        // @solution-end
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        // @solution-start
        "Up!".to_string()
        // @solution-end
    }
}

impl Dog {
    pub fn baby_name() -> String {
        // @solution-start
        "Spot".to_string()
        // @solution-end
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        // @solution-start
        "puppy".to_string()
        // @solution-end
    }
}

impl Animal for Cat {
    fn baby_name() -> String {
        // @solution-start
        "kitten".to_string()
        // @solution-end
    }
}

/// The inherent, pilot and wizard versions of `fly`.
pub fn all_flights(person: &Human) -> [String; 3] {
    // @solution-start
    [person.fly(), Pilot::fly(person), Wizard::fly(person)]
    // @solution-end
}

/// `(Dog::baby_name(), <Dog as Animal>::baby_name())`
pub fn dog_names() -> (String, String) {
    // @solution-start
    (Dog::baby_name(), <Dog as Animal>::baby_name())
    // @solution-end
}

/// The baby name of any animal type.
pub fn baby_name_of<A: Animal>() -> String {
    // @solution-start
    A::baby_name()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_ways_to_fly() {
        assert_eq!(
            all_flights(&Human),
            [
                "*waving arms furiously*".to_string(),
                "This is your captain speaking.".to_string(),
                "Up!".to_string()
            ]
        );
    }

    #[test]
    fn inherent_call_wins() {
        assert_eq!(Human.fly(), "*waving arms furiously*");
        assert_eq!(<Human as Wizard>::fly(&Human), "Up!");
    }

    #[test]
    fn associated_functions_without_self() {
        assert_eq!(dog_names(), ("Spot".to_string(), "puppy".to_string()));
        assert_eq!(baby_name_of::<Cat>(), "kitten");
        assert_eq!(baby_name_of::<Dog>(), "puppy");
    }
}
