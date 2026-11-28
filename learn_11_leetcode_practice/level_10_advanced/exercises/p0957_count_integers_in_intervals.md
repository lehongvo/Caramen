# Problem 0957: Count Integers in Intervals

- **Difficulty:** Hard
- **Topics:** closed intervals in a `BTreeMap`, amortised merging with `while let`, maintaining a running total
- **Source:** [LeetCode 2276. Count Integers in Intervals](https://leetcode.com/problems/count-integers-in-intervals/)

## Problem

Maintain a set of **closed** integer intervals `[left, right]` and report how many distinct
integers they cover together.

Implement the `CountIntervals` struct:

- `CountIntervals::new() -> Self`: the set is empty.
- `add(&mut self, left: i32, right: i32)`: adds the interval `[left, right]`.
- `count(&self) -> i32`: the number of integers in at least one added interval.

## Examples

```text
add(2, 3)
add(7, 10)
count()      => 6        (2, 3, 7, 8, 9, 10)
add(5, 8)
count()      => 8        (2, 3, 5, 6, 7, 8, 9, 10)
```

## Constraints

- `1 <= left <= right <= 10^9`
- At most `10^5` calls to `add` and `count` in total; `count` is called at least once.

## Hints

1. Store disjoint intervals in a `BTreeMap<start, end>` and keep the covered total in a field, so
   `count` is `O(1)`.
2. When adding, repeatedly take the interval with the largest start `<= right`; if it reaches
   `left`, remove it (subtracting its size) and grow the new interval to include it.
3. Every interval is removed at most once, so the total work is amortised `O(log n)` per call.

## Starter code

Write your code in [`src/practice/p0957_count_integers_in_intervals.rs`](../src/practice/p0957_count_integers_in_intervals.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0957
```

```rust
use std::collections::BTreeMap;

pub struct CountIntervals {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl CountIntervals {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn add(&mut self, left: i32, right: i32) {
        todo!("Write your solution here")
    }

    pub fn count(&self) -> i32 {
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
        let mut c = CountIntervals::new();
        c.add(2, 3);
        c.add(7, 10);
        assert_eq!(c.count(), 6);
        c.add(5, 8);
        assert_eq!(c.count(), 8);
    }

    #[test]
    fn nested_and_identical_intervals() {
        let mut c = CountIntervals::new();
        c.add(1, 100);
        c.add(10, 20);
        c.add(1, 100);
        assert_eq!(c.count(), 100);
        c.add(101, 101);
        assert_eq!(c.count(), 101);
    }

    #[test]
    fn one_interval_swallows_many() {
        let mut c = CountIntervals::new();
        for i in 0..1_000 {
            c.add(i * 10 + 1, i * 10 + 5);
        }
        assert_eq!(c.count(), 5_000);
        c.add(1, 10_000);
        assert_eq!(c.count(), 10_000);
    }

    #[test]
    fn matches_a_set_model() {
        use std::collections::HashSet;
        let mut c = CountIntervals::new();
        let mut model = HashSet::new();
        for step in 0..500 {
            let l = (step * 37) % 300 + 1;
            let r = l + (step * 11) % 15;
            c.add(l, r);
            model.extend(l..=r);
            assert_eq!(c.count() as usize, model.len(), "step {step}");
        }
    }

    #[test]
    fn full_range() {
        let mut c = CountIntervals::new();
        c.add(1, 1_000_000_000);
        c.add(500, 600);
        assert_eq!(c.count(), 1_000_000_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the map holds disjoint closed intervals. `add` absorbs every stored interval that
overlaps `[left, right]` (they are found from the right end with `range(..=right).next_back()`),
adjusting the running total by removing their sizes, then inserts the union and adds its size.

**Complexity:** amortised `O(log n)` per `add`, `O(1)` per `count`.

**Rust concepts:**
- `while let Some(..) = ...` loops until no overlapping interval is left.
- Copying the entry out (`map(|(&s, &e)| (s, e))`) ends the immutable borrow before `remove`.
- Because the stored intervals are disjoint, the total never exceeds `10^9`, so it fits in the
  `i32` that LeetCode's signature returns.

```rust
use std::collections::BTreeMap;

pub struct CountIntervals {
    intervals: BTreeMap<i32, i32>,
    total: i32,
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl CountIntervals {
    pub fn new() -> Self {
        CountIntervals {
            intervals: BTreeMap::new(),
            total: 0,
        }
    }

    pub fn add(&mut self, left: i32, right: i32) {
        let (mut left, mut right) = (left, right);
        while let Some((s, e)) = self
            .intervals
            .range(..=right)
            .next_back()
            .map(|(&s, &e)| (s, e))
        {
            if e < left {
                break;
            }
            self.intervals.remove(&s);
            self.total -= e - s + 1;
            left = left.min(s);
            right = right.max(e);
        }
        self.intervals.insert(left, right);
        self.total += right - left + 1;
    }

    pub fn count(&self) -> i32 {
        self.total
    }
}
```

Full tested solution: [`src/solutions/p0957_count_integers_in_intervals.rs`](../src/solutions/p0957_count_integers_in_intervals.rs)

</details>
