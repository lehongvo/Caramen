# Problem 0869: Catch Worker Panics

- **Difficulty:** Medium
- **Topics:** `JoinHandle::join` returning `Err` when a thread panics, panic payloads
  (`Box<dyn Any + Send>`), `downcast_ref`, boxed `FnOnce` jobs
- **Source:** Original · [`JoinHandle::join`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html) ·
  [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html)

## Problem

A panic in a spawned thread does not crash the whole program: it ends that thread, and the
panic shows up as an `Err` when the thread is joined. Use this to run untrusted jobs safely.

Implement `run_isolated(jobs: Vec<Job>) -> Vec<Result<i32, String>>`, where
`Job = Box<dyn FnOnce() -> i32 + Send + 'static>`. Run **every job on its own thread** (start
them all before joining any) and return one entry per job, in input order:

- `Ok(value)` if the job returned `value`;
- `Err(message)` if the job panicked, where `message` is the panic message when the payload is a
  string (`panic!("boom")` gives `"boom"`, `panic!("bad {}", 7)` gives `"bad 7"`), and the
  exact text `"unknown panic"` for any other payload type.

## Examples

```text
jobs = [|| 1, || panic!("boom"), || 3]
run_isolated(jobs) -> [Ok(1), Err("boom"), Ok(3)]

jobs = [|| std::panic::panic_any(42_u8)]
run_isolated(jobs) -> [Err("unknown panic")]
```

## Constraints

- `0 <= jobs.len() <= 100`
- (The panic messages also appear on the test output; that is expected.)

## Hints

1. `handle.join()` has type `Result<i32, Box<dyn Any + Send + 'static>>`. The `Err` value is the
   panic payload.
2. `panic!` with a plain string literal stores a `&'static str`; with formatting arguments it
   stores a `String`. Try `payload.downcast_ref::<&str>()` and then
   `payload.downcast_ref::<String>()`.
3. `Result::map_err` converts the payload into your message.

## Starter code

Write your code in [`src/practice/p0869_catch_worker_panics.rs`](../src/practice/p0869_catch_worker_panics.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0869
```

```rust
use std::any::Any;
use std::thread;

/// A job that can be sent to another thread and run once.
pub type Job = Box<dyn FnOnce() -> i32 + Send + 'static>;

pub fn run_isolated(jobs: Vec<Job>) -> Vec<Result<i32, String>> {
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
    fn example_one_panic_among_three() {
        let jobs: Vec<Job> = vec![
            Box::new(|| 1),
            Box::new(|| -> i32 { panic!("boom") }),
            Box::new(|| 3),
        ];
        assert_eq!(
            run_isolated(jobs),
            vec![Ok(1), Err("boom".to_string()), Ok(3)]
        );
    }

    #[test]
    fn example_non_string_payload() {
        let jobs: Vec<Job> = vec![Box::new(|| -> i32 { std::panic::panic_any(42_u8) })];
        assert_eq!(run_isolated(jobs), vec![Err("unknown panic".to_string())]);
    }

    #[test]
    fn formatted_message_is_a_string_payload() {
        let n = 7;
        let jobs: Vec<Job> = vec![Box::new(move || -> i32 { panic!("bad input: {n}") })];
        assert_eq!(run_isolated(jobs), vec![Err("bad input: 7".to_string())]);
    }

    #[test]
    fn empty_job_list() {
        assert!(run_isolated(Vec::new()).is_empty());
    }

    #[test]
    fn runtime_errors_are_caught_too() {
        let jobs: Vec<Job> = vec![
            Box::new(|| {
                let v: Vec<i32> = Vec::new();
                v[0]
            }),
            Box::new(|| (1..=10).sum::<i32>()),
        ];
        let got = run_isolated(jobs);
        let message = got[0].clone().unwrap_err();
        assert!(message.starts_with("index out of bounds"), "{message}");
        assert_eq!(got[1], Ok(55));
    }

    #[test]
    fn many_jobs_keep_their_order() {
        let jobs: Vec<Job> = (0..50)
            .map(|i| -> Job {
                Box::new(move || {
                    if i % 5 == 0 {
                        panic!("job {i} failed");
                    }
                    i * 2
                })
            })
            .collect();
        let got = run_isolated(jobs);
        for (i, r) in got.iter().enumerate() {
            let i = i as i32;
            if i % 5 == 0 {
                assert_eq!(r, &Err(format!("job {i} failed")));
            } else {
                assert_eq!(r, &Ok(i * 2));
            }
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** spawn one thread per job and collect the handles. Join each handle and map its error
(the payload) to a message with a helper that tries the two string types and falls back to
`"unknown panic"`.

**Rust concepts:**
- A boxed closure `Box<dyn FnOnce() -> i32 + Send>` can be called directly (`job()`), because
  `Box<dyn FnOnce>` implements `FnOnce` itself.
- `dyn Any` supports runtime type checks: `downcast_ref::<T>()` returns `Some(&T)` only if the
  value really is a `T`.
- Catching panics this way is for isolation (like a worker pool that survives a bad job), not
  for normal error handling; use `Result` for expected errors.

```rust
use std::any::Any;
use std::thread;

/// A job that can be sent to another thread and run once.
pub type Job = Box<dyn FnOnce() -> i32 + Send + 'static>;

/// Extracts a readable message from a panic payload.
fn panic_message(payload: Box<dyn Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic".to_string()
    }
}

pub fn run_isolated(jobs: Vec<Job>) -> Vec<Result<i32, String>> {
    let handles: Vec<thread::JoinHandle<i32>> = jobs.into_iter().map(thread::spawn).collect();
    handles
        .into_iter()
        .map(|h| h.join().map_err(panic_message))
        .collect()
}
```

Full tested solution: [`src/solutions/p0869_catch_worker_panics.rs`](../src/solutions/p0869_catch_worker_panics.rs)

</details>
