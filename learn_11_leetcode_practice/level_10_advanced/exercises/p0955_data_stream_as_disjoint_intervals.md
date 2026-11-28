# Problem 0955: Data Stream as Disjoint Intervals

- **Difficulty:** Hard
- **Topics:** `BTreeMap` as an interval set, `range(..=x).next_back()` to find the predecessor, merging neighbours, `remove` + `insert`
- **Source:** [LeetCode 352. Data Stream as Disjoint Intervals](https://leetcode.com/problems/data-stream-as-disjoint-intervals/)

## Problem

Non-negative integers arrive one at a time. At any moment, summarise the distinct numbers seen so
far as a sorted list of disjoint, **maximal** closed intervals `[start, end]`: consecutive numbers
belong to the same interval.

Implement the `SummaryRanges` struct:

- `SummaryRanges::new() -> Self`.
- `add_num(&mut self, value: i32)`: adds `value` (adding a number twice changes nothing).
- `get_intervals(&self) -> Vec<Vec<i32>>`: the intervals as `[start, end]` pairs, sorted by start.

## Examples

```text
add_num(1)       => intervals [[1, 1]]
add_num(3)       => [[1, 1], [3, 3]]
add_num(7)       => [[1, 1], [3, 3], [7, 7]]
add_num(2)       => [[1, 3], [7, 7]]
add_num(6)       => [[1, 3], [6, 7]]
```

## Constraints

- `0 <= value <= 10^4`
- At most `3 * 10^4` calls to `add_num` and at most `10^2` calls to `get_intervals`.

## Hints

1. Store the intervals in a `BTreeMap<start, end>`.
2. The only interval that can contain `value` or end at `value - 1` is the one with the largest
   start `<= value`; the only one that can start at `value + 1` is `map.get(&(value + 1))`.
3. Four cases: already covered, join both neighbours, extend one neighbour, or a new `[v, v]`.

## Starter code

Write your code in [`src/practice/p0955_data_stream_as_disjoint_intervals.rs`](../src/practice/p0955_data_stream_as_disjoint_intervals.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0955
```

```rust
use std::collections::BTreeMap;

pub struct SummaryRanges {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl SummaryRanges {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn add_num(&mut self, value: i32) {
        todo!("Write your solution here")
    }

    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
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
    fn example_sequence() {
        let mut r = SummaryRanges::new();
        let mut snapshots = Vec::new();
        for v in [1, 3, 7, 2, 6] {
            r.add_num(v);
            snapshots.push(r.get_intervals());
        }
        assert_eq!(
            snapshots,
            vec![
                vec![vec![1, 1]],
                vec![vec![1, 1], vec![3, 3]],
                vec![vec![1, 1], vec![3, 3], vec![7, 7]],
                vec![vec![1, 3], vec![7, 7]],
                vec![vec![1, 3], vec![6, 7]],
            ]
        );
    }

    #[test]
    fn duplicates_and_zero() {
        let mut r = SummaryRanges::new();
        for v in [0, 0, 2, 2, 1, 1] {
            r.add_num(v);
        }
        assert_eq!(r.get_intervals(), vec![vec![0, 2]]);
    }

    #[test]
    fn values_inside_existing_intervals() {
        let mut r = SummaryRanges::new();
        for v in [5, 6, 7, 8, 6, 10] {
            r.add_num(v);
        }
        assert_eq!(r.get_intervals(), vec![vec![5, 8], vec![10, 10]]);
        r.add_num(9);
        assert_eq!(r.get_intervals(), vec![vec![5, 10]]);
    }

    #[test]
    fn many_values() {
        let mut r = SummaryRanges::new();
        for v in (0..=10_000).step_by(2) {
            r.add_num(v);
        }
        assert_eq!(r.get_intervals().len(), 5_001);
        for v in (1..10_000).step_by(2) {
            r.add_num(v);
        }
        assert_eq!(r.get_intervals(), vec![vec![0, 10_000]]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** look up the predecessor interval `[ps, pe]` (largest start `<= value`) and the successor
interval starting at `value + 1`. If the predecessor already covers `value`, stop. Otherwise the
new interval starts at `ps` if `pe == value - 1` (else at `value`) and ends at the successor's end
if it exists (else at `value`); the successor entry is removed and the merged interval inserted.

**Complexity:** `add_num` is `O(log k)` for `k` intervals; `get_intervals` is `O(k)`.

**Rust concepts:**
- `BTreeMap::range(..=value).next_back()` finds the greatest key `<= value` in `O(log k)`.
- Copying the found `(start, end)` out with `map(|(&s, &e)| (s, e))` ends the borrow before the map
  is modified.

```rust
use std::collections::BTreeMap;

pub struct SummaryRanges {
    intervals: BTreeMap<i32, i32>,
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl SummaryRanges {
    pub fn new() -> Self {
        SummaryRanges {
            intervals: BTreeMap::new(),
        }
    }

    pub fn add_num(&mut self, value: i32) {
        let previous = self
            .intervals
            .range(..=value)
            .next_back()
            .map(|(&s, &e)| (s, e));
        if matches!(previous, Some((_, end)) if end >= value) {
            // Already covered.
            return;
        }
        let start = match previous {
            Some((s, e)) if e == value - 1 => s,
            _ => value,
        };
        let end = self.intervals.remove(&(value + 1)).unwrap_or(value);
        self.intervals.insert(start, end);
    }

    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals.iter().map(|(&s, &e)| vec![s, e]).collect()
    }
}
```

Full tested solution: [`src/solutions/p0955_data_stream_as_disjoint_intervals.rs`](../src/solutions/p0955_data_stream_as_disjoint_intervals.rs)

</details>
