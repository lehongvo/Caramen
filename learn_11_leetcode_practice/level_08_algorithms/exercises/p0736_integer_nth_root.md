# Problem 0736: Integer N-th Root

- **Difficulty:** Medium
- **Topics:** binary search on the answer, monotonic predicate, `checked_pow`, `Option::is_some_and`
- **Source:** Original · [`u64::checked_pow`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_pow)

## Problem

Write a function that returns the **integer `n`-th root** of `x`: the largest integer `r` such
that `r^n <= x`. For example the integer cube root of `26` is `2` (`2^3 = 8 <= 26` but
`3^3 = 27 > 26`).

Do not use floating point (`f64` cannot represent every `u64` exactly, so `powf` gives wrong
answers for large inputs). Use **binary search on the answer**: search the range of possible
roots with a test that says whether a candidate is "small enough". Your function must be correct
for every `x` up to `u64::MAX` and must never overflow.

Implement `integer_nth_root(x: u64, n: u32) -> u64`.

## Examples

```text
integer_nth_root(27, 3)         = 3
integer_nth_root(26, 3)         = 2
integer_nth_root(0, 5)          = 0
integer_nth_root(u64::MAX, 1)   = 18446744073709551615
integer_nth_root(u64::MAX, 2)   = 4294967295          (2^32 - 1)
integer_nth_root(10^18, 3)      = 1000000
```

## Constraints

- `0 <= x <= u64::MAX`
- `1 <= n <= 64`

## Hints

1. The test "`r^n <= x`" is `true` for small `r` and `false` from some point on. You are looking
   for the last `r` where it is `true`.
2. `r^n` may not fit in a `u64`. `checked_pow` returns `None` in that case, which also means
   "too big".
3. For `n >= 2` the root is at most `2^32` (because `(2^32)^2 = 2^64` already overflows). That
   bound keeps `hi + 1` from overflowing too.

## Starter code

Write your code in [`src/practice/p0736_integer_nth_root.rs`](../src/practice/p0736_integer_nth_root.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0736
```

```rust
pub fn integer_nth_root(x: u64, n: u32) -> u64 {
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
    fn example_perfect_cube() {
        assert_eq!(integer_nth_root(27, 3), 3);
    }

    #[test]
    fn example_just_below_a_cube() {
        assert_eq!(integer_nth_root(26, 3), 2);
    }

    #[test]
    fn zero_and_one() {
        assert_eq!(integer_nth_root(0, 5), 0);
        assert_eq!(integer_nth_root(1, 64), 1);
        assert_eq!(integer_nth_root(0, 1), 0);
    }

    #[test]
    fn example_largest_inputs() {
        assert_eq!(integer_nth_root(u64::MAX, 1), u64::MAX);
        assert_eq!(integer_nth_root(u64::MAX, 2), 4_294_967_295);
        assert_eq!(integer_nth_root(u64::MAX, 64), 1);
    }

    #[test]
    fn powers_of_two() {
        assert_eq!(integer_nth_root(1 << 63, 63), 2);
        assert_eq!(integer_nth_root(1 << 63, 64), 1);
        assert_eq!(integer_nth_root((1 << 63) - 1, 63), 1);
    }

    #[test]
    fn example_powers_of_ten() {
        assert_eq!(
            integer_nth_root(1_000_000_000_000_000_000, 2),
            1_000_000_000
        );
        assert_eq!(integer_nth_root(1_000_000_000_000_000_000, 3), 1_000_000);
        assert_eq!(integer_nth_root(999_999_999_999_999_999, 3), 999_999);
    }

    #[test]
    fn matches_brute_force_for_small_values() {
        for x in 0..2_000u64 {
            for n in 1..=4u32 {
                let expected = (0..=x).take_while(|r| r.pow(n) <= x).last().unwrap_or(0);
                assert_eq!(integer_nth_root(x, n), expected, "x = {x}, n = {n}");
            }
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** binary search on the answer: we do not search inside an array but over the
candidate answers `0..=bound`, using the monotonic predicate `fits(r) = r^n <= x` (if `r` fits,
every smaller value fits too). The half-open search finds the first `r` where `fits` is
`false`; the answer is the value just before it.

- `fits(r)` is `r.checked_pow(n).is_some_and(|p| p <= x)`: an overflow means `r^n > u64::MAX >= x`.
- For `n == 1` the answer is `x` itself. For `n >= 2`, `fits` is false at `bound + 1` with
  `bound = min(x, 2^32)`, so the search starts with `hi = bound + 1`, which cannot overflow.
  `fits(0)` is always true, so the result `lo - 1` cannot underflow.

The same pattern solves the next three problems: choose the range of possible answers, write a
monotonic "is this answer good enough?" test, and binary search for the boundary.

**Complexity:** `O(log(min(x, 2^32)) * log n)` time (each `checked_pow` uses fast
exponentiation), `O(1)` space.

**Rust concepts:**
- [`checked_pow`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_pow) returns
  `Option<u64>`: `None` on overflow instead of a panic or a wrapped value.
- [`Option::is_some_and`](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some_and)
  tests the value inside `Some` and returns `false` for `None`.
- A closure `let fits = |r: u64| ...` captures `x` and `n` and is reused in the loop.

```rust
pub fn integer_nth_root(x: u64, n: u32) -> u64 {
    if n == 1 {
        return x;
    }
    let fits = |r: u64| r.checked_pow(n).is_some_and(|p| p <= x);
    // For n >= 2: fits(0) is true and fits(hi) is false.
    let (mut lo, mut hi) = (0u64, x.min(1 << 32) + 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if fits(mid) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo - 1 // the last value that fits
}
```

Full tested solution: [`src/solutions/p0736_integer_nth_root.rs`](../src/solutions/p0736_integer_nth_root.rs)

</details>
