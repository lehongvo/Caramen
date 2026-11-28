# Problem 0446: Retry an Operation

- **Difficulty:** Medium
- **Topics:** generic `F: FnMut(u32) -> Result<T, E>`, retry loops, collecting failures, predicate closures `P: Fn(&E) -> bool`, `assert!` for preconditions
- **Source:** Original · [std `FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) · [The Rust Book 9.3: Guidelines for error handling](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling)

## Problem

Network calls and other flaky operations are often retried a few times before giving up. A
generic helper takes the operation as a closure and decides when to call it again.

Implement:

- `retry<T, E, F>(max_attempts: u32, op: F) -> Result<T, Vec<E>>` where
  `F: FnMut(u32) -> Result<T, E>`. Call `op(1)`, `op(2)`, ... (the argument is the attempt
  number) until it returns `Ok`, and return that value. If all `max_attempts` attempts fail,
  return **all** the errors, in order. With `max_attempts == 0`, `op` is never called and the
  result is `Err(vec![])`.
- `retry_if<T, E, F, P>(max_attempts: u32, op: F, should_retry: P) -> Result<T, E>` where
  `F: FnMut(u32) -> Result<T, E>` and `P: Fn(&E) -> bool`. Like `retry`, but stop immediately
  (returning that error) when `should_retry(&err)` is `false`; after the last attempt return the
  last error. `max_attempts` must be at least 1: panic with the message
  `"max_attempts must be at least 1"` otherwise.

## Examples

```text
retry(3, |n| if n == 2 { Ok("ok") } else { Err(n) })      -> Ok("ok")      (op called twice)
retry(3, |n| Err::<(), _>(format!("fail {n}")))           -> Err(["fail 1", "fail 2", "fail 3"])
retry(0, |_| Ok::<i32, String>(1))                        -> Err([])
retry_if(5, |n| Err::<(), _>(n), |e| *e < 3)              -> Err(3)        (3 is not retryable)
retry_if(2, |n| Err::<(), _>(n), |_| true)                -> Err(2)        (attempts exhausted)
retry_if(0, ...)                                          -> panics "max_attempts must be at least 1"
```

## Constraints

- `max_attempts <= 10^4`.

## Hints

1. `for attempt in 1..=max_attempts { match op(attempt) { ... } }`.
2. `op` changes its own state (it may count calls), so the parameter must be `mut op: F`.
3. `assert!(condition, "message")` panics with the message when the condition is false.

## Starter code

Write your code in [`src/practice/p0446_retry_with_closure.rs`](../src/practice/p0446_retry_with_closure.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0446
```

```rust
pub fn retry<T, E, F>(max_attempts: u32, mut op: F) -> Result<T, Vec<E>>
where
    F: FnMut(u32) -> Result<T, E>,
{
    todo!("Write your solution here")
}

pub fn retry_if<T, E, F, P>(max_attempts: u32, mut op: F, should_retry: P) -> Result<T, E>
where
    F: FnMut(u32) -> Result<T, E>,
    P: Fn(&E) -> bool,
{
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
    fn retry_succeeds_after_failures() {
        let mut calls = 0;
        let result = retry(3, |n| {
            calls += 1;
            if n == 2 { Ok("ok") } else { Err(n) }
        });
        assert_eq!(result, Ok("ok"));
        assert_eq!(calls, 2);
    }

    #[test]
    fn retry_collects_every_error() {
        assert_eq!(
            retry(3, |n| Err::<(), _>(format!("fail {n}"))),
            Err(vec![
                "fail 1".to_string(),
                "fail 2".to_string(),
                "fail 3".to_string()
            ])
        );
        assert_eq!(retry(0, |_| Ok::<i32, String>(1)), Err(vec![]));
        assert_eq!(retry(1, |_| Ok::<i32, String>(1)), Ok(1));
    }

    #[test]
    fn retry_with_a_stateful_flaky_operation() {
        let mut outage_left = 4;
        let result: Result<u32, Vec<&str>> = retry(10, |attempt| {
            if outage_left > 0 {
                outage_left -= 1;
                Err("timeout")
            } else {
                Ok(attempt)
            }
        });
        assert_eq!(result, Ok(5));
    }

    #[test]
    fn retry_if_stops_on_permanent_errors() {
        // `Err::<(), u32>` is the `Err` variant used as a function `u32 -> Result<(), u32>`.
        assert_eq!(retry_if(5, Err::<(), u32>, |e| *e < 3), Err(3));
        assert_eq!(retry_if(2, Err::<(), u32>, |_| true), Err(2));
        assert_eq!(retry_if(1, Err::<(), u32>, |_| true), Err(1));
        let mut calls = 0;
        let result = retry_if(
            10,
            |n| {
                calls += 1;
                if n < 4 { Err("busy") } else { Ok(n * 100) }
            },
            |e| *e == "busy",
        );
        assert_eq!(result, Ok(400));
        assert_eq!(calls, 4);
    }

    #[test]
    #[should_panic(expected = "max_attempts must be at least 1")]
    fn retry_if_rejects_zero_attempts() {
        let _ = retry_if(0, |_| Ok::<i32, i32>(1), |_| true);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a plain loop over the attempt numbers; the closure decides success or failure, the
helper decides what to do next. `retry` collects every error so the caller can see the full
history; `retry_if` lets the caller distinguish permanent errors from temporary ones.

**Rust concepts:**
- The bound `FnMut` accepts closures that keep state between attempts (counters, simulated
  outages...); `Fn` would be too strict, `FnOnce` could not be called twice.
- Four generic parameters (`T`, `E`, `F`, `P`) keep the helper independent of what is retried.
- Calling `retry_if` with zero attempts is a bug in the caller (there would be no error to
  return), so it panics with `assert!` instead of inventing a value.

```rust
pub fn retry<T, E, F>(max_attempts: u32, mut op: F) -> Result<T, Vec<E>>
where
    F: FnMut(u32) -> Result<T, E>,
{
    let mut errors = Vec::new();
    for attempt in 1..=max_attempts {
        match op(attempt) {
            Ok(value) => return Ok(value),
            Err(e) => errors.push(e),
        }
    }
    Err(errors)
}

pub fn retry_if<T, E, F, P>(max_attempts: u32, mut op: F, should_retry: P) -> Result<T, E>
where
    F: FnMut(u32) -> Result<T, E>,
    P: Fn(&E) -> bool,
{
    assert!(max_attempts >= 1, "max_attempts must be at least 1");
    let mut attempt = 1;
    loop {
        match op(attempt) {
            Ok(value) => return Ok(value),
            Err(e) if attempt == max_attempts || !should_retry(&e) => return Err(e),
            Err(_) => attempt += 1,
        }
    }
}
```

Full tested solution: [`src/solutions/p0446_retry_with_closure.rs`](../src/solutions/p0446_retry_with_closure.rs)

</details>
