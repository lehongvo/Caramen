//! # Problem 0447: Event Listeners with Boxed FnMut
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Vec<Box<dyn FnMut>>`, `impl FnMut(..) + 'static` parameters, `iter_mut`, `filter_map` with stateful closures, `#[derive(Default)]`
//! - **Source:** Original · [std `FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) · [The Rust Book 18.2: Using trait objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
//!
//! ## Problem
//!
//! An *event bus* lets parts of a program react to named events without knowing about each other.
//! Listeners are closures registered for an event name; they may keep their own state (for example
//! a counter), so they are `FnMut`, and since every closure has a different type they are stored
//! boxed.
//!
//! Implement `EventBus`:
//!
//! - `EventBus::new() -> EventBus`: no listeners.
//! - `subscribe(&mut self, event: &str, listener: impl FnMut(&str) -> Option<String> + 'static)`:
//!   register `listener` for `event`.
//! - `emit(&mut self, event: &str, payload: &str) -> Vec<String>`: call every listener registered for
//!   exactly `event`, in subscription order, with `payload`. Collect the `Some` answers (a listener
//!   returning `None` just stays silent). Listeners of other events are not called.
//! - `listener_count(&self, event: &str) -> usize`.
//!
//! ## Examples
//!
//! ```text
//! let mut bus = EventBus::new();
//! bus.subscribe("login", |user| Some(format!("welcome {user}")));
//! let mut count = 0;
//! bus.subscribe("login", move |_| { count += 1; Some(format!("login #{count}")) });
//! bus.subscribe("logout", |_| None);
//! bus.emit("login", "ann")   -> ["welcome ann", "login #1"]
//! bus.emit("login", "bob")   -> ["welcome bob", "login #2"]
//! bus.emit("logout", "ann")  -> []
//! bus.emit("crash", "!")     -> []
//! bus.listener_count("login") -> 2
//! ```
//!
//! ## Constraints
//!
//! - At most `10^3` listeners.
//!
//! ## Hints
//!
//! 1. Store `Vec<(String, Box<dyn FnMut(&str) -> Option<String>>)>`.
//! 2. Calling an `FnMut` needs `&mut` access, so iterate with `iter_mut()`.
//! 3. `filter(...)` on the event name, then `filter_map(|(_, listener)| listener(payload))`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** keep `(event name, boxed listener)` pairs in subscription order. Emitting walks them
//! mutably, calls the matching ones and keeps their non-`None` answers.
//!
//! **Rust concepts:**
//! - `Box<dyn FnMut(&str) -> Option<String>>` stores closures of different types in one `Vec`.
//! - The `'static` bound on `subscribe` means a listener cannot borrow short-lived local variables;
//!   state such as `count` must be **moved** into the closure.
//! - `emit` takes `&mut self` because calling an `FnMut` mutates the closure's captured state.
//! - `#[derive(Default)]` plus `new()` returning `Self::default()` keeps Clippy's
//!   `new_without_default` lint happy.
// @solution-end

type Listener = Box<dyn FnMut(&str) -> Option<String>>;

#[derive(Default)]
pub struct EventBus {
    // @solution-start remove
    listeners: Vec<(String, Listener)>,
    // @solution-end
}

impl EventBus {
    pub fn new() -> EventBus {
        // @solution-start
        Self::default()
        // @solution-end
    }

    pub fn subscribe(
        &mut self,
        event: &str,
        listener: impl FnMut(&str) -> Option<String> + 'static,
    ) {
        // @solution-start
        self.listeners.push((event.to_string(), Box::new(listener)));
        // @solution-end
    }

    pub fn emit(&mut self, event: &str, payload: &str) -> Vec<String> {
        // @solution-start
        self.listeners
            .iter_mut()
            .filter(|(name, _)| name == event)
            .filter_map(|(_, listener)| listener(payload))
            .collect()
        // @solution-end
    }

    pub fn listener_count(&self, event: &str) -> usize {
        // @solution-start
        self.listeners
            .iter()
            .filter(|(name, _)| name == event)
            .count()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_bus() -> EventBus {
        let mut bus = EventBus::new();
        bus.subscribe("login", |user| Some(format!("welcome {user}")));
        let mut count = 0;
        bus.subscribe("login", move |_| {
            count += 1;
            Some(format!("login #{count}"))
        });
        bus.subscribe("logout", |_| None);
        bus
    }

    #[test]
    fn emit_calls_matching_listeners_in_order() {
        let mut bus = sample_bus();
        assert_eq!(bus.emit("login", "ann"), vec!["welcome ann", "login #1"]);
        assert_eq!(bus.emit("login", "bob"), vec!["welcome bob", "login #2"]);
    }

    #[test]
    fn silent_and_unknown_events() {
        let mut bus = sample_bus();
        assert_eq!(bus.emit("logout", "ann"), Vec::<String>::new());
        assert_eq!(bus.emit("crash", "!"), Vec::<String>::new());
        assert_eq!(bus.emit("LOGIN", "ann"), Vec::<String>::new());
    }

    #[test]
    fn listener_counts() {
        let bus = sample_bus();
        assert_eq!(bus.listener_count("login"), 2);
        assert_eq!(bus.listener_count("logout"), 1);
        assert_eq!(bus.listener_count("crash"), 0);
        assert_eq!(EventBus::new().listener_count("login"), 0);
    }

    #[test]
    fn listeners_keep_private_state() {
        let mut bus = EventBus::new();
        let mut history: Vec<String> = Vec::new();
        bus.subscribe("msg", move |text| {
            history.push(text.to_string());
            (history.len() % 2 == 0).then(|| history.join("+"))
        });
        assert_eq!(bus.emit("msg", "a"), Vec::<String>::new());
        assert_eq!(bus.emit("msg", "b"), vec!["a+b"]);
        assert_eq!(bus.emit("msg", "c"), Vec::<String>::new());
        assert_eq!(bus.emit("msg", "d"), vec!["a+b+c+d"]);
    }
}
