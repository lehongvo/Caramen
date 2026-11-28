# Problem 0249: Describe Pairs with Match Guards

- **Difficulty:** Easy
- **Topics:** match guards `if ...`, guards that use outer variables, guards with `|` alternatives, arm order
- **Source:** Original · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#extra-conditionals-with-match-guards)

## Problem

A **match guard** is an extra `if` condition on a `match` arm: the arm is chosen only if the
pattern matches *and* the condition is true. Implement three functions with guards:

- `describe_point(x: i32, y: i32) -> &'static str`, checking the rules in this order:
  `"origin"` if both are 0; `"on an axis"` if exactly one is 0; `"diagonal"` if `x == y`;
  `"anti-diagonal"` if `x == -y`; `"same sign"` if both are positive or both negative;
  `"mixed signs"` otherwise.
- `compare_reading(reading: Option<i32>, limit: i32) -> &'static str`: `"missing"` for `None`,
  `"over"` if the value is above `limit`, `"at limit"` if equal, `"under"` otherwise. Note that
  `limit` is a *variable*: it must be compared in a guard (a pattern `Some(limit)` would create a
  **new** variable that matches anything).
- `ticket_price(age: u8, student: bool) -> u32`: `0..=5` free (`0`); `6..=17` costs `6`; ages
  `18..=25` **or** `60` and above cost `8` if `student` is true; ages `60` and above cost `9`
  otherwise; everybody else pays `12`.

## Examples

```text
describe_point(0, 0)   => "origin"
describe_point(0, 7)   => "on an axis"
describe_point(3, 3)   => "diagonal"
describe_point(-4, 4)  => "anti-diagonal"
describe_point(-1, -9) => "same sign"
describe_point(2, -5)  => "mixed signs"
compare_reading(Some(120), 100) => "over"
compare_reading(Some(100), 100) => "at limit"
compare_reading(None, 100)      => "missing"
ticket_price(20, true)  => 8
ticket_price(70, true)  => 8
ticket_price(70, false) => 9
ticket_price(30, true)  => 12
```

## Constraints

- No special constraints (`x == -y` must not overflow: think about `i32::MIN`).

## Hints

1. `(0, _) | (_, 0) => "on an axis"` works after the `(0, 0)` arm has been handled.
2. `Some(v) if v > limit => "over"` uses the outer variable `limit` inside the guard.
3. In `18..=25 | 60.. if student => 8`, the guard applies to **both** alternatives.

## Starter code

Write your code in [`src/practice/p0249_match_guards_pairs.rs`](../src/practice/p0249_match_guards_pairs.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0249
```

```rust
/// Describes where a point lies relative to the axes and diagonals.
pub fn describe_point(x: i32, y: i32) -> &'static str {
    todo!("Write your solution here")
}

/// Compares an optional reading with a limit.
pub fn compare_reading(reading: Option<i32>, limit: i32) -> &'static str {
    todo!("Write your solution here")
}

/// The ticket price for an age and student status.
pub fn ticket_price(age: u8, student: bool) -> u32 {
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
    fn points() {
        assert_eq!(describe_point(0, 0), "origin");
        assert_eq!(describe_point(0, 7), "on an axis");
        assert_eq!(describe_point(-2, 0), "on an axis");
        assert_eq!(describe_point(3, 3), "diagonal");
        assert_eq!(describe_point(-4, 4), "anti-diagonal");
        assert_eq!(describe_point(-1, -9), "same sign");
        assert_eq!(describe_point(5, 1), "same sign");
        assert_eq!(describe_point(2, -5), "mixed signs");
    }

    #[test]
    fn points_at_the_extremes() {
        assert_eq!(describe_point(i32::MIN, i32::MIN), "diagonal");
        assert_eq!(describe_point(i32::MAX, i32::MIN), "mixed signs");
        assert_eq!(describe_point(i32::MAX, -i32::MAX), "anti-diagonal");
    }

    #[test]
    fn readings_use_the_outer_limit() {
        assert_eq!(compare_reading(Some(120), 100), "over");
        assert_eq!(compare_reading(Some(100), 100), "at limit");
        assert_eq!(compare_reading(Some(-5), 100), "under");
        assert_eq!(compare_reading(None, 100), "missing");
        assert_eq!(compare_reading(Some(7), 7), "at limit");
    }

    #[test]
    fn ticket_prices() {
        assert_eq!(ticket_price(3, true), 0);
        assert_eq!(ticket_price(12, false), 6);
        assert_eq!(ticket_price(20, true), 8);
        assert_eq!(ticket_price(20, false), 12);
        assert_eq!(ticket_price(70, true), 8);
        assert_eq!(ticket_price(70, false), 9);
        assert_eq!(ticket_price(30, true), 12);
        assert_eq!(ticket_price(255, false), 9);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** translate each rule into one arm, in the stated order; patterns pick the shape, and
guards add the conditions that patterns cannot express (comparisons between two values, or
with a variable).

**Rust concepts:**
- A guard is checked only after its pattern matched; if it is false, matching continues with
  the next arm.
- The compiler does not use guards for exhaustiveness: a final arm without a guard (`_` or a
  plain binding) is required.
- `p | q if cond` means `(p | q) if cond`: the guard covers every alternative.
- `x == -y` overflows for `y == i32::MIN`; `i64::from(x) == -i64::from(y)` avoids it.
- `x.signum() == y.signum()` tests "same sign" without multiplying (which could overflow).

```rust
/// Describes where a point lies relative to the axes and diagonals.
pub fn describe_point(x: i32, y: i32) -> &'static str {
    match (x, y) {
        (0, 0) => "origin",
        (0, _) | (_, 0) => "on an axis",
        (a, b) if a == b => "diagonal",
        (a, b) if i64::from(a) == -i64::from(b) => "anti-diagonal",
        (a, b) if a.signum() == b.signum() => "same sign",
        _ => "mixed signs",
    }
}

/// Compares an optional reading with a limit.
pub fn compare_reading(reading: Option<i32>, limit: i32) -> &'static str {
    match reading {
        None => "missing",
        Some(v) if v > limit => "over",
        Some(v) if v == limit => "at limit",
        Some(_) => "under",
    }
}

/// The ticket price for an age and student status.
pub fn ticket_price(age: u8, student: bool) -> u32 {
    match age {
        0..=5 => 0,
        6..=17 => 6,
        18..=25 | 60.. if student => 8,
        60.. => 9,
        _ => 12,
    }
}
```

Full tested solution: [`src/solutions/p0249_match_guards_pairs.rs`](../src/solutions/p0249_match_guards_pairs.rs)

</details>
