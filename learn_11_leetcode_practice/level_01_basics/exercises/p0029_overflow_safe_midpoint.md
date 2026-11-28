# Problem 0029: Overflow-Safe Midpoint

- **Difficulty:** Easy
- **Topics:** overflow in intermediate results, widening to `i64`, `div_euclid`, `as` casts, `Option`
- **Source:** Original · [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow)

## Problem

The classic bug in binary search is `mid = (low + high) / 2`: the sum can overflow even though
the midpoint itself always fits. Write two functions that avoid this:

1. `midpoint(a: i32, b: i32) -> i32` returns `floor((a + b) / 2)`, the mathematical average
   **rounded down** (toward negative infinity), for any two `i32` values.
2. `average_floor(nums: &[i32]) -> Option<i32>` returns the average of all values, rounded down
   the same way, or `None` for an empty slice.

## Examples

```text
midpoint(1, 2)                  -> 1
midpoint(-3, 0)                 -> -2      floor(-1.5) = -2 (plain `/` would give -1)
midpoint(i32::MAX, i32::MAX)    -> 2147483647
midpoint(i32::MIN, i32::MAX)    -> -1      floor(-0.5) = -1

average_floor(&[1, 2, 4])       -> Some(2) 7 / 3 = 2.33...
average_floor(&[-1, -2])        -> Some(-2)
average_floor(&[])              -> None
```

## Constraints

- `0 <= nums.len() <= 10^5`; elements are any `i32`.

## Hints

1. Do the arithmetic in `i64`, where the sum of two (or `10^5`) `i32` values cannot overflow.
2. `/` rounds toward zero, but `div_euclid` with a positive divisor rounds down.
3. The average of `i32` values is always between their minimum and maximum, so converting the
   result back with `as i32` cannot truncate.

## Starter code

Write your code in [`src/practice/p0029_overflow_safe_midpoint.rs`](../src/practice/p0029_overflow_safe_midpoint.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0029
```

```rust
pub fn midpoint(a: i32, b: i32) -> i32 {
    todo!("Write your solution here")
}

pub fn average_floor(nums: &[i32]) -> Option<i32> {
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
    fn midpoint_small_values() {
        assert_eq!(midpoint(1, 2), 1);
        assert_eq!(midpoint(2, 4), 3);
        assert_eq!(midpoint(-3, 0), -2);
        assert_eq!(midpoint(0, -3), -2);
        assert_eq!(midpoint(-4, -4), -4);
    }

    #[test]
    fn midpoint_extremes_do_not_overflow() {
        assert_eq!(midpoint(i32::MAX, i32::MAX), i32::MAX);
        assert_eq!(midpoint(i32::MIN, i32::MIN), i32::MIN);
        assert_eq!(midpoint(i32::MIN, i32::MAX), -1);
        assert_eq!(midpoint(i32::MAX - 1, i32::MAX), i32::MAX - 1);
    }

    #[test]
    fn midpoint_matches_bit_trick() {
        for a in [-7, -1, 0, 1, 6, i32::MIN, i32::MAX, 123_456] {
            for b in [-8, -3, 0, 5, 9, i32::MIN, i32::MAX] {
                assert_eq!(midpoint(a, b), (a & b) + ((a ^ b) >> 1), "a={a}, b={b}");
            }
        }
    }

    #[test]
    fn average_examples() {
        assert_eq!(average_floor(&[1, 2, 4]), Some(2));
        assert_eq!(average_floor(&[-1, -2]), Some(-2));
        assert_eq!(average_floor(&[7]), Some(7));
        assert_eq!(average_floor(&[]), None);
    }

    #[test]
    fn average_of_many_extremes() {
        let big = vec![i32::MAX; 100_000];
        assert_eq!(average_floor(&big), Some(i32::MAX));
        let small = vec![i32::MIN; 100_000];
        assert_eq!(average_floor(&small), Some(i32::MIN));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** widen to `i64`, add, and divide with `div_euclid` (which rounds toward negative
infinity for a positive divisor). For the slice, accumulate an `i64` sum and divide by the
length converted to `i64`.

**Rust concepts:**
- Overflow is about **intermediate** values: `a + b` may overflow even when the final answer is
  small. Choosing a wider type for the computation is the simplest fix.
- `nums.len()` is a `usize`; `nums.len() as i64` converts it (safe here: at most `10^5`).
- An overflow-free trick without widening is `(a & b) + ((a ^ b) >> 1)`: the shared bits plus
  half of the differing bits. `>>` on a signed integer is an arithmetic shift, so this also
  rounds down.

```rust
pub fn midpoint(a: i32, b: i32) -> i32 {
    let sum = i64::from(a) + i64::from(b);
    // The floor of the average is between a and b, so it fits in an i32.
    sum.div_euclid(2) as i32
}

pub fn average_floor(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }
    let mut sum: i64 = 0;
    for &x in nums {
        sum += i64::from(x);
    }
    Some(sum.div_euclid(nums.len() as i64) as i32)
}
```

Full tested solution: [`src/solutions/p0029_overflow_safe_midpoint.rs`](../src/solutions/p0029_overflow_safe_midpoint.rs)

</details>
