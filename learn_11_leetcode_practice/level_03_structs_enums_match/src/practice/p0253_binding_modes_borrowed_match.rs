// ==================================================================
// PRACTICE 0253: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0253_binding_modes_borrowed_match.md
// Run the tests:          cargo test practice::p0253
// ==================================================================

//! # Problem 0253: Matching Through References
//!
//! - **Difficulty:** Medium
//! - **Topics:** default binding modes, `ref` and `ref mut`, matching on `&T` and `&mut T`, destructuring in `for` loops, partial moves
//! - **Source:** Original · [The Rust Reference: Binding modes](https://doc.rust-lang.org/reference/patterns.html#binding-modes) · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)
//!
//! ## Problem
//!
//! When a pattern meets a reference, Rust decides whether the names it binds are values, shared
//! references or mutable references. These functions all work on
//!
//! ```text
//! struct Person { name: String, age: u32, nickname: Option<String> }
//! ```
//!
//! and must **not** clone any `String` unless the result type requires an owned value:
//!
//! - `display_name(p: &Person) -> &str`: the nickname if there is one, otherwise the name;
//! - `total_age(people: &[Person]) -> u32`: the sum of all ages, written as
//!   `for Person { age, .. } in people { ... }`;
//! - `shout_nicknames(people: &mut [Person]) -> usize`: turn every nickname into ASCII uppercase in
//!   place and return how many nicknames there were;
//! - `greeting_and_back(p: Person) -> (String, Person)`: build `"Hi <name> (<age>)"` and return it
//!   **together with the same `Person`**. Destructure `p` with `ref` so that it is not moved
//!   (`let Person { ref name, age, .. } = p;`);
//! - `split_person(p: Person) -> (String, Option<String>)`: take the person apart by value and
//!   return the owned name and nickname (no cloning: move them out).
//!
//! ## Examples
//!
//! ```text
//! ana = Person { name: "Ana", age: 30, nickname: Some("Annie") }
//! bo  = Person { name: "Bo",  age: 41, nickname: None }
//! display_name(&ana)              => "Annie"
//! display_name(&bo)               => "Bo"
//! total_age(&[ana, bo])           => 71
//! shout_nicknames(&mut [ana, bo]) => 1       (ana's nickname becomes "ANNIE")
//! greeting_and_back(bo)           => ("Hi Bo (41)", bo)
//! split_person(ana)               => ("Ana", Some("Annie"))
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` people; the sum of ages fits in a `u32`.
//!
//! ## Hints
//!
//! 1. `match &p.nickname { Some(n) => n, None => &p.name }`: matching a `&Option<String>` with
//!    `Some(n)` makes `n` a `&String` automatically.
//! 2. `if let Some(nick) = &mut p.nickname { nick.make_ascii_uppercase(); }` gives `nick: &mut String`.
//! 3. `ref name` borrows the field instead of moving it, so `p` stays whole and can be returned.

/// A person with an optional nickname.
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub nickname: Option<String>,
}

/// The nickname if present, else the name.
pub fn display_name(p: &Person) -> &str {
    todo!("Write your solution here")
}

/// The sum of all ages.
pub fn total_age(people: &[Person]) -> u32 {
    todo!("Write your solution here")
}

/// Upper-cases every nickname in place; returns how many there were.
pub fn shout_nicknames(people: &mut [Person]) -> usize {
    todo!("Write your solution here")
}

/// A greeting, plus the untouched person.
pub fn greeting_and_back(p: Person) -> (String, Person) {
    todo!("Write your solution here")
}

/// The owned name and nickname.
pub fn split_person(p: Person) -> (String, Option<String>) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ana() -> Person {
        Person {
            name: "Ana".to_string(),
            age: 30,
            nickname: Some("Annie".to_string()),
        }
    }

    fn bo() -> Person {
        Person {
            name: "Bo".to_string(),
            age: 41,
            nickname: None,
        }
    }

    #[test]
    fn display_names() {
        assert_eq!(display_name(&ana()), "Annie");
        assert_eq!(display_name(&bo()), "Bo");
    }

    #[test]
    fn ages_are_summed_through_references() {
        let people = vec![ana(), bo()];
        assert_eq!(total_age(&people), 71);
        assert_eq!(total_age(&[]), 0);
        // `people` is still usable: it was only borrowed.
        assert_eq!(people.len(), 2);
    }

    #[test]
    fn shouting_in_place() {
        let mut people = vec![ana(), bo(), ana()];
        assert_eq!(shout_nicknames(&mut people), 2);
        assert_eq!(people[0].nickname.as_deref(), Some("ANNIE"));
        assert_eq!(people[1].nickname, None);
        assert_eq!(people[2].name, "Ana");
    }

    #[test]
    fn ref_keeps_the_person_whole() {
        let (greeting, back) = greeting_and_back(bo());
        assert_eq!(greeting, "Hi Bo (41)");
        assert_eq!(back, bo());
    }

    #[test]
    fn splitting_moves_fields_out() {
        assert_eq!(
            split_person(ana()),
            ("Ana".to_string(), Some("Annie".to_string()))
        );
        assert_eq!(split_person(bo()), ("Bo".to_string(), None));
    }

    #[test]
    fn many_people() {
        let mut people: Vec<Person> = (0..10_000)
            .map(|i| Person {
                name: format!("p{i}"),
                age: i % 100,
                nickname: if i % 3 == 0 {
                    Some(format!("n{i}"))
                } else {
                    None
                },
            })
            .collect();
        assert_eq!(total_age(&people), 100 * (0..100).sum::<u32>());
        assert_eq!(shout_nicknames(&mut people), 3_334);
        assert_eq!(display_name(&people[9_999]), "N9999");
    }
}
