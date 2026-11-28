# Problem 0030: Float Rounding Modes

- **Difficulty:** Easy
- **Topics:** `f64::floor`, `ceil`, `round`, `trunc`, float-to-integer `as` casts (saturating, NaN), tuples
- **Source:** Original · [`f64` methods](https://doc.rust-lang.org/std/primitive.f64.html#method.round)

## Problem

Converting a float to an integer requires choosing a rounding rule. Write
`rounding_modes(x: f64) -> (i64, i64, i64, i64)` that returns `(floor, ceil, round, trunc)` of
`x`, each converted to `i64`:

- **floor**: the largest integer `<= x`;
- **ceil**: the smallest integer `>= x`;
- **round**: the nearest integer, with halfway cases rounded **away from zero** (`2.5 -> 3`, `-2.5 -> -3`);
- **trunc**: drop the fractional part (round toward zero).

Values that do not fit in an `i64` must become `i64::MAX` or `i64::MIN` (whichever is closer),
and NaN ("not a number") must become `0`. Infinity counts as "too large".

## Examples

```text
rounding_modes(2.5)    -> (2, 3, 3, 2)
rounding_modes(-2.5)   -> (-3, -2, -3, -2)
rounding_modes(7.0)    -> (7, 7, 7, 7)
rounding_modes(-0.4)   -> (-1, 0, 0, 0)
rounding_modes(1e300)  -> (i64::MAX, i64::MAX, i64::MAX, i64::MAX)
rounding_modes(f64::NAN) -> (0, 0, 0, 0)
```

## Constraints

- Any `f64` value, including infinities and NaN.

## Hints

1. `f64` has methods named exactly `floor`, `ceil`, `round` and `trunc`; they return an `f64`.
2. Casting a float with `as i64` already saturates out-of-range values and maps NaN to 0.

## Starter code

Write your code in [`src/practice/p0030_float_rounding_modes.rs`](../src/practice/p0030_float_rounding_modes.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0030
```

```rust
pub fn rounding_modes(x: f64) -> (i64, i64, i64, i64) {
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
    fn halfway_cases() {
        assert_eq!(rounding_modes(2.5), (2, 3, 3, 2));
        assert_eq!(rounding_modes(-2.5), (-3, -2, -3, -2));
        assert_eq!(rounding_modes(0.5), (0, 1, 1, 0));
    }

    #[test]
    fn whole_numbers_are_unchanged() {
        assert_eq!(rounding_modes(7.0), (7, 7, 7, 7));
        assert_eq!(rounding_modes(-7.0), (-7, -7, -7, -7));
        assert_eq!(rounding_modes(0.0), (0, 0, 0, 0));
    }

    #[test]
    fn small_fractions() {
        assert_eq!(rounding_modes(-0.4), (-1, 0, 0, 0));
        assert_eq!(rounding_modes(1.2), (1, 2, 1, 1));
        assert_eq!(rounding_modes(-1.7), (-2, -1, -2, -1));
    }

    #[test]
    fn out_of_range_values_saturate() {
        let max = (i64::MAX, i64::MAX, i64::MAX, i64::MAX);
        let min = (i64::MIN, i64::MIN, i64::MIN, i64::MIN);
        assert_eq!(rounding_modes(1e300), max);
        assert_eq!(rounding_modes(f64::INFINITY), max);
        assert_eq!(rounding_modes(-1e300), min);
        assert_eq!(rounding_modes(f64::NEG_INFINITY), min);
    }

    #[test]
    fn nan_becomes_zero() {
        assert_eq!(rounding_modes(f64::NAN), (0, 0, 0, 0));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** call the four rounding methods and cast each result with `as i64`. The cast takes
care of the out-of-range and NaN rules.

**Rust concepts:**
- Float-to-integer `as` casts **saturate** (since Rust 1.45): too-large values become `MAX`,
  too-small ones become `MIN`, and NaN becomes `0`. Unlike integer-to-integer casts, nothing
  wraps around.
- `x as i64` on its own truncates toward zero, so `trunc` is redundant before the cast; it is
  written out here to mirror the four rules.
- `f64::round` rounds half away from zero; `round_ties_even` rounds `2.5` to `2` ("banker's rounding").
- Float methods are called with method syntax: `x.floor()`.

```rust
pub fn rounding_modes(x: f64) -> (i64, i64, i64, i64) {
    (
        x.floor() as i64,
        x.ceil() as i64,
        x.round() as i64,
        x.trunc() as i64,
    )
}
```

Full tested solution: [`src/solutions/p0030_float_rounding_modes.rs`](../src/solutions/p0030_float_rounding_modes.rs)

</details>
