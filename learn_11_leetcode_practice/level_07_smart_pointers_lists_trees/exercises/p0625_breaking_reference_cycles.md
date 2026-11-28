# Problem 0625: Breaking Reference Cycles

- **Difficulty:** Medium
- **Topics:** reference cycles and leaks, `Weak<T>` back links, `RefCell<Option<Rc<T>>>`, `Drop` as a leak detector
- **Source:** Original · [The Rust Book 15.6](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html) · [`Weak::upgrade`](https://doc.rust-lang.org/std/rc/struct.Weak.html#method.upgrade)

## Problem

A person lives in an apartment, and the apartment knows its tenant. If `Person` held an
`Rc<Apartment>` **and** `Apartment` held an `Rc<Person>`, the two would keep each other alive
forever: when the program drops its own handles, both strong counts stay at `1` and neither
destructor ever runs. That is a memory leak, and it is possible in safe Rust.

The types below make the person the owner (`Rc`) and the apartment the observer (`Weak`). Their
`Drop` implementations are given: they write to a shared log, so the tests can see what was
freed.

```text
pub struct Person {
    pub name: String,
    pub home: RefCell<Option<Rc<Apartment>>>,
    log: Log,
}

pub struct Apartment {
    pub number: u32,
    pub tenant: RefCell<Weak<Person>>,
    log: Log,
}
```

Implement:

- `move_in(person: &Rc<Person>, apartment: &Rc<Apartment>)`: link both ways (strong from the
  person, weak from the apartment). If the person already had a home, that old apartment must
  no longer name them as its tenant;
- `tenant_name(apartment: &Apartment) -> Option<String>`: the name of the current tenant, if
  that person is still alive;
- `move_out(person: &Person)`: undo both links (does nothing if the person has no home).

## Examples

```text
let ana = Person::new("ana", &log);
let flat = Apartment::new(12, &log);
move_in(&ana, &flat);
tenant_name(&flat)          -> Some("ana")
Rc::strong_count(&flat) == 2, Rc::strong_count(&ana) == 1, Rc::weak_count(&ana) == 1
drop(flat);                 // still alive: ana owns it
drop(ana);                  // log: ["person ana dropped", "apartment 12 dropped"]
```

## Constraints

- No special constraints.

## Hints

1. `*apartment.tenant.borrow_mut() = Rc::downgrade(person);`
2. `person.home.borrow_mut().replace(Rc::clone(apartment))` stores the new home and returns the
   old one (if any), so you can reset the old apartment's tenant with `Weak::new()`.
3. `tenant_name` upgrades the `Weak` and reads the name from the resulting `Rc<Person>`.

## Starter code

Write your code in [`src/practice/p0625_breaking_reference_cycles.rs`](../src/practice/p0625_breaking_reference_cycles.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0625
```

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Person {
    pub name: String,
    pub home: RefCell<Option<Rc<Apartment>>>,
    log: Log,
}

pub struct Apartment {
    pub number: u32,
    pub tenant: RefCell<Weak<Person>>,
    log: Log,
}

impl Person {
    /// Given: a person without a home.
    pub fn new(name: &str, log: &Log) -> Rc<Person> {
        Rc::new(Person {
            name: name.to_string(),
            home: RefCell::new(None),
            log: Rc::clone(log),
        })
    }
}

impl Apartment {
    /// Given: an empty apartment.
    pub fn new(number: u32, log: &Log) -> Rc<Apartment> {
        Rc::new(Apartment {
            number,
            tenant: RefCell::new(Weak::new()),
            log: Rc::clone(log),
        })
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        self.log
            .borrow_mut()
            .push(format!("person {} dropped", self.name));
    }
}

impl Drop for Apartment {
    fn drop(&mut self) {
        self.log
            .borrow_mut()
            .push(format!("apartment {} dropped", self.number));
    }
}

pub fn move_in(person: &Rc<Person>, apartment: &Rc<Apartment>) {
    todo!("Write your solution here")
}

pub fn tenant_name(apartment: &Apartment) -> Option<String> {
    todo!("Write your solution here")
}

pub fn move_out(person: &Person) {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_values_are_freed() {
        let log = Log::default();
        let ana = Person::new("ana", &log);
        let flat = Apartment::new(12, &log);
        move_in(&ana, &flat);
        assert_eq!(tenant_name(&flat).as_deref(), Some("ana"));
        assert_eq!(Rc::strong_count(&flat), 2);
        assert_eq!(Rc::strong_count(&ana), 1);
        assert_eq!(Rc::weak_count(&ana), 1);
        drop(flat);
        assert!(log.borrow().is_empty());
        drop(ana);
        assert_eq!(
            *log.borrow(),
            ["person ana dropped", "apartment 12 dropped"]
        );
    }

    #[test]
    fn apartment_forgets_a_dropped_tenant() {
        let log = Log::default();
        let flat = Apartment::new(7, &log);
        let bo = Person::new("bo", &log);
        move_in(&bo, &flat);
        drop(bo);
        assert_eq!(tenant_name(&flat), None);
        assert_eq!(Rc::strong_count(&flat), 1);
        assert_eq!(*log.borrow(), ["person bo dropped"]);
    }

    #[test]
    fn moving_to_a_new_home_clears_the_old_tenant() {
        let log = Log::default();
        let cy = Person::new("cy", &log);
        let old = Apartment::new(1, &log);
        let new = Apartment::new(2, &log);
        move_in(&cy, &old);
        move_in(&cy, &new);
        assert_eq!(tenant_name(&old), None);
        assert_eq!(tenant_name(&new).as_deref(), Some("cy"));
        assert_eq!(Rc::strong_count(&old), 1);
        move_in(&cy, &new);
        assert_eq!(tenant_name(&new).as_deref(), Some("cy"));
    }

    #[test]
    fn move_out_undoes_both_links() {
        let log = Log::default();
        let di = Person::new("di", &log);
        let flat = Apartment::new(3, &log);
        move_out(&di);
        move_in(&di, &flat);
        move_out(&di);
        assert_eq!(tenant_name(&flat), None);
        assert!(di.home.borrow().is_none());
        assert_eq!(Rc::strong_count(&flat), 1);
        assert_eq!(Rc::weak_count(&di), 0);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** ownership goes one way only. The person's `home` holds the strong `Rc`; the
apartment's `tenant` holds a `Weak`. When the program drops its handles, the person's strong
count reaches zero, its destructor runs, and dropping its `home` field releases the apartment.

**Complexity:** `O(1)` per operation (the name is cloned in `tenant_name`).

**Rust concepts:**
- `Rc::strong_count` decides when a value is dropped; `Rc::weak_count` does not.
- `Option::replace` and `Option::take` swap a link out of a `RefCell<Option<..>>` in one step.
- The given `Drop` impls work as a leak detector: a value that leaks never writes its line.

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Person {
    pub name: String,
    pub home: RefCell<Option<Rc<Apartment>>>,
    log: Log,
}

pub struct Apartment {
    pub number: u32,
    pub tenant: RefCell<Weak<Person>>,
    log: Log,
}

impl Person {
    /// Given: a person without a home.
    pub fn new(name: &str, log: &Log) -> Rc<Person> {
        Rc::new(Person {
            name: name.to_string(),
            home: RefCell::new(None),
            log: Rc::clone(log),
        })
    }
}

impl Apartment {
    /// Given: an empty apartment.
    pub fn new(number: u32, log: &Log) -> Rc<Apartment> {
        Rc::new(Apartment {
            number,
            tenant: RefCell::new(Weak::new()),
            log: Rc::clone(log),
        })
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        self.log
            .borrow_mut()
            .push(format!("person {} dropped", self.name));
    }
}

impl Drop for Apartment {
    fn drop(&mut self) {
        self.log
            .borrow_mut()
            .push(format!("apartment {} dropped", self.number));
    }
}

pub fn move_in(person: &Rc<Person>, apartment: &Rc<Apartment>) {
    let old_home = person.home.borrow_mut().replace(Rc::clone(apartment));
    if let Some(old) = old_home {
        if !Rc::ptr_eq(&old, apartment) {
            *old.tenant.borrow_mut() = Weak::new();
        }
    }
    *apartment.tenant.borrow_mut() = Rc::downgrade(person);
}

pub fn tenant_name(apartment: &Apartment) -> Option<String> {
    apartment
        .tenant
        .borrow()
        .upgrade()
        .map(|person| person.name.clone())
}

pub fn move_out(person: &Person) {
    if let Some(home) = person.home.borrow_mut().take() {
        *home.tenant.borrow_mut() = Weak::new();
    }
}
```

Full tested solution: [`src/solutions/p0625_breaking_reference_cycles.rs`](../src/solutions/p0625_breaking_reference_cycles.rs)

</details>
