# Problem 0538: Ordering Tasks by Priority

- **Difficulty:** Medium
- **Topics:** manual `Ord` and `PartialOrd`, `Ordering::then_with`, consistency with `Eq`, `BinaryHeap` as a max-heap, `std::cmp::Reverse`
- **Source:** Original · [std::cmp::Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html#how-can-i-implement-ord)

## Problem

A task has a `priority: u8`, a `deadline: u32` and a `name: String` (all `pub`). Define the order
"`a > b` means `a` should run **before** `b`":

1. Higher `priority` runs first.
2. On equal priority, the **earlier** `deadline` runs first.
3. On equal priority and deadline, the name that comes first **alphabetically** runs first.

Implement `Ord` (with this order) and `PartialOrd` (delegating to `Ord`) for `Task`. `PartialEq` and
`Eq` are derived; your `Ord` must agree with them (two tasks compare `Equal` exactly when all fields
are equal).

Then:

- `next_task(tasks: &[Task]) -> Option<&Task>`: the task that runs first (the maximum).
- `schedule(tasks: Vec<Task>) -> Vec<String>`: all task names in execution order, produced by
  pushing every task into a `std::collections::BinaryHeap` and popping until it is empty.
- `latest_first(tasks: Vec<Task>) -> Vec<String>`: the names sorted by **deadline descending**
  (ties: the order of `schedule`), using `sort_by_key` with `std::cmp::Reverse`.

## Examples

```text
tasks = [("email", p1, d30), ("deploy", p3, d50), ("review", p3, d20), ("backup", p3, d20)]
schedule(tasks)     => ["backup", "review", "deploy", "email"]
latest_first(tasks) => ["deploy", "email", "backup", "review"]
```

## Constraints

- `0 <= tasks.len() <= 10^5`

## Hints

1. `self.priority.cmp(&other.priority)` puts the higher priority on the "greater" side.
2. For "earlier deadline is greater", compare the other way round: `other.deadline.cmp(&self.deadline)`.
3. `BinaryHeap::pop` returns the **greatest** element, which is exactly the next task to run.

## Starter code

Write your code in [`src/practice/p0538_ordering_for_tasks.rs`](../src/practice/p0538_ordering_for_tasks.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0538
```

```rust
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub priority: u8,
    pub deadline: u32,
    pub name: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!("Write your solution here")
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!("Write your solution here")
    }
}

/// The task that runs first.
pub fn next_task(tasks: &[Task]) -> Option<&Task> {
    todo!("Write your solution here")
}

/// Task names in execution order, via a `BinaryHeap`.
pub fn schedule(tasks: Vec<Task>) -> Vec<String> {
    todo!("Write your solution here")
}

/// Task names by deadline descending; ties in execution order.
pub fn latest_first(tasks: Vec<Task>) -> Vec<String> {
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

    fn task(name: &str, priority: u8, deadline: u32) -> Task {
        Task {
            priority,
            deadline,
            name: name.to_string(),
        }
    }

    fn sample() -> Vec<Task> {
        vec![
            task("email", 1, 30),
            task("deploy", 3, 50),
            task("review", 3, 20),
            task("backup", 3, 20),
        ]
    }

    #[test]
    fn comparisons_follow_the_rules() {
        assert!(task("a", 2, 99) > task("b", 1, 1));
        assert!(task("a", 2, 10) > task("b", 2, 11));
        assert!(task("a", 2, 10) > task("b", 2, 10));
        assert_eq!(task("a", 2, 10).cmp(&task("a", 2, 10)), Ordering::Equal);
        assert_eq!(
            task("x", 0, 0).partial_cmp(&task("y", 0, 0)),
            Some(Ordering::Greater)
        );
    }

    #[test]
    fn example_schedule() {
        assert_eq!(
            schedule(sample()),
            vec!["backup", "review", "deploy", "email"]
        );
        assert_eq!(
            next_task(&sample()).map(|t| t.name.as_str()),
            Some("backup")
        );
    }

    #[test]
    fn example_latest_first() {
        assert_eq!(
            latest_first(sample()),
            vec!["deploy", "email", "backup", "review"]
        );
    }

    #[test]
    fn empty_inputs() {
        assert!(schedule(vec![]).is_empty());
        assert!(next_task(&[]).is_none());
        assert!(latest_first(vec![]).is_empty());
    }

    #[test]
    fn heap_matches_sorting() {
        let tasks: Vec<Task> = (0..100_000_u32)
            .map(|i| task(&format!("t{:05}", i % 977), (i % 7) as u8, i % 13))
            .collect();
        let mut sorted = tasks.clone();
        sorted.sort_by(|a, b| b.cmp(a));
        let expected: Vec<String> = sorted.into_iter().map(|t| t.name).collect();
        assert_eq!(schedule(tasks), expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `cmp` chains three comparisons with `then_with`, reversing the operands where "smaller is
better" (deadline, name). With that order, the heap pops tasks in execution order and `max` finds the
next task.

**Complexity:** `schedule` is `O(n log n)`; `next_task` is `O(n)`.

**Rust concepts:**
- `PartialOrd` should be implemented as `Some(self.cmp(other))` when a type is `Ord`, so the two
  orders can never disagree (Clippy warns about mismatches).
- `Ord` must be consistent with `Eq`: here every field takes part in the comparison, and derived
  `PartialEq` also compares every field.
- `Reverse(x)` wraps a value and reverses its order, useful for one-off descending sorts without
  writing a new `Ord` impl. `sort_by_key` is stable, so equal deadlines keep the heap order.

```rust
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub priority: u8,
    pub deadline: u32,
    pub name: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| other.deadline.cmp(&self.deadline))
            .then_with(|| other.name.cmp(&self.name))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The task that runs first.
pub fn next_task(tasks: &[Task]) -> Option<&Task> {
    tasks.iter().max()
}

/// Task names in execution order, via a `BinaryHeap`.
pub fn schedule(tasks: Vec<Task>) -> Vec<String> {
    let mut heap: BinaryHeap<Task> = tasks.into_iter().collect();
    let mut order = Vec::with_capacity(heap.len());
    while let Some(task) = heap.pop() {
        order.push(task.name);
    }
    order
}

/// Task names by deadline descending; ties in execution order.
pub fn latest_first(tasks: Vec<Task>) -> Vec<String> {
    let mut tasks = tasks;
    tasks.sort_by(|a, b| b.cmp(a)); // execution order first
    tasks.sort_by_key(|t| Reverse(t.deadline)); // stable: keeps execution order on ties
    tasks.into_iter().map(|t| t.name).collect()
}
```

Full tested solution: [`src/solutions/p0538_ordering_for_tasks.rs`](../src/solutions/p0538_ordering_for_tasks.rs)

</details>
