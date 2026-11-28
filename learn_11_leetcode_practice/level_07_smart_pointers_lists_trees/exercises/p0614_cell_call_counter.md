# Problem 0614: Call Counter with Cell

- **Difficulty:** Easy
- **Topics:** `Cell<T>`, interior mutability through `&self`, `Cell::get`/`set`/`replace`, `Cell` in a shared slice
- **Source:** Original · [The Rust Book 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) · [`std::cell::Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html)

## Problem

Sometimes a method that is logically read-only still needs to update some bookkeeping, such as
a statistics counter. With `&self` you cannot assign to a normal field, but you can update a
`Cell<T>`: it lets you **replace** a `Copy` value through a shared reference. `Cell` never hands
out references to its content, so there is nothing to check at run time and it cannot panic.

Implement `Greeter`:

- `Greeter::new(name: &str) -> Greeter`;
- `greet(&self) -> String`: return `"Hello, <name>!"` and count the call;
- `calls(&self) -> u32`: how many times `greet` was called since creation or the last reset;
- `reset(&self) -> u32`: set the count back to `0` and return the count it had.

Then implement the free function `record_visits(counters: &[Cell<u32>], visits: &[usize])`,
which adds `1` to `counters[i]` for every `i` in `visits`. Note the **shared** slice: no `&mut`.

## Examples

```text
let g = Greeter::new("Ana");          // not `mut`
g.greet()  -> "Hello, Ana!"
g.greet()  -> "Hello, Ana!"
g.calls()  -> 2
g.reset()  -> 2
g.calls()  -> 0

counters = [Cell(0), Cell(0), Cell(0)], visits = [0, 2, 2, 2]
after record_visits: [1, 0, 3]
```

## Constraints

- Every index in `visits` is a valid index of `counters`.

## Hints

1. Store `calls: Cell<u32>` in the struct. `cell.set(cell.get() + 1)` increments it.
2. `Cell::replace(new)` stores `new` and returns the old value in one step.

## Starter code

Write your code in [`src/practice/p0614_cell_call_counter.rs`](../src/practice/p0614_cell_call_counter.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0614
```

```rust
use std::cell::Cell;

pub struct Greeter {}

impl Greeter {
    pub fn new(name: &str) -> Greeter {
        todo!("Write your solution here")
    }

    pub fn greet(&self) -> String {
        todo!("Write your solution here")
    }

    pub fn calls(&self) -> u32 {
        todo!("Write your solution here")
    }

    pub fn reset(&self) -> u32 {
        todo!("Write your solution here")
    }
}

pub fn record_visits(counters: &[Cell<u32>], visits: &[usize]) {
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
    fn greet_counts_calls_through_shared_reference() {
        let g = Greeter::new("Ana");
        assert_eq!(g.greet(), "Hello, Ana!");
        assert_eq!(g.greet(), "Hello, Ana!");
        assert_eq!(g.calls(), 2);
    }

    #[test]
    fn reset_returns_previous_count() {
        let g = Greeter::new("Bo");
        assert_eq!(g.calls(), 0);
        g.greet();
        g.greet();
        g.greet();
        assert_eq!(g.reset(), 3);
        assert_eq!(g.calls(), 0);
        assert_eq!(g.reset(), 0);
    }

    #[test]
    fn several_shared_references_update_one_counter() {
        let g = Greeter::new("Cy");
        let (first, second) = (&g, &g);
        first.greet();
        second.greet();
        assert_eq!(g.calls(), 2);
    }

    #[test]
    fn record_visits_example() {
        let counters = [Cell::new(0), Cell::new(0), Cell::new(0)];
        record_visits(&counters, &[0, 2, 2, 2]);
        let values: Vec<u32> = counters.iter().map(Cell::get).collect();
        assert_eq!(values, vec![1, 0, 3]);
    }

    #[test]
    fn record_visits_many() {
        let counters: Vec<Cell<u32>> = (0..10).map(|_| Cell::new(0)).collect();
        let visits: Vec<usize> = (0..10_000).map(|i| i % 10).collect();
        record_visits(&counters, &visits);
        record_visits(&counters, &[]);
        assert!(counters.iter().all(|c| c.get() == 1_000));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** keep the name as a `String` and the counter as a `Cell<u32>`. Every operation reads or
writes the counter by value.

**Complexity:** `O(1)` per method call (plus the cost of formatting the greeting);
`record_visits` is `O(visits.len())`.

**Rust concepts:**
- `Cell<T>` works for `Copy` types with `get`, and for any type with `set`, `replace`, `swap`
  and `take` (the last one needs `T: Default`).
- `Cell` is not `Sync`: it is for single-threaded interior mutability. Threads need atomics or
  a `Mutex` (level 9).
- A `&[Cell<u32>]` slice lets many parts of a program update counters that they only borrow.

```rust
use std::cell::Cell;

pub struct Greeter {
    name: String,
    calls: Cell<u32>,
}

impl Greeter {
    pub fn new(name: &str) -> Greeter {
        Greeter {
            name: name.to_string(),
            calls: Cell::new(0),
        }
    }

    pub fn greet(&self) -> String {
        self.calls.set(self.calls.get() + 1);
        format!("Hello, {}!", self.name)
    }

    pub fn calls(&self) -> u32 {
        self.calls.get()
    }

    pub fn reset(&self) -> u32 {
        self.calls.replace(0)
    }
}

pub fn record_visits(counters: &[Cell<u32>], visits: &[usize]) {
    for &index in visits {
        let counter = &counters[index];
        counter.set(counter.get() + 1);
    }
}
```

Full tested solution: [`src/solutions/p0614_cell_call_counter.rs`](../src/solutions/p0614_cell_call_counter.rs)

</details>
