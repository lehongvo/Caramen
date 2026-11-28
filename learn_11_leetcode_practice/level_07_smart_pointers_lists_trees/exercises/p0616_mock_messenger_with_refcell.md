# Problem 0616: Mock Messenger with RefCell

- **Difficulty:** Easy
- **Topics:** `RefCell<Vec<String>>`, `borrow_mut` behind `&self`, test doubles, traits with `&self` methods
- **Source:** Original · [The Rust Book 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) · [`std::cell::RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)

## Problem

A library tracks how close a user is to a quota and sends warnings through a `Messenger`:

```text
pub trait Messenger {
    fn send(&self, msg: &str);
}
```

`send` takes `&self`, because sending an e-mail does not change the messenger. That is a
problem for a **test double** that wants to *record* every message: recording means mutating.

1. Implement `LimitTracker::set_value(&mut self, value: usize)`. It stores `value` and sends
   at most one message, chosen by how much of `max` is used:
   - `value >= max`: `"Error: You are over your quota!"`
   - else `value >= 90%` of `max`: `"Urgent warning: You've used up over 90% of your quota!"`
   - else `value >= 75%` of `max`: `"Warning: You've used up over 75% of your quota!"`
   - otherwise nothing.

   Compare with integers, not floats (for example `value * 4 >= max * 3` for 75%).
2. Implement `MockMessenger` (`new`, `sent(&self) -> Vec<String>`) and `Messenger` for it, so
   that every `send` call is recorded in order, even though `send` only gets `&self`.

## Examples

```text
let mock = MockMessenger::new();
let mut tracker = LimitTracker::new(&mock, 100);
tracker.set_value(50);     // nothing
tracker.set_value(80);     // "Warning: You've used up over 75% of your quota!"
tracker.set_value(95);     // "Urgent warning: ..."
tracker.set_value(100);    // "Error: You are over your quota!"
mock.sent().len() == 3
```

## Constraints

- `1 <= max <= 10^9`, `0 <= value <= 10^9`

## Hints

1. Give `MockMessenger` a field `sent_messages: RefCell<Vec<String>>`.
2. In `send`, `self.sent_messages.borrow_mut().push(...)`: `borrow_mut` works through `&self`
   and checks the borrowing rules at run time instead of compile time.
3. In `sent`, `self.sent_messages.borrow().clone()` returns a copy of the list.

## Starter code

Write your code in [`src/practice/p0616_mock_messenger_with_refcell.rs`](../src/practice/p0616_mock_messenger_with_refcell.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0616
```

```rust
use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    /// Given: creates a tracker with the current value `0`.
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    /// Given: the value stored by the last `set_value` call.
    pub fn value(&self) -> usize {
        self.value
    }

    pub fn set_value(&mut self, value: usize) {
        todo!("Write your solution here")
    }
}

#[derive(Default)]
pub struct MockMessenger {}

impl MockMessenger {
    pub fn new() -> MockMessenger {
        todo!("Write your solution here")
    }

    pub fn sent(&self) -> Vec<String> {
        todo!("Write your solution here")
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
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
    fn records_one_message_per_threshold() {
        let mock = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock, 100);
        tracker.set_value(50);
        tracker.set_value(80);
        tracker.set_value(95);
        tracker.set_value(100);
        assert_eq!(tracker.value(), 100);
        assert_eq!(
            mock.sent(),
            vec![
                "Warning: You've used up over 75% of your quota!",
                "Urgent warning: You've used up over 90% of your quota!",
                "Error: You are over your quota!",
            ]
        );
    }

    #[test]
    fn below_threshold_sends_nothing() {
        let mock = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock, 100);
        tracker.set_value(0);
        tracker.set_value(74);
        assert!(mock.sent().is_empty());
    }

    #[test]
    fn exact_boundaries() {
        let mock = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock, 20);
        tracker.set_value(15); // exactly 75%
        tracker.set_value(18); // exactly 90%
        tracker.set_value(25); // over the quota
        let sent = mock.sent();
        assert_eq!(sent.len(), 3);
        assert!(sent[0].starts_with("Warning"));
        assert!(sent[1].starts_with("Urgent"));
        assert!(sent[2].starts_with("Error"));
    }

    #[test]
    fn mock_records_direct_sends_in_order() {
        let mock = MockMessenger::new();
        mock.send("a");
        mock.send("b");
        assert_eq!(mock.sent(), vec!["a", "b"]);
    }

    #[test]
    fn huge_values_do_not_overflow() {
        let mock = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock, 1_000_000_000);
        tracker.set_value(900_000_000);
        assert_eq!(
            mock.sent(),
            vec!["Urgent warning: You've used up over 90% of your quota!"]
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the tracker checks the thresholds from the highest down and sends one message. The
mock wraps its list in a `RefCell`, so `send(&self)` can push into it; the `RefMut` guard is a
temporary that is released at the end of the statement.

**Complexity:** `O(1)` per `set_value` and `send` (amortized); `sent` copies the list.

**Rust concepts:**
- `RefCell<T>` moves the "one writer or many readers" check from compile time to run time.
- This is the Book's motivating example: the trait's signature (`&self`) cannot change, but the
  implementation still needs to mutate.
- The thresholds use `u64` arithmetic so `value * 10` cannot overflow for the allowed inputs.

```rust
use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    /// Given: creates a tracker with the current value `0`.
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    /// Given: the value stored by the last `set_value` call.
    pub fn value(&self) -> usize {
        self.value
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let (value, max) = (value as u64, self.max as u64);
        if value >= max {
            self.messenger.send("Error: You are over your quota!");
        } else if value * 10 >= max * 9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if value * 4 >= max * 3 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[derive(Default)]
pub struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    pub fn new() -> MockMessenger {
        MockMessenger::default()
    }

    pub fn sent(&self) -> Vec<String> {
        self.sent_messages.borrow().clone()
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        self.sent_messages.borrow_mut().push(msg.to_string());
    }
}
```

Full tested solution: [`src/solutions/p0616_mock_messenger_with_refcell.rs`](../src/solutions/p0616_mock_messenger_with_refcell.rs)

</details>
