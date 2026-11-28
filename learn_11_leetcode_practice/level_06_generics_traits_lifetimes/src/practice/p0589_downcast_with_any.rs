// ==================================================================
// PRACTICE 0589: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0589_downcast_with_any.md
// Run the tests:          cargo test practice::p0589
// ==================================================================

//! # Problem 0589: Downcasting with `Any`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `std::any::Any`, `downcast_ref`/`downcast_mut`/`Box::downcast`, `TypeId`, trait upcasting `&dyn Component` to `&dyn Any` (Rust 1.86+), the `Box::type_id` pitfall
//! - **Source:** Original · [std::any](https://doc.rust-lang.org/std/any/index.html)
//!
//! ## Problem
//!
//! An entity in a small game engine owns a list of components of **different** types, stored as
//! `Box<dyn Component>`. Sometimes code needs the concrete type back ("give me this entity's
//! `Health`"): that is **downcasting**, and `std::any::Any` provides it.
//!
//! ```text
//! pub trait Component: Any { fn name(&self) -> &'static str; }
//! ```
//!
//! Because `Any` is a supertrait, a `&dyn Component` can be **upcast** to `&dyn Any` (stable since
//! Rust 1.86), and `&dyn Any` has `downcast_ref::<T>()`.
//!
//! Components: `Health(pub u32)` (`"health"`), `Position { x: i32, y: i32 }` (`"position"`) and
//! `Tag(pub String)` (`"tag"`). Implement `Entity` with a private `components: Vec<Box<dyn Component>>`:
//!
//! - `new()` and `Default`.
//! - `insert<C: Component>(&mut self, component: C)`: adds it, **replacing** an existing component of
//!   the same type.
//! - `get<C: Component>(&self) -> Option<&C>` and `get_mut<C: Component>(&mut self) -> Option<&mut C>`.
//! - `remove<C: Component>(&mut self) -> Option<C>`: takes the component out and returns it by value.
//! - `has<C: Component>(&self) -> bool`, `names(&self) -> Vec<&'static str>` (insertion order).
//!
//! ## Examples
//!
//! ```text
//! let mut e = Entity::new();
//! e.insert(Health(10));
//! e.insert(Position { x: 1, y: 2 });
//! e.get::<Health>().map(|h| h.0)       => Some(10)
//! e.get_mut::<Position>().unwrap().x += 5;
//! e.insert(Health(3));                 // replaces the old Health
//! e.names()                            => ["health", "position"]
//! e.remove::<Health>()                 => Some(Health(3))
//! e.has::<Health>()                    => false
//! ```
//!
//! ## Constraints
//!
//! - At most one component per type.
//!
//! ## Hints
//!
//! 1. `let any: &dyn Any = component.as_ref();` upcasts a `&Box<dyn Component>` content to `&dyn Any`.
//! 2. Pitfall: on a `&Box<dyn Component>`, `.type_id()` returns the id of the **`Box`** type, because
//!    `Box<dyn Component>` itself implements `Any`. Upcast first or write `(**b).type_id()`.
//! 3. `Box<dyn Component>` converts into `Box<dyn Any>`, whose `downcast::<C>()` returns `Result<Box<C>,
//!    Box<dyn Any>>`.

use std::any::{Any, TypeId};

/// A piece of data attached to an entity.
pub trait Component: Any {
    fn name(&self) -> &'static str;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Health(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag(pub String);

impl Component for Health {
    fn name(&self) -> &'static str {
        todo!("Write your solution here")
    }
}

impl Component for Position {
    fn name(&self) -> &'static str {
        todo!("Write your solution here")
    }
}

impl Component for Tag {
    fn name(&self) -> &'static str {
        todo!("Write your solution here")
    }
}

/// A bag of components, at most one per type.
pub struct Entity {}

impl Entity {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn insert<C: Component>(&mut self, component: C) {
        todo!("Write your solution here")
    }

    pub fn get<C: Component>(&self) -> Option<&C> {
        todo!("Write your solution here")
    }

    pub fn get_mut<C: Component>(&mut self) -> Option<&mut C> {
        todo!("Write your solution here")
    }

    pub fn remove<C: Component>(&mut self) -> Option<C> {
        todo!("Write your solution here")
    }

    pub fn has<C: Component>(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn names(&self) -> Vec<&'static str> {
        todo!("Write your solution here")
    }
}

impl Default for Entity {
    fn default() -> Self {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_session() {
        let mut e = Entity::new();
        e.insert(Health(10));
        e.insert(Position { x: 1, y: 2 });
        assert_eq!(e.get::<Health>().map(|h| h.0), Some(10));
        e.get_mut::<Position>().unwrap().x += 5;
        assert_eq!(e.get::<Position>(), Some(&Position { x: 6, y: 2 }));
        e.insert(Health(3));
        assert_eq!(e.names(), vec!["health", "position"]);
        assert_eq!(e.remove::<Health>(), Some(Health(3)));
        assert!(!e.has::<Health>());
        assert!(e.has::<Position>());
    }

    #[test]
    fn missing_components() {
        let mut e = Entity::default();
        assert!(e.get::<Tag>().is_none());
        assert!(e.get_mut::<Tag>().is_none());
        assert!(e.remove::<Tag>().is_none());
        e.insert(Tag("player".into()));
        assert_eq!(e.get::<Tag>().map(|t| t.0.as_str()), Some("player"));
        assert!(e.get::<Health>().is_none());
    }

    #[test]
    fn user_defined_components() {
        #[derive(Debug, PartialEq)]
        struct Velocity(f32, f32);
        impl Component for Velocity {
            fn name(&self) -> &'static str {
                "velocity"
            }
        }
        let mut e = Entity::new();
        e.insert(Velocity(1.0, 0.5));
        e.insert(Health(1));
        e.insert(Velocity(2.0, 0.0));
        assert_eq!(e.names(), vec!["velocity", "health"]);
        assert_eq!(e.remove::<Velocity>(), Some(Velocity(2.0, 0.0)));
        assert_eq!(e.names(), vec!["health"]);
    }

    #[test]
    fn box_type_id_pitfall() {
        let boxed: Box<dyn Component> = Box::new(Health(1));
        // `type_id` called on the Box itself reports the Box type...
        assert_eq!(Any::type_id(&boxed), TypeId::of::<Box<dyn Component>>());
        // ...while upcasting the contents reports the concrete type.
        let inner: &dyn Any = boxed.as_ref();
        assert_eq!(inner.type_id(), TypeId::of::<Health>());
        let mut e = Entity::new();
        e.insert(Health(7));
        assert_eq!(e.get::<Health>(), Some(&Health(7)));
    }
}
