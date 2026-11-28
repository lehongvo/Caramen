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
//! 3. In `get_mut`, `b.as_mut()` on a `&mut Box<dyn Plugin>` gives `&mut (dyn Plugin + 'static)`,
//!    but the elided return type means `&'a mut (dyn Plugin + 'a)`. `&mut T` is invariant in `T`, so
//!    the `'static` must be shortened by a *coercion*, and the closure passed to `Option::map` has no
//!    declared return type to coerce to: `.map(|b| b.as_mut())` fails with "lifetime may not live long
//!    enough". Write `.map(|b| b.as_mut() as &mut dyn Plugin)`, or
//!    `let b = self.plugins.iter_mut().find(..)?; Some(b.as_mut())`. Ignore the compiler's `+ '_` and
//!    `move` suggestions here.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the host is a `Vec<Box<dyn Plugin>>`. Registration checks the names first, dispatch walks
//! the plugins with `find_map`, and removal moves the box out of the vector.
//!
//! **Rust concepts:**
//! - Trait objects can carry mutable state: calling `handle(&mut self)` through `&mut Box<dyn Plugin>`
//!   updates the concrete plugin hidden behind the box.
//! - Returning `Box<dyn Plugin>` from `unregister` hands ownership back to the caller, who can register
//!   it again later.
//! - `Option<&mut dyn Plugin>` borrows from the host; while the caller holds it, the host cannot be used
//!   (one mutable borrow at a time).
//! - Every trait object has a lifetime bound, and an omitted one gets a default
//!   ([Reference: default trait object lifetimes](https://doc.rust-lang.org/reference/lifetime-elision.html#default-trait-object-lifetimes)):
//!   `Box<dyn Plugin>` means `Box<dyn Plugin + 'static>`, while `&'a mut dyn Plugin` means
//!   `&'a mut (dyn Plugin + 'a)`. Going from the first to the second needs an explicit **coercion**:
//!   `as &mut dyn Plugin` inside the `map` closure, or `Some(b.as_mut())` where the function's return
//!   type is the target ([coercion sites](https://doc.rust-lang.org/reference/type-coercions.html#coercion-sites)).
//!   Shared references are covariant, so a `get(&self, ..) -> Option<&dyn Plugin>` could use
//!   `.map(|b| b.as_ref())` without a cast.
// @solution-end

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
        // @solution-start
        "echo"
        // @solution-end
    }

    fn handle(&mut self, input: &str) -> Option<String> {
        // @solution-start
        input.strip_prefix("echo ").map(str::to_string)
        // @solution-end
    }
}

impl Plugin for Counter {
    fn name(&self) -> &str {
        // @solution-start
        "counter"
        // @solution-end
    }

    fn handle(&mut self, input: &str) -> Option<String> {
        // @solution-start
        if input == "count" {
            self.count += 1;
            Some(self.count.to_string())
        } else {
            None
        }
        // @solution-end
    }
}

impl Plugin for Shout {
    fn name(&self) -> &str {
        // @solution-start
        "shout"
        // @solution-end
    }

    fn handle(&mut self, input: &str) -> Option<String> {
        // @solution-start
        input
            .strip_prefix("shout ")
            .map(|rest| format!("{}!", rest.to_uppercase()))
        // @solution-end
    }
}

/// Owns plugins and routes inputs to them.
pub struct PluginHost {
    // @solution-start remove
    plugins: Vec<Box<dyn Plugin>>,
    // @solution-end
}

impl PluginHost {
    pub fn new() -> Self {
        // @solution-start
        PluginHost {
            plugins: Vec::new(),
        }
        // @solution-end
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<(), String> {
        // @solution-start
        if self.plugins.iter().any(|p| p.name() == plugin.name()) {
            return Err(format!("plugin '{}' is already registered", plugin.name()));
        }
        self.plugins.push(plugin);
        Ok(())
        // @solution-end
    }

    pub fn unregister(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        // @solution-start
        let index = self.plugins.iter().position(|p| p.name() == name)?;
        Some(self.plugins.remove(index))
        // @solution-end
    }

    pub fn names(&self) -> Vec<&str> {
        // @solution-start
        self.plugins.iter().map(|p| p.name()).collect()
        // @solution-end
    }

    pub fn dispatch(&mut self, input: &str) -> Option<String> {
        // @solution-start
        self.plugins.iter_mut().find_map(|p| p.handle(input))
        // @solution-end
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut dyn Plugin> {
        // @solution-start
        // The cast shortens `dyn Plugin + 'static` (inside the box) to the borrow's lifetime;
        // without it the closure returns the `'static` type and the function does not compile.
        self.plugins
            .iter_mut()
            .find(|p| p.name() == name)
            .map(|b| b.as_mut() as &mut dyn Plugin)
        // @solution-end
    }
}

impl Default for PluginHost {
    fn default() -> Self {
        // @solution-start
        Self::new()
        // @solution-end
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
