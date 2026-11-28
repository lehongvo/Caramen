// ==================================================================
// PRACTICE 0583: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0583_plugin_registry.md
// Run the tests:          cargo test practice::p0583
// ==================================================================

//! # Problem 0583: A Plugin Registry
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Box<dyn Plugin>` with `&mut self` methods, stateful trait objects, returning `Option<&mut dyn Plugin>`, removing boxed objects
//! - **Source:** Original · [The Rust Book 18.2](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
//!
//! ## Problem
//!
//! A chat bot is extended with plugins. Each plugin implements
//!
//! ```text
//! pub trait Plugin {
//!     fn name(&self) -> &str;
//!     fn handle(&mut self, input: &str) -> Option<String>;   // Some(reply) if it handles the input
//! }
//! ```
//!
//! Three plugins are provided by you:
//!
//! - `Echo`: name `"echo"`; handles inputs starting with `"echo "` by replying with the rest.
//! - `Counter { count: u32 }`: name `"counter"`; handles exactly `"count"`, increments its count and
//!   replies with the new value (state lives inside the plugin, hence `&mut self`).
//! - `Shout`: name `"shout"`; handles inputs starting with `"shout "` and replies with the rest in
//!   uppercase followed by `"!"`.
//!
//! `PluginHost` owns the plugins in registration order and offers:
//!
//! - `new()` and `Default`.
//! - `register(&mut self, plugin: Box<dyn Plugin>) -> Result<(), String>`: fails with
//!   `"plugin '{name}' is already registered"` if a plugin with the same name exists.
//! - `unregister(&mut self, name: &str) -> Option<Box<dyn Plugin>>`: removes and returns it.
//! - `names(&self) -> Vec<&str>`.
//! - `dispatch(&mut self, input: &str) -> Option<String>`: asks the plugins in order and returns the
//!   first reply; `None` if no plugin handles the input.
//! - `get_mut(&mut self, name: &str) -> Option<&mut dyn Plugin>`.
//!
//! ## Examples
//!
//! ```text
//! let mut host = PluginHost::new();
//! host.register(Box::new(Echo))?;
//! host.register(Box::new(Counter { count: 0 }))?;
//! host.dispatch("echo hello")   => Some("hello")
//! host.dispatch("count")        => Some("1")
//! host.dispatch("count")        => Some("2")
//! host.dispatch("dance")        => None
//! host.register(Box::new(Echo)) => Err("plugin 'echo' is already registered")
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `self.plugins.iter_mut().find_map(|p| p.handle(input))` stops at the first `Some`.
//! 2. `iter().position(..)` plus `Vec::remove` takes a plugin out by name.
//! 3. `b.as_mut()` turns a `&mut Box<dyn Plugin>` into a `&mut dyn Plugin`.

/// A bot extension.
pub trait Plugin {
    fn name(&self) -> &str;
    fn handle(&mut self, input: &str) -> Option<String>;
}

pub struct Echo;

pub struct Counter {
    pub count: u32,
}

pub struct Shout;

impl Plugin for Echo {
    fn name(&self) -> &str {
        todo!("Write your solution here")
    }

    fn handle(&mut self, input: &str) -> Option<String> {
        todo!("Write your solution here")
    }
}

impl Plugin for Counter {
    fn name(&self) -> &str {
        todo!("Write your solution here")
    }

    fn handle(&mut self, input: &str) -> Option<String> {
        todo!("Write your solution here")
    }
}

impl Plugin for Shout {
    fn name(&self) -> &str {
        todo!("Write your solution here")
    }

    fn handle(&mut self, input: &str) -> Option<String> {
        todo!("Write your solution here")
    }
}

/// Owns plugins and routes inputs to them.
pub struct PluginHost {}

impl PluginHost {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<(), String> {
        todo!("Write your solution here")
    }

    pub fn unregister(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        todo!("Write your solution here")
    }

    pub fn names(&self) -> Vec<&str> {
        todo!("Write your solution here")
    }

    pub fn dispatch(&mut self, input: &str) -> Option<String> {
        todo!("Write your solution here")
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut dyn Plugin> {
        todo!("Write your solution here")
    }
}

impl Default for PluginHost {
    fn default() -> Self {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn host() -> PluginHost {
        let mut host = PluginHost::new();
        host.register(Box::new(Echo)).unwrap();
        host.register(Box::new(Counter { count: 0 })).unwrap();
        host.register(Box::new(Shout)).unwrap();
        host
    }

    #[test]
    fn dispatching_inputs() {
        let mut h = host();
        assert_eq!(h.dispatch("echo hello"), Some("hello".to_string()));
        assert_eq!(h.dispatch("count"), Some("1".to_string()));
        assert_eq!(h.dispatch("count"), Some("2".to_string()));
        assert_eq!(h.dispatch("shout hi there"), Some("HI THERE!".to_string()));
        assert_eq!(h.dispatch("dance"), None);
    }

    #[test]
    fn duplicates_and_names() {
        let mut h = host();
        assert_eq!(h.names(), vec!["echo", "counter", "shout"]);
        assert_eq!(
            h.register(Box::new(Echo)),
            Err("plugin 'echo' is already registered".to_string())
        );
        assert_eq!(h.names().len(), 3);
    }

    #[test]
    fn unregister_and_register_again_keeps_state() {
        let mut h = host();
        h.dispatch("count");
        let counter = h.unregister("counter").expect("registered");
        assert_eq!(h.dispatch("count"), None);
        assert!(h.unregister("counter").is_none());
        h.register(counter).unwrap();
        assert_eq!(h.dispatch("count"), Some("2".to_string()));
        assert_eq!(h.names(), vec!["echo", "shout", "counter"]);
    }

    #[test]
    fn user_plugins_and_get_mut() {
        struct Reverse;
        impl Plugin for Reverse {
            fn name(&self) -> &str {
                "reverse"
            }
            fn handle(&mut self, input: &str) -> Option<String> {
                Some(input.chars().rev().collect())
            }
        }
        let mut h = PluginHost::default();
        h.register(Box::new(Reverse)).unwrap();
        h.register(Box::new(Echo)).unwrap();
        // Reverse handles everything and comes first.
        assert_eq!(h.dispatch("echo abc"), Some("cba ohce".to_string()));
        let echo = h.get_mut("echo").expect("registered");
        assert_eq!(echo.handle("echo direct"), Some("direct".to_string()));
        assert!(h.get_mut("missing").is_none());
    }
}
