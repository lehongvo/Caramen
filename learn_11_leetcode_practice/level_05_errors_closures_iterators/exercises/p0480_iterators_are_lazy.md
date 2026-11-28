# Problem 0480: Iterators Are Lazy

- **Difficulty:** Medium
- **Topics:** laziness, short-circuiting consumers, counting closure calls with `FnMut`, `take`, `take_while` consuming one extra item, item-by-item processing
- **Source:** Original · [The Rust Book 13.2: Processing a series of items with iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) · [std `iter`: Laziness](https://doc.rust-lang.org/std/iter/index.html#laziness)

## Problem

Iterator adapters do **nothing** until a consumer asks for items, and then they process the items
**one at a time** through the whole chain. Consumers that can stop early (`find`, `any`, or
`collect` after `take`, ...) stop pulling as soon as they have their answer. Each function below
returns, along with its result, how much work the closures actually did.

Implement:

- `built_but_never_run(nums: &[i32]) -> usize`: build `nums.iter().map(...)` where the closure
  counts its calls, but never consume it. Return the number of calls (it must be `0`).
- `first_square_over(nums: &[i64], limit: i64) -> (Option<i64>, usize)`: the first square
  greater than `limit` (squares computed by `map`, searched with `find`), and how many squares
  were computed.
- `first_doubled(nums: &[i32], n: usize) -> (Vec<i32>, usize)`: the first `n` values of
  `nums` doubled (with `map` + `take`), and how many times the doubling closure ran.
- `sum_until_negative(nums: &[i32]) -> (i32, usize)`: the sum of the numbers before the first
  negative one (with `take_while`), and how many numbers the predicate examined.
- `trace_first_evens(nums: &[i32], n: usize) -> Vec<String>`: find the first `n` even numbers with
  a single `filter_map` closure that logs `"check X"` for every number it examines, then append
  `"found [..]"` (the `{:?}` of the found numbers) to the log and return the log.

## Examples

```text
built_but_never_run(&[1, 2, 3])             -> 0
first_square_over(&[1, 2, 3, 4, 5], 5)      -> (Some(9), 3)
first_doubled(&[1, 2, 3, 4, 5], 2)          -> ([2, 4], 2)
sum_until_negative(&[1, 2, -1, 5])          -> (3, 3)     (-1 is examined, then iteration stops)
trace_first_evens(&[1, 2, 3, 4, 5, 6], 2)   -> ["check 1", "check 2", "check 3", "check 4", "found [2, 4]"]
```

## Constraints

- No special constraints.

## Hints

1. A counter `let mut calls = 0;` can be incremented inside an adapter's closure (it is `FnMut`).
   The counter stays borrowed until the iterator is consumed or dropped, so read it afterwards.
2. `take(n)` stops pulling after `n` items: the `map` before it runs only `n` times.
3. `take_while` has to look at the first failing item to know it must stop.

## Starter code

Write your code in [`src/practice/p0480_iterators_are_lazy.rs`](../src/practice/p0480_iterators_are_lazy.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0480
```

```rust
pub fn built_but_never_run(nums: &[i32]) -> usize {
    todo!("Write your solution here")
}

pub fn first_square_over(nums: &[i64], limit: i64) -> (Option<i64>, usize) {
    todo!("Write your solution here")
}

pub fn first_doubled(nums: &[i32], n: usize) -> (Vec<i32>, usize) {
    todo!("Write your solution here")
}

pub fn sum_until_negative(nums: &[i32]) -> (i32, usize) {
    todo!("Write your solution here")
}

pub fn trace_first_evens(nums: &[i32], n: usize) -> Vec<String> {
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
    fn nothing_runs_without_a_consumer() {
        assert_eq!(built_but_never_run(&[1, 2, 3]), 0);
        assert_eq!(built_but_never_run(&[]), 0);
    }

    #[test]
    fn find_stops_at_the_first_match() {
        assert_eq!(first_square_over(&[1, 2, 3, 4, 5], 5), (Some(9), 3));
        assert_eq!(first_square_over(&[1, 2], 100), (None, 2));
        assert_eq!(first_square_over(&[-20, 1, 2], 5), (Some(400), 1));
    }

    #[test]
    fn take_limits_the_work() {
        assert_eq!(first_doubled(&[1, 2, 3, 4, 5], 2), (vec![2, 4], 2));
        assert_eq!(first_doubled(&[1, 2], 10), (vec![2, 4], 2));
        assert_eq!(first_doubled(&[1, 2, 3], 0), (vec![], 0));
    }

    #[test]
    fn take_while_examines_the_failing_item() {
        assert_eq!(sum_until_negative(&[1, 2, -1, 5]), (3, 3));
        assert_eq!(sum_until_negative(&[1, 2, 3]), (6, 3));
        assert_eq!(sum_until_negative(&[-1, 2, 3]), (0, 1));
    }

    #[test]
    fn items_flow_one_at_a_time() {
        assert_eq!(
            trace_first_evens(&[1, 2, 3, 4, 5, 6], 2),
            vec!["check 1", "check 2", "check 3", "check 4", "found [2, 4]"]
        );
        assert_eq!(
            trace_first_evens(&[1, 3], 1),
            vec!["check 1", "check 3", "found []"]
        );
        assert_eq!(trace_first_evens(&[2, 4], 0), vec!["found []"]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** put a counter inside each closure and observe it after the consumer finishes. The
numbers reveal the evaluation strategy: nothing happens without a consumer, each item travels
through every adapter before the next item starts, and early-stopping consumers skip the rest.

**Rust concepts:**
- Adapters such as `map` are `#[must_use]`: the compiler warns when you build one and drop it,
  because the closure would never run.
- Closures capturing `calls` by mutable reference make the iterator borrow `calls`; the borrow
  ends when the iterator is consumed (or goes out of scope), and then `calls` can be read.
- `find`, `take` and `take_while` are lazy and short-circuiting; `count`, `sum` and `collect` on
  an unbounded chain would consume everything.
- Laziness is what makes infinite iterators and efficient pipelines possible.

```rust
pub fn built_but_never_run(nums: &[i32]) -> usize {
    let mut calls = 0;
    {
        let _pipeline = nums.iter().map(|x| {
            calls += 1;
            x * 2
        });
        // `_pipeline` is dropped here without ever being consumed.
    }
    calls
}

pub fn first_square_over(nums: &[i64], limit: i64) -> (Option<i64>, usize) {
    let mut calls = 0;
    let found = nums
        .iter()
        .map(|x| {
            calls += 1;
            x * x
        })
        .find(|&sq| sq > limit);
    (found, calls)
}

pub fn first_doubled(nums: &[i32], n: usize) -> (Vec<i32>, usize) {
    let mut calls = 0;
    let doubled = nums
        .iter()
        .map(|x| {
            calls += 1;
            x * 2
        })
        .take(n)
        .collect();
    (doubled, calls)
}

pub fn sum_until_negative(nums: &[i32]) -> (i32, usize) {
    let mut examined = 0;
    let sum = nums
        .iter()
        .take_while(|&&x| {
            examined += 1;
            x >= 0
        })
        .sum();
    (sum, examined)
}

pub fn trace_first_evens(nums: &[i32], n: usize) -> Vec<String> {
    let mut log = Vec::new();
    let found: Vec<i32> = nums
        .iter()
        .filter_map(|&x| {
            log.push(format!("check {x}"));
            (x % 2 == 0).then_some(x)
        })
        .take(n)
        .collect();
    log.push(format!("found {found:?}"));
    log
}
```

Full tested solution: [`src/solutions/p0480_iterators_are_lazy.rs`](../src/solutions/p0480_iterators_are_lazy.rs)

</details>
