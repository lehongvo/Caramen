# Problem 0904: Recursive `max!` and `min!` Macros

- **Difficulty:** Medium
- **Topics:** `macro_rules!`, `$x:expr` fragments, repetitions `$(...),+`, recursive macros, trailing commas
- **Source:** Original · [The Rust Book 20.5](https://doc.rust-lang.org/book/ch20-05-macros.html) · [Rust by Example: Variadic interfaces](https://doc.rust-lang.org/rust-by-example/macros/variadics.html)

## Problem

`std::cmp::max` takes exactly two arguments. Write two **variadic** macros:

- `max!(a)` is `a`; `max!(a, b, c, ...)` is the largest of its arguments.
- `min!(a)` is `a`; `min!(a, b, c, ...)` is the smallest.

Rules:

- The arguments only need `PartialOrd`, so the macros also work for `f64` and `&str`.
- On ties the **leftmost** of the equal values wins (this is visible with values that compare
  equal but are not identical, see the tests).
- Every argument expression is evaluated **exactly once**, from left to right.
- A trailing comma is accepted: `max!(1, 2,)`.

Also write `clamp_all(values: &[i32], low: i32, high: i32) -> Vec<i32>` that clamps every value
into `[low, high]` using your macros (`max!(low, min!(v, high))`).

## Examples

```text
max!(3)                  => 3
max!(1, 7, 3)            => 7
min!(2.5, -1.0, 4.0)     => -1.0
max!("pear", "apple",)   => "pear"
clamp_all(&[-5, 3, 12], 0, 10) => [0, 3, 10]
```

## Constraints

- At least one argument (`max!()` does not need to compile).

## Hints

1. Two arms: one for a single expression, one for "first expression, then one or more others".
2. In the second arm, call the macro recursively on the rest: `max!($($rest),+)`.
3. Bind each piece to a `let` inside a block `{{ ... }}` so it is evaluated only once.

## Starter code

Write your code in [`src/practice/p0904_recursive_max_min_macros.rs`](../src/practice/p0904_recursive_max_min_macros.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0904
```

```rust
/// Expands to the largest argument; the leftmost one on ties.
macro_rules! max {
    ($($t:tt)*) => {
        todo!()
    };
}

/// Expands to the smallest argument; the leftmost one on ties.
macro_rules! min {
    ($($t:tt)*) => {
        todo!()
    };
}

/// Clamps every value into `[low, high]` with `max!` and `min!`.
pub fn clamp_all(values: &[i32], low: i32, high: i32) -> Vec<i32> {
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
    use std::cell::RefCell;
    use std::cmp::Ordering;

    /// Compares by `key` only, so two values can be equal but distinguishable by `tag`.
    #[derive(Debug, Clone, Copy)]
    struct Tagged {
        key: i32,
        tag: char,
    }

    impl PartialEq for Tagged {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    impl PartialOrd for Tagged {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.key.partial_cmp(&other.key)
        }
    }

    #[test]
    fn integers_floats_and_strs() {
        let one: i32 = max!(3);
        let big: i32 = max!(1, 7, 3);
        let small: i32 = min!(4, -2, 9, -2, 0);
        let float: f64 = min!(2.5, -1.0, 4.0);
        let word: &str = max!("pear", "apple",);
        assert_eq!((one, big, small, float, word), (3, 7, -2, -1.0, "pear"));
    }

    #[test]
    fn leftmost_wins_on_ties() {
        let a = Tagged { key: 5, tag: 'a' };
        let b = Tagged { key: 5, tag: 'b' };
        let c = Tagged { key: 1, tag: 'c' };
        let largest: Tagged = max!(c, a, b);
        let smallest: Tagged = min!(a, b);
        assert_eq!((largest.tag, smallest.tag), ('a', 'a'));
    }

    #[test]
    fn each_argument_is_evaluated_once_left_to_right() {
        let log = RefCell::new(Vec::new());
        let value = |n: i32| {
            log.borrow_mut().push(n);
            n
        };
        let result: i32 = max!(value(2), value(9), value(4), value(1));
        assert_eq!(result, 9);
        assert_eq!(*log.borrow(), vec![2, 9, 4, 1]);
    }

    #[test]
    fn clamp_all_uses_both_macros() {
        assert_eq!(clamp_all(&[-5, 3, 12], 0, 10), vec![0, 3, 10]);
        assert_eq!(clamp_all(&[], 0, 1), Vec::<i32>::new());
        assert_eq!(clamp_all(&[i32::MIN, i32::MAX], -1, 1), vec![-1, 1]);
    }

    #[test]
    fn many_arguments() {
        let m: u64 = max!(
            10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 5, 15, 25, 35, 45, 55, 65, 75, 85, 95, 99
        );
        let n: u64 = min!(
            10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 5, 15, 25, 35, 45, 55, 65, 75, 85, 95, 99
        );
        assert_eq!((m, n), (100, 5));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the recursive arm evaluates the first argument into `first`, recursively computes the
maximum of the remaining arguments into `rest`, and keeps `first` unless `rest` is strictly
larger. That keeps the leftmost value on ties and evaluates each expression once, left to right.

**Rust concepts:**
- Macro arms are tried top to bottom; the first one that matches is expanded.
- `$($rest:expr),+` captures one or more comma-separated expressions; `$(,)?` makes a trailing
  comma optional.
- The double braces `{{ ... }}` produce a **block expression**, so `let` statements are allowed
  inside an expression position, and the macro's local variables are hygienic: they cannot clash
  with the caller's variables named `first` or `rest`.
- A `macro_rules!` macro defined in a file is visible to the code **after** it in that file,
  including the `tests` module, without `#[macro_export]`.

```rust
/// Expands to the largest argument; the leftmost one on ties.
macro_rules! max {
    ($x:expr $(,)?) => {
        $x
    };
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let first = $first;
        let rest = max!($($rest),+);
        if rest > first { rest } else { first }
    }};
}

/// Expands to the smallest argument; the leftmost one on ties.
macro_rules! min {
    ($x:expr $(,)?) => {
        $x
    };
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let first = $first;
        let rest = min!($($rest),+);
        if rest < first { rest } else { first }
    }};
}

/// Clamps every value into `[low, high]` with `max!` and `min!`.
pub fn clamp_all(values: &[i32], low: i32, high: i32) -> Vec<i32> {
    values.iter().map(|&v| max!(low, min!(v, high))).collect()
}
```

Full tested solution: [`src/solutions/p0904_recursive_max_min_macros.rs`](../src/solutions/p0904_recursive_max_min_macros.rs)

</details>
