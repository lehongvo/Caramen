# Problem 0025: Checked Sum

- **Difficulty:** Easy
- **Topics:** integer overflow, `checked_add`, `Option`, widening to `i64`, iterating a slice by reference
- **Source:** Original · [`i32::checked_add`](https://doc.rust-lang.org/std/primitive.i32.html#method.checked_add)

## Problem

A sensor stores its running total in a 32-bit register. Write two functions that add up a slice
of readings:

1. `checked_sum(nums: &[i32]) -> Option<i32>` adds the readings **from left to right** in an
   `i32`. If any partial sum would leave the `i32` range, it returns `None`; otherwise it returns
   `Some(total)`. The sum of an empty slice is `Some(0)`.
2. `exact_sum(nums: &[i32]) -> i64` returns the true mathematical sum, which never overflows
   because it is accumulated in an `i64`.

## Examples

```text
checked_sum(&[1, 2, 3])                 -> Some(6)
checked_sum(&[])                        -> Some(0)
checked_sum(&[i32::MAX, 1])             -> None
checked_sum(&[i32::MAX, 1, -1])         -> None     the partial sum i32::MAX + 1 already overflows
exact_sum(&[i32::MAX, 1, -1])           -> 2147483647
exact_sum(&[i32::MIN, i32::MIN])        -> -4294967296
```

## Constraints

- `0 <= nums.len() <= 10^5`

## Hints

1. `a.checked_add(b)` returns `Some(a + b)` when the result fits and `None` when it overflows.
2. Use `match` on that `Option` inside a `for` loop and `return None` as soon as it fails.
3. For `exact_sum`, convert each element with `i64::from(x)` before adding.

## Starter code

Write your code in [`src/practice/p0025_checked_sum.rs`](../src/practice/p0025_checked_sum.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0025
```

```rust
pub fn checked_sum(nums: &[i32]) -> Option<i32> {
    todo!("Write your solution here")
}

pub fn exact_sum(nums: &[i32]) -> i64 {
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
    fn small_sums() {
        assert_eq!(checked_sum(&[1, 2, 3]), Some(6));
        assert_eq!(checked_sum(&[-5, 5]), Some(0));
        assert_eq!(exact_sum(&[1, 2, 3]), 6);
    }

    #[test]
    fn empty_slice() {
        assert_eq!(checked_sum(&[]), Some(0));
        assert_eq!(exact_sum(&[]), 0);
    }

    #[test]
    fn overflow_is_detected() {
        assert_eq!(checked_sum(&[i32::MAX, 1]), None);
        assert_eq!(checked_sum(&[i32::MIN, -1]), None);
        assert_eq!(checked_sum(&[i32::MAX, 1, -1]), None);
    }

    #[test]
    fn reaching_the_limits_exactly_is_fine() {
        assert_eq!(checked_sum(&[i32::MAX - 1, 1]), Some(i32::MAX));
        assert_eq!(checked_sum(&[i32::MIN + 1, -1]), Some(i32::MIN));
        assert_eq!(checked_sum(&[i32::MAX, i32::MIN]), Some(-1));
    }

    #[test]
    fn exact_sum_handles_huge_totals() {
        assert_eq!(exact_sum(&[i32::MAX, 1, -1]), 2_147_483_647);
        assert_eq!(exact_sum(&[i32::MIN, i32::MIN]), -4_294_967_296);
        let big = vec![i32::MAX; 100_000];
        assert_eq!(exact_sum(&big), 214_748_364_700_000);
        assert_eq!(checked_sum(&big), None);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** keep a `mut total: i32`. For each element, `match total.checked_add(x)`: on `Some`
store the new total, on `None` return `None` immediately. For the exact sum, widen every element
to `i64`; `10^5` values of at most `2^31` in absolute value add up to less than `2^48`.

**Rust concepts:**
- Plain `+` on integers **panics** on overflow in debug builds (including tests) and wraps in
  release builds. The `checked_*` methods make overflow an explicit, testable case.
- `for &x in nums` iterates over references to the elements (`&i32`) and the `&x` pattern copies
  each value out.
- `return` exits the function early from inside the loop; the final `Some(total)` is the normal
  result. The `?` operator shortens exactly this pattern: `total = total.checked_add(x)?;`
  means "use the value inside `Some`, or return `None` from the function right now" (problem
  0028 uses it).

```rust
pub fn checked_sum(nums: &[i32]) -> Option<i32> {
    let mut total: i32 = 0;
    for &x in nums {
        match total.checked_add(x) {
            Some(value) => total = value,
            None => return None,
        }
    }
    Some(total)
}

pub fn exact_sum(nums: &[i32]) -> i64 {
    let mut total: i64 = 0;
    for &x in nums {
        total += i64::from(x);
    }
    total
}
```

Full tested solution: [`src/solutions/p0025_checked_sum.rs`](../src/solutions/p0025_checked_sum.rs)

</details>
