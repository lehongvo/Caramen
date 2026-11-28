# Problem 0116: Longest Increasing Run

- **Difficulty:** Easy
- **Topics:** returning a sub-slice, run detection with indices, slice ranges
- **Source:** Original · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html)

## Problem

A *run* is a contiguous part of a slice in which every element is **strictly greater** than the
one before it. A single element is a run of length 1.

Implement `longest_increasing_run(values: &[i32]) -> &[i32]`, which returns the longest run as a
sub-slice of `values`. If several runs have the maximum length, return the one that starts
**first**. For an empty input, return an empty slice.

The result must borrow from `values` (do not build a new `Vec`).

## Examples

```text
Input:  values = [5, 1, 2, 3, 2, 4, 6, 8, 1]
Output: [2, 4, 6, 8]

Input:  values = [3, 3, 3]
Output: [3]            (equal neighbours break a run; the first run wins)

Input:  values = [1, 2, 0, 5]
Output: [1, 2]         (both [1, 2] and [0, 5] have length 2)

Input:  values = []
Output: []
```

## Constraints

- `0 <= values.len() <= 10^5`

## Hints

1. Track where the current run started. A run ends at index `i` when `values[i] <= values[i - 1]`,
   or when you reach the end of the slice.
2. Remember the best run as a pair of indices `(start, end)` and slice once at the end:
   `&values[start..end]`.

## Starter code

Write your code in [`src/practice/p0116_longest_increasing_run.rs`](../src/practice/p0116_longest_increasing_run.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0116
```

```rust
pub fn longest_increasing_run(values: &[i32]) -> &[i32] {
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
    fn example_run_in_the_middle() {
        assert_eq!(longest_increasing_run(&[5, 1, 2, 3, 2, 4, 6, 8, 1]), &[2, 4, 6, 8]);
    }

    #[test]
    fn example_equal_values() {
        assert_eq!(longest_increasing_run(&[3, 3, 3]), &[3]);
    }

    #[test]
    fn example_tie_keeps_first_run() {
        assert_eq!(longest_increasing_run(&[1, 2, 0, 5]), &[1, 2]);
    }

    #[test]
    fn example_empty() {
        assert!(longest_increasing_run(&[]).is_empty());
    }

    #[test]
    fn run_at_the_end_and_whole_slice() {
        assert_eq!(longest_increasing_run(&[9, 1, 2, 3]), &[1, 2, 3]);
        assert_eq!(longest_increasing_run(&[-3, -1, 4]), &[-3, -1, 4]);
        assert_eq!(longest_increasing_run(&[7]), &[7]);
    }

    #[test]
    fn result_points_into_the_input() {
        let values = vec![4, 3, 5, 6];
        let run = longest_increasing_run(&values);
        assert_eq!(run.as_ptr(), values[1..].as_ptr());
    }

    #[test]
    fn large_input() {
        // Runs of length 100: 0..=99, 0..=99, ... Overwriting indices 50_000..50_300 with
        // 1000..=1299 extends the run that starts at index 49_900 (0, 1, ..., 99, 1000, ...).
        let mut values: Vec<i32> = (0..100_000).map(|i| i % 100).collect();
        for (k, v) in values[50_000..50_300].iter_mut().enumerate() {
            *v = 1_000 + k as i32;
        }
        let run = longest_increasing_run(&values);
        assert_eq!(run.len(), 400);
        assert_eq!(run[0], 0);
        assert_eq!(run[399], 1_299);
        assert_eq!(run.as_ptr(), values[49_900..].as_ptr());
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** scan once. `start` marks the beginning of the current run. At every index where the run
breaks (and once more at the end), compare the finished run `start..i` with the best one so far,
keeping the earlier run on ties by using a strict `>`.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- Working with indices and slicing only at the end keeps the borrow simple: the function returns
  one sub-slice tied to `values` by lifetime elision.
- Iterating `1..=values.len()` lets the same code close the final run (`i == values.len()`).
- `||` short-circuits, so `values[i]` is never read when `i == values.len()`.

```rust
pub fn longest_increasing_run(values: &[i32]) -> &[i32] {
    let (mut best_start, mut best_end) = (0, 0);
    let mut start = 0;
    for i in 1..=values.len() {
        if i == values.len() || values[i] <= values[i - 1] {
            // The run `start..i` has just ended.
            if i - start > best_end - best_start {
                best_start = start;
                best_end = i;
            }
            start = i;
        }
    }
    &values[best_start..best_end]
}
```

Full tested solution: [`src/solutions/p0116_longest_increasing_run.rs`](../src/solutions/p0116_longest_increasing_run.rs)

</details>
