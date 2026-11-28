# Problem 0477: A Fibonacci Iterator that Stops on Overflow

- **Difficulty:** Easy
- **Topics:** custom `Iterator`, `checked_add` + `Option` state, `?` inside `next`, `take_while`, `last`, `count`, `nth`
- **Source:** Original · [std `Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) · [std `u64::checked_add`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add)

## Problem

The Fibonacci numbers are `F(0) = 0`, `F(1) = 1`, `F(n) = F(n - 1) + F(n - 2)`. They grow so fast
that `F(94)` no longer fits in a `u64`. Write an iterator that yields `F(0), F(1), F(2), ...` and
**ends** (returns `None` from then on) instead of overflowing: its last item is the largest
Fibonacci number that fits in a `u64`.

Implement:

- `Fibonacci::new() -> Fibonacci` and `impl Iterator for Fibonacci` (`type Item = u64`).
  (`Default` is already implemented by calling `new`.)
- `fib_below(limit: u64) -> Vec<u64>`: all Fibonacci numbers strictly below `limit`, in order.
- `even_fib_sum(limit: u64) -> u64`: the sum of the **even** Fibonacci numbers `<= limit`.
- `nth_fib(n: usize) -> Option<u64>`: `F(n)`, or `None` if it does not fit in a `u64`.
- `largest_fib() -> u64` and `fib_count() -> usize`: the last item of the iterator and how many
  items it yields in total.

## Examples

```text
Fibonacci::new() yields 0, 1, 1, 2, 3, 5, 8, 13, ...
fib_below(20)          -> [0, 1, 1, 2, 3, 5, 8, 13]
even_fib_sum(100)      -> 44            (0 + 2 + 8 + 34)
nth_fib(10)            -> Some(55)
nth_fib(94)            -> None
largest_fib()          -> 12200160415121876738   (F(93))
fib_count()            -> 94            (F(0) through F(93))
```

## Constraints

- No special constraints.

## Hints

1. Keep the state as two `Option<u64>` values: the value to return now and the next one. When
   `checked_add` fails, the "next" becomes `None`.
2. In `next`, `let value = self.current?;` ends the iteration once `current` is `None`.
3. The helpers are one adapter chain each: `take_while`, `filter`, `nth`, `last`, `count`.

## Starter code

Write your code in [`src/practice/p0477_fibonacci_iterator.rs`](../src/practice/p0477_fibonacci_iterator.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0477
```

```rust
pub struct Fibonacci {}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        todo!("Write your solution here")
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Write your solution here")
    }
}

pub fn fib_below(limit: u64) -> Vec<u64> {
    todo!("Write your solution here")
}

pub fn even_fib_sum(limit: u64) -> u64 {
    todo!("Write your solution here")
}

pub fn nth_fib(n: usize) -> Option<u64> {
    todo!("Write your solution here")
}

pub fn largest_fib() -> u64 {
    todo!("Write your solution here")
}

pub fn fib_count() -> usize {
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
    fn first_numbers() {
        let first: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(first, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
        assert_eq!(Fibonacci::default().nth(2), Some(1));
    }

    #[test]
    fn below_a_limit() {
        assert_eq!(fib_below(20), vec![0, 1, 1, 2, 3, 5, 8, 13]);
        assert_eq!(fib_below(0), Vec::<u64>::new());
        assert_eq!(fib_below(1), vec![0]);
    }

    #[test]
    fn even_sums() {
        assert_eq!(even_fib_sum(100), 44);
        assert_eq!(even_fib_sum(4_000_000), 4_613_732);
        assert_eq!(even_fib_sum(0), 0);
    }

    #[test]
    fn nth_numbers() {
        assert_eq!(nth_fib(10), Some(55));
        assert_eq!(nth_fib(93), Some(12_200_160_415_121_876_738));
        assert_eq!(nth_fib(94), None);
    }

    #[test]
    fn stops_before_overflow_and_stays_done() {
        assert_eq!(largest_fib(), 12_200_160_415_121_876_738);
        assert_eq!(fib_count(), 94);
        let mut it = Fibonacci::new();
        assert_eq!(it.by_ref().count(), 94);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** shift a window of two values along the sequence. The next-next value is computed with
`checked_add`; once it overflows, the window slides into `None`s and the iterator ends naturally,
after having yielded the last representable number.

**Rust concepts:**
- `?` works inside `next` because `next` returns an `Option`.
- `Option::and_then` chains the addition only while there is a next value.
- The iterator is finite (94 items), so `last()` and `count()` terminate; on an infinite
  iterator they would loop forever.
- `take_while` stops at the first number that is too large; the Fibonacci sequence is
  non-decreasing, so no later number could qualify again.

```rust
pub struct Fibonacci {
    current: Option<u64>,
    next: Option<u64>,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci {
            current: Some(0),
            next: Some(1),
        }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current?;
        self.current = self.next;
        self.next = self.next.and_then(|n| value.checked_add(n));
        Some(value)
    }
}

pub fn fib_below(limit: u64) -> Vec<u64> {
    Fibonacci::new().take_while(|&f| f < limit).collect()
}

pub fn even_fib_sum(limit: u64) -> u64 {
    Fibonacci::new()
        .take_while(|&f| f <= limit)
        .filter(|f| f % 2 == 0)
        .sum()
}

pub fn nth_fib(n: usize) -> Option<u64> {
    Fibonacci::new().nth(n)
}

pub fn largest_fib() -> u64 {
    Fibonacci::new().last().unwrap_or(0)
}

pub fn fib_count() -> usize {
    Fibonacci::new().count()
}
```

Full tested solution: [`src/solutions/p0477_fibonacci_iterator.rs`](../src/solutions/p0477_fibonacci_iterator.rs)

</details>
