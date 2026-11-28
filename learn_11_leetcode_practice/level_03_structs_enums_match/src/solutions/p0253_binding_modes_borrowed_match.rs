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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** pick the right way to look into the value for each job: through `&` to read, through
//! `&mut` to modify in place, with `ref` to peek inside an owned value without moving out of it,
//! and by value to take the parts out.
//!
//! **Rust concepts:**
//! - *Default binding modes*: when a non-reference pattern such as `Some(n)` or `Person { age, .. }`
//!   is matched against a reference, the reference is looked through and the bindings become
//!   references too (`&String`, `&u32`), or mutable references for `&mut`.
//! - `ref` and `ref mut` request a by-reference binding explicitly when matching an **owned**
//!   value, which avoids a move. Since default binding modes they are rarely needed, but they are
//!   still the tool for cases like `greeting_and_back`.
//! - `u32 + &u32` works because the arithmetic operators are implemented for references too, so
//!   `sum += age` compiles with `age: &u32`.
//! - Destructuring an owned struct by value (`let Person { name, nickname, .. } = p;`) moves its
//!   fields out; afterwards `p` as a whole can no longer be used.
// @solution-end

/// A person with an optional nickname.
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub nickname: Option<String>,
}

/// The nickname if present, else the name.
pub fn display_name(p: &Person) -> &str {
    // @solution-start
    match &p.nickname {
        Some(n) => n,
        None => &p.name,
    }
    // @solution-end
}

/// The sum of all ages.
pub fn total_age(people: &[Person]) -> u32 {
    // @solution-start
    let mut sum = 0;
    for Person { age, .. } in people {
        sum += age;
    }
    sum
    // @solution-end
}

/// Upper-cases every nickname in place; returns how many there were.
pub fn shout_nicknames(people: &mut [Person]) -> usize {
    // @solution-start
    let mut count = 0;
    for p in people.iter_mut() {
        if let Some(nick) = &mut p.nickname {
            nick.make_ascii_uppercase();
            count += 1;
        }
    }
    count
    // @solution-end
}

/// A greeting, plus the untouched person.
pub fn greeting_and_back(p: Person) -> (String, Person) {
    // @solution-start
    let Person { ref name, age, .. } = p;
    let greeting = format!("Hi {name} ({age})");
    (greeting, p)
    // @solution-end
}

/// The owned name and nickname.
pub fn split_person(p: Person) -> (String, Option<String>) {
    // @solution-start
    let Person { name, nickname, .. } = p;
    (name, nickname)
    // @solution-end
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
