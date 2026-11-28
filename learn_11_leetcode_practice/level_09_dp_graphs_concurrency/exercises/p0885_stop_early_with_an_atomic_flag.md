# Problem 0885: Stop Early with an Atomic Flag

- **Difficulty:** Medium
- **Topics:** cooperative cancellation with `AtomicBool`, `AtomicUsize::fetch_min` to keep the
  best answer, generic predicates shared by scoped threads
- **Source:** Original · [`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/index.html)

## Problem

Threads cannot be killed from the outside in Rust; to stop early they must **cooperate** by
checking a shared flag. Implement two parallel searches over a slice, each using up to
`workers` scoped threads on contiguous chunks of `ceil(len / workers)` items:

1. `parallel_any(items, workers, pred) -> bool`: `true` if some item satisfies `pred`. As soon
   as one worker finds a match it raises an `AtomicBool`, and every worker checks the flag
   before testing its next item and stops if it is set.
2. `parallel_position(items, workers, pred) -> Option<usize>`: the **smallest** index whose item
   satisfies `pred`. Keep the best index found so far in an `AtomicUsize` (start with
   `usize::MAX`) updated with `fetch_min`. A worker stops at its first match, and also as soon as
   the index it would test next is not smaller than the best index found so far.

Signatures (`T: Sync`, `F: Fn(&T) -> bool + Sync`):

```text
pub fn parallel_any<T, F>(items: &[T], workers: usize, pred: F) -> bool
pub fn parallel_position<T, F>(items: &[T], workers: usize, pred: F) -> Option<usize>
```

## Examples

```text
parallel_any(&[1, 3, 5, 6, 7], 2, |x| x % 2 == 0)      -> true
parallel_any(&[1, 3, 5], 3, |x| x % 2 == 0)            -> false
parallel_position(&[4, 1, 8, 1, 9], 3, |&x| x == 1)    -> Some(1)
parallel_position(&[] as &[i32], 3, |_| true)          -> None
```

## Constraints

- `1 <= workers <= 32`; `0 <= items.len() <= 10^6`

## Hints

1. The flag only says "stop now", it does not protect other data, so `Ordering::Relaxed` loads
   and stores are enough; joining the scope makes the final value visible.
2. `fetch_min(i)` stores `min(old, i)` atomically, so two workers racing to report a match
   cannot overwrite a smaller index with a larger one.
3. With a single worker, the search is sequential: it tests items in order and stops at the
   first match.

## Starter code

Write your code in [`src/practice/p0885_stop_early_with_an_atomic_flag.rs`](../src/practice/p0885_stop_early_with_an_atomic_flag.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0885
```

```rust
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

pub fn parallel_any<T, F>(items: &[T], workers: usize, pred: F) -> bool
where
    T: Sync,
    F: Fn(&T) -> bool + Sync,
{
    todo!("Write your solution here")
}

pub fn parallel_position<T, F>(items: &[T], workers: usize, pred: F) -> Option<usize>
where
    T: Sync,
    F: Fn(&T) -> bool + Sync,
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
    fn example_any_true() {
        assert!(parallel_any(&[1, 3, 5, 6, 7], 2, |x| x % 2 == 0));
    }

    #[test]
    fn example_any_false() {
        assert!(!parallel_any(&[1, 3, 5], 3, |x| x % 2 == 0));
    }

    #[test]
    fn example_position() {
        assert_eq!(parallel_position(&[4, 1, 8, 1, 9], 3, |&x| x == 1), Some(1));
    }

    #[test]
    fn example_empty() {
        let empty: &[i32] = &[];
        assert_eq!(parallel_position(empty, 3, |_| true), None);
        assert!(!parallel_any(empty, 3, |_| true));
    }

    #[test]
    fn smallest_index_wins_across_chunks() {
        // Matches at 999_999 (last chunk) and 250_001 (second chunk); the smaller one is returned.
        let mut items = vec![0_u8; 1_000_000];
        items[999_999] = 1;
        items[250_001] = 1;
        for workers in [1, 2, 4, 32] {
            assert_eq!(
                parallel_position(&items, workers, |&x| x == 1),
                Some(250_001)
            );
            assert!(parallel_any(&items, workers, |&x| x == 1));
        }
        assert_eq!(parallel_position(&items, 4, |&x| x == 2), None);
    }

    #[test]
    fn single_worker_stops_at_the_first_match() {
        let calls = AtomicUsize::new(0);
        let items: Vec<u32> = (0..10_000).collect();
        let pos = parallel_position(&items, 1, |&x| {
            calls.fetch_add(1, Ordering::Relaxed);
            x == 41
        });
        assert_eq!(pos, Some(41));
        assert_eq!(calls.load(Ordering::Relaxed), 42);
        calls.store(0, Ordering::Relaxed);
        assert!(parallel_any(&items, 1, |&x| {
            calls.fetch_add(1, Ordering::Relaxed);
            x == 9
        }));
        assert_eq!(calls.load(Ordering::Relaxed), 10);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** for `parallel_any`, every scoped worker walks its chunk, returning early when the flag
is set, and sets the flag when its predicate matches. For `parallel_position`, workers walk their
chunk with global indices; before each test they compare the index with `best.load()`, and on a
match they call `best.fetch_min(index)` and stop. Indices inside a chunk increase, so once a
worker reaches an index `>= best` nothing better can follow in that chunk.

**Rust concepts:**
- `AtomicBool` / `AtomicUsize` are plain values on the stack here; the scoped threads borrow them.
- `fetch_min` and `fetch_max` (stable since Rust 1.45) are single atomic read-modify-write
  operations, like `fetch_add`.
- Cancellation is *cooperative*: a worker busy inside a long `pred` call only notices the flag
  afterwards.

```rust
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

pub fn parallel_any<T, F>(items: &[T], workers: usize, pred: F) -> bool
where
    T: Sync,
    F: Fn(&T) -> bool + Sync,
{
    if items.is_empty() {
        return false;
    }
    let found = AtomicBool::new(false);
    let size = items.len().div_ceil(workers);
    let (found_ref, pred) = (&found, &pred);
    thread::scope(|s| {
        for chunk in items.chunks(size) {
            s.spawn(move || {
                for item in chunk {
                    if found_ref.load(Ordering::Relaxed) {
                        return;
                    }
                    if pred(item) {
                        found_ref.store(true, Ordering::Relaxed);
                        return;
                    }
                }
            });
        }
    });
    found.load(Ordering::Relaxed)
}

pub fn parallel_position<T, F>(items: &[T], workers: usize, pred: F) -> Option<usize>
where
    T: Sync,
    F: Fn(&T) -> bool + Sync,
{
    if items.is_empty() {
        return None;
    }
    let best = AtomicUsize::new(usize::MAX);
    let size = items.len().div_ceil(workers);
    let (best_ref, pred) = (&best, &pred);
    thread::scope(|s| {
        for (c, chunk) in items.chunks(size).enumerate() {
            s.spawn(move || {
                for (offset, item) in chunk.iter().enumerate() {
                    let index = c * size + offset;
                    if index >= best_ref.load(Ordering::Relaxed) {
                        return;
                    }
                    if pred(item) {
                        best_ref.fetch_min(index, Ordering::Relaxed);
                        return;
                    }
                }
            });
        }
    });
    let index = best.load(Ordering::Relaxed);
    (index != usize::MAX).then_some(index)
}
```

Full tested solution: [`src/solutions/p0885_stop_early_with_an_atomic_flag.rs`](../src/solutions/p0885_stop_early_with_an_atomic_flag.rs)

</details>
