# Problem 0615: Lazy Cached Value with Cell

- **Difficulty:** Easy
- **Topics:** `Cell<Option<T>>`, memoization behind `&self`, `Cell::take`, `Cell::replace`
- **Source:** Original · [The Rust Book 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) · [`Cell::take`](https://doc.rust-lang.org/std/cell/struct.Cell.html#method.take)

## Problem

`CollatzSteps` answers one question about a number `n >= 1`: how many steps does the Collatz
process need to reach `1`? (One step turns an even `n` into `n / 2` and an odd `n` into
`3 * n + 1`.) Pretend the computation is expensive: it must run **only when needed**, and its
result is cached. All methods take `&self`, so the cache lives in `Cell`s.

Implement:

- `CollatzSteps::new(n: u64) -> CollatzSteps`: nothing is computed yet;
- `value(&self) -> u64`: the number of steps; compute it on the first call, then reuse the cache;
- `computations(&self) -> u32`: how many times the expensive computation actually ran;
- `invalidate(&self) -> Option<u64>`: drop the cached value (if any) and return it;
- `set_input(&self, n: u64) -> u64`: change the input and return the previous one. The cache is
  cleared only if the input really changed.

## Examples

```text
let c = CollatzSteps::new(6);    // 6, 3, 10, 5, 16, 8, 4, 2, 1
c.computations() -> 0
c.value()        -> 8
c.value()        -> 8
c.computations() -> 1            // the second call used the cache
c.set_input(6)   -> 6            // same input: cache kept
c.set_input(27)  -> 6            // new input: cache cleared
c.value()        -> 111
c.computations() -> 2
c.invalidate()   -> Some(111)
c.invalidate()   -> None
```

## Constraints

- `1 <= n <= 10^6`

## Hints

1. `Option<u64>` is `Copy`, so a `Cell<Option<u64>>` supports `get()`: `None` means "not
   computed yet".
2. `Cell::take()` returns the content and leaves `Default::default()` (here `None`) behind.

## Starter code

Write your code in [`src/practice/p0615_lazy_value_with_cell.rs`](../src/practice/p0615_lazy_value_with_cell.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0615
```

```rust
use std::cell::Cell;

pub struct CollatzSteps {}

impl CollatzSteps {
    pub fn new(n: u64) -> CollatzSteps {
        todo!("Write your solution here")
    }

    pub fn value(&self) -> u64 {
        todo!("Write your solution here")
    }

    pub fn computations(&self) -> u32 {
        todo!("Write your solution here")
    }

    pub fn invalidate(&self) -> Option<u64> {
        todo!("Write your solution here")
    }

    pub fn set_input(&self, n: u64) -> u64 {
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
    fn computes_once_and_caches() {
        let c = CollatzSteps::new(6);
        assert_eq!(c.computations(), 0);
        assert_eq!(c.value(), 8);
        assert_eq!(c.value(), 8);
        assert_eq!(c.computations(), 1);
    }

    #[test]
    fn set_input_clears_cache_only_on_change() {
        let c = CollatzSteps::new(6);
        c.value();
        assert_eq!(c.set_input(6), 6);
        assert_eq!(c.value(), 8);
        assert_eq!(c.computations(), 1);
        assert_eq!(c.set_input(27), 6);
        assert_eq!(c.value(), 111);
        assert_eq!(c.computations(), 2);
    }

    #[test]
    fn invalidate_returns_the_cached_value() {
        let c = CollatzSteps::new(27);
        assert_eq!(c.invalidate(), None);
        c.value();
        assert_eq!(c.invalidate(), Some(111));
        assert_eq!(c.invalidate(), None);
        assert_eq!(c.value(), 111);
        assert_eq!(c.computations(), 2);
    }

    #[test]
    fn one_needs_zero_steps() {
        let c = CollatzSteps::new(1);
        assert_eq!(c.value(), 0);
        assert_eq!(c.computations(), 1);
    }

    #[test]
    fn large_input() {
        // 837799 has the longest Collatz sequence below one million.
        let c = CollatzSteps::new(837_799);
        assert_eq!(c.value(), 524);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** three cells: the input, the cached answer and the computation counter. `value`
returns the cached answer if there is one; otherwise it runs the loop, stores the result and
bumps the counter.

**Complexity:** the first `value` call costs the length of the Collatz sequence; later calls
are `O(1)`.

**Rust concepts:**
- Interior mutability lets a cache stay an implementation detail: callers see a read-only API.
- `Cell::replace` returns the old value, which makes "set and report the previous value" a
  single call.
- For values that are not `Copy`, the standard library offers `std::cell::OnceCell` (stable
  since 1.70), which hands out `&T`.

```rust
use std::cell::Cell;

pub struct CollatzSteps {
    input: Cell<u64>,
    cached: Cell<Option<u64>>,
    computations: Cell<u32>,
}

fn collatz_steps(mut n: u64) -> u64 {
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    steps
}

impl CollatzSteps {
    pub fn new(n: u64) -> CollatzSteps {
        CollatzSteps {
            input: Cell::new(n),
            cached: Cell::new(None),
            computations: Cell::new(0),
        }
    }

    pub fn value(&self) -> u64 {
        if let Some(steps) = self.cached.get() {
            return steps;
        }
        let steps = collatz_steps(self.input.get());
        self.cached.set(Some(steps));
        self.computations.set(self.computations.get() + 1);
        steps
    }

    pub fn computations(&self) -> u32 {
        self.computations.get()
    }

    pub fn invalidate(&self) -> Option<u64> {
        self.cached.take()
    }

    pub fn set_input(&self, n: u64) -> u64 {
        let previous = self.input.replace(n);
        if previous != n {
            self.cached.set(None);
        }
        previous
    }
}
```

Full tested solution: [`src/solutions/p0615_lazy_value_with_cell.rs`](../src/solutions/p0615_lazy_value_with_cell.rs)

</details>
