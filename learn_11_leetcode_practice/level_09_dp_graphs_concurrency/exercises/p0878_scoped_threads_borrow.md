# Problem 0878: Borrow Data with Scoped Threads

- **Difficulty:** Easy
- **Topics:** `std::thread::scope`, threads that borrow local data (no `Arc`, no cloning),
  `ScopedJoinHandle`
- **Source:** Original · [`thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html)

## Problem

Threads started with `thread::spawn` may outlive the function that started them, so they cannot
borrow its local variables. `thread::scope(|s| { ... })` creates a scope that **joins every
thread spawned inside it before returning**, so those threads may borrow anything that outlives
the scope.

Implement, using scoped threads and without copying the input:

1. `scoped_min_max_sum(data: &[i32], parts: usize) -> Option<(i32, i32, i64)>`: split `data`
   into at most `parts` contiguous chunks of `ceil(len / parts)` elements, compute the minimum,
   the maximum and the sum of each chunk on its own scoped thread, and combine them. Return
   `None` for an empty slice.
2. `count_each(text: &str, needles: &[char]) -> Vec<usize>`: start one scoped thread per needle;
   thread `i` counts how many times `needles[i]` occurs in `text`. All threads read the same
   borrowed `text`. Return the counts in needle order.

## Examples

```text
scoped_min_max_sum(&[3, -1, 4, 1, 5], 2) -> Some((-1, 5, 12))
scoped_min_max_sum(&[], 4)               -> None
count_each("banana", &['a', 'n', 'z'])   -> [3, 2, 0]
```

## Constraints

- `1 <= parts <= 64`; `0 <= data.len() <= 10^6`
- `0 <= needles.len() <= 32`; `text` may contain any Unicode characters.

## Hints

1. `thread::scope(|s| { let h = s.spawn(|| work(&data)); h.join() })`: the closure passed to
   `s.spawn` can capture references because the scope outlives it.
2. `data.chunks(size)` yields borrowed sub-slices; spawn one thread per chunk and collect the
   handles before joining.
3. Round the chunk size up with `len.div_ceil(parts)` (stable for unsigned integers since
   Rust 1.73).

## Starter code

Write your code in [`src/practice/p0878_scoped_threads_borrow.rs`](../src/practice/p0878_scoped_threads_borrow.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0878
```

```rust
use std::thread;

pub fn scoped_min_max_sum(data: &[i32], parts: usize) -> Option<(i32, i32, i64)> {
    todo!("Write your solution here")
}

pub fn count_each(text: &str, needles: &[char]) -> Vec<usize> {
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
    fn example_two_parts() {
        assert_eq!(scoped_min_max_sum(&[3, -1, 4, 1, 5], 2), Some((-1, 5, 12)));
    }

    #[test]
    fn example_empty_slice() {
        assert_eq!(scoped_min_max_sum(&[], 4), None);
    }

    #[test]
    fn example_count_each() {
        assert_eq!(count_each("banana", &['a', 'n', 'z']), vec![3, 2, 0]);
    }

    #[test]
    fn more_parts_than_elements() {
        assert_eq!(scoped_min_max_sum(&[7], 64), Some((7, 7, 7)));
        assert_eq!(scoped_min_max_sum(&[2, 9], 64), Some((2, 9, 11)));
    }

    #[test]
    fn unicode_and_no_needles() {
        assert_eq!(count_each("ñoño über ñ", &['ñ', 'ü', ' ']), vec![3, 1, 2]);
        assert!(count_each("abc", &[]).is_empty());
    }

    #[test]
    fn large_input_does_not_overflow() {
        let data = vec![i32::MAX; 1_000_000];
        let got = scoped_min_max_sum(&data, 8);
        assert_eq!(
            got,
            Some((i32::MAX, i32::MAX, i64::from(i32::MAX) * 1_000_000))
        );
        let mixed: Vec<i32> = (-500_000..500_000).collect();
        assert_eq!(
            scoped_min_max_sum(&mixed, 7),
            Some((-500_000, 499_999, -500_000))
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** inside `thread::scope`, map every chunk to a scoped thread that folds it into
`(min, max, sum)`, collect the handles, join them and fold the partial results together. For
`count_each`, spawn one scoped thread per needle that runs `text.chars().filter(...).count()`.
No `Arc` and no copy of the data is needed, because the scope guarantees the threads end before
`data` and `text` go out of scope.

**Rust concepts:**
- `thread::scope` (stable since 1.63) returns the value of its closure, so results can be
  produced directly inside it.
- The spawned closures use `move` to take their own *copy of the reference* (`chunk: &[i32]`,
  `needle: char`), while the data itself stays borrowed.
- `ScopedJoinHandle::join` returns a `Result` like `JoinHandle::join`.

```rust
use std::thread;

pub fn scoped_min_max_sum(data: &[i32], parts: usize) -> Option<(i32, i32, i64)> {
    if data.is_empty() {
        return None;
    }
    let size = data.len().div_ceil(parts);
    thread::scope(|s| {
        let handles: Vec<_> = data
            .chunks(size)
            .map(|chunk| {
                s.spawn(move || {
                    chunk
                        .iter()
                        .fold((i32::MAX, i32::MIN, 0_i64), |(lo, hi, sum), &x| {
                            (lo.min(x), hi.max(x), sum + i64::from(x))
                        })
                })
            })
            .collect();
        let combined = handles
            .into_iter()
            .map(|h| h.join().expect("folding a chunk cannot panic"))
            .fold((i32::MAX, i32::MIN, 0_i64), |(lo, hi, sum), (a, b, c)| {
                (lo.min(a), hi.max(b), sum + c)
            });
        Some(combined)
    })
}

pub fn count_each(text: &str, needles: &[char]) -> Vec<usize> {
    thread::scope(|s| {
        let handles: Vec<_> = needles
            .iter()
            .map(|&needle| s.spawn(move || text.chars().filter(|&c| c == needle).count()))
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("counting cannot panic"))
            .collect()
    })
}
```

Full tested solution: [`src/solutions/p0878_scoped_threads_borrow.rs`](../src/solutions/p0878_scoped_threads_borrow.rs)

</details>
