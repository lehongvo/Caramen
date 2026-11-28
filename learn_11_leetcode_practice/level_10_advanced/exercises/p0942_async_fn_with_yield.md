# Problem 0942: `async fn` and `.await` on Your Executor

- **Difficulty:** Hard
- **Topics:** `async fn`, `.await`, laziness of futures, borrowing parameters in `async fn`, recursive `async fn` with `Box::pin`, counting polls
- **Source:** Original · [The Rust Book 17.1: Futures and the Async Syntax](https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html) · [Reference: Async functions](https://doc.rust-lang.org/reference/items/functions.html#async-functions)

## Problem

An `async fn` returns a future; its body only runs when that future is polled, and every
`.await` on a pending future suspends the whole chain. With the given `block_on` (which also
reports how many times it polled) and `yield_now()` (pending exactly once), write these async
functions:

1. `async fn add_slowly(a: u32, b: u32, yields: usize) -> u32`: await `yield_now()` `yields`
   times, then return `a + b`.
2. `async fn sum_slowly(values: &[u32]) -> u32`: add the values one by one with
   `add_slowly(total, value, 1)`; the future **borrows** `values`.
3. `async fn record(name: &str, log: &RefCell<Vec<String>>) -> usize`: push `"start <name>"`, yield
   once, push `"end <name>"`, return the name's length.
4. `async fn record_all(names: &[&str], log: &RefCell<Vec<String>>) -> usize`: `record` each name
   **in order** (sequentially, one after another) and return the sum of the lengths.
5. `fn factorial(n: u64) -> Pin<Box<dyn Future<Output = u64>>>`: a recursive async computation
   that yields once per level: `factorial(0)` is `1` after one yield; `factorial(n)` yields once,
   then awaits `factorial(n - 1)` and multiplies. (A directly recursive `async fn` would have an
   infinitely large state machine; boxing the recursive future breaks the cycle.)

## Examples

```text
block_on(add_slowly(2, 3, 4))           => (5, 5)        (4 pending polls + 1 final poll)
block_on(sum_slowly(&[1, 2, 3]))        => (6, 4)
let log = RefCell::new(vec![]);
let fut = record("ab", &log);           // nothing logged yet: futures are lazy
block_on(fut)                           => (2, 2), log: ["start ab", "end ab"]
block_on(factorial(5))                  => (120, 7)
```

## Constraints

- `factorial` is called with `n <= 20`.

## Hints

1. A `for` loop with `yield_now().await;` inside works in an `async fn`.
2. An `async fn` taking references returns a future that borrows them, so the arguments must
   outlive the future; that is why `sum_slowly(&values)` can be awaited directly.
3. `Box::pin(async move { ... })` creates the `Pin<Box<dyn Future<Output = u64>>>`.

## Starter code

Write your code in [`src/practice/p0942_async_fn_with_yield.rs`](../src/practice/p0942_async_fn_with_yield.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0942
```

```rust
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

pub async fn add_slowly(a: u32, b: u32, yields: usize) -> u32 {
    todo!("Write your solution here")
}

pub async fn sum_slowly(values: &[u32]) -> u32 {
    todo!("Write your solution here")
}

pub async fn record(name: &str, log: &RefCell<Vec<String>>) -> usize {
    todo!("Write your solution here")
}

pub async fn record_all(names: &[&str], log: &RefCell<Vec<String>>) -> usize {
    todo!("Write your solution here")
}

pub fn factorial(n: u64) -> Pin<Box<dyn Future<Output = u64>>> {
    Box::pin(async { todo!() })
}

// ---------------------------------------------------------------- given helpers

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Runs a future to completion; returns its output and the number of polls it needed.
pub fn block_on<F: Future>(future: F) -> (F::Output, usize) {
    let mut future = std::pin::pin!(future);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    let mut polls = 0;
    loop {
        polls += 1;
        if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
            return (output, polls);
        }
        thread::park();
    }
}

/// Pending (after waking itself) on the first poll, ready on the second.
pub struct YieldNow {
    yielded: bool,
}

pub fn yield_now() -> YieldNow {
    YieldNow { yielded: false }
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            return Poll::Ready(());
        }
        self.yielded = true;
        cx.waker().wake_by_ref();
        Poll::Pending
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
    fn yields_cost_polls() {
        assert_eq!(block_on(add_slowly(2, 3, 4)), (5, 5));
        assert_eq!(block_on(add_slowly(1, 1, 0)), (2, 1));
        assert_eq!(block_on(sum_slowly(&[1, 2, 3])), (6, 4));
        let values: Vec<u32> = (1..=100).collect();
        assert_eq!(block_on(sum_slowly(&values)), (5_050, 101));
    }

    #[test]
    fn futures_are_lazy() {
        let log = RefCell::new(Vec::new());
        let fut = record("ab", &log);
        assert!(
            log.borrow().is_empty(),
            "calling an async fn must not run it"
        );
        assert_eq!(block_on(fut), (2, 2));
        assert_eq!(*log.borrow(), ["start ab", "end ab"]);
    }

    #[test]
    fn sequential_awaits_do_not_interleave() {
        let log = RefCell::new(Vec::new());
        let (total, polls) = block_on(record_all(&["x", "yy", "zzz"], &log));
        assert_eq!((total, polls), (6, 4));
        assert_eq!(
            *log.borrow(),
            [
                "start x",
                "end x",
                "start yy",
                "end yy",
                "start zzz",
                "end zzz"
            ]
        );
    }

    #[test]
    fn recursive_async() {
        assert_eq!(block_on(factorial(0)), (1, 2));
        assert_eq!(block_on(factorial(5)), (120, 7));
        assert_eq!(block_on(factorial(20)).0, 2_432_902_008_176_640_000);
    }

    #[test]
    fn async_blocks_compose_with_async_fns() {
        let log = RefCell::new(Vec::new());
        let (out, polls) = block_on(async {
            let a = add_slowly(10, 20, 2).await;
            let b = record("q", &log).await;
            a + b as u32
        });
        assert_eq!((out, polls), (31, 4));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each function is straight-line async code. Every `yield_now().await` costs one extra
poll of the outermost future, so the poll counts follow directly: `add_slowly(_, _, k)` takes
`k + 1` polls, `sum_slowly` of `n` values takes `n + 1`, and `factorial(n)` yields `n + 1` times
and needs `n + 2` polls.

**Rust concepts:**
- Calling an `async fn` only builds a future; nothing runs until an executor polls it (the
  "lazy" test checks this).
- `async fn f(x: &T)` desugars to `fn f<'a>(x: &'a T) -> impl Future + 'a`: the returned future
  captures the borrow.
- Recursion: the type of `factorial`'s future would contain itself; `Pin<Box<dyn Future>>` gives
  it a fixed size. (`async fn` recursion is allowed since Rust 1.77 as long as the recursive call
  is boxed.)

```rust
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

pub async fn add_slowly(a: u32, b: u32, yields: usize) -> u32 {
    for _ in 0..yields {
        yield_now().await;
    }
    a + b
}

pub async fn sum_slowly(values: &[u32]) -> u32 {
    let mut total = 0;
    for &value in values {
        total = add_slowly(total, value, 1).await;
    }
    total
}

pub async fn record(name: &str, log: &RefCell<Vec<String>>) -> usize {
    log.borrow_mut().push(format!("start {name}"));
    yield_now().await;
    log.borrow_mut().push(format!("end {name}"));
    name.len()
}

pub async fn record_all(names: &[&str], log: &RefCell<Vec<String>>) -> usize {
    let mut total = 0;
    for name in names {
        total += record(name, log).await;
    }
    total
}

pub fn factorial(n: u64) -> Pin<Box<dyn Future<Output = u64>>> {
    Box::pin(async move {
        yield_now().await;
        if n == 0 {
            1
        } else {
            n * factorial(n - 1).await
        }
    })
}

// ---------------------------------------------------------------- given helpers

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Runs a future to completion; returns its output and the number of polls it needed.
pub fn block_on<F: Future>(future: F) -> (F::Output, usize) {
    let mut future = std::pin::pin!(future);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    let mut polls = 0;
    loop {
        polls += 1;
        if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
            return (output, polls);
        }
        thread::park();
    }
}

/// Pending (after waking itself) on the first poll, ready on the second.
pub struct YieldNow {
    yielded: bool,
}

pub fn yield_now() -> YieldNow {
    YieldNow { yielded: false }
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            return Poll::Ready(());
        }
        self.yielded = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
```

Full tested solution: [`src/solutions/p0942_async_fn_with_yield.rs`](../src/solutions/p0942_async_fn_with_yield.rs)

</details>
