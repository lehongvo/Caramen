# Problem 0577: A Run-Length Decoding Iterator

- **Difficulty:** Medium
- **Topics:** an iterator borrowing a slice (`RunLengthDecode<'a, T>` yielding `&'a T`), overriding provided methods (`nth`, `size_hint`), generic encoding with `T: PartialEq + Clone`
- **Source:** Original · [std::iter::Iterator::nth](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth)

## Problem

Run-length encoding stores a sequence as `(value, count)` pairs: `a a a c c` is `[('a', 3), ('c', 2)]`.

1. `run_length_encode<T: PartialEq + Clone>(items: &[T]) -> Vec<(T, usize)>`: merges runs of equal
   neighbours (every count is at least 1).
2. `RunLengthDecode<'a, T>` iterates over the decoded sequence **without expanding it**, yielding
   references `&'a T` into the encoded slice. Create it with `RunLengthDecode::new(runs: &'a [(T,
   usize)])`. Runs with count `0` are allowed and simply produce nothing.
   - Implement `Iterator` with `type Item = &'a T`.
   - Override `size_hint` to return the exact number of remaining items.
   - Override `nth(&mut self, n)` so that it **skips whole runs** instead of calling `next` `n`
     times: `RunLengthDecode::new(&[(7, 1_000_000_000_000)]).nth(999_999_999_999)` must be instant.

The decoder keeps `runs`, the index of the current run and how many items of that run were already
used.

## Examples

```text
run_length_encode(&['a', 'a', 'a', 'c', 'c'])          => [('a', 3), ('c', 2)]
RunLengthDecode::new(&[('a', 3), ('b', 0), ('c', 2)])  => a, a, a, c, c
RunLengthDecode::new(&[(1, 2), (2, 2)]).nth(2)         => Some(&2)
RunLengthDecode::new(&[(1, 2), (2, 2)]).size_hint()    => (4, Some(4))
```

## Constraints

- Counts up to `10^12`; total remaining items fit in `usize` (64-bit targets).

## Hints

1. In `next`, skip runs that are exhausted (`used == count`) before yielding.
2. In `nth`, while `n` is at least the number of items left in the current run, subtract them and
   move to the next run; then take the item `n` positions into the current run.
3. Overriding a provided method only changes performance; its results must stay identical to the
   default implementation.

## Starter code

Write your code in [`src/practice/p0577_run_length_iterator.rs`](../src/practice/p0577_run_length_iterator.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0577
```

```rust
/// Merges runs of equal neighbours into `(value, count)` pairs.
pub fn run_length_encode<T: PartialEq + Clone>(items: &[T]) -> Vec<(T, usize)> {
    todo!("Write your solution here")
}

/// Iterates over a run-length encoded sequence without expanding it.
#[derive(Debug, Clone)]
pub struct RunLengthDecode<'a, T> {
    runs: &'a [(T, usize)],
    run: usize,
    used: usize,
}

impl<'a, T> RunLengthDecode<'a, T> {
    pub fn new(runs: &'a [(T, usize)]) -> Self {
        todo!("Write your solution here")
    }
}

