# Problem 0608: Scope Guard

- **Difficulty:** Easy
- **Topics:** `Drop` running cleanup code, `FnOnce` stored in `Option`, `Option::take`, panics and unwinding
- **Source:** Original · [The Rust Book 15.3](https://doc.rust-lang.org/book/ch15-03-drop.html) · [`std::ops::Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html)

## Problem

A *scope guard* runs a cleanup action when it goes out of scope, however the scope is left:
normally, with an early `return`, or while a panic unwinds the stack. It is the Rust version of
`finally` or `defer`.

```text
pub struct ScopeGuard<F: FnOnce()> {
    action: Option<F>,
}
```

Implement:

- `ScopeGuard::new(action: F) -> ScopeGuard<F>`: create an armed guard;
- `impl Drop for ScopeGuard<F>`: run the action **exactly once**, if the guard is still armed;
- `dismiss(self)`: disarm and consume the guard; its action must never run.

## Examples

```text
let hits = Cell::new(0);
{
    let _guard = ScopeGuard::new(|| hits.set(hits.get() + 1));
    // hits.get() == 0 here
}
// hits.get() == 1: the action ran when `_guard` went out of scope

let guard = ScopeGuard::new(|| hits.set(100));
guard.dismiss();          // hits.get() is still 1
```

## Constraints

- No special constraints.

## Hints

1. An `FnOnce` closure is consumed when called, so you need to *own* it to call it. In `drop`
   you only have `&mut self`; `self.action.take()` moves the closure out of the `Option` and
   leaves `None` behind.
2. To dismiss, clear the `Option` before `self` is dropped at the end of `dismiss`.
3. Drop runs while a panic unwinds, which is exactly what makes guards useful.

## Starter code

Write your code in [`src/practice/p0608_scope_guard.rs`](../src/practice/p0608_scope_guard.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0608
```

```rust
pub struct ScopeGuard<F: FnOnce()> {
    action: Option<F>,
}

impl<F: FnOnce()> ScopeGuard<F> {
    pub fn new(action: F) -> Self {
        todo!("Write your solution here")
    }

    pub fn dismiss(mut self) {
        todo!("Write your solution here")
    }
}

impl<F: FnOnce()> Drop for ScopeGuard<F> {
    fn drop(&mut self) {
        // TODO: run the action if the guard is still armed (never panic here yourself).
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
    use std::cell::{Cell, RefCell};
    use std::panic::{self, AssertUnwindSafe};

    #[test]
    fn runs_when_the_scope_ends() {
        let hits = Cell::new(0);
        {
            let _guard = ScopeGuard::new(|| hits.set(hits.get() + 1));
            assert_eq!(hits.get(), 0);
        }
        assert_eq!(hits.get(), 1);
    }

    #[test]
    fn runs_early_with_mem_drop() {
        let hits = Cell::new(0);
        let guard = ScopeGuard::new(|| hits.set(hits.get() + 1));
        drop(guard);
        assert_eq!(hits.get(), 1);
    }

    #[test]
    fn dismissed_guard_never_runs() {
        let hits = Cell::new(0);
        let guard = ScopeGuard::new(|| hits.set(100));
        guard.dismiss();
        assert_eq!(hits.get(), 0);
        let _armed = ScopeGuard::new(|| hits.set(hits.get() + 1));
        assert_eq!(hits.get(), 0);
    }

    #[test]
    fn runs_while_a_panic_unwinds() {
        let cleaned = Cell::new(false);
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = ScopeGuard::new(|| cleaned.set(true));
            panic!("boom");
        }));
        assert!(result.is_err());
        assert!(cleaned.get());
    }

    #[test]
    fn several_guards_run_in_reverse_order() {
        let order = RefCell::new(Vec::new());
        {
            let _first = ScopeGuard::new(|| order.borrow_mut().push("first"));
            let _second = ScopeGuard::new(|| order.borrow_mut().push("second"));
            let _third = ScopeGuard::new(|| order.borrow_mut().push("third"));
        }
        assert_eq!(*order.borrow(), ["third", "second", "first"]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** keep the action as `Some(f)`. `drop` takes it out and calls it; `dismiss` sets it to
`None`, so when `self` is dropped at the end of `dismiss`, the destructor finds nothing to run.

**Complexity:** `O(1)` besides the cost of the action itself.

**Rust concepts:**
- `Option<F>` + `take()` is the standard way to call an `FnOnce` (or move any field out) from a
  method that only receives `&mut self`.
- Destructors run during unwinding, so the cleanup also happens when the scope panics.
- Several guards in one scope run in reverse order of creation, like any other locals.
- `std::mem::forget(guard)` would also skip the action, but it leaks the closure; clearing the
  `Option` is cleaner.

```rust
pub struct ScopeGuard<F: FnOnce()> {
    action: Option<F>,
}

impl<F: FnOnce()> ScopeGuard<F> {
    pub fn new(action: F) -> Self {
        ScopeGuard {
            action: Some(action),
        }
    }

    pub fn dismiss(mut self) {
        self.action = None;
    }
}

impl<F: FnOnce()> Drop for ScopeGuard<F> {
    fn drop(&mut self) {
        if let Some(action) = self.action.take() {
            action();
        }
    }
}
```

Full tested solution: [`src/solutions/p0608_scope_guard.rs`](../src/solutions/p0608_scope_guard.rs)

</details>
