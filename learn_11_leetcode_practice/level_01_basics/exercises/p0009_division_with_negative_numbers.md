# Problem 0009: Division with Negative Numbers

- **Difficulty:** Easy
- **Topics:** integer `/` and `%` with negative operands, `div_euclid`, `rem_euclid`, tuples
- **Source:** Original · [`i32::rem_euclid`](https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid)

## Problem

Integer division is easy for positive numbers, but programming languages disagree about what
`-7 / 2` should be. Rust defines two families of operations, and this problem asks you to use
both:

1. `truncating_div_rem(a: i32, b: i32) -> (i32, i32)` returns `(a / b, a % b)` with Rust's
   built-in operators. The quotient is rounded **toward zero** and the remainder has the same
   sign as `a` (or is zero).
2. `euclidean_div_rem(a: i32, b: i32) -> (i32, i32)` returns `(q, r)` such that
   `a == b * q + r` and `0 <= r < |b|`. The remainder is **never negative**.

In both cases `a == b * quotient + remainder` holds.

## Examples

```text
a   b    truncating_div_rem   euclidean_div_rem
 7   2   ( 3,  1)             ( 3, 1)
-7   2   (-3, -1)             (-4, 1)     -7 = 2 * (-4) + 1
 7  -2   (-3,  1)             (-3, 1)      7 = (-2) * (-3) + 1
-7  -2   ( 3, -1)             ( 4, 1)     -7 = (-2) * 4 + 1
```

## Constraints

- `b != 0`
- `(a, b) != (i32::MIN, -1)` (that division overflows, because `2147483648` does not fit in an `i32`)

## Hints

1. The first function is a one-liner with `/` and `%`.
2. Every integer type has the methods `div_euclid` and `rem_euclid`.

## Starter code

Write your code in [`src/practice/p0009_division_with_negative_numbers.rs`](../src/practice/p0009_division_with_negative_numbers.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0009
```

```rust
pub fn truncating_div_rem(a: i32, b: i32) -> (i32, i32) {
    todo!("Write your solution here")
}

pub fn euclidean_div_rem(a: i32, b: i32) -> (i32, i32) {
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
    fn truncating_examples() {
        assert_eq!(truncating_div_rem(7, 2), (3, 1));
        assert_eq!(truncating_div_rem(-7, 2), (-3, -1));
        assert_eq!(truncating_div_rem(7, -2), (-3, 1));
        assert_eq!(truncating_div_rem(-7, -2), (3, -1));
    }

    #[test]
    fn euclidean_examples() {
        assert_eq!(euclidean_div_rem(7, 2), (3, 1));
        assert_eq!(euclidean_div_rem(-7, 2), (-4, 1));
        assert_eq!(euclidean_div_rem(7, -2), (-3, 1));
        assert_eq!(euclidean_div_rem(-7, -2), (4, 1));
    }

    #[test]
    fn exact_division_has_zero_remainder() {
        assert_eq!(truncating_div_rem(-12, 4), (-3, 0));
        assert_eq!(euclidean_div_rem(-12, 4), (-3, 0));
    }

    #[test]
    fn identity_holds_for_many_pairs() {
        for a in -20..=20 {
            for b in [-7, -3, -1, 1, 2, 5] {
                let (q, r) = truncating_div_rem(a, b);
                assert_eq!(b * q + r, a);
                let (q, r) = euclidean_div_rem(a, b);
                assert_eq!(b * q + r, a);
                assert!((0..b.abs()).contains(&r));
            }
        }
    }

    #[test]
    fn extreme_values() {
        assert_eq!(truncating_div_rem(i32::MIN, 1), (i32::MIN, 0));
        assert_eq!(euclidean_div_rem(i32::MIN, 10), (-214_748_365, 2));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `/` and `%` already implement truncating division. `i32::div_euclid` and
`i32::rem_euclid` implement Euclidean division, whose remainder is always in `0..|b|`.

**Rust concepts:**
- Rust's `/` on integers truncates toward zero, and `%` is the matching remainder, so its sign
  follows the dividend. This is the same as C, C++, Java and JavaScript, but different from
  Python, where `-7 // 2 == -4`.
- `rem_euclid` is what you want whenever a remainder is used as an **index** or a position on a
  circle (clock hours, days of the week): it never goes negative.
- Dividing by zero panics, and `i32::MIN / -1` overflows; that is why the constraints exclude them.
  `checked_div` returns `None` in both cases instead of panicking.

```rust
pub fn truncating_div_rem(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

pub fn euclidean_div_rem(a: i32, b: i32) -> (i32, i32) {
    (a.div_euclid(b), a.rem_euclid(b))
}
```

Full tested solution: [`src/solutions/p0009_division_with_negative_numbers.rs`](../src/solutions/p0009_division_with_negative_numbers.rs)

</details>