impl<'a, T> Iterator for RunLengthDecode<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        todo!("Write your solution here")
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!("Write your solution here")
    }

    fn nth(&mut self, mut n: usize) -> Option<&'a T> {
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
    fn encoding() {
        assert_eq!(
            run_length_encode(&['a', 'a', 'a', 'c', 'c']),
            vec![('a', 3), ('c', 2)]
        );
        assert_eq!(run_length_encode::<u8>(&[]), vec![]);
        assert_eq!(
            run_length_encode(&["x", "y", "x"]),
            vec![("x", 1), ("y", 1), ("x", 1)]
        );
    }

    #[test]
    fn decoding_skips_empty_runs() {
        let runs = [('a', 3), ('b', 0), ('c', 2)];
        let decoded: String = RunLengthDecode::new(&runs).collect();
        assert_eq!(decoded, "aaacc");
        let none: [(u8, usize); 2] = [(1, 0), (2, 0)];
        assert_eq!(RunLengthDecode::new(&none).next(), None);
    }

    #[test]
    fn nth_and_size_hint_match_the_defaults() {
        let runs = [(1, 2), (2, 2), (3, 0), (4, 1)];
        let mut it = RunLengthDecode::new(&runs);
        assert_eq!(it.size_hint(), (5, Some(5)));
        assert_eq!(it.nth(2), Some(&2));
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&4));
        assert_eq!(it.next(), None);
        for n in 0..7 {
            let expected = RunLengthDecode::new(&runs)
                .collect::<Vec<_>>()
                .get(n)
                .copied();
            assert_eq!(RunLengthDecode::new(&runs).nth(n), expected, "n = {n}");
        }
    }

    #[test]
    fn huge_runs_are_skipped_quickly() {
        let runs = [(7_u8, 1_000_000_000_000), (9, 5)];
        let mut it = RunLengthDecode::new(&runs);
        assert_eq!(it.nth(999_999_999_999), Some(&7));
        assert_eq!(it.next(), Some(&9));
        assert_eq!(it.size_hint(), (4, Some(4)));
        assert_eq!(RunLengthDecode::new(&runs).nth(2_000_000_000_000), None);
    }

    #[test]
    fn round_trip() {
        let data: Vec<u32> = (0..10_000).map(|i| (i / 7) % 3).collect();
        let runs = run_length_encode(&data);
        let decoded: Vec<u32> = RunLengthDecode::new(&runs).copied().collect();
        assert_eq!(decoded, data);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the state `(run, used)` points at the next item. `next` advances past empty or exhausted
runs and yields a reference to the run's value. `nth` jumps run by run, which is `O(number of
runs)` instead of `O(n)`. `size_hint` sums what is left.

**Rust concepts:**
- The iterator holds `&'a [(T, usize)]`, so its items can be `&'a T`: they borrow from the encoded
  data and outlive the iterator.
- `Iterator` provides `nth`, `count`, `last`, ... in terms of `next`; overriding them is allowed and
  common for performance (std's slice iterators do it).
- An exact `size_hint` lets `collect` allocate the right capacity up front.

```rust
/// Merges runs of equal neighbours into `(value, count)` pairs.
pub fn run_length_encode<T: PartialEq + Clone>(items: &[T]) -> Vec<(T, usize)> {
    let mut runs: Vec<(T, usize)> = Vec::new();
    for item in items {
        match runs.last_mut() {
            Some((value, count)) if value == item => *count += 1,
            _ => runs.push((item.clone(), 1)),
        }
    }
    runs
}

/// Iterates over a run-length encoded sequence without expanding it.
#[derive(Debug, Clone)]
pub struct RunLengthDecode<'a, T> {
    runs: &'a [(T, usize)],
    run: usize,
    used: usize,
}

impl<'a, T> RunLengthDecode<'a, T> {
    pub fn new(runs: &'a [(T, usize)]) -> Self {
        RunLengthDecode {
            runs,
            run: 0,
            used: 0,
        }
    }
}

impl<'a, T> Iterator for RunLengthDecode<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        while let Some((value, count)) = self.runs.get(self.run) {
            if self.used < *count {
                self.used += 1;
                return Some(value);
            }
            self.run += 1;
            self.used = 0;
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let total: usize = self.runs[self.run.min(self.runs.len())..]
            .iter()
            .map(|&(_, c)| c)
            .sum();
        let left = total - self.used.min(total);
        (left, Some(left))
    }

    fn nth(&mut self, mut n: usize) -> Option<&'a T> {
        while let Some((_, count)) = self.runs.get(self.run) {
            let left_in_run = count - self.used;
            if n < left_in_run {
                self.used += n;
                return self.next();
            }
            n -= left_in_run;
            self.run += 1;
            self.used = 0;
        }
        None
    }
}
```

Full tested solution: [`src/solutions/p0577_run_length_iterator.rs`](../src/solutions/p0577_run_length_iterator.rs)

</details>
