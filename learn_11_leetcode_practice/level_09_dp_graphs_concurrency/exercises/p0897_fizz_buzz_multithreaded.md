# Problem 0897: Fizz Buzz Multithreaded

- **Difficulty:** Medium
- **Topics:** four threads sharing one counter, each waiting for "its" kind of number, clean
  termination of every thread when the counter passes `n`
- **Source:** Adapted from [LeetCode 1195. Fizz Buzz Multithreaded](https://leetcode.com/problems/fizz-buzz-multithreaded/)

## Problem

LeetCode has no Rust template for this problem, so the API is defined here.

A `FizzBuzz` is created with a number `n` and shared by four threads, each calling one method
once. Together they must output, for `i = 1, 2, ..., n` in order:

- `"fizzbuzz"` (by the `fizzbuzz` thread) if `i` is divisible by 3 and 5;
- `"fizz"` (by the `fizz` thread) if `i` is divisible by 3 only;
- `"buzz"` (by the `buzz` thread) if `i` is divisible by 5 only;
- the number `i` itself (by the `number` thread) otherwise.

Every method returns once all `n` values are done; no thread may be left waiting forever.

```text
pub struct FizzBuzz { /* your fields */ }
impl FizzBuzz {
    pub fn new(n: u32) -> Self;
    pub fn fizz(&self, print_fizz: impl Fn());
    pub fn buzz(&self, print_buzz: impl Fn());
    pub fn fizzbuzz(&self, print_fizzbuzz: impl Fn());
    pub fn number(&self, print_number: impl Fn(u32));
}
```

## Examples

```text
n = 5  -> 1, 2, fizz, 4, buzz
n = 15 -> 1, 2, fizz, 4, buzz, fizz, 7, 8, fizz, buzz, 11, fizz, 13, 14, fizzbuzz
```

## Constraints

- `1 <= n <= 1000`

## Hints

1. Shared state: `current`, the next value to handle (starting at `1`), in a `Mutex`.
2. Each thread waits until `current > n` (then it returns) or `current` is a value it handles;
   then it prints, increments `current` and calls `notify_all`.
3. Write the loop once as a helper taking a predicate `Fn(u32) -> bool` and a printing closure.

## Starter code

Write your code in [`src/practice/p0897_fizz_buzz_multithreaded.rs`](../src/practice/p0897_fizz_buzz_multithreaded.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0897
```

```rust
use std::sync::{Condvar, Mutex, PoisonError};

pub struct FizzBuzz {}

impl FizzBuzz {
    pub fn new(n: u32) -> Self {
        todo!("Write your solution here")
    }

    pub fn fizz(&self, print_fizz: impl Fn()) {
        todo!("Write your solution here")
    }

    pub fn buzz(&self, print_buzz: impl Fn()) {
        todo!("Write your solution here")
    }

    pub fn fizzbuzz(&self, print_fizzbuzz: impl Fn()) {
        todo!("Write your solution here")
    }

    pub fn number(&self, print_number: impl Fn(u32)) {
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
    use std::thread;

    fn run(n: u32) -> Vec<String> {
        let fb = FizzBuzz::new(n);
        let out = Mutex::new(Vec::new());
        let push = |s: String| out.lock().unwrap().push(s);
        thread::scope(|s| {
            s.spawn(|| fb.number(|i| push(i.to_string())));
            s.spawn(|| fb.fizzbuzz(|| push("fizzbuzz".to_string())));
            s.spawn(|| fb.buzz(|| push("buzz".to_string())));
            s.spawn(|| fb.fizz(|| push("fizz".to_string())));
        });
        out.into_inner().unwrap()
    }

    fn expected(n: u32) -> Vec<String> {
        (1..=n)
            .map(|i| match (i % 3, i % 5) {
                (0, 0) => "fizzbuzz".to_string(),
                (0, _) => "fizz".to_string(),
                (_, 0) => "buzz".to_string(),
                _ => i.to_string(),
            })
            .collect()
    }

    #[test]
    fn example_n_5() {
        assert_eq!(run(5), vec!["1", "2", "fizz", "4", "buzz"]);
    }

    #[test]
    fn example_n_15() {
        assert_eq!(run(15), expected(15));
        assert_eq!(run(15)[14], "fizzbuzz");
    }

    #[test]
    fn n_smaller_than_any_special_value() {
        assert_eq!(run(1), vec!["1"]);
        assert_eq!(run(2), vec!["1", "2"]);
    }

    #[test]
    fn long_run_and_repetitions() {
        assert_eq!(run(1000), expected(1000));
        for _ in 0..30 {
            assert_eq!(run(30), expected(30));
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a helper `serve(accepts, print)` loops: lock `current`, `wait_while(c <= n &&
!accepts(c))`, return if `c > n`, otherwise print `c`, increment, `notify_all`. The four
public methods call it with the four classification predicates. When the last value is handled,
the final `notify_all` wakes every other thread, which sees `current > n` and returns.

**Complexity:** `O(n)` prints; every increment wakes the four threads, so each thread is woken `O(n)`
times. `O(1)` extra space.

**Rust concepts:**
- Closures as strategy parameters: `impl Fn(u32) -> bool` for "which values are mine" and
  `impl Fn(u32)` for "how to print"; `fizz` adapts `print_fizz: impl Fn()` with `|_| print_fizz()`.
- The termination condition is part of the wait predicate; forgetting it is the classic way to
  leave a thread blocked forever.

```rust
use std::sync::{Condvar, Mutex, PoisonError};

pub struct FizzBuzz {
    n: u32,
    current: Mutex<u32>,
    changed: Condvar,
}

impl FizzBuzz {
    pub fn new(n: u32) -> Self {
        Self {
            n,
            current: Mutex::new(1),
            changed: Condvar::new(),
        }
    }

    fn serve(&self, accepts: impl Fn(u32) -> bool, print: impl Fn(u32)) {
        loop {
            let guard = self.current.lock().unwrap_or_else(PoisonError::into_inner);
            let mut current = self
                .changed
                .wait_while(guard, |c| *c <= self.n && !accepts(*c))
                .unwrap_or_else(PoisonError::into_inner);
            if *current > self.n {
                return;
            }
            print(*current);
            *current += 1;
            self.changed.notify_all();
        }
    }

    pub fn fizz(&self, print_fizz: impl Fn()) {
        self.serve(|i| i % 3 == 0 && i % 5 != 0, |_| print_fizz());
    }

    pub fn buzz(&self, print_buzz: impl Fn()) {
        self.serve(|i| i % 5 == 0 && i % 3 != 0, |_| print_buzz());
    }

    pub fn fizzbuzz(&self, print_fizzbuzz: impl Fn()) {
        self.serve(|i| i % 15 == 0, |_| print_fizzbuzz());
    }

    pub fn number(&self, print_number: impl Fn(u32)) {
        self.serve(|i| i % 3 != 0 && i % 5 != 0, print_number);
    }
}
```

Full tested solution: [`src/solutions/p0897_fizz_buzz_multithreaded.rs`](../src/solutions/p0897_fizz_buzz_multithreaded.rs)

</details>
