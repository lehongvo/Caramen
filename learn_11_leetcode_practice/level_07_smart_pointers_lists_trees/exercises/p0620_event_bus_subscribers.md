# Problem 0620: Event Bus with Shared Inboxes

- **Difficulty:** Medium
- **Topics:** `Rc<RefCell<Vec<String>>>`, `HashMap<String, Vec<Rc<..>>>`, `Rc::ptr_eq`, `Vec::retain`
- **Source:** Original · [The Rust Book 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) · [`Rc::ptr_eq`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.ptr_eq)

## Problem

An event bus delivers messages published on a *topic* to every inbox subscribed to that topic.
An inbox is shared: the subscriber reads it, the bus writes into it.

```text
pub type Inbox = Rc<RefCell<Vec<String>>>;
```

Implement `EventBus`:

- `EventBus::new() -> EventBus`;
- `subscribe(&mut self, topic: &str) -> Inbox`: create a new empty inbox subscribed to `topic`
  and return it;
- `subscribe_inbox(&mut self, topic: &str, inbox: &Inbox)`: subscribe an **existing** inbox to
  one more topic. Subscribing the same inbox twice to the same topic has no extra effect;
- `publish(&self, topic: &str, message: &str) -> usize`: push `"<topic>: <message>"` into every
  inbox subscribed to `topic`, in subscription order, and return the number of inboxes reached;
- `unsubscribe(&mut self, topic: &str, inbox: &Inbox) -> bool`: remove that inbox (compared by
  identity, not by content) from the topic; return whether it was subscribed.

## Examples

```text
let mut bus = EventBus::new();
let ana = bus.subscribe("news");
let bo = bus.subscribe("news");
bus.subscribe_inbox("sports", &ana);
bus.publish("news", "hi")     -> 2
bus.publish("sports", "goal") -> 1
bus.publish("weather", "sun") -> 0
*ana.borrow() == ["news: hi", "sports: goal"]
*bo.borrow()  == ["news: hi"]
bus.unsubscribe("news", &bo)  -> true
bus.publish("news", "bye")    -> 1
```

## Constraints

- At most `10^3` topics, inboxes and operations.

## Hints

1. Store `HashMap<String, Vec<Inbox>>`: the bus keeps its own `Rc` clone of each inbox.
2. Two inboxes with the same messages are still different inboxes: use `Rc::ptr_eq`.
3. `publish` only needs `&self`: the `Rc` handles are shared, and each inbox's `RefCell` allows
   the push.

## Starter code

Write your code in [`src/practice/p0620_event_bus_subscribers.rs`](../src/practice/p0620_event_bus_subscribers.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0620
```

```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type Inbox = Rc<RefCell<Vec<String>>>;

#[derive(Default)]
pub struct EventBus {}

impl EventBus {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn subscribe(&mut self, topic: &str) -> Inbox {
        todo!("Write your solution here")
    }

    pub fn subscribe_inbox(&mut self, topic: &str, inbox: &Inbox) {
        todo!("Write your solution here")
    }

    pub fn publish(&self, topic: &str, message: &str) -> usize {
        todo!("Write your solution here")
    }

    pub fn unsubscribe(&mut self, topic: &str, inbox: &Inbox) -> bool {
        todo!("Write your solution here")
    }
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
    fn example_flow() {
        let mut bus = EventBus::new();
        let ana = bus.subscribe("news");
        let bo = bus.subscribe("news");
        bus.subscribe_inbox("sports", &ana);
        assert_eq!(bus.publish("news", "hi"), 2);
        assert_eq!(bus.publish("sports", "goal"), 1);
        assert_eq!(bus.publish("weather", "sun"), 0);
        assert_eq!(*ana.borrow(), ["news: hi", "sports: goal"]);
        assert_eq!(*bo.borrow(), ["news: hi"]);
        assert!(bus.unsubscribe("news", &bo));
        assert_eq!(bus.publish("news", "bye"), 1);
        assert_eq!(*bo.borrow(), ["news: hi"]);
    }

    #[test]
    fn duplicate_subscription_is_ignored() {
        let mut bus = EventBus::new();
        let inbox = bus.subscribe("t");
        bus.subscribe_inbox("t", &inbox);
        assert_eq!(bus.publish("t", "x"), 1);
        assert_eq!(inbox.borrow().len(), 1);
        assert_eq!(Rc::strong_count(&inbox), 2);
    }

    #[test]
    fn unsubscribe_uses_identity_not_content() {
        let mut bus = EventBus::new();
        let a = bus.subscribe("t");
        let look_alike = Inbox::default();
        assert!(!bus.unsubscribe("t", &look_alike));
        assert!(!bus.unsubscribe("missing", &a));
        assert!(bus.unsubscribe("t", &a));
        assert!(!bus.unsubscribe("t", &a));
        assert_eq!(Rc::strong_count(&a), 1);
    }

    #[test]
    fn inbox_outlives_the_bus() {
        let inbox = {
            let mut bus = EventBus::new();
            let inbox = bus.subscribe("t");
            bus.publish("t", "kept");
            inbox
        };
        assert_eq!(Rc::strong_count(&inbox), 1);
        assert_eq!(*inbox.borrow(), ["t: kept"]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a map from topic to the list of subscribed inbox handles. `publish` borrows each inbox
mutably for one push. `unsubscribe` removes by pointer identity with `retain`.

**Complexity:** `publish` is `O(s)` for `s` subscribers of the topic; `subscribe_inbox` and
`unsubscribe` are `O(s)` (duplicate check / retain).

**Rust concepts:**
- The bus and the subscriber both own the same inbox (`Rc`), and both can use it mutably at
  different times (`RefCell`).
- `publish(&self)` mutates the inboxes but not the bus: interior mutability lives in the inbox.
- `Vec::retain` with `!Rc::ptr_eq(..)` removes exactly the given handle.

```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type Inbox = Rc<RefCell<Vec<String>>>;

#[derive(Default)]
pub struct EventBus {
    topics: HashMap<String, Vec<Inbox>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe(&mut self, topic: &str) -> Inbox {
        let inbox = Inbox::default();
        self.subscribe_inbox(topic, &inbox);
        inbox
    }

    pub fn subscribe_inbox(&mut self, topic: &str, inbox: &Inbox) {
        let subscribers = self.topics.entry(topic.to_string()).or_default();
        if !subscribers
            .iter()
            .any(|existing| Rc::ptr_eq(existing, inbox))
        {
            subscribers.push(Rc::clone(inbox));
        }
    }

    pub fn publish(&self, topic: &str, message: &str) -> usize {
        let Some(subscribers) = self.topics.get(topic) else {
            return 0;
        };
        for inbox in subscribers {
            inbox.borrow_mut().push(format!("{topic}: {message}"));
        }
        subscribers.len()
    }

    pub fn unsubscribe(&mut self, topic: &str, inbox: &Inbox) -> bool {
        let Some(subscribers) = self.topics.get_mut(topic) else {
            return false;
        };
        let before = subscribers.len();
        subscribers.retain(|existing| !Rc::ptr_eq(existing, inbox));
        subscribers.len() != before
    }
}
```

Full tested solution: [`src/solutions/p0620_event_bus_subscribers.rs`](../src/solutions/p0620_event_bus_subscribers.rs)

</details>
