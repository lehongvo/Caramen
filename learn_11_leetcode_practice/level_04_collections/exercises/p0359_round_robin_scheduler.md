# Problem 0359: Round-Robin Scheduler

- **Difficulty:** Easy
- **Topics:** `VecDeque` as a FIFO queue: `push_back`, `pop_front`, `while let`, a queue of queues
- **Source:** Original · [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)

## Problem

`std::collections::VecDeque` is a double-ended queue: adding or removing at **either** end is
`O(1)`. (Removing from the front of a `Vec` is `O(n)`, because every other element shifts.)
Use it to implement:

1. `round_robin(jobs: &[(&str, u32)], quantum: u32) -> Vec<(String, u32)>` simulates a CPU
   scheduler. Each job is `(name, time_needed)`. Jobs wait in a queue in the given order. The
   CPU repeatedly takes the job at the front and runs it for `quantum` time units, or less if
   the job needs less. A finished job leaves; an unfinished job goes to the **back** of the
   queue. The clock starts at `0` and only advances while a job runs. Return `(name, finish_time)`
   for every job, in the order the jobs finish.
2. `interleave(queues: Vec<Vec<i32>>) -> Vec<i32>` takes one element from the first queue, then
   one from the second, and so on, cycling over the queues that still have elements, until all
   are empty. Each inner vector is a queue whose front is its first element.

## Examples

```text
round_robin([("A", 3), ("B", 1), ("C", 2)], 2)
  t=0..2  A runs (1 left)   queue: B, C, A
  t=2..3  B runs, finishes  queue: C, A
  t=3..5  C runs, finishes  queue: A
  t=5..6  A runs, finishes
  => [("B", 3), ("C", 5), ("A", 6)]

interleave([[1, 2, 3], [10], [20, 21]]) => [1, 10, 20, 2, 21, 3]
```

## Constraints

- `0 <= jobs.len() <= 10^4`, `1 <= time_needed <= 10^4`, `1 <= quantum <= 10^4`
- At most `10^5` elements in total in `queues`.

## Hints

1. `VecDeque::from(vec)` or `iter().collect()` builds a queue; `pop_front()` returns an `Option`.
2. `while let Some(job) = queue.pop_front() { ... }` runs until the queue is empty.
3. For `interleave`, keep a queue of queues: pop a queue, take its front element, and push the
   queue back if it still has elements.

## Starter code

Write your code in [`src/practice/p0359_round_robin_scheduler.rs`](../src/practice/p0359_round_robin_scheduler.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0359
```

```rust
use std::collections::VecDeque;

/// Simulates round-robin scheduling and returns `(name, finish_time)` in finishing order.
pub fn round_robin(jobs: &[(&str, u32)], quantum: u32) -> Vec<(String, u32)> {
    todo!("Write your solution here")
}

/// Takes one element from each non-empty queue in turn until all are empty.
pub fn interleave(queues: Vec<Vec<i32>>) -> Vec<i32> {
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
    fn round_robin_example() {
        assert_eq!(
            round_robin(&[("A", 3), ("B", 1), ("C", 2)], 2),
            vec![
                ("B".to_string(), 3),
                ("C".to_string(), 5),
                ("A".to_string(), 6)
            ]
        );
    }

    #[test]
    fn round_robin_large_quantum_is_first_come_first_served() {
        assert_eq!(
            round_robin(&[("x", 4), ("y", 2)], 100),
            vec![("x".to_string(), 4), ("y".to_string(), 6)]
        );
        assert_eq!(round_robin(&[], 3), Vec::new());
    }

    #[test]
    fn interleave_example() {
        assert_eq!(
            interleave(vec![vec![1, 2, 3], vec![10], vec![20, 21]]),
            vec![1, 10, 20, 2, 21, 3]
        );
        assert_eq!(interleave(vec![vec![], vec![5], vec![]]), vec![5]);
        assert_eq!(interleave(vec![]), Vec::<i32>::new());
    }

    #[test]
    fn many_jobs_quantum_one() {
        let names: Vec<String> = (0..100).map(|i| format!("j{i}")).collect();
        let jobs: Vec<(&str, u32)> = names.iter().map(|n| (n.as_str(), 100)).collect();
        let finished = round_robin(&jobs, 1);
        assert_eq!(finished.len(), 100);
        // Everybody needs 100 slices; the last round finishes them in queue order.
        assert_eq!(finished[0], ("j0".to_string(), 9_901));
        assert_eq!(finished[99], ("j99".to_string(), 10_000));
    }

    #[test]
    fn interleave_large() {
        let queues: Vec<Vec<i32>> = (0..10)
            .map(|q| (0..10_000).map(|i| i * 10 + q).collect())
            .collect();
        let result = interleave(queues);
        assert_eq!(result, (0..100_000).collect::<Vec<i32>>());
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** both functions are queue simulations. In `round_robin` each queue item is the job's
name and its remaining time; one loop iteration runs one time slice. In `interleave`, the queue
holds the inner queues themselves, so exhausted queues simply stop being pushed back.

**Rust concepts:**
- `VecDeque` is a ring buffer: `push_back` and `pop_front` are `O(1)`, which a `Vec` cannot do at
  the front.
- `while let Some(...) = queue.pop_front()` combines "is the queue empty?" and "take the front".
- `remaining.min(quantum)` is the length of the time slice.
- `VecDeque<VecDeque<i32>>` nests like any other collection; moving an inner queue in and out of
  the outer one moves only its small header, not its elements.

```rust
use std::collections::VecDeque;

/// Simulates round-robin scheduling and returns `(name, finish_time)` in finishing order.
pub fn round_robin(jobs: &[(&str, u32)], quantum: u32) -> Vec<(String, u32)> {
    let mut queue: VecDeque<(&str, u32)> = jobs.iter().copied().collect();
    let mut clock = 0;
    let mut finished = Vec::with_capacity(jobs.len());
    while let Some((name, remaining)) = queue.pop_front() {
        let slice = remaining.min(quantum);
        clock += slice;
        if slice == remaining {
            finished.push((name.to_string(), clock));
        } else {
            queue.push_back((name, remaining - slice));
        }
    }
    finished
}

/// Takes one element from each non-empty queue in turn until all are empty.
pub fn interleave(queues: Vec<Vec<i32>>) -> Vec<i32> {
    let mut outer: VecDeque<VecDeque<i32>> = queues
        .into_iter()
        .map(VecDeque::from)
        .filter(|q| !q.is_empty())
        .collect();
    let mut result = Vec::new();
    while let Some(mut inner) = outer.pop_front() {
        if let Some(x) = inner.pop_front() {
            result.push(x);
        }
        if !inner.is_empty() {
            outer.push_back(inner);
        }
    }
    result
}
```

Full tested solution: [`src/solutions/p0359_round_robin_scheduler.rs`](../src/solutions/p0359_round_robin_scheduler.rs)

</details>
