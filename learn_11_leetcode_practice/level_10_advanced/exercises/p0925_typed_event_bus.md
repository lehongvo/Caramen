# Problem 0925: A Typed Event Bus

- **Difficulty:** Hard
- **Topics:** `TypeId`, `HashMap<TypeId, Vec<Box<dyn Fn(&dyn Any)>>>`, type erasure, generic methods with `'static` bounds, closures wrapping closures
- **Source:** Original · [std: `TypeId`](https://doc.rust-lang.org/std/any/struct.TypeId.html) · [std: `Any::downcast_ref`](https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref)

## Problem

An event bus lets parts of a program react to events without knowing each other. Subscribers
register for **one event type**; publishing an event calls exactly the handlers registered for
its type. All handlers live in one map, so their types must be **erased** and recovered later.

Implement `EventBus` (choose the private fields; a `HashMap<TypeId, Vec<Box<dyn Fn(&dyn Any)>>>`
works well):

- `new()` and `Default`.
- `subscribe<E: 'static>(&mut self, handler: impl Fn(&E) + 'static)`: registers `handler` for
  events of type `E`.
- `publish<E: 'static>(&self, event: &E) -> usize`: calls every handler registered for `E`, in
  subscription order, and returns how many were called. It also counts the event (see below).
- `handler_count<E: 'static>(&self) -> usize`: the number of handlers for `E`.
- `published(&self) -> usize`: how many events were published so far, of any type, including
  events nobody listened to. (`publish` takes `&self`, so use a `Cell<usize>`.)

## Examples

```text
struct UserJoined { name: String }
struct Message(String);

bus.subscribe(|e: &UserJoined| log.push(format!("welcome {}", e.name)));
bus.subscribe(|m: &Message| log.push(format!("msg {}", m.0)));
bus.publish(&UserJoined { name: "ada".into() })   => 1, log: ["welcome ada"]
bus.publish(&42_u32)                              => 0  (nobody listens to u32)
bus.published()                                   => 2
```

## Constraints

- No special constraints.

## Hints

1. The key for event type `E` is `TypeId::of::<E>()`.
2. Store `Box::new(move |any: &dyn Any| if let Some(e) = any.downcast_ref::<E>() { handler(e) })`:
   the wrapper remembers `E` even though its own type no longer mentions it.
3. When publishing, pass `event` as `&dyn Any` (an `&E` coerces to it because `E: 'static`).

## Starter code

Write your code in [`src/practice/p0925_typed_event_bus.rs`](../src/practice/p0925_typed_event_bus.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0925
```

```rust
use std::any::{Any, TypeId};
use std::cell::Cell;
use std::collections::HashMap;

type Handler = Box<dyn Fn(&dyn Any)>;

pub struct EventBus {}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn subscribe<E: 'static>(&mut self, handler: impl Fn(&E) + 'static) {
        todo!("Write your solution here")
    }

    pub fn publish<E: 'static>(&self, event: &E) -> usize {
        todo!("Write your solution here")
    }

    pub fn handler_count<E: 'static>(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn published(&self) -> usize {
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
    use std::cell::RefCell;
    use std::rc::Rc;

    struct UserJoined {
        name: String,
    }

    struct Message(String);

    fn shared_log() -> Rc<RefCell<Vec<String>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[test]
    fn handlers_receive_only_their_event_type() {
        let log = shared_log();
        let mut bus = EventBus::new();
        let l = Rc::clone(&log);
        bus.subscribe(move |e: &UserJoined| l.borrow_mut().push(format!("welcome {}", e.name)));
        let l = Rc::clone(&log);
        bus.subscribe(move |m: &Message| l.borrow_mut().push(format!("msg {}", m.0)));
        assert_eq!(
            bus.publish(&UserJoined {
                name: "ada".to_string()
            }),
            1
        );
        assert_eq!(bus.publish(&Message("hi".to_string())), 1);
        assert_eq!(bus.publish(&42_u32), 0);
        assert_eq!(*log.borrow(), vec!["welcome ada", "msg hi"]);
        assert_eq!(bus.published(), 3);
    }

    #[test]
    fn several_handlers_run_in_subscription_order() {
        let log = shared_log();
        let mut bus = EventBus::default();
        for tag in ["first", "second", "third"] {
            let l = Rc::clone(&log);
            bus.subscribe(move |n: &i32| l.borrow_mut().push(format!("{tag}:{n}")));
        }
        assert_eq!(bus.handler_count::<i32>(), 3);
        assert_eq!(bus.handler_count::<i64>(), 0);
        assert_eq!(bus.publish(&7_i32), 3);
        assert_eq!(*log.borrow(), vec!["first:7", "second:7", "third:7"]);
        // i64 is a different type from i32: nobody is called.
        assert_eq!(bus.publish(&7_i64), 0);
    }

    #[test]
    fn handlers_can_keep_state() {
        let total = Rc::new(Cell::new(0_u64));
        let mut bus = EventBus::new();
        let t = Rc::clone(&total);
        bus.subscribe(move |amount: &u64| t.set(t.get() + amount));
        for amount in 1..=100_u64 {
            bus.publish(&amount);
        }
        assert_eq!(total.get(), 5_050);
        assert_eq!(bus.published(), 100);
    }

    #[test]
    fn empty_bus() {
        let bus = EventBus::new();
        assert_eq!(bus.published(), 0);
        assert_eq!(bus.publish(&"nobody"), 0);
        assert_eq!(bus.published(), 1);
        assert_eq!(bus.handler_count::<&str>(), 0);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `subscribe` wraps the typed handler in an untyped closure that downcasts its argument
back to `E`, and files it under `TypeId::of::<E>()`. `publish` looks up the same key, so the
downcast in the wrapper always succeeds; it calls each wrapper with the event as `&dyn Any`.

**Complexity:** `subscribe` is `O(1)` amortised; `publish` is `O(handlers for E)`.

**Rust concepts:**
- `TypeId::of::<E>()` is a unique, comparable identifier for a `'static` type, which makes it a
  good `HashMap` key for type-indexed storage.
- Type erasure: `Box<dyn Fn(&dyn Any)>` hides the concrete handler type and its event type; the
  generic `subscribe` is where `E` is still known, so that is where the downcast is written.
- `impl Fn(&E) + 'static` is required because the handler is stored in a `Box<dyn ...>` with no
  lifetime, which means `'static`.
- `Cell<usize>` gives interior mutability for a counter behind `&self`.

```rust
use std::any::{Any, TypeId};
use std::cell::Cell;
use std::collections::HashMap;

type Handler = Box<dyn Fn(&dyn Any)>;

pub struct EventBus {
    handlers: HashMap<TypeId, Vec<Handler>>,
    published: Cell<usize>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            handlers: HashMap::new(),
            published: Cell::new(0),
        }
    }

    pub fn subscribe<E: 'static>(&mut self, handler: impl Fn(&E) + 'static) {
        let wrapper: Handler = Box::new(move |any: &dyn Any| {
            if let Some(event) = any.downcast_ref::<E>() {
                handler(event);
            }
        });
        self.handlers
            .entry(TypeId::of::<E>())
            .or_default()
            .push(wrapper);
    }

    pub fn publish<E: 'static>(&self, event: &E) -> usize {
        self.published.set(self.published.get() + 1);
        let Some(handlers) = self.handlers.get(&TypeId::of::<E>()) else {
            return 0;
        };
        for handler in handlers {
            handler(event);
        }
        handlers.len()
    }

    pub fn handler_count<E: 'static>(&self) -> usize {
        self.handlers.get(&TypeId::of::<E>()).map_or(0, Vec::len)
    }

    pub fn published(&self) -> usize {
        self.published.get()
    }
}
```

Full tested solution: [`src/solutions/p0925_typed_event_bus.rs`](../src/solutions/p0925_typed_event_bus.rs)

</details>
